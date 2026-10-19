# Cosmos RAM

Show used memory beside a RAM icon in the COSMIC panel. Hover over the value to see used and total memory.
Cosmos is independent of System76. COSMIC libraries and protocols keep their upstream names.

## Requirements

Use Linux with a working COSMIC desktop session and readable `/proc/meminfo`.
Install native build packages for your distribution:

```sh
# Fedora
sudo dnf install gcc gcc-c++ cmake git pkgconf-pkg-config just desktop-file-utils expat-devel fontconfig-devel freetype-devel libxkbcommon-devel wayland-devel

# Debian 13 or newer
sudo apt update
sudo apt install build-essential cmake git pkgconf just desktop-file-utils libexpat1-dev libfontconfig-dev libfreetype-dev libxkbcommon-dev libwayland-dev

# Arch Linux
sudo pacman -Syu --needed base-devel cmake git pkgconf just desktop-file-utils expat fontconfig freetype2 libxkbcommon wayland
```

The lists follow [libcosmic's native requirements](https://github.com/pop-os/libcosmic#dependencies) and distribution package metadata.
They are package examples, not a claim that all three distributions were tested.
Install current stable Rust with [rustup](https://rustup.rs), as your normal user. Distribution Rust can be too old.
After rustup setup, open a new terminal and run:

```sh
rustup update stable
rustup default stable
```

## Build and install

Use `sudo` only for package prerequisites above, never for these build or install commands:

```sh
git clone https://github.com/petterssonjonas/cosmos-applet-ram
cd cosmos-applet-ram
just build-release
just install
```

Build and install are separate. Installation uses the existing release build.
The executable goes to `~/.local/bin/cosmos-applet-ram`. The desktop entry, metainfo, and icon go to `~/.local/share`.
The desktop entry uses an absolute executable path. Keep `CARGO_TARGET_DIR` the same for both commands if you set it.
In a development container, use your host-mapped normal user, not root.

Open **COSMIC Settings → Desktop → Panel → Configure panel applets**.
Add **Cosmos RAM** (`dev.jp.CosmosAppletRam`). If it is not listed, reopen Settings or log out and back in.
Use a horizontal panel for the icon and value on one line. Remove and add the applet again after an upgrade.
Installation does not change the panel layout.

## Measurement and limits

- Reads `/proc/meminfo` at startup and every two seconds.
- Used memory is `MemTotal - MemAvailable`, as in modern `free`. Reclaimable cache is available memory. Swap is excluded.
- Uses binary GiB/MiB units, labelled **GB/MB**. The panel rounds to whole GB; the tooltip shows whole MB.
- Text turns red when available memory is 10% or less of actual total memory. The threshold uses unrounded values.
- Unreadable or invalid data shows **N/A**, not a stale value or false zero. Updates resume when readings recover.

No accounts, credentials, or saved settings are required.

## Upgrade from cosmic-ram-applet

Stop the old applet and remove its **RAM Usage** panel entry. Install the renamed build, then add **Cosmos RAM**.
There is no persistent data to migrate. `just migrate` reports this without copying anything. There are no old-name runtime aliases.
Remove legacy files separately under the old prefix: `bin/cosmic-ram-applet`,
`share/applications/io.github.cosmic-ram-applet.desktop`, `share/metainfo/io.github.cosmic-ram-applet.metainfo.xml`, and
`share/icons/hicolor/scalable/apps/io.github.cosmic-ram-applet.svg`.

## Uninstall

Remove the applet from the panel. Run `just uninstall` from this source directory, without `sudo`.
Uninstall removes the new application files. It does not remove legacy files or change panel settings.

## License and upstream

Code: MPL-2.0, as declared in `Cargo.toml`.
Based on the [COSMIC applet template](https://github.com/pop-os/cosmic-applet-template), using [libcosmic](https://github.com/pop-os/libcosmic).
