use serde::Deserialize;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DiskInfo {
    pub name: String,
    pub path: String,
    pub size: String,
    pub model: String,
    pub removable: bool,
    pub mounted: bool,
}

#[derive(Debug, Deserialize)]
struct LsblkOutput {
    blockdevices: Vec<LsblkDevice>,
}

#[derive(Debug, Deserialize)]
struct LsblkDevice {
    name: String,
    #[serde(rename = "type")]
    device_type: String,
    size: Option<String>,
    model: Option<String>,
    rm: Option<bool>,
    mountpoint: Option<String>,
    mountpoints: Option<Vec<Option<String>>>,
}

pub fn parse_lsblk_json(json: &str) -> Result<Vec<DiskInfo>, serde_json::Error> {
    let output: LsblkOutput = serde_json::from_str(json)?;

    let disks = output
        .blockdevices
        .into_iter()
        .filter(|device| device.device_type == "disk")
        .filter(|device| !device.name.starts_with("loop"))
        .filter(|device| !device.name.starts_with("zram"))
        .filter(|device| !device.name.starts_with("sr"))
        .map(|device| {
            let mounted = device
                .mountpoint
                .as_deref()
                .is_some_and(|mountpoint| !mountpoint.is_empty())
                || device.mountpoints.as_ref().is_some_and(|mountpoints| {
                    mountpoints.iter().any(|mountpoint| {
                        mountpoint.as_deref().is_some_and(|value| !value.is_empty())
                    })
                });

            DiskInfo {
                path: format!("/dev/{}", device.name),
                name: device.name,
                size: device.size.unwrap_or_else(|| "unknown".to_string()),
                model: device.model.unwrap_or_else(|| "unknown".to_string()),
                removable: device.rm.unwrap_or(false),
                mounted,
            }
        })
        .collect();

    Ok(disks)
}

pub fn is_valid_target_disk(path: &str) -> bool {
    if let Some(rest) = path.strip_prefix("/dev/") {
        let is_sata = rest.starts_with("sd") && rest.len() == 3;
        let is_virtio = rest.starts_with("vd") && rest.len() == 3;
        let is_xen = rest.starts_with("xvd") && rest.len() == 4;
        let is_nvme = rest
            .strip_prefix("nvme")
            .is_some_and(|value| value.contains('n') && !value.contains('p'));

        return is_sata || is_virtio || is_xen || is_nvme;
    }

    false
}
