"""Run manually inside a graphical session; briefly shows the prototype window.

Checks IPC/lifecycle only, not compositor focus or visual correctness.
Run without arguments to use target/debug/rodro-turbo-search. An optional binary
path may be passed as the first argument.
"""

import os
from pathlib import Path
import subprocess
import sys
import tempfile
import time

APP_ID = "io.github.rodro.TurboSearch"
BINARY = str(Path(sys.argv[1] if len(sys.argv) > 1 else
                  "target/debug/rodro-turbo-search").resolve())


def bus_call(method, *args):
    return subprocess.check_output([
        "gdbus", "call", "--session", "--dest", "org.freedesktop.DBus",
        "--object-path", "/org/freedesktop/DBus", "--method",
        f"org.freedesktop.DBus.{method}", *args,
    ], text=True, timeout=5)


def run(*args, expected=0):
    result = subprocess.run([BINARY, *args], capture_output=True, text=True, timeout=10)
    assert result.returncode == expected, result.stderr


def cpu_ticks(pid):
    fields = Path(f"/proc/{pid}/stat").read_text().rsplit(")", 1)[1].split()
    return int(fields[11]) + int(fields[12])


def memory(pid):
    lines = Path(f"/proc/{pid}/smaps_rollup").read_text().splitlines()
    return ", ".join(line.strip() for line in lines if line.startswith(("Rss:", "Pss:")))


def sample_idle(pid):
    time.sleep(3)  # Exclude startup/mapping work from the idle interval.
    started = time.monotonic()
    ticks = cpu_ticks(pid)
    time.sleep(3)
    cpu = (cpu_ticks(pid) - ticks) / os.sysconf("SC_CLK_TCK")
    print(f"Settled hidden CPU over 3s: {100 * cpu / (time.monotonic() - started):.2f}% of one core")


assert "false" in bus_call("NameHasOwner", APP_ID), "Close the existing instance first."
run("--help")
run("invalid-command", expected=2)
run("quit")
assert "false" in bus_call("NameHasOwner", APP_ID)

with tempfile.TemporaryFile(mode="w+") as log:
    process = subprocess.Popen([BINARY], stdout=log, stderr=log)
    try:
        deadline = time.monotonic() + 10
        while "false" in bus_call("NameHasOwner", APP_ID):
            assert process.poll() is None, "Resident exited at startup."
            assert time.monotonic() < deadline, "Timed out waiting for D-Bus registration."
            time.sleep(0.05)
        owner = bus_call("GetNameOwner", APP_ID)
        print("At D-Bus registration (startup still settling):", memory(process.pid))
        sample_idle(process.pid)
        run()
        assert bus_call("GetNameOwner", APP_ID) == owner, "Primary instance changed."
        for _ in range(20):
            run("toggle")
            time.sleep(0.05)
            run("toggle")
        assert process.poll() is None
        assert bus_call("GetNameOwner", APP_ID) == owner
        print("After 20 toggle pairs:", memory(process.pid))
        sample_idle(process.pid)
        run("quit")
        assert process.wait(timeout=5) == 0
        assert "false" in bus_call("NameHasOwner", APP_ID)
        print("PASS: resident, single instance, 40 toggle commands, clean quit.")
    finally:
        if process.poll() is None:
            process.terminate()
            process.wait(timeout=5)
        log.seek(0)
        output = log.read()
        if output:
            print("Application output:\n" + output)
