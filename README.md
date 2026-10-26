# LocalSend CLI — Windows & Linux Installers

> **LocalSend CLI** — официальный консольный клиент для [LocalSend](https://github.com/localsend/localsend), позволяющий безопасно и быстро передавать файлы и сообщения между устройствами в локальной сети без интернета.

Данный репозиторий содержит инструменты для сборки полноценных установочных пакетов:
* **Windows**: инсталляторы формата `.exe` (Inno Setup) с автоматической настройкой `PATH`, интеграцией в контекстное меню Проводника и деинсталлятором.
* **Linux**: пакеты формата `.deb` для **amd64**, **arm64** и **aarch64** с поддержкой `systemd`-демона, `.desktop` ярлыка и автодополнения.

---

## Скачать установщики (Releases)

Все готовые пакеты доступны в разделе [Releases](https://github.com/sshremoteh/localsend-cli/releases):

| Платформа | Архитектура | Формат | Ссылка |
| :--- | :--- | :--- | :--- |
| **Windows** | x86_64 (64-bit) | `.exe` Setup | [LocalSend-CLI-1.18.2-windows-x86_64-setup.exe](https://github.com/sshremoteh/localsend-cli/releases/download/v1.18.2/LocalSend-CLI-1.18.2-windows-x86_64-setup.exe) |
| **Windows** | ARM64 | `.exe` Setup | [LocalSend-CLI-1.18.2-windows-arm64-setup.exe](https://github.com/sshremoteh/localsend-cli/releases/download/v1.18.2/LocalSend-CLI-1.18.2-windows-arm64-setup.exe) |
| **Linux** | AArch64 | `.deb` | [localsend-cli_1.18.2_aarch64.deb](https://github.com/sshremoteh/localsend-cli/releases/download/v1.18.2/localsend-cli_1.18.2_aarch64.deb) |
| **Linux** | ARM64 (Debian/Ubuntu) | `.deb` | [localsend-cli_1.18.2_arm64.deb](https://github.com/sshremoteh/localsend-cli/releases/download/v1.18.2/localsend-cli_1.18.2_arm64.deb) |
| **Linux** | amd64 (x86_64) | `.deb` | [localsend-cli_1.18.2_amd64.deb](https://github.com/sshremoteh/localsend-cli/releases/download/v1.18.2/localsend-cli_1.18.2_amd64.deb) |
| **Linux** | AArch64 (ELF binary) | Binary | [localsend-cli-1.18.2-linux-aarch64](https://github.com/sshremoteh/localsend-cli/releases/download/v1.18.2/localsend-cli-1.18.2-linux-aarch64) |

Хэши файлов для проверки целостности: [SHA256SUMS.txt](https://github.com/sshremoteh/localsend-cli/releases/download/v1.18.2/SHA256SUMS.txt).

---

## Установка

### Windows (EXE)
1. Скачайте и запустите `LocalSend-CLI-1.18.2-windows-x86_64-setup.exe`.
2. Мастер установки предложит:
   * Добавить каталог программы в системную переменную `PATH` (рекомендуется).
   * Добавить пункт **«Отправить через LocalSend CLI»** в контекстное меню файлов и папок Проводника.
   * Создать ярлыки в меню «Пуск» и на рабочем столе.
3. После установки откройте новое окно PowerShell или CMD и введите `localsend` или `localsend-cli`.

Тихая установка (без диалоговых окон):
```powershell
.\LocalSend-CLI-1.18.2-windows-x86_64-setup.exe /VERYSILENT /NORESTART
```

### Linux (Debian / Ubuntu / Raspberry Pi OS / Armbian)
```bash
# Для архитектуры aarch64 / arm64:
sudo apt install ./localsend-cli_1.18.2_arm64.deb
# или:
sudo dpkg -i localsend-cli_1.18.2_aarch64.deb

# Для архитектуры x86_64 (amd64):
sudo apt install ./localsend-cli_1.18.2_amd64.deb
```

---

## Использование

### 1. Интерактивный TUI-режим
Запуск информационного табло приёма/отправки:
```bash
localsend
# или:
localsend-cli
```

**Горячие клавиши в интерактивном режиме:**
* `1`–`9` — мгновенная отправка файлов на устройство под соответствующим номером.
* `D` — показать список обнаруженных устройств в локальной сети.
* `W` + `S` — включить раздачу файлов по HTTP-ссылке для браузеров.
* `W` + `R` — включить приём файлов через веб-интерфейс в браузере.
* `Y` / `N` / `P` — принять / отклонить / принять и связать входящую передачу.
* `Ctrl+C` — отмена передачи / выход.

### 2. Быстрая отправка файлов через терминал
```bash
# Отправка одного файла
localsend-cli -f /path/to/archive.zip

# Отправка нескольких файлов
localsend-cli -f file1.txt -f photo.jpg
```

### 3. Опции запуска
* `--alias <NAME>` — имя вашего устройства, видимое другим участникам сети.
* `--port <PORT>` — порт сервера (по умолчанию `53317`).
* `--destination <DIR>` — каталог для сохранения входящих файлов (по умолчанию `~/Downloads`).

---

## Фоновый приём файлов через systemd (Linux)

Пакет `.deb` включает юнит пользовательского сервиса systemd:
```bash
# Запустить сервис постоянного приёма:
systemctl --user enable --now localsend-cli

# Проверить статус:
systemctl --user status localsend-cli

# Остановить сервис:
systemctl --user stop localsend-cli
```

---

## Настройка фаервола

Для работы протокола LocalSend требуются открытые порты в локальной сети:
* **Порт 53317 UDP** — мультикаст и обнаружение устройств.
* **Порт 53317 TCP** — передача файлов по HTTP/HTTPS.

В Linux (UFW):
```bash
sudo ufw allow 53317/tcp
sudo ufw allow 53317/udp
```
