# Dependencias

Dependencia Rust directa: `gtk4` 0.11.5 (alias `gtk`), licencia MIT.
Reexporta GIO, GLib y GDK para evitar declaraciones directas redundantes.
Se usan las APIs base de GTK4, sin exigir una versión reciente de GNOME.

- [gtk4-rs y licencia](https://github.com/gtk-rs/gtk4-rs)
- [gtk-rs core y licencias](https://github.com/gtk-rs/gtk-rs-core)
- GTK4 y GLib del sistema: LGPL-2.1-or-later; enlace dinámico.
- Cargo.lock registra las versiones transitivas exactas. La lista de paquetes
  y sus licencias declaradas puede regenerarse con `cargo metadata --locked
  --format-version 1` (campos `packages[].name`, `version`, `license`).

No se asigna una licencia open source al código de este proyecto.

## Inventario de Cargo.lock

| Paquete | Versión | Licencia declarada |
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

## Instalación y empaquetado

El instalador usa Python 3 (PSF-2.0) y PyGObject del sistema (LGPL-2.1-or-later)
para GSettings. Los scripts de empaquetado usan herramientas habituales de Linux:
shell POSIX, tar y Cargo. No se descargan dependencias Python ni se usa pip.
