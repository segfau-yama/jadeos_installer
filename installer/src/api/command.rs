pub trait CommandStrategy {
    fn command_name(&self) -> &'static str;
}

macro_rules! define_commands {
    ($(($command_type:ident, $command_static:ident, $command_name:literal)),+ $(,)?) => {
        $(
            #[derive(Debug, Clone, Copy, Default)]
            pub struct $command_type;

            impl CommandStrategy for $command_type {
                fn command_name(&self) -> &'static str {
                    $command_name
                }
            }

            pub static $command_static: $command_type = $command_type;
        )+
    };
}

define_commands!(
    (LsblkCommand, LSBLK_COMMAND, "lsblk"),
    (GitCommand, GIT_COMMAND, "git"),
    (PartedCommand, PARTED_COMMAND, "parted"),
    (MkdirCommand, MKDIR_COMMAND, "mkdir"),
    (MountCommand, MOUNT_COMMAND, "mount"),
    (UmountCommand, UMOUNT_COMMAND, "umount"),
    (RmCommand, RM_COMMAND, "rm"),
    (MkfsFatCommand, MKFS_FAT_COMMAND, "mkfs.fat"),
    (MkfsExt4Command, MKFS_EXT4_COMMAND, "mkfs.ext4"),
    (
        NixosGenerateConfigCommand,
        NIXOS_GENERATE_CONFIG_COMMAND,
        "nixos-generate-config"
    ),
    (NixosInstallCommand, NIXOS_INSTALL_COMMAND, "nixos-install"),
    (NixosEnterCommand, NIXOS_ENTER_COMMAND, "nixos-enter"),
    (TeeCommand, TEE_COMMAND, "tee"),
    (TrueCommand, TRUE_COMMAND, "true"),
);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn command_types_expose_expected_command_names() {
        for (strategy, command_name) in [
            (&LSBLK_COMMAND as &dyn CommandStrategy, "lsblk"),
            (&GIT_COMMAND as &dyn CommandStrategy, "git"),
            (&PARTED_COMMAND as &dyn CommandStrategy, "parted"),
            (&MKDIR_COMMAND as &dyn CommandStrategy, "mkdir"),
            (&MOUNT_COMMAND as &dyn CommandStrategy, "mount"),
            (&UMOUNT_COMMAND as &dyn CommandStrategy, "umount"),
            (&RM_COMMAND as &dyn CommandStrategy, "rm"),
            (&MKFS_FAT_COMMAND as &dyn CommandStrategy, "mkfs.fat"),
            (&MKFS_EXT4_COMMAND as &dyn CommandStrategy, "mkfs.ext4"),
            (
                &NIXOS_GENERATE_CONFIG_COMMAND as &dyn CommandStrategy,
                "nixos-generate-config",
            ),
            (
                &NIXOS_INSTALL_COMMAND as &dyn CommandStrategy,
                "nixos-install",
            ),
            (&NIXOS_ENTER_COMMAND as &dyn CommandStrategy, "nixos-enter"),
            (&TEE_COMMAND as &dyn CommandStrategy, "tee"),
            (&TRUE_COMMAND as &dyn CommandStrategy, "true"),
        ] {
            assert_eq!(strategy.command_name(), command_name);
        }
    }
}
