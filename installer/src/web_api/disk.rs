use crate::api::disk::{DiskDeviceInfo, DiskError};

const GIB: u64 = 1024 * 1024 * 1024;

pub fn list_disks() -> Result<Vec<DiskDeviceInfo>, DiskError> {
    Ok(vec![
        DiskDeviceInfo {
            name: "nvme0n1".to_string(),
            path: "/dev/nvme0n1".to_string(),
            size_bytes: 512 * GIB,
            model: "JadeOS Web Demo NVMe".to_string(),
            removable: false,
            mounted: false,
        },
        DiskDeviceInfo {
            name: "sda".to_string(),
            path: "/dev/sda".to_string(),
            size_bytes: 256 * GIB,
            model: "USB-C Portable SSD".to_string(),
            removable: true,
            mounted: false,
        },
    ])
}
