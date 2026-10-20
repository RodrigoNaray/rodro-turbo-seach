# Contributing

Thank you for considering a contribution to Rodro Turbo Search. This is a personal
functional MVP, maintained as time allows. English and Spanish are both welcome.
For setup and current scope, see [README.md](README.md) / [README.es.md](README.es.md).

## Bugs and proposals

Search existing issues first. A useful report includes the distribution, desktop,
Wayland/X11 session, GTK and Rust versions, installation method, reproduction
steps, and expected/actual behavior. Sanitize logs and screenshots. For sensitive
reports, use [SECURITY.md](SECURITY.md).

For a substantial change, open an issue to discuss the problem and scope first.
Small bug fixes, documentation improvements, and translations can be proposed
directly. Keep pull requests focused and explain the change and checks performed.
Do not add unrelated refactors or dependencies. Do not present untested platforms
or performance claims as supported behavior.

## Local checks

```sh
cargo build --locked
cargo fmt --check
cargo clippy --locked --all-targets -- -D warnings
cargo test --locked
/usr/bin/python3 tests/installer_test.py
```

For UI changes, follow the opt-in Wayland test instructions in the README and
manually check focus, outside dismissal, dragging, and both modes. The GUI test
does not automate actual mouse dragging or verify visual appearance. Stop the
resident instance before comparing development and installed binaries.

## Licensing and privacy

Contributions must be yours to license and compatible with **GPL-3.0-only**.
Unless explicitly stated otherwise for an approved third-party component,
contributions are submitted under the project's GPL-3.0-only terms. No copyright
assignment is required. Preserve existing authors' notices; add attribution and
license information for any new third-party material. Update the dependency
inventory/notices if dependencies change.

Never commit credentials, local configuration, build outputs, or private logs.
Use a GitHub noreply address for commits if you do not want a personal email in
public Git history. Check the staged diff before committing.

## En español

Se aceptan issues y pull requests en español. Describí pasos para reproducir,
entorno y resultados esperados/observados; no incluyas datos privados. Para cambios
grandes, discutí el alcance en un issue. Las contribuciones originales se reciben
bajo GPL-3.0-only y conservan la autoría de sus colaboradores.
