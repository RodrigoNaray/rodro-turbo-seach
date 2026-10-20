# Rodro Turbo Search

**English** | [Español](README.es.md)

A lightweight application search and launcher written in **Rust and GTK4** for
**Linux with GNOME on Wayland**, with an integrated calculator and optional
filename search.

**Status:** functional MVP, personal project in development. It is not presented
as a stable release or a broadly tested desktop product. The interface and CLI
messages are currently in Spanish.

## Motivation

Opening an application, calculating a value, or finding a document can interrupt
the current task with several menus and windows. This project explores a compact,
keyboard-driven entry point for these tasks: open a bar, type, and activate a result.
It is also a practical exercise in Rust, native desktop integration, bounded
background work, and maintainable modules. “Lightweight” is a design goal; no
comparative performance or memory benchmarks are claimed.

## Current features

- **Application launcher:** finds visible `.desktop` applications through GIO,
  ranks case-insensitive exact, prefix, and substring matches, and shows up to
  five applications. The catalog refreshes when the bar opens.
- **Optional file mode:** click the logo to add up to five filename/folder matches
  below application results. A translucent purple fill marks this mode; clicking
  again returns to the black application mode. Search covers the home directory.
- **Calculator:** evaluates arithmetic, parentheses, powers, percentages, unary
  signs, and scientific notation. Enter or clicking the result copies it to the
  clipboard and hides the bar. For example, `125 * 1.22` gives `152.5`,
  `(2 + 3) * 4` gives `20`, and `200 * 15%` gives `30`. `=42` forces calculation;
  a plain integer remains a search. Dot or comma can be the decimal separator.
- **Desktop workflow:** one resident instance, Alt + Space after installation,
  arrow-key selection, Enter to open, Escape or an outside focus change to hide.
  Successful activation clears the query; hiding an unfinished search preserves
  it in memory. No search history is written to disk.
- **Movable interface:** drag anywhere except the logo, including over text and
  results. Simple clicks retain their action. Dragging text moves the window;
  use the keyboard to select text. Rounded translucent backgrounds, a logo hover,
  and a short mode animation are implemented. The fill animation respects GTK's
  animation preference.

## Technology and requirements

Rust 2024 edition, `gtk4` 0.11.5, GIO/GLib/GDK, session D-Bus, and GNOME GSettings.
The per-user installer uses Python 3 and the system PyGObject package. There is
no application web service, telemetry, database, persistent file index, or disk watcher.

The intended environment is **GNOME on Wayland on Linux**. Local checks were run
with GTK 4.14.5 and Rust 1.99.1; this is not a compatibility matrix. Other desktops,
X11, Windows, and macOS have not been validated. GTK's cross-platform support does
not imply support from this application. The installer requires GNOME schemas.

To build, use current stable Rust/Cargo (the locked GTK bindings require Rust
1.92 or newer), a C toolchain, `pkg-config`, and GTK4 development libraries.
GTK APIs use the base 4.0 feature set, but the full dependency combination has only
been checked in the local environment. A prebuilt binary additionally depends
on the architecture, libc, and shared libraries of its build environment.

On Debian/Ubuntu-based systems, install the build and installer requirements:

```sh
sudo apt-get install build-essential pkg-config libgtk-4-dev python3 python3-gi
```

Rust/Cargo must be installed separately. `rustfmt` and Clippy are needed for the
contributor checks, not for running an installed binary.

## Build and run

The current GitHub remote has the spelling `rodro-turbo-seach`; it is preserved.
The application and Cargo package are named `rodro-turbo-search`.

```sh
git clone https://github.com/RodrigoNaray/rodro-turbo-seach.git
cd rodro-turbo-seach
cargo build --locked
./target/debug/rodro-turbo-search toggle
```

Run from a graphical session with a session D-Bus. `toggle` starts the resident
instance if needed. Running without arguments starts it hidden:

```sh
./target/debug/rodro-turbo-search         # start hidden
./target/debug/rodro-turbo-search toggle  # show/hide
./target/debug/rodro-turbo-search quit    # stop the resident instance
./target/debug/rodro-turbo-search --help
```

After closing the bar, the process intentionally remains alive. The development
binary and installed binary share the same application ID; stop the current
instance before comparing builds.

## Install for your user

From the source checkout, inside a GNOME session:

```sh
./install.sh
```

This builds an optimized release, installs in `~/.local`, adds an application
menu entry and session autostart, and configures **Alt + Space**. It does not
require sudo; installing system prerequisites above does. If that shortcut is
used by GNOME's window menu, the installer records the setting before freeing
it. It stops if another custom shortcut uses it. Rerun to update.

Use Alt + Space, type a query, select with ↑/↓, and press Enter or click a result.
Click the logo to include files; the calculator works in either mode. Opening
an application/file or copying a calculation hides and clears the bar. An opening
error leaves the query available to retry. An empty bar shows no placeholder or results.

Default paths:

| Item | Path |
| --- | --- |
| Executable | `~/.local/bin/rodro-turbo-search` |
| Menu entry | `~/.local/share/applications/io.github.rodro.TurboSearch.desktop` |
| Autostart | `~/.config/autostart/io.github.rodro.TurboSearch.desktop` |
| Installed icon | `~/.local/share/icons/hicolor/scalable/apps/io.github.rodro.TurboSearch.svg` |
| Uninstall state | `~/.local/share/rodro-turbo-search/install-state.json` |

Uninstall with:

```sh
/usr/bin/python3 ~/.local/share/rodro-turbo-search/setup.py uninstall
```

With custom `XDG_DATA_HOME`/`XDG_CONFIG_HOME`, use the locations printed by the
installer. Uninstall restores the saved window-menu shortcut if it has not
subsequently been changed by the user.

## Known limits

- GNOME/Wayland controls activation, placement, monitors, and focus. Absolute
  initial placement, always-on-top behavior, and focus in every setup are not
  guaranteed. Outside dismissal follows focus/pointer events, not global mouse hooks.
  Dragging and focus need manual desktop checks; the automated GUI test does not
  simulate a compositor-driven drag.
- File search needs at least two characters and waits 200 ms after typing. One
  worker searches names only; it skips hidden entries, symbolic links,
  `node_modules`, `target`, and `vendor`. It checks limits of 30,000 entries,
  16 directory descents, 2,048 pending directories, and 700 ms between disk
  operations. Hitting a limit reports partial results. Large trees can omit
  matches; unreadable directories are skipped. Slow filesystems can delay the
  worker beyond this budget, though disk traversal is off the GTK thread.
- Application matches precede file matches. Calculation-like queries take
  precedence over application/file search. Files open via the default handler.
- Arithmetic uses floating point and about 12 significant display digits; it
  is not exact decimal arithmetic. There are no thousands separators.
- There is no settings interface, saved query history, or persistence of the
  selected mode after the process exits. Installation updates are atomic per
  file, not a transaction spanning all installed files.

## Checks and local packaging

```sh
cargo fmt --check
cargo clippy --locked --all-targets -- -D warnings
cargo test --locked
/usr/bin/python3 tests/installer_test.py
```

Installer tests use temporary directories and in-memory GSettings. They need
the installed GNOME schemas, but do not modify real user shortcuts.
The opt-in GUI test briefly opens a test window and launches `/usr/bin/true`:

```sh
GDK_BACKEND=wayland cargo test --locked ui::tests::search_selection_launch_and_failure -- --ignored --test-threads=1
```

Inside a graphical session, with no existing resident instance, the lifecycle
smoke check briefly toggles the real bar:

```sh
python3 tests/session_smoke.py target/debug/rodro-turbo-search
```

It checks IPC, instance reuse, toggling, and shutdown, not visual correctness or
real drag behavior. There are no GitHub Actions workflows in the current repository.

Create a local prebuilt archive for the current Linux architecture with:

```sh
./scripts/package.sh
```

It writes a `dist/rodro-turbo-search-linux-*.tar.gz` archive whose filename includes
the build host architecture; the script prints the exact path. It does not publish
anything. Extract and run its `./install.sh` to install without Rust. No prebuilt
release availability is promised. Use the release scripts for shared binaries:
they remap local build paths and reject binaries containing the builder's paths.
Before distributing binaries, also provide their complete corresponding source,
including linked Rust dependencies and build scripts, as required by GPLv3;
the local binary archive alone is not a complete source distribution. See
[DEPENDENCIES.md](DEPENDENCIES.md) and [PUBLICATION.md](PUBLICATION.md).

## Repository map

| Location | Responsibility |
| --- | --- |
| `src/cli.rs`, `src/platform.rs` | Commands, session integration, single instance |
| `src/catalog.rs`, `src/search.rs` | Application discovery and ranking |
| `src/calculator.rs`, `src/files.rs` | Arithmetic and bounded filename search |
| `src/ui.rs`, `src/background.rs` | GTK interface and color transition |
| `src/window_drag.rs`, `src/dismissal.rs` | Dragging and focus handling |
| `scripts/` | Per-user installation, release build, local packaging |
| `tests/` | Installer and graphical lifecycle checks |
| `assets/` | SVG artwork |

[DECISIONS.md](DECISIONS.md) records design decisions in Spanish.
[DEPENDENCIES.md](DEPENDENCIES.md) and [THIRD_PARTY_NOTICES.md](THIRD_PARTY_NOTICES.md)
record dependency licenses and retained notices.

## Contributing and reporting problems

Use the repository's [Issues](https://github.com/RodrigoNaray/rodro-turbo-seach/issues)
for reproducible bugs or proposals. Include desktop/session, distribution,
GTK/Rust versions, reproduction steps, and expected/actual results. Remove private
paths, filenames, and clipboard contents from reports and screenshots. For
vulnerabilities, follow [SECURITY.md](SECURITY.md).

Small, focused contributions and documentation corrections are welcome. Read
[CONTRIBUTING.md](CONTRIBUTING.md) before submitting a pull request. Maintenance
is on a personal, best-effort basis; there is no release schedule or response SLA.
Reports and contributions in English or Spanish are welcome.

## License and author

Copyright © 2026 Rodrigo Naray.

Original project work is licensed under **GNU GPL version 3 only**
(`GPL-3.0-only`), with no automatic grant for later versions. See the complete,
unmodified [LICENSE](LICENSE) and [COPYRIGHT](COPYRIGHT). Third-party notices and
font-derived artwork retain their respective terms in
[THIRD_PARTY_NOTICES.md](THIRD_PARTY_NOTICES.md). No warranty is provided.
