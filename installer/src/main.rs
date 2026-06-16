use jade_installer::app::app;

fn main() {
    use dioxus::desktop::{tao::window::Fullscreen, Config, WindowBuilder};
    use dioxus::LaunchBuilder;

    let window = WindowBuilder::new()
        .with_title("JadeOS Installer")
        .with_decorations(false)
        .with_fullscreen(Some(Fullscreen::Borderless(None)));

    LaunchBuilder::desktop()
        .with_cfg(Config::new().with_window(window))
        .launch(app);
}
