name := 'cosmic-ram-applet'
appid := 'io.github.cosmic-ram-applet'
prefix := env('HOME') / '.local'
rootdir := ''
cargo-target-dir := env('CARGO_TARGET_DIR', 'target')
base-dir := absolute_path(clean(rootdir / prefix))

# Build the applet.
default: build-release

build-release:
    cargo build --release --locked

check:
    cargo clippy --all-targets --locked -- -D warnings
    cargo test --locked

# Run a standalone applet surface for development.
run:
    cargo run --locked

# Install for the current user by default; use prefix=/usr for system packaging.
install:
    install -Dm0755 {{cargo-target-dir / 'release' / name}} {{base-dir / 'bin' / name + '.new'}}
    mv -f {{base-dir / 'bin' / name + '.new'}} {{base-dir / 'bin' / name}}
    install -Dm0644 resources/app.desktop {{base-dir / 'share/applications' / appid + '.desktop'}}
    desktop-file-edit --set-key=Exec --set-value='"{{absolute_path(prefix / 'bin' / name)}}"' {{base-dir / 'share/applications' / appid + '.desktop'}}
    install -Dm0644 resources/app.metainfo.xml {{base-dir / 'share/metainfo' / appid + '.metainfo.xml'}}
    install -Dm0644 resources/icon.svg {{base-dir / 'share/icons/hicolor/scalable/apps' / appid + '.svg'}}

uninstall:
    rm -f {{base-dir / 'bin' / name}} {{base-dir / 'share/applications' / appid + '.desktop'}} {{base-dir / 'share/metainfo' / appid + '.metainfo.xml'}} {{base-dir / 'share/icons/hicolor/scalable/apps' / appid + '.svg'}}
