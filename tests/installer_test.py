"""Installer integration in temporary directories with in-memory GSettings only."""
import importlib.util
import os
from pathlib import Path
import tempfile
import unittest

os.environ['GSETTINGS_BACKEND'] = 'memory'
os.environ['XDG_CURRENT_DESKTOP'] = 'GNOME'
MODULE = Path(__file__).resolve().parents[1] / 'scripts/setup.py'


class InstallerTest(unittest.TestCase):
    def setUp(self):
        self.temp = tempfile.TemporaryDirectory(prefix='turbo-installer-')
        root = Path(self.temp.name)
        os.environ['HOME'] = str(root)
        os.environ['XDG_DATA_HOME'] = str(root / 'data')
        os.environ['XDG_CONFIG_HOME'] = str(root / 'config')
        spec = importlib.util.spec_from_file_location('setup_test', MODULE)
        self.setup = importlib.util.module_from_spec(spec)
        spec.loader.exec_module(self.setup)
        self.media, self.wm, self.custom = self.setup.settings()
        self.media.reset('custom-keybindings')
        self.wm.reset('activate-window-menu')
        for key in ['name', 'command', 'binding']:
            self.custom.reset(key)
        self.binary = root / 'source-binary'
        self.binary.write_text('#!/bin/sh\nexit 0\n')
        self.binary.chmod(0o755)

    def tearDown(self):
        self.temp.cleanup()

    def test_install_update_uninstall_preserves_other_shortcuts(self):
        other = '/org/gnome/settings-daemon/plugins/media-keys/custom-keybindings/other/'
        later = '/org/gnome/settings-daemon/plugins/media-keys/custom-keybindings/later/'
        self.media.set_strv('custom-keybindings', [other])
        self.wm.set_strv('activate-window-menu', ['<Alt>space', '<Control>F10'])
        self.setup.install(self.binary)
        self.setup.install(self.binary)
        self.assertEqual(list(self.media.get_strv('custom-keybindings')), [other, self.setup.CUSTOM_PATH])
        self.assertEqual(list(self.wm.get_strv('activate-window-menu')), ['<Control>F10'])
        self.assertTrue(self.setup.AUTOSTART.exists())
        self.assertTrue(os.access(self.setup.BINARY, os.X_OK))
        self.media.set_strv('custom-keybindings', [other, self.setup.CUSTOM_PATH, later])
        self.setup.uninstall()
        self.assertEqual(list(self.media.get_strv('custom-keybindings')), [other, later])
        self.assertEqual(list(self.wm.get_strv('activate-window-menu')), ['<Alt>space', '<Control>F10'])
        self.assertFalse(self.setup.BINARY.exists())
        self.assertFalse(self.setup.STATE.exists())
        self.assertFalse(self.setup.AUTOSTART.exists())

    def test_preserves_user_changes_to_window_menu(self):
        self.setup.install(self.binary)
        self.wm.set_strv('activate-window-menu', ['<Super>F10'])
        self.setup.uninstall()
        self.assertEqual(list(self.wm.get_strv('activate-window-menu')), ['<Super>F10'])

    def test_foreign_shortcut_conflict_does_not_install(self):
        path = '/org/gnome/settings-daemon/plugins/media-keys/custom-keybindings/conflict/'
        other = self.setup.Gio.Settings.new_with_path(self.setup.CUSTOM_SCHEMA, path)
        other.set_string('binding', '<Alt>space')
        other.set_string('name', 'Otra app')
        self.media.set_strv('custom-keybindings', [path])
        with self.assertRaises(RuntimeError):
            self.setup.install(self.binary)
        self.assertFalse(self.setup.BINARY.exists())
        self.assertFalse(self.setup.STATE.exists())

    def test_first_install_rolls_back_on_file_error(self):
        original = self.setup.atomic_write
        def failing_write(path, *args):
            if path == self.setup.AUTOSTART:
                raise OSError('simulated disk failure')
            return original(path, *args)
        self.setup.atomic_write = failing_write
        with self.assertRaises(OSError):
            self.setup.install(self.binary)
        self.assertFalse(self.setup.BINARY.exists())
        self.assertFalse(self.setup.DESKTOP.exists())
        self.assertFalse(self.setup.STATE.exists())
        self.assertEqual(list(self.wm.get_strv('activate-window-menu')), ['<Alt>space'])


if __name__ == '__main__':
    unittest.main()
