#!/usr/bin/python3
"""Build distributable binaries without embedding the developer's local paths."""
import os
from pathlib import Path
import shlex
import subprocess


def main():
    project = Path(__file__).resolve().parents[1]
    home = Path.home()
    cargo_home = Path(os.environ.get('CARGO_HOME', home / '.cargo')).resolve()
    rustup_home = Path(os.environ.get('RUSTUP_HOME', home / '.rustup')).resolve()
    environment = os.environ.copy()
    encoded = environment.get('CARGO_ENCODED_RUSTFLAGS')
    flags = encoded.split('\x1f') if encoded else shlex.split(environment.get('RUSTFLAGS', ''))
    # Rust uses the last matching mapping; keep the most specific paths last.
    for source, destination in [
        (home, '/build/user'),
        (rustup_home, '/build/rustup'),
        (cargo_home, '/build/cargo'),
        (project, '/build/rodro-turbo-search'),
    ]:
        flags.append(f'--remap-path-prefix={source}={destination}')
    environment['CARGO_ENCODED_RUSTFLAGS'] = '\x1f'.join(flags)
    subprocess.run([
        'cargo', 'build', '--release', '--locked',
        '--manifest-path', str(project / 'Cargo.toml'),
        '--target-dir', str(project / 'target'),
    ], env=environment, cwd=project, check=True)

    binary = (project / 'target/release/rodro-turbo-search').read_bytes()
    private_paths = [home, cargo_home, rustup_home, project]
    if any(os.fsencode(path) in binary for path in private_paths):
        raise SystemExit('El ejecutable contiene rutas locales; se detiene el empaquetado.')
    print('Release preparada sin rutas locales del desarrollador.')


if __name__ == '__main__':
    main()
