use std::error::Error;
use std::fmt::{Display, Formatter};
use std::process::Command;

use serde::Deserialize;

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
    Io(std::io::Error),
    CommandFailed(String),
    Parse(serde_json::Error),
}

impl Display for DiskError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(error) => write!(f, "failed to inspect block devices: {error}"),
            Self::CommandFailed(message) => {
                write!(f, "lsblk did not complete successfully: {message}")
            }
            Self::Parse(error) => write!(f, "failed to parse lsblk JSON: {error}"),
        }
    }
}

impl Error for DiskError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Io(error) => Some(error),
            Self::Parse(error) => Some(error),
            Self::CommandFailed(_) => None,
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
    rm: Option<u8>,
    #[serde(rename = "type")]
    device_type: String,
    #[serde(default)]
    mountpoints: Vec<Option<String>>,
}

pub fn list_disks() -> Result<Vec<DiskDeviceInfo>, DiskError> {
    let output = Command::new("lsblk")
        .args([
            "--json",
            "-b",
            "-o",
            "NAME,PATH,SIZE,MODEL,RM,TYPE,MOUNTPOINTS",
        ])
        .output()
        .map_err(DiskError::Io)?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr).trim().to_string();
        return Err(DiskError::CommandFailed(stderr));
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    parse_lsblk_output(&stdout)
}

pub fn parse_lsblk_output(stdout: &str) -> Result<Vec<DiskDeviceInfo>, DiskError> {
    let parsed: LsblkResponse = serde_json::from_str(stdout).map_err(DiskError::Parse)?;

    let mut disks = parsed
        .blockdevices
        .into_iter()
        .filter(|device| device.device_type == "disk")
        .filter_map(|device| {
            let name = device.name;
            let path = match device.path {
                Some(path) => path,
                None => format!("/dev/{name}"),
            };

            if should_skip_device(&name) || !is_disk_device_path(&path) {
                return None;
            }

            Some(DiskDeviceInfo {
                name,
                path,
                size_bytes: device.size.unwrap_or_default(),
                model: device
                    .model
                    .unwrap_or_else(|| "Unknown model".to_string())
                    .trim()
                    .to_string(),
                removable: device.rm.unwrap_or_default() != 0,
                mounted: device
                    .mountpoints
                    .iter()
                    .flatten()
                    .any(|mountpoint| !mountpoint.trim().is_empty()),
            })
        })
        .collect::<Vec<_>>();

    disks.sort_by(|left, right| left.path.cmp(&right.path));
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

    if let Some(rest) = name.strip_prefix("nvme") {
        return matches_nvme_disk(rest);
    }

    if let Some(rest) = name.strip_prefix("mmcblk") {
        return !rest.is_empty() && rest.chars().all(|character| character.is_ascii_digit());
    }

    if let Some(rest) = name.strip_prefix("xvd") {
        return matches_alpha_suffix(rest);
    }

    if let Some(rest) = name
        .strip_prefix("sd")
        .or_else(|| name.strip_prefix("vd"))
        .or_else(|| name.strip_prefix("hd"))
    {
        return matches_alpha_suffix(rest);
    }

    false
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

fn matches_alpha_suffix(value: &str) -> bool {
    !value.is_empty()
        && value
            .chars()
            .all(|character| character.is_ascii_lowercase())
}
