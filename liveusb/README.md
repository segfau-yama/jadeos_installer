# JadeOS LiveUSB

`liveusb/` には、JadeOS GUI installer を同梱した NixOS Live ISO の定義を置く。

## 含まれるもの

- `liveusb/prebuilt-installer/` からそのまま同梱する `jade-installer` バイナリと assets
- `greetd + cage` による自動ログインと kiosk 起動
- installer が前提とする最低限の CLI (`lsblk`, `parted`, `mkfs.fat`, `mkfs.ext4` など)
- ISO 内へ同梱するテンプレート置き場 (`/etc/jadeos-installer/templates`)

## prebuilt installer の更新

`liveusb/prebuilt-installer/` には、ISO に埋め込むビルド済みバイナリを置く。

```text
liveusb/prebuilt-installer/
  jade-installer
  assets/
  lib/
```

通常は `instller/` を更新したあとに release build を作り、その成果物をここへコピーする。ホスト環境の ABI と Nixpkgs が揃わない共有ライブラリがある場合は、`lib/` にその `.so` も同梱する。

## ビルド

```bash
cd liveusb
nix build .#iso
```

生成物:

```text
result/iso/jadeos-installer-*.iso
```

## 起動確認

```bash
qemu-system-x86_64 \
  -enable-kvm \
  -m 4096 \
  -cdrom result/iso/*.iso
```

起動後は `installer` ユーザーで自動ログインし、`cage` 上で `jade-installer` が起動する。
