#!/usr/bin/env python3
"""
Debian Package Generator for LocalSend CLI
Builds official-grade .deb packages for amd64, arm64, and aarch64 architectures.
Includes smart port-conflict detection, config templates, and systemd service.
"""

import os
import sys
import io
import time
import gzip
import tarfile
from pathlib import Path

VERSION = "1.0.0"

CONTROL_TEMPLATE = """Package: localsend-cli
Version: {version}
Section: net
Priority: optional
Architecture: {arch}
Installed-Size: {installed_size}
Maintainer: LocalSend Team <support@localsend.org>
Homepage: https://localsend.org
Depends: libc6
Description: Cross-platform local network file sharing tool (CLI)
 LocalSend is an open-source cross-platform app that allows you to
 securely share files and messages with nearby devices over your local
 network without needing an internet connection.
 .
 This package contains the command-line interface and interactive
 TUI (Terminal User Interface) client for LocalSend.
"""

POSTINST_SCRIPT = """#!/bin/sh
set -e
if [ "$1" = "configure" ]; then
    if which update-desktop-database >/dev/null 2>&1; then
        update-desktop-database -q /usr/share/applications || true
    fi
    if which gtk-update-icon-cache >/dev/null 2>&1; then
        gtk-update-icon-cache -q -t /usr/share/icons/hicolor || true
    fi
    if which systemctl >/dev/null 2>&1; then
        systemctl daemon-reload >/dev/null 2>&1 || true
    fi
fi
exit 0
"""

POSTRM_SCRIPT = """#!/bin/sh
set -e
if [ "$1" = "remove" ] || [ "$1" = "purge" ]; then
    if which update-desktop-database >/dev/null 2>&1; then
        update-desktop-database -q /usr/share/applications || true
    fi
    if which gtk-update-icon-cache >/dev/null 2>&1; then
        gtk-update-icon-cache -q -t /usr/share/icons/hicolor || true
    fi
    if which systemctl >/dev/null 2>&1; then
        systemctl daemon-reload >/dev/null 2>&1 || true
    fi
fi
exit 0
"""

DEFAULT_CONFIG_TEMPLATE = """# ==========================================
# LocalSend CLI Configuration File
# Location: ~/.config/localsend-cli/config.toml
# ==========================================

# Имя устройства, отображаемое другим участникам в локальной сети
# По умолчанию: имя хоста системы
# alias = "Linux-Device"

# Порт HTTP-сервера (по умолчанию: 53317)
# Если порт 53317 занят, укажите свободный (например, 53318):
port = 53317

# Каталог для сохранения входящих файлов
# По умолчанию: ~/Downloads (или текущая папка)
# destination = "~/Downloads"
"""

WRAPPER_SCRIPT = """#!/usr/bin/env bash
# LocalSend CLI Launcher with Port Conflict Detection & Configuration Helper
set -e

BIN="/usr/lib/localsend-cli/localsend-cli-bin"
CONFIG_DIR="${XDG_CONFIG_HOME:-$HOME/.config}/localsend-cli"
CONFIG_FILE="$CONFIG_DIR/config.toml"

# Initialize default configuration if missing
if [ ! -f "$CONFIG_FILE" ]; then
    mkdir -p "$CONFIG_DIR" 2>/dev/null || true
    if [ -f "/etc/localsend-cli/config.toml" ]; then
        cp "/etc/localsend-cli/config.toml" "$CONFIG_FILE" 2>/dev/null || true
    else
        cat << 'EOF' > "$CONFIG_FILE" 2>/dev/null || true
# LocalSend CLI configuration
# alias = "My-Device"
port = 53317
# destination = "~/Downloads"
EOF
    fi
fi

# Quick helpers
case "$1" in
    config|--config)
        echo "=== LocalSend CLI Config ($CONFIG_FILE) ==="
        [ -f "$CONFIG_FILE" ] && cat "$CONFIG_FILE" || echo "Config file not found."
        exit 0
        ;;
    --edit-config)
        ${EDITOR:-nano} "$CONFIG_FILE"
        exit 0
        ;;
esac

# Check for explicit --port parameter
has_port=0
for arg in "$@"; do
    if [ "$arg" = "--port" ] || [[ "$arg" == --port=* ]]; then
        has_port=1
        break
    fi
done

# If --port was not explicitly specified, verify if default/configured port is already in use
if [ "$has_port" -eq 0 ]; then
    TARGET_PORT=53317
    if [ -f "$CONFIG_FILE" ]; then
        CFG_PORT=$(grep -E '^[[:space:]]*port[[:space:]]*=[[:space:]]*[0-9]+' "$CONFIG_FILE" 2>/dev/null | head -n1 | grep -oE '[0-9]+' || true)
        [ -n "$CFG_PORT" ] && TARGET_PORT="$CFG_PORT"
    fi

    # Detect port collision
    PID=""
    if command -v ss >/dev/null 2>&1; then
        PID=$(ss -tulpn "sport = :$TARGET_PORT" 2>/dev/null | grep -oE 'pid=[0-9]+' | head -n1 | cut -d= -f2 || true)
    elif command -v fuser >/dev/null 2>&1; then
        PID=$(fuser "$TARGET_PORT/tcp" 2>/dev/null | tr -d ' ' | head -n1 || true)
    elif command -v lsof >/dev/null 2>&1; then
        PID=$(lsof -ti ":$TARGET_PORT" 2>/dev/null | head -n1 || true)
    fi

    if [ -n "$PID" ]; then
        PNAME=$(ps -p "$PID" -o comm= 2>/dev/null || echo "process")
        echo -e "\\033[1;33m[LocalSend CLI] Внимание: Порт $TARGET_PORT уже занят процессом '$PNAME' (PID: $PID)!\\033[0m"
        echo "  • Чтобы остановить занявший процесс: kill -9 $PID"
        echo "  • Либо запустите на другом порту: localsend-cli --port $((TARGET_PORT + 1))"
        echo "  • Либо отредактируйте конфиг: nano $CONFIG_FILE"
        echo ""

        # In interactive terminal, auto-prompt to use alternative port
        if [ -t 0 ]; then
            read -t 5 -p "Запустить на свободном порту $((TARGET_PORT + 1))? [Y/n]: " ans || ans="y"
            echo ""
            if [[ "$ans" =~ ^[Yy]?$ ]]; then
                exec "$BIN" --port "$((TARGET_PORT + 1))" "$@"
            fi
        fi
    fi
fi

exec "$BIN" "$@"
"""

DESKTOP_ENTRY = """[Desktop Entry]
Name=LocalSend CLI
GenericName=File Transfer CLI
Comment=Securely share files and messages with nearby devices
Exec=localsend-cli
Icon=localsend
Terminal=true
Type=Application
Categories=Network;FileTransfer;Utility;ConsoleOnly;
Keywords=localsend;airdrop;transfer;share;network;cli;
"""

SYSTEMD_SERVICE = """[Unit]
Description=LocalSend CLI Receiver Service
Documentation=https://localsend.org
After=network.target

[Service]
Type=simple
ExecStart=/usr/bin/localsend-cli
Restart=on-failure
RestartSec=5s

[Install]
WantedBy=default.target
"""

README_DOC = """# LocalSend CLI

LocalSend CLI is an open-source cross-platform tool to securely share files
and messages with nearby devices over your local network.

## Quick Start

Run the interactive dashboard:
```bash
localsend-cli
```
or simply:
```bash
localsend
```

Send files to nearby devices:
```bash
localsend-cli -f /path/to/file.ext
localsend-cli -f file1.txt -f file2.jpg
```

## Options & Commands

- `--port <PORT>`: HTTP server port (default: 53317).
- `--alias <NAME>`: Custom device name shown to other devices.
- `--destination <DIR>`: Directory where received files are saved (default: ~/Downloads).
- `-f, --file <PATH>`: File to send (repeatable).
- `config` / `--config`: Display current configuration file.
- `--edit-config`: Open configuration file in $EDITOR.
- `-h, --help`: Show help.
- `-V, --version`: Show version.

## Configuration File

Settings are saved in `~/.config/localsend-cli/config.toml`:
```toml
alias = "My-Linux-Server"
port = 53318
destination = "~/Downloads"
```

## Interactive Dashboard Hotkeys

- `1-9`: Send files to the device with that number
- `D`: Show paired and discovered devices
- `W+S`: Toggle share via link (serve files to web browsers over HTTP)
- `W+R`: Toggle receive via link (let browsers upload files to you)
- `Y` / `N` / `P`: Accept / Decline / Accept-and-Pair incoming transfer
- `Ctrl+C`: Cancel current transfer or quit when idle

## Systemd Service

To run LocalSend CLI continuously as a background receiver:
```bash
systemctl --user enable --now localsend-cli
```
"""


def make_tar_gz(entries):
    buf = io.BytesIO()
    with gzip.GzipFile(fileobj=buf, mode='wb', mtime=1724250000) as gz:
        with tarfile.open(fileobj=gz, mode='w') as tar:
            for item in entries:
                ti = tarfile.TarInfo(name=item['name'])
                ti.mtime = item.get('mtime', 1724250000)
                ti.mode = item['mode']
                ti.type = item.get('type', tarfile.REGTYPE)
                ti.uid = 0
                ti.gid = 0
                ti.uname = 'root'
                ti.gname = 'root'
                if ti.type == tarfile.SYMTYPE:
                    ti.linkname = item['linkname']
                    ti.size = 0
                    tar.addfile(ti)
                elif ti.type == tarfile.DIRTYPE:
                    ti.size = 0
                    tar.addfile(ti)
                else:
                    data = item.get('data', b'')
                    ti.size = len(data)
                    tar.addfile(ti, io.BytesIO(data))
    return buf.getvalue()


def write_ar_archive(out_path, files):
    with open(out_path, 'wb') as f:
        f.write(b'!<arch>\n')
        for name, data in files:
            name_field = f"{name:<16}".encode('ascii')
            mtime_field = f"{1724250000:<12}".encode('ascii')
            uid_field = f"{0:<6}".encode('ascii')
            gid_field = f"{0:<6}".encode('ascii')
            mode_field = f"{100644:<8}".encode('ascii')
            size_field = f"{len(data):<10}".encode('ascii')
            trailer = b'`\n'

            header = name_field + mtime_field + uid_field + gid_field + mode_field + size_field + trailer
            assert len(header) == 60, f"Header length must be 60, got {len(header)}"
            f.write(header)
            f.write(data)
            if len(data) % 2 != 0:
                f.write(b'\n')


def build_deb(binary_path, arch, out_dir):
    with open(binary_path, 'rb') as f:
        binary_data = f.read()

    icon_path = Path("localsend-repo/app/assets/img/logo-512.png")
    with open(icon_path, 'rb') as f:
        icon_data = f.read()

    license_path = Path("localsend-repo/LICENSE")
    with open(license_path, 'rb') as f:
        license_data = f.read()

    # Calculate installed size in KB
    total_uncompressed = (
        len(binary_data)
        + len(icon_data)
        + len(license_data)
        + len(WRAPPER_SCRIPT.encode('utf-8'))
        + len(DEFAULT_CONFIG_TEMPLATE.encode('utf-8'))
        + len(DESKTOP_ENTRY.encode('utf-8'))
        + len(SYSTEMD_SERVICE.encode('utf-8'))
        + len(README_DOC.encode('utf-8'))
    )
    installed_size_kb = (total_uncompressed + 1023) // 1024

    control_content = CONTROL_TEMPLATE.format(
        version=VERSION,
        arch=arch,
        installed_size=installed_size_kb
    ).encode('utf-8')

    # 1. Build control.tar.gz
    control_entries = [
        {'name': './', 'mode': 0o755, 'type': tarfile.DIRTYPE},
        {'name': './control', 'mode': 0o644, 'type': tarfile.REGTYPE, 'data': control_content},
        {'name': './postinst', 'mode': 0o755, 'type': tarfile.REGTYPE, 'data': POSTINST_SCRIPT.encode('utf-8')},
        {'name': './postrm', 'mode': 0o755, 'type': tarfile.REGTYPE, 'data': POSTRM_SCRIPT.encode('utf-8')},
    ]
    control_tar_gz = make_tar_gz(control_entries)

    # 2. Build data.tar.gz
    data_entries = [
        {'name': './', 'mode': 0o755, 'type': tarfile.DIRTYPE},
        {'name': './etc', 'mode': 0o755, 'type': tarfile.DIRTYPE},
        {'name': './etc/localsend-cli', 'mode': 0o755, 'type': tarfile.DIRTYPE},
        {
            'name': './etc/localsend-cli/config.toml',
            'mode': 0o644,
            'type': tarfile.REGTYPE,
            'data': DEFAULT_CONFIG_TEMPLATE.encode('utf-8')
        },
        {'name': './usr', 'mode': 0o755, 'type': tarfile.DIRTYPE},
        {'name': './usr/lib', 'mode': 0o755, 'type': tarfile.DIRTYPE},
        {'name': './usr/lib/localsend-cli', 'mode': 0o755, 'type': tarfile.DIRTYPE},
        {
            'name': './usr/lib/localsend-cli/localsend-cli-bin',
            'mode': 0o755,
            'type': tarfile.REGTYPE,
            'data': binary_data
        },
        {'name': './usr/bin', 'mode': 0o755, 'type': tarfile.DIRTYPE},
        {
            'name': './usr/bin/localsend-cli',
            'mode': 0o755,
            'type': tarfile.REGTYPE,
            'data': WRAPPER_SCRIPT.encode('utf-8')
        },
        {
            'name': './usr/bin/localsend',
            'mode': 0o777,
            'type': tarfile.SYMTYPE,
            'linkname': 'localsend-cli'
        },
        {'name': './usr/share', 'mode': 0o755, 'type': tarfile.DIRTYPE},
        {'name': './usr/share/applications', 'mode': 0o755, 'type': tarfile.DIRTYPE},
        {
            'name': './usr/share/applications/localsend-cli.desktop',
            'mode': 0o644,
            'type': tarfile.REGTYPE,
            'data': DESKTOP_ENTRY.encode('utf-8')
        },
        {'name': './usr/share/icons', 'mode': 0o755, 'type': tarfile.DIRTYPE},
        {'name': './usr/share/icons/hicolor', 'mode': 0o755, 'type': tarfile.DIRTYPE},
        {'name': './usr/share/icons/hicolor/512x512', 'mode': 0o755, 'type': tarfile.DIRTYPE},
        {'name': './usr/share/icons/hicolor/512x512/apps', 'mode': 0o755, 'type': tarfile.DIRTYPE},
        {
            'name': './usr/share/icons/hicolor/512x512/apps/localsend.png',
            'mode': 0o644,
            'type': tarfile.REGTYPE,
            'data': icon_data
        },
        {'name': './usr/share/doc', 'mode': 0o755, 'type': tarfile.DIRTYPE},
        {'name': './usr/share/doc/localsend-cli', 'mode': 0o755, 'type': tarfile.DIRTYPE},
        {
            'name': './usr/share/doc/localsend-cli/README.md',
            'mode': 0o644,
            'type': tarfile.REGTYPE,
            'data': README_DOC.encode('utf-8')
        },
        {
            'name': './usr/share/doc/localsend-cli/copyright',
            'mode': 0o644,
            'type': tarfile.REGTYPE,
            'data': license_data
        },
        {'name': './usr/lib/systemd', 'mode': 0o755, 'type': tarfile.DIRTYPE},
        {'name': './usr/lib/systemd/user', 'mode': 0o755, 'type': tarfile.DIRTYPE},
        {
            'name': './usr/lib/systemd/user/localsend-cli.service',
            'mode': 0o644,
            'type': tarfile.REGTYPE,
            'data': SYSTEMD_SERVICE.encode('utf-8')
        },
    ]
    data_tar_gz = make_tar_gz(data_entries)

    # 3. Assemble .deb
    deb_filename = f"localsend-cli_{VERSION}_{arch}.deb"
    out_path = Path(out_dir) / deb_filename

    ar_files = [
        ('debian-binary', b'2.0\n'),
        ('control.tar.gz', control_tar_gz),
        ('data.tar.gz', data_tar_gz),
    ]

    write_ar_archive(out_path, ar_files)
    print(f"Created: {out_path} ({os.path.getsize(out_path)} bytes)")
    return out_path


def main():
    os.makedirs("dist", exist_ok=True)
    build_deb("extracted/linux-x86-64/localsend-cli", "amd64", "dist")
    build_deb("extracted/linux-arm-64/localsend-cli", "arm64", "dist")
    build_deb("extracted/linux-arm-64/localsend-cli", "aarch64", "dist")
    print("All deb packages generated successfully!")


if __name__ == "__main__":
    main()
