use jade_installer::api::disk::parse_lsblk_json;

#[test]
fn parse_lsblk_filters_non_installable_devices() {
    let fixture = std::fs::read_to_string("tests/fixtures/lsblk.json").expect("read fixture");
    let disks = parse_lsblk_json(&fixture).expect("parse fixture");

    let paths: Vec<&str> = disks.iter().map(|disk| disk.path.as_str()).collect();
    assert_eq!(paths, vec!["/dev/nvme0n1", "/dev/sda"]);

    let mounted_disk = disks
        .iter()
        .find(|disk| disk.path == "/dev/sda")
        .expect("mounted disk present");
    assert!(mounted_disk.mounted);
}
