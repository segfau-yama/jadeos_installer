{
  lib,
  stdenvNoCC,
  autoPatchelfHook,
  wrapGAppsHook,
  gtk3,
  webkitgtk_4_1,
  libsoup_3,
  openssl,
  glib,
  gdk-pixbuf,
  cairo,
  pango,
  atk,
  libayatana-appindicator,
  libX11,
  libXinerama,
  libXtst,
  libxkbcommon,
}:
stdenvNoCC.mkDerivation (finalAttrs: {
  pname = "jade-installer";
  version = "0.1.0-prebuilt";

  src = ./prebuilt-installer;
  dontUnpack = true;

  nativeBuildInputs = [
    autoPatchelfHook
    wrapGAppsHook
  ];

  buildInputs = [
    atk
    cairo
    gdk-pixbuf
    glib
    gtk3
    libayatana-appindicator
    libX11
    libXinerama
    libXtst
    libxkbcommon
    libsoup_3
    openssl
    pango
    webkitgtk_4_1
  ];

  installPhase = ''
    runHook preInstall

    mkdir -p "$out/bin" "$out/lib/${finalAttrs.pname}" "$out/lib/${finalAttrs.pname}/vendor-lib"
    cp "$src/jade-installer" "$out/bin/jade-installer"
    chmod +x "$out/bin/jade-installer"
    cp -r "$src/assets" "$out/lib/${finalAttrs.pname}/assets"
    if [ -d "$src/lib" ]; then
      cp -r "$src/lib/." "$out/lib/${finalAttrs.pname}/vendor-lib/"
      addAutoPatchelfSearchPath "$out/lib/${finalAttrs.pname}/vendor-lib"
    fi

    runHook postInstall
  '';

  preFixup = ''
    gappsWrapperArgs+=(
      --set DIOXUS_ASSET_ROOT "$out/lib/${finalAttrs.pname}"
      --set WEBKIT_DISABLE_DMABUF_RENDERER 1
      --set-default GDK_BACKEND wayland,x11
    )
  '';

  meta = with lib; {
    description = "JadeOS GUI installer bundled from a prebuilt binary";
    homepage = "https://github.com/suichan/jadeos_installer";
    license = licenses.mit;
    platforms = platforms.linux;
    mainProgram = "jade-installer";
  };
})
