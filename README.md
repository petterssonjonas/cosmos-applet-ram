# cosmic-ram-applet

A small COSMIC panel applet displaying a symbolic RAM stick icon followed by used memory, for example **45GB**. Hover to see used and actual total RAM in MB.

Based on [pop-os/cosmic-applet-template](https://github.com/pop-os/cosmic-applet-template), using [libcosmic](https://github.com/pop-os/libcosmic).

## Memory measurement

- Reads `/proc/meminfo` immediately on startup and every two seconds.
- Used RAM is `MemTotal - MemAvailable`, matching modern `free`. Reclaimable cache is available memory; swap is not included.
- Uses 1024-based RAM units (GiB/MiB), labelled GB/MB. The panel rounds to the nearest whole GB; the tooltip shows whole MB as `used/total MB`. For example, 63,438 MB of actual RAM is approximately 61.95 GB, so full usage rounds to 62GB.
- The text turns red when `MemAvailable` is **10% or less** of `MemTotal`, and returns to its normal color above 10%. The threshold uses unrounded readings and scales with the machine's actual memory capacity.
- Displays `N/A` if memory cannot be read, rather than showing a stale or false zero value. Normal updates resume automatically when readings recover.
- Uses the panel's font, size, padding, and theme colors. The RAM icon is embedded, so it does not depend on the installed icon theme.

## Build and install

Requires Linux, a COSMIC desktop session, Rust 1.93 or newer, `just`, `desktop-file-utils`, and the [libcosmic build dependencies](https://github.com/pop-os/libcosmic#building).

```sh
cd cosmic-ram-applet
just build-release
just install
```

Installation defaults to `~/.local`: the executable goes in `~/.local/bin`, and applet metadata and the icon go in `~/.local/share`. The installed desktop entry uses the executable's absolute path, so the panel does not need `~/.local/bin` in its `PATH`.

Open **COSMIC Settings → Desktop → Panel → Applets**, add **RAM Usage**, and position it where you want. If it is not listed immediately after installation, reopen Settings or log out and back in. A horizontal panel provides room for the icon and value on one line.

For a system-wide installation after building:

```sh
sudo just prefix=/usr install
```

Remove the applet from the panel before uninstalling:

```sh
just uninstall
```

Use the same `prefix` and privileges used for installation when uninstalling a system-wide installation.

## Development

```sh
just run       # Launch a standalone applet surface in a COSMIC session
just check     # Clippy and memory parser tests
```

The tests cover available-versus-free memory, swap exclusion, binary-unit rounding, actual memory capacity, the inclusive 10% warning boundary on small and large machines, and invalid readings. Compare the hover values with `free --mebi`. Values can differ briefly because each samples at a different time.

The local Git repository retains the template's history. Its upstream remote is named `template`; no GitHub repository has been published.
