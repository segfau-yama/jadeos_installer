# JadeOS GUI インストーラー簡易設計

> 注: AI の振る舞い方針や Logos 系のレビュー手順は `SKILL.md` で管理する。  
> この文書は JadeOS インストーラー自体の設計と制限をまとめる。

## 目的

JadeOS の Live CD 上で動作する、設定項目を絞った GUI インストーラーを作成する。

このインストーラーは、NixOS ベースの JadeOS を対象ディスクへインストールすることだけを目的とする。
GitHub 連携、複雑なプロファイル選択、手動パーティション、TUI fallback、root daemon などは初期実装では扱わない。

---

# 設計方針

## 基本方針

* 最初から完成形を作らない
* 設定項目を増やしすぎない
* 画面数を 5 つ以内に抑える
* GitHub 連携は実装しない
* 手動パーティションは実装しない
* profile/module 選択は実装しない
* Dioxus プロジェクトは単一 crate で始める
* `api/` と `gui/` だけに分ける
* まずは `nixos-install` まで到達することを最優先にする

---

# MVP で固定する仕様

## 固定するもの

| 項目              | 固定値              |
| --------------- | ---------------- |
| boot mode       | UEFI             |
| partition table | GPT              |
| filesystem      | ext4             |
| partitioning    | ディスク全消去          |
| swap            | なし               |
| encryption      | なし               |
| desktop         | Niri             |
| locale          | `ja_JP.UTF-8`    |
| timezone        | `Asia/Tokyo`     |
| keyboard        | `jp`             |
| GitHub          | なし               |
| repository      | Live CD 内蔵テンプレート |
| install target  | `/mnt`           |
| config path     | `/mnt/etc/nixos` |

---

# ユーザーが設定する項目

MVP でユーザーが入力・選択する項目は次だけにする。

| 項目            | 内容           |
| ------------- | ------------ |
| hostname      | インストール後のホスト名 |
| username      | 作成する一般ユーザー   |
| password      | ユーザーのパスワード   |
| target disk   | インストール先ディスク  |
| confirm erase | ディスク消去の最終確認  |

つまり、ユーザーが判断する項目は **5 個**に制限する。

---

# 画面構成

## 1. Welcome 画面

### 目的

JadeOS インストーラーの開始画面。

### 表示内容

* JadeOS をインストールする
* このインストーラーは選択したディスクを全消去する
* MVP では UEFI + GPT + ext4 のみ対応
* GitHub 連携、手動パーティション、暗号化は未対応

### 操作

* `Start` ボタンで次へ進む

---

## 2. User 画面

### 目的

インストール後に使うユーザーを作成する。

### 入力項目

* hostname
* username
* password
* password confirmation

### 固定設定

* user は normal user
* `wheel` group に追加
* `networkmanager` group に追加
* shell は bash または zsh のどちらかを固定
* auto login は MVP では無効

### validation

* hostname が空ならエラー
* username が空ならエラー
* password が空ならエラー
* password confirmation が一致しなければエラー

---

## 3. Disk 画面

### 目的

インストール先ディスクを選ぶ。

### 表示内容

* disk 一覧
* disk size
* model
* removable かどうか
* mounted されているか

### 入力項目

* target disk

### 制限

* loop device は表示しない
* zram は表示しない
* optical drive は表示しない
* mounted 中の disk は警告を表示する

### validation

* target disk が未選択ならエラー
* target disk が存在しなければエラー
* `/dev/sda` や `/dev/nvme0n1` のような disk device のみ許可する
* partition device は選ばせない

---

## 4. Summary 画面

### 目的

インストール前の最終確認を行う。

### 表示内容

* hostname
* username
* target disk
* disk erase warning
* fixed settings

  * UEFI
  * GPT
  * ext4
  * no swap
  * no encryption
  * Niri
  * Japanese locale

### 実行予定の処理

```text
1. target disk を全消去
2. EFI partition を作成
3. root partition を作成
4. EFI partition を FAT32 で format
5. root partition を ext4 で format
6. root partition を /mnt に mount
7. EFI partition を /mnt/boot に mount
8. JadeOS 設定を /mnt/etc/nixos に生成
9. nixos-generate-config --root /mnt を実行
10. nixos-install --flake /mnt/etc/nixos#jadeos を実行
11. ユーザーパスワードを設定
```

### 最終確認

次のような明示確認を入れる。

```text
選択したディスクの内容をすべて削除することを理解しました。
```

このチェックが有効でない限り、Install ボタンは押せない。

---

## 5. Install 画面

### 目的

インストールの進行状況とログを表示する。

### 表示内容

* 現在のステップ
* 進捗バー
* 実行中の command
* command output
* error message
* install log

### phase

```text
1. Validate
2. Partition
3. Format
4. Mount
5. Generate config
6. Install system
7. Set password
8. Finish
```

### 完了後

* `Reboot` ボタンを表示する
* インストールログの保存先を表示する

---

# インストール処理

## 実行する主なコマンド

MVP では、内部的に次のような処理に限定する。

```text
sgdisk --zap-all ${target_disk}

sgdisk -n 1:1MiB:+512MiB -t 1:EF00 ${target_disk}
sgdisk -n 2:0:0 -t 2:8300 ${target_disk}

mkfs.fat -F32 ${efi_partition}
mkfs.ext4 -F ${root_partition}

mount ${root_partition} /mnt
mkdir -p /mnt/boot
mount ${efi_partition} /mnt/boot

mkdir -p /mnt/etc/nixos
generate jadeos flake and configuration files

nixos-generate-config --root /mnt
nixos-install --flake /mnt/etc/nixos#jadeos
nixos-enter --root /mnt -c 'passwd ${username}'
```

---

# ディレクトリ構成

## 最小構成

```text
jade-installer/
├── Cargo.toml
├── Dioxus.toml
├── installer.toml
├── src/
│   ├── main.rs
│   ├── app.rs
│   ├── api/
│   │   ├── mod.rs
│   │   ├── config.rs
│   │   ├── disk.rs
│   │   └── install.rs
│   └── gui/
│       ├── mod.rs
│       ├── state.rs
│       └── pages/
│           ├── mod.rs
│           ├── welcome.rs
│           ├── user.rs
│           ├── disk.rs
│           ├── summary.rs
│           └── install.rs
└── tests/
    ├── config_test.rs
    ├── install_plan_test.rs
    └── fixtures/
        └── lsblk.json
```

---

# 各ファイルの責務

## `src/app.rs`

Dioxus の root component を定義する。

### 責務

* 現在の画面を表示する
* 画面遷移を管理する
* `InstallerState` を保持する

---

## `src/gui/state.rs`

GUI 全体で共有する状態を定義する。

### 含める型

```text
InstallerState
InstallerStep
InstallerConfig
```

### `InstallerConfig` の項目

```text
hostname
username
target_disk
disk_erase_confirmed
```

password は `installer.toml` に保存しない。
メモリ上の `InstallerState` にのみ保持する。

---

## `src/api/config.rs`

`installer.toml` の保存と読み込みを担当する。

### 責務

* `InstallerConfig` を TOML に保存する
* `InstallerConfig` を TOML から読み込む
* password は保存しない

---

## `src/api/disk.rs`

ディスク情報の取得を担当する。

### 責務

* `lsblk` を実行する
* disk 一覧を返す
* loop / zram / rom を除外する
* partition ではなく disk device だけを返す

---

## `src/api/install.rs`

インストール処理を担当する。

### 責務

* validation
* install plan 生成
* command 実行
* log 収集
* phase 更新

### 制限

GUI から直接 `sgdisk` や `nixos-install` を呼ばない。
必ず `api/install.rs` の関数を経由する。

---

# `installer.toml`

## MVP 版

保存する設定は最小限にする。

```toml
schema_version = 1

hostname = "jadeos"
username = "jade"
target_disk = "/dev/nvme0n1"
disk_erase_confirmed = false
```

## 保存しないもの

```text
password
password confirmation
GitHub token
LUKS passphrase
sudo password
root password
```

---

# Live CD 設計

## 目的

JadeOS インストーラーを起動できる NixOS Live CD を作る。

## 方針

Live CD には JadeOS 本体のすべてを入れない。
インストーラーを起動するための最小環境だけを入れる。

## 含めるもの

```text
jade-installer
nixos-install
nixos-generate-config
git
NetworkManager
sgdisk
parted
dosfstools
e2fsprogs
webkitgtk
xdotool
```

## 含めないもの

```text
GitHub 自動連携
開発用 IDE
ゲーム環境
CAD
複雑な desktop 環境
JadeOS 本体の全パッケージ
```

---

# NixOS 設定生成

## 方針

JadeOS の設定は複雑に分割しない。

MVP では `/mnt/etc/nixos` に直接生成する。

```text
/mnt/etc/nixos/
├── flake.nix
├── configuration.nix
└── hardware-configuration.nix
```

host 別ディレクトリはまだ作らない。

## 理由

初期段階で `hosts/${hostname}` 構成を作ると、flake attr、host 名、path、hardware-configuration の配置が増えて複雑になるため。

MVP では flake attr を固定する。

```text
nixosConfigurations.jadeos
```

そのため install command も固定する。

```text
nixos-install --flake /mnt/etc/nixos#jadeos
```

---

# テスト方針

## 最初に作る test

```text
1. installer.toml を保存・読み込みできる
2. password が installer.toml に保存されない
3. 空 hostname を弾く
4. 空 username を弾く
5. 空 target_disk を弾く
6. disk erase 未確認なら install plan を作らない
7. install plan に destructive action が含まれる
8. install plan に nixos-generate-config が含まれる
9. install plan に nixos-install --flake /mnt/etc/nixos#jadeos が含まれる
10. lsblk JSON から disk 一覧を parse できる
```

## まだ作らない test

```text
Dioxus E2E
GitHub API
実ディスク partition
実際の nixos-install
VM boot test
TUI fallback test
```

---

# 明示的に後回しにする機能

## v0.1 では作らない

```text
GitHub login
GitHub fork
GitHub push
既存 repository 選択
既存 host 復元
profile 選択
module 選択
manual partitioning
dual boot
btrfs
LUKS
swap
/home 分離
/nix 分離
TUI fallback
root daemon
複数 crate workspace
```

---

# v0.2 以降で検討する機能

v0.1 が実際に動作してから、必要なものだけ追加する。

候補:

```text
profile 選択
GPU 選択
locale 選択
keyboard 選択
swap file
GitHub 連携
TUI fallback
root daemon
btrfs
LUKS
/home 分離
```

追加順序は、利用頻度と実装コストで判断する。
最初から roadmap を固定しない。

---

# 設計の制限ルール

このプロジェクトでは、複雑化を防ぐために次の制限を置く。

## 画面数の制限

v0.1 では画面数を 5 つまでにする。

```text
Welcome
User
Disk
Summary
Install
```

## 設定項目の制限

v0.1 でユーザーが直接設定する項目は 5 つまでにする。

```text
hostname
username
password
target_disk
disk erase confirmation
```

## ディレクトリ分割の制限

v0.1 では `src/api` と `src/gui` だけにする。

```text
src/api
src/gui
```

`model/`、`util/`、`crates/` は作らない。

## 機能追加の条件

新しい機能は、次の条件を満たすまで追加しない。

```text
1. v0.1 のインストール導線が最後まで動く
2. installer.toml の保存・読み込み test が通る
3. install plan test が通る
4. 実機または VM で最低 1 回インストールに成功する
```

---

# 最終 MVP

JadeOS GUI インストーラー v0.1 の成功条件は次の通り。

```text
1. Live CD が起動する
2. Dioxus GUI が起動する
3. hostname / username / password を入力できる
4. target disk を選択できる
5. Summary でディスク消去を確認できる
6. install を開始できる
7. /mnt/etc/nixos に設定が生成される
8. nixos-generate-config --root /mnt が成功する
9. nixos-install --flake /mnt/etc/nixos#jadeos が成功する
10. 再起動後、JadeOS にログインできる
```
