use std::error::Error;
use std::fmt::{Display, Formatter};

use serde::de::{Deserializer, Error as DeError};
use serde::Deserialize;

#[cfg(not(all(feature = "web", not(feature = "desktop"))))]
use crate::api::command::LSBLK_COMMAND;
#[cfg(not(all(feature = "web", not(feature = "desktop"))))]
use crate::api::execute::{CommandError, CommandExecutor};

const DEFAULT_MODEL: &str = "Unknown model";

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DiskDeviceInfo {
    pub name: String,
    pub path: String,
    pub size_bytes: u64,
    pub model: String,
    pub removable: bool,
    pub mounted: bool,
}

impl DiskDeviceInfo {
    pub fn size_gib(&self) -> f64 {
        self.size_bytes as f64 / 1024.0 / 1024.0 / 1024.0
    }
}

#[derive(Debug)]
pub enum DiskError {
    #[cfg(not(all(feature = "web", not(feature = "desktop"))))]
    Command(CommandError),
    Parse(serde_json::Error),
}

impl Display for DiskError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            #[cfg(not(all(feature = "web", not(feature = "desktop"))))]
            Self::Command(error) => write!(f, "failed to inspect block devices: {error}"),
            Self::Parse(error) => write!(f, "failed to parse lsblk JSON: {error}"),
        }
    }
}

impl Error for DiskError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            #[cfg(not(all(feature = "web", not(feature = "desktop"))))]
            Self::Command(error) => Some(error),
            Self::Parse(error) => Some(error),
        }
    }
}

#[derive(Debug, Deserialize)]
struct LsblkResponse {
    #[serde(default)]
    blockdevices: Vec<LsblkDevice>,
}

#[derive(Debug, Deserialize)]
struct LsblkDevice {
    name: String,
    path: Option<String>,
    size: Option<u64>,
    model: Option<String>,
    #[serde(default, deserialize_with = "deserialize_removable")]
    rm: Option<bool>,
    #[serde(rename = "type")]
    device_type: String,
    #[serde(default)]
    mountpoints: Vec<Option<String>>,
}

impl LsblkDevice {
    fn into_disk_info(self) -> Option<DiskDeviceInfo> {
        if self.device_type != "disk" {
            return None;
        }

        let path = self.path.unwrap_or_else(|| format!("/dev/{}", self.name));
        if should_skip_device(&self.name) || !is_disk_device_path(&path) {
            return None;
        }

        Some(DiskDeviceInfo {
            name: self.name,
            path,
            size_bytes: self.size.unwrap_or_default(),
            model: normalize_model(self.model),
            removable: self.rm.unwrap_or(false),
            mounted: has_mountpoint(&self.mountpoints),
        })
    }
}

#[cfg(all(feature = "web", not(feature = "desktop")))]
pub fn list_disks() -> Result<Vec<DiskDeviceInfo>, DiskError> {
    crate::web_api::disk::list_disks()
}

#[cfg(not(all(feature = "web", not(feature = "desktop"))))]
pub fn list_disks() -> Result<Vec<DiskDeviceInfo>, DiskError> {
    let mut executor = CommandExecutor::new();
    executor.set_strategy(&LSBLK_COMMAND);

    let stdout = executor
        .execute(&[
            "--json",
            "-b",
            "-o",
            "NAME,PATH,SIZE,MODEL,RM,TYPE,MOUNTPOINTS",
        ])
        .map_err(DiskError::Command)?;

    parse_lsblk_output(&stdout)
}

pub fn parse_lsblk_output(stdout: &str) -> Result<Vec<DiskDeviceInfo>, DiskError> {
    let parsed: LsblkResponse = serde_json::from_str(stdout).map_err(DiskError::Parse)?;

    let mut disks = parsed
        .blockdevices
        .into_iter()
        .filter_map(LsblkDevice::into_disk_info)
        .collect::<Vec<_>>();

    disks.sort_unstable_by(|left, right| left.path.cmp(&right.path));
    Ok(disks)
}

pub fn is_disk_device_path(path: &str) -> bool {
    let Some(name) = path.strip_prefix("/dev/") else {
        return false;
    };

    is_whole_disk_name(name)
}

fn should_skip_device(name: &str) -> bool {
    name.starts_with("loop") || name.starts_with("zram") || name.starts_with("sr")
}

fn is_whole_disk_name(name: &str) -> bool {
    if name.is_empty() || should_skip_device(name) {
        return false;
    }

    matches_prefix(name, "nvme", matches_nvme_disk)
        || matches_prefix(name, "mmcblk", matches_numeric_suffix)
        || ["xvd", "sd", "vd", "hd"]
            .into_iter()
            .any(|prefix| matches_prefix(name, prefix, matches_alpha_suffix))
}

fn matches_nvme_disk(rest: &str) -> bool {
    let Some((controller, namespace)) = rest.split_once('n') else {
        return false;
    };

    !controller.is_empty()
        && !namespace.is_empty()
        && controller
            .chars()
            .all(|character| character.is_ascii_digit())
        && namespace
            .chars()
            .all(|character| character.is_ascii_digit())
}

fn matches_prefix(name: &str, prefix: &str, matcher: fn(&str) -> bool) -> bool {
    name.strip_prefix(prefix).is_some_and(matcher)
}

fn matches_numeric_suffix(value: &str) -> bool {
    !value.is_empty() && value.chars().all(|character| character.is_ascii_digit())
}

fn matches_alpha_suffix(value: &str) -> bool {
    !value.is_empty()
        && value
            .chars()
            .all(|character| character.is_ascii_lowercase())
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
enum RemovableValue {
    Bool(bool),
    Number(u8),
    String(String),
}

impl RemovableValue {
    fn into_bool<E: DeError>(self) -> Result<bool, E> {
        match self {
            Self::Bool(removable) => Ok(removable),
            Self::Number(removable) => Ok(removable != 0),
            Self::String(removable) => match removable.trim() {
                "0" | "false" => Ok(false),
                "1" | "true" => Ok(true),
                other => Err(E::custom(format!(
                    "unsupported lsblk removable value: {other}"
                ))),
            },
        }
    }
}

fn deserialize_removable<'de, D>(deserializer: D) -> Result<Option<bool>, D::Error>
where
    D: Deserializer<'de>,
{
    let value = Option::<RemovableValue>::deserialize(deserializer)?;

    value.map(RemovableValue::into_bool::<D::Error>).transpose()
}

fn normalize_model(model: Option<String>) -> String {
    model
        .unwrap_or_else(|| DEFAULT_MODEL.to_string())
        .trim()
        .to_string()
}

fn has_mountpoint(mountpoints: &[Option<String>]) -> bool {
    mountpoints
        .iter()
        .flatten()
        .any(|mountpoint| !mountpoint.trim().is_empty())
}

#[cfg(test)]
mod tests {
    use super::is_disk_device_path;

    #[test]
    fn whole_disk_paths_are_accepted() {
        for path in [
            "/dev/sda",
            "/dev/vda",
            "/dev/hda",
            "/dev/xvda",
            "/dev/nvme0n1",
            "/dev/mmcblk0",
        ] {
            assert!(is_disk_device_path(path), "{path} should be accepted");
        }
    }

    #[test]
    fn partitions_and_virtual_devices_are_rejected() {
        for path in [
            "/dev/sda1",
            "/dev/nvme0n1p1",
            "/dev/mmcblk0p1",
            "/dev/loop0",
            "/dev/zram0",
            "/dev/sr0",
            "/tmp/not-a-disk",
        ] {
            assert!(!is_disk_device_path(path), "{path} should be rejected");
        }
    }
}
