#[cfg(feature = "desktop")]
fn main() {
    use dioxus::desktop::{tao::window::Fullscreen, Config, WindowBuilder};
    use dioxus::LaunchBuilder;
    use jade_installer::app::app;

    let window = WindowBuilder::new()
        .with_title("JadeOS Installer")
        .with_decorations(false)
        .with_fullscreen(Some(Fullscreen::Borderless(None)));

    LaunchBuilder::desktop()
        .with_cfg(Config::new().with_window(window))
        .launch(app);
}

#[cfg(all(feature = "web", not(feature = "desktop")))]
fn main() {
    use jade_installer::app::app;

    dioxus::LaunchBuilder::web().launch(app);
}

#[cfg(not(any(feature = "desktop", feature = "web")))]
fn main() {
    panic!("Enable the `desktop` or `web` feature to launch jade-installer.");
}
