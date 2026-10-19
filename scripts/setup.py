#!/usr/bin/python3
"""Per-user GNOME installation. No root, shell evaluation or background service."""
import argparse
import json
import os
from pathlib import Path
import shlex
import subprocess
import sys
import tempfile

from gi.repository import Gio

APP_ID = 'io.github.rodro.TurboSearch'
CUSTOM_SCHEMA = 'org.gnome.settings-daemon.plugins.media-keys.custom-keybinding'
CUSTOM_PATH = '/org/gnome/settings-daemon/plugins/media-keys/custom-keybindings/rodro-turbo-search/'
HOME = Path.home()
DATA = Path(os.environ.get('XDG_DATA_HOME', HOME / '.local/share'))
CONFIG = Path(os.environ.get('XDG_CONFIG_HOME', HOME / '.config'))
INSTALL = DATA / 'rodro-turbo-search'
BINARY = HOME / '.local/bin/rodro-turbo-search'
DESKTOP = DATA / 'applications' / f'{APP_ID}.desktop'
AUTOSTART = CONFIG / 'autostart' / f'{APP_ID}.desktop'
STATE = INSTALL / 'install-state.json'
COMMAND = f'{shlex.quote(str(BINARY))} toggle'


def settings():
    source = Gio.SettingsSchemaSource.get_default()
    for schema in ['org.gnome.settings-daemon.plugins.media-keys',
                   'org.gnome.desktop.wm.keybindings', CUSTOM_SCHEMA]:
        if not source or not source.lookup(schema, True):
            raise RuntimeError('Este instalador necesita una sesión GNOME con GSettings.')
    return (Gio.Settings.new('org.gnome.settings-daemon.plugins.media-keys'),
            Gio.Settings.new('org.gnome.desktop.wm.keybindings'),
            Gio.Settings.new_with_path(CUSTOM_SCHEMA, CUSTOM_PATH))


def alt_space(value):
    return value.lower().replace('<mod1>', '<alt>') == '<alt>space'


def atomic_write(path, data, mode=0o644):
    path.parent.mkdir(parents=True, exist_ok=True)
    with tempfile.NamedTemporaryFile(dir=path.parent, delete=False) as temp:
        temporary = Path(temp.name)
        temp.write(data)
    try:
        temporary.chmod(mode)
        temporary.replace(path)
    finally:
        temporary.unlink(missing_ok=True)


def desktop_entry(autostart=False):
    # Desktop Exec has its own escaping rules, distinct from shell quoting.
    executable = str(BINARY).replace('\\', '\\\\').replace('"', '\\"').replace('`', '\\`').replace('$', '\\$').replace('%', '%%')
    return ('[Desktop Entry]\nType=Application\nName=Rodro Turbo Search\n'
            'Comment=Buscar y abrir aplicaciones\nIcon=system-search\n'
            f'Exec="{executable}"' + ('' if autostart else ' toggle') + '\n'
            'Terminal=false\nCategories=Utility;\nStartupNotify=true\n'
            + ('NoDisplay=true\nX-GNOME-Autostart-enabled=true\n' if autostart else ''))


def stop():
    if BINARY.exists():
        subprocess.run([str(BINARY), 'quit'], check=True, timeout=10)


def uninstall():
    if not STATE.exists():
        print('No hay una instalación registrada.')
        return
    state = json.loads(STATE.read_text())
    media, wm, custom = settings()
    stop()
    if custom.get_string('command') == COMMAND:
        media.set_strv('custom-keybindings', [p for p in media.get_strv('custom-keybindings') if p != CUSTOM_PATH])
        for key in ['name', 'command', 'binding']:
            custom.reset(key)
    if list(wm.get_strv('activate-window-menu')) == state['menu_installed']:
        wm.set_strv('activate-window-menu', state['menu_before'])
    Gio.Settings.sync()
    for path, content in [(DESKTOP, desktop_entry()), (AUTOSTART, desktop_entry(True))]:
        if path.exists() and path.read_text() == content:
            path.unlink()
    BINARY.unlink(missing_ok=True)
    (INSTALL / 'setup.py').unlink(missing_ok=True)
    STATE.unlink()
    print('Rodro Turbo Search desinstalado. Se conservan los cambios posteriores de otros atajos.')


def install(source):
    if 'GNOME' not in os.environ.get('XDG_CURRENT_DESKTOP', '').upper():
        raise RuntimeError('Ejecutá el instalador dentro de tu sesión GNOME.')
    if not source.is_file():
        raise RuntimeError(f'No se encontró el binario: {source}')
    subprocess.run([str(source), '--help'], check=True, stdout=subprocess.DEVNULL, timeout=10)
    media, wm, custom = settings()
    for obj, key in [(media, 'custom-keybindings'), (wm, 'activate-window-menu'),
                     (custom, 'name'), (custom, 'command'), (custom, 'binding')]:
        if not obj.is_writable(key):
            raise RuntimeError(f'La configuración {key} está bloqueada por el sistema.')
    paths = list(media.get_strv('custom-keybindings'))
    for path in paths:
        if path == CUSTOM_PATH:
            continue
        other = Gio.Settings.new_with_path(CUSTOM_SCHEMA, path)
        if alt_space(other.get_string('binding')):
            raise RuntimeError(f'Alt + Espacio ya pertenece a «{other.get_string("name")}». Liberá ese atajo para instalar; no se modificó.')
    first_install = not STATE.exists()
    if first_install:
        for path in [BINARY, DESKTOP, AUTOSTART]:
            if path.exists():
                raise RuntimeError(f'Ya existe {path} y no pertenece a una instalación registrada.')
        if CUSTOM_PATH in paths or custom.get_string('command'):
            raise RuntimeError('La ruta del atajo ya está ocupada por una configuración no registrada.')
        menu_before = list(wm.get_strv('activate-window-menu'))
        state = {'menu_before': menu_before,
                 'menu_installed': [value for value in menu_before if not alt_space(value)]}
        atomic_write(STATE, json.dumps(state, indent=2).encode(), 0o600)
    else:
        state = json.loads(STATE.read_text())
        if any(alt_space(value) for value in wm.get_strv('activate-window-menu')):
            raise RuntimeError('Alt + Espacio volvió a asignarse al menú de ventana. Liberalo antes de actualizar.')
    try:
        # Stop an older development instance too: it shares the same application ID.
        subprocess.run([str(source), 'quit'], check=True, timeout=10)
        atomic_write(BINARY, source.read_bytes(), 0o755)
        atomic_write(DESKTOP, desktop_entry().encode())
        atomic_write(AUTOSTART, desktop_entry(True).encode())
        atomic_write(INSTALL / 'setup.py', Path(__file__).read_bytes(), 0o755)
        custom.set_string('name', 'Rodro Turbo Search')
        custom.set_string('command', COMMAND)
        custom.set_string('binding', '<Alt>space')
        if first_install:
            wm.set_strv('activate-window-menu', state['menu_installed'])
        if CUSTOM_PATH not in paths:
            media.set_strv('custom-keybindings', paths + [CUSTOM_PATH])
        Gio.Settings.sync()
        if CUSTOM_PATH not in media.get_strv('custom-keybindings') or custom.get_string('binding') != '<Alt>space':
            raise RuntimeError('GNOME no guardó el atajo.')
    except Exception:
        if first_install:
            uninstall()
        raise
    # Keep diagnostics out of the terminal after installation; launch on the next shortcut.
    # The desktop entry starts the app at next login, and toggle starts it immediately on demand.
    print('Instalado. Presioná Alt + Espacio, escribí un nombre y pulsá Enter.')
    print('También está disponible en el menú de aplicaciones e iniciará con tu sesión.')
    print(f'Para desinstalar: /usr/bin/python3 {shlex.quote(str(INSTALL / "setup.py"))} uninstall')


def main():
    parser = argparse.ArgumentParser(description='Instalador de Rodro Turbo Search para tu usuario')
    parser.add_argument('action', choices=['install', 'uninstall'])
    parser.add_argument('--binary', type=Path)
    args = parser.parse_args()
    if os.geteuid() == 0:
        parser.error('Ejecutá sin sudo: la instalación pertenece a tu usuario.')
    if args.action == 'install':
        if args.binary is None:
            parser.error('install requiere --binary')
        install(args.binary.resolve())
    else:
        uninstall()


if __name__ == '__main__':
    try:
        main()
    except (RuntimeError, OSError, subprocess.SubprocessError) as error:
        print(f'No se pudo completar: {error}', file=sys.stderr)
        sys.exit(1)
