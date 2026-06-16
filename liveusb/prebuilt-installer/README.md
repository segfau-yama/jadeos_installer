このディレクトリには LiveUSB に直接同梱するビルド済み installer を置く。

必要な構成:

```text
prebuilt-installer/
  jade-installer
  assets/
  lib/
```

`lib/` は、ホスト環境でビルドした ELF が Nixpkgs の ABI と一致しない共有ライブラリを同梱するための置き場として使う。

更新手順の例:

```bash
cd instller
cargo build --release
cd ..
cp instller/target/release/jade-installer liveusb/prebuilt-installer/jade-installer
cp -r instller/assets liveusb/prebuilt-installer/assets
cp /usr/lib/libxdo.so.4 liveusb/prebuilt-installer/lib/
```
