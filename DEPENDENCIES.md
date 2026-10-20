# Dependencies and license review

Original project work: **GPL-3.0-only**. Copyright © 2026 Rodrigo Naray.
See [LICENSE](LICENSE), [COPYRIGHT](COPYRIGHT), and retained full notices in
[THIRD_PARTY_NOTICES.md](THIRD_PARTY_NOTICES.md).

## Scope and compatibility

The only direct Cargo dependency is `gtk4` 0.11.5 (alias `gtk`, MIT). It reexports
GIO/GLib/GDK. The locked graph has **64 third-party packages**, including build-only
and target-specific packages. No vendored third-party application code was found
in `src/`; there are no Git submodules or bundled font binaries.

The resolved declarations are MIT, MIT/Apache-2.0 alternatives, Unlicense/MIT,
Apache-2.0 WITH LLVM-exception, and `(MIT OR Apache-2.0) AND Unicode-3.0`.
MIT is selected where offered. The Apache-2.0 and Unicode terms are compatible
with GPLv3; their notices are retained. The LLVM exception adds permissions, and
its full text is retained with target-lexicon. No incompatible Rust dependency
was identified in this locked graph. This is a review of declared licenses and
shipped notice files, not an exhaustive provenance audit of every upstream line.

Primary references:

- [GNU license compatibility list](https://www.gnu.org/licenses/license-list.html)
- [gtk4-rs license](https://github.com/gtk-rs/gtk4-rs/blob/main/LICENSE)
- [GTK licensing](https://www.gtk.org/)
- [DejaVu license](https://dejavu-fonts.github.io/License.html)

GTK/GLib and other native libraries are linked dynamically and supplied by the
system. GTK/GLib use LGPL terms (primarily LGPL-2.1-or-later); Cairo, Pango,
GdkPixbuf, Graphene and their dependencies retain their own licenses. The local
binary's dynamic dependencies can be inspected with `ldd`. No system shared
libraries are bundled by `scripts/package.sh`. Python and PyGObject are installed
system tools, not copied into the archive. PyGObject uses LGPL terms. Distributing
native libraries yourself would require a separate inventory of those exact builds.

## Artwork and attribution

`assets/rodro-turbo-search-v2-rayo.svg` is the embedded and installed logo. It
contains a DejaVu Serif glyph converted to paths, with font-family metadata still
present. Preserve the DejaVu/Bitstream notices included in THIRD_PARTY_NOTICES.md.
Do not interpret the project's GPL notice as replacing the font's license. Font
output/artwork and font software are distinct; no font program is embedded.
The maintainer should confirm ownership or permission for the rest of the original
artwork before publication. No external image/font payload was found in the SVG.
An untracked alternative logo is not part of the committed source or package.
Desktop application icons come from GIO/the user's icon theme; they are not copied
into the repository.

## Distribution checklist

The packaging script includes LICENSE, COPYRIGHT, this inventory, both READMEs,
and third-party notices. They are additional to the executable, not a claim that
the binary archive alone satisfies all GPL distribution obligations.

When distributing object code, provide complete corresponding source for the
exact build, including linked Rust dependencies and build/install scripts, through
a GPLv3-permitted method. One practical preparation is to archive the matching
project source plus `cargo vendor --locked` output and its Cargo configuration,
check an offline rebuild, and offer both source and binary together. System
libraries are generally outside Corresponding Source under GPLv3's System Library
exception; evaluate any libraries actually bundled separately. This repository
preparation does not publish a binary or set up a source offer.

## Locked dependency inventory

Regenerate from `cargo metadata --locked --format-version 1` after changing the
lockfile. Notice text was checked against the locally downloaded crate packages.

| Package | Version | Declared SPDX license |
| --- | --- | --- |
| autocfg | 1.5.1 | Apache-2.0 OR MIT |
| bitflags | 2.13.2 | MIT OR Apache-2.0 |
| cairo-rs | 0.22.9 | MIT |
| cairo-sys-rs | 0.22.9 | MIT |
| cfg-expr | 0.20.10 | MIT OR Apache-2.0 |
| equivalent | 1.0.3 | Apache-2.0 OR MIT |
| field-offset | 0.3.6 | MIT OR Apache-2.0 |
| futures-channel | 0.3.34 | MIT OR Apache-2.0 |
| futures-core | 0.3.34 | MIT OR Apache-2.0 |
| futures-executor | 0.3.34 | MIT OR Apache-2.0 |
| futures-io | 0.3.34 | MIT OR Apache-2.0 |
| futures-macro | 0.3.34 | MIT OR Apache-2.0 |
| futures-task | 0.3.34 | MIT OR Apache-2.0 |
| futures-util | 0.3.34 | MIT OR Apache-2.0 |
| gdk-pixbuf | 0.22.0 | MIT |
| gdk-pixbuf-sys | 0.22.9 | MIT |
| gdk4 | 0.11.5 | MIT |
| gdk4-sys | 0.11.5 | MIT |
| gio | 0.22.10 | MIT |
| gio-sys | 0.22.9 | MIT |
| glib | 0.22.10 | MIT |
| glib-macros | 0.22.9 | MIT |
| glib-sys | 0.22.9 | MIT |
| gobject-sys | 0.22.9 | MIT |
| graphene-rs | 0.22.8 | MIT |
| graphene-sys | 0.22.9 | MIT |
| gsk4 | 0.11.5 | MIT |
| gsk4-sys | 0.11.5 | MIT |
| gtk4 | 0.11.5 | MIT |
| gtk4-macros | 0.11.5 | MIT |
| gtk4-sys | 0.11.5 | MIT |
| hashbrown | 0.17.1 | MIT OR Apache-2.0 |
| heck | 0.5.0 | MIT OR Apache-2.0 |
| indexmap | 2.14.2 | Apache-2.0 OR MIT |
| libc | 0.2.191 | MIT OR Apache-2.0 |
| memchr | 2.8.3 | Unlicense OR MIT |
| memoffset | 0.9.1 | MIT |
| pango | 0.22.9 | MIT |
| pango-sys | 0.22.9 | MIT |
| pin-project-lite | 0.2.17 | Apache-2.0 OR MIT |
| pkg-config | 0.3.34 | MIT OR Apache-2.0 |
| proc-macro-crate | 3.5.0 | MIT OR Apache-2.0 |
| proc-macro2 | 1.0.107 | MIT OR Apache-2.0 |
| quote | 1.0.47 | MIT OR Apache-2.0 |
| rustc_version | 0.4.1 | MIT OR Apache-2.0 |
| semver | 1.0.28 | MIT OR Apache-2.0 |
| serde_core | 1.0.229 | MIT OR Apache-2.0 |
| serde_derive | 1.0.229 | MIT OR Apache-2.0 |
| serde_spanned | 1.1.2 | MIT OR Apache-2.0 |
| slab | 0.4.12 | MIT |
| smallvec | 1.16.3 | MIT OR Apache-2.0 |
| syn | 3.0.9 | MIT OR Apache-2.0 |
| system-deps | 9.0.0 | MIT OR Apache-2.0 |
| target-lexicon | 0.13.5 | Apache-2.0 WITH LLVM-exception |
| toml | 1.1.8+spec-1.1.0 | MIT OR Apache-2.0 |
| toml_datetime | 1.1.2+spec-1.1.0 | MIT OR Apache-2.0 |
| toml_edit | 0.25.17+spec-1.1.0 | MIT OR Apache-2.0 |
| toml_parser | 1.1.5+spec-1.1.0 | MIT OR Apache-2.0 |
| toml_writer | 1.1.3+spec-1.1.0 | MIT OR Apache-2.0 |
| unicode-ident | 1.0.27 | (MIT OR Apache-2.0) AND Unicode-3.0 |
| version-compare | 0.2.1 | MIT |
| windows-link | 0.2.1 | MIT OR Apache-2.0 |
| windows-sys | 0.61.2 | MIT OR Apache-2.0 |
| winnow | 1.0.4 | MIT |

## GPL text integrity

LICENSE was downloaded verbatim from https://www.gnu.org/licenses/gpl-3.0.txt
and matches the system's `/usr/share/common-licenses/GPL-3` byte for byte.
SHA-256: `3972dc9744f6499f0f9b2dbf76696f2ae7ad8af9b23dde66d6af86c9dfb36986`. The author notice is in COPYRIGHT, not inserted into
the official license document. GPL-3.0-only is selected in Cargo.toml and COPYRIGHT;
the license's generic example about “later versions” is part of the unmodified
license text and does not change that selection.
