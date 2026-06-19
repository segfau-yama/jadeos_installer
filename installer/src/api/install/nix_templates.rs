const HOST_FLAKE_PARTS_TEMPLATE: &str = include_str!("templates/host-flake-parts.nix");
const HOST_CONFIGURATION_TEMPLATE: &str = include_str!("templates/host-configuration.nix");
const GUI_USER_TEMPLATE: &str = include_str!("templates/gui-user.nix");
const INSTALL_ARGS_TEMPLATE: &str = include_str!("templates/install-args.nix");
const HARDWARE_CONFIGURATION_TEMPLATE: &str = include_str!("templates/hardware-configuration.nix");

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct HostTemplateContext<'a> {
    pub system_arch: &'a str,
    pub hostname: &'a str,
    pub username: &'a str,
    pub detected_cpu: &'a str,
    pub detected_gpu: &'a str,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct InstallArgsContext<'a> {
    pub efi_partition: &'a str,
    pub root_partition: &'a str,
}

pub fn render_host_flake_parts(context: &HostTemplateContext<'_>) -> Result<String, String> {
    render_template(
        "host-flake-parts.nix",
        HOST_FLAKE_PARTS_TEMPLATE,
        &[
            ("__JADE_SYSTEM_ARCH__", nix_string(context.system_arch)),
            ("__JADE_HOSTNAME__", nix_string(context.hostname)),
        ],
    )
}

pub fn render_host_configuration(context: &HostTemplateContext<'_>) -> Result<String, String> {
    render_template(
        "host-configuration.nix",
        HOST_CONFIGURATION_TEMPLATE,
        &[
            ("__JADE_HOSTNAME_ATTR__", nix_attr(context.hostname)),
            ("__JADE_HOSTNAME__", nix_string(context.hostname)),
            ("__JADE_GPU__", nix_string(context.detected_gpu)),
            ("__JADE_CPU__", nix_string(context.detected_cpu)),
            ("__JADE_USERNAME_ATTR__", nix_attr(context.username)),
            (
                "__JADE_HARDWARE_PATH__",
                nix_string(&format!(
                    "${{inputs.self}}/nixos/{}/hardware-configuration.nix",
                    context.hostname
                )),
            ),
        ],
    )
}

pub fn render_gui_user_configuration(context: &HostTemplateContext<'_>) -> Result<String, String> {
    render_template(
        "gui-user.nix",
        GUI_USER_TEMPLATE,
        &[("__JADE_USERNAME__", nix_string(context.username))],
    )
}

pub fn render_install_args(context: &InstallArgsContext<'_>) -> Result<String, String> {
    render_template(
        "install-args.nix",
        INSTALL_ARGS_TEMPLATE,
        &[
            ("__JADE_BOOT_PARTITION__", nix_string(context.efi_partition)),
            (
                "__JADE_ROOT_PARTITION__",
                nix_string(context.root_partition),
            ),
        ],
    )
}

pub fn render_hardware_configuration(generated_hardware_output: &str) -> Result<String, String> {
    render_template(
        "hardware-configuration.nix",
        HARDWARE_CONFIGURATION_TEMPLATE,
        &[(
            "__JADE_GENERATED_HARDWARE_MODULE__",
            format!("{};", indent_block(generated_hardware_output.trim_end(), 4)),
        )],
    )
}

fn render_template(
    template_name: &str,
    template: &str,
    replacements: &[(&str, String)],
) -> Result<String, String> {
    let mut rendered = template.to_string();

    for (placeholder, value) in replacements {
        if !rendered.contains(placeholder) {
            return Err(format!(
                "template {template_name} is missing placeholder {placeholder}"
            ));
        }
        rendered = rendered.replace(placeholder, value);
    }

    if rendered.contains("__JADE_") {
        return Err(format!(
            "template {template_name} still contains unresolved placeholders"
        ));
    }

    Ok(rendered)
}

fn indent_block(content: &str, spaces: usize) -> String {
    let padding = " ".repeat(spaces);
    content
        .lines()
        .map(|line| {
            if line.is_empty() {
                String::new()
            } else {
                format!("{padding}{line}")
            }
        })
        .collect::<Vec<_>>()
        .join("\n")
}

fn nix_attr(value: &str) -> String {
    nix_string(value)
}

fn nix_string(value: &str) -> String {
    format!("\"{}\"", escape_nix_string(value))
}

fn escape_nix_string(value: &str) -> String {
    value
        .replace('\\', "\\\\")
        .replace('"', "\\\"")
        .replace('\n', "\\n")
        .replace('\r', "\\r")
        .replace('\t', "\\t")
        .replace("${", "\\${")
}

#[cfg(test)]
mod tests {
    use super::{
        render_gui_user_configuration, render_hardware_configuration, render_host_configuration,
        render_host_flake_parts, render_install_args, HostTemplateContext, InstallArgsContext,
    };

    #[test]
    fn generated_host_flake_uses_hostname() {
        let context = HostTemplateContext {
            system_arch: "x86_64-linux",
            hostname: "jadeos",
            username: "jade",
            detected_cpu: "amd",
            detected_gpu: "none",
        };

        let content = render_host_flake_parts(&context).expect("template should render");

        assert!(content.contains("mkNixos \"x86_64-linux\" \"jadeos\""));
    }

    #[test]
    fn generated_host_configuration_matches_repo_layout() {
        let context = HostTemplateContext {
            system_arch: "x86_64-linux",
            hostname: "jadeos",
            username: "jade",
            detected_cpu: "intel",
            detected_gpu: "amd",
        };

        let content = render_host_configuration(&context).expect("template should render");

        assert!(content.contains("flake.modules.nixos.\"jadeos\""));
        assert!(content.contains("system-base"));
        assert!(content.contains("desktop"));
        assert!(content.contains("inputs.self.modules.nixos.\"jade\""));
        assert!(content.contains("\"\\${inputs.self}/nixos/jadeos/hardware-configuration.nix\""));
        assert!(content.contains("networking.hostName = \"jadeos\";"));
        assert!(content.contains("my.hardware.cpu = lib.mkDefault \"intel\";"));
        assert!(content.contains("my.hardware.gpu = lib.mkDefault \"amd\";"));
        assert!(!content.contains("hyprland"));
    }

    #[test]
    fn generated_gui_user_configuration_registers_home_manager_profile() {
        let context = HostTemplateContext {
            system_arch: "x86_64-linux",
            hostname: "jadeos",
            username: "jade",
            detected_cpu: "intel",
            detected_gpu: "amd",
        };

        let content = render_gui_user_configuration(&context).expect("template should render");

        assert!(content.contains("flake.modules.nixos.\"${username}\""));
        assert!(content.contains("users.users.\"${username}\""));
        assert!(content.contains("home-manager.users.\"${username}\""));
        assert!(content.contains("inputs.self.modules.homeManager.\"${username}\""));
        assert!(content.contains("imports = with inputs.self.modules.homeManager; ["));
        assert!(content.contains("desktop"));
    }

    #[test]
    fn generated_install_args_are_bound_to_selected_partitions() {
        let context = InstallArgsContext {
            efi_partition: "/dev/nvme0n1p1",
            root_partition: "/dev/nvme0n1p2",
        };

        let content = render_install_args(&context).expect("template should render");

        assert!(content.contains("boot = \"/dev/nvme0n1p1\";"));
        assert!(content.contains("root = \"/dev/nvme0n1p2\";"));
        assert!(content.contains("swap = \"\";"));
    }

    #[test]
    fn generated_hardware_configuration_wraps_detected_output() {
        let content = render_hardware_configuration(
            "{ lib, modulesPath, ... }:\n{\n  imports = [ (modulesPath + \"/installer/scan/not-detected.nix\") ];\n}",
        )
        .expect("template should render");

        assert!(content.contains("generatedHardwareModule ="));
        assert!(content.contains("fileSystems.\"/boot\" = lib.mkForce"));
        assert!(content.contains("boot.loader.systemd-boot.enable = true;"));
        assert!(!content.contains("config.my.installDisk"));
    }
}
