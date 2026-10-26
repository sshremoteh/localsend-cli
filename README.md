# LocalSend CLI — Windows & Linux Installers

> **LocalSend CLI** — официальный консольный клиент для [LocalSend](https://github.com/localsend/localsend), позволяющий безопасно и быстро передавать файлы и сообщения между устройствами в локальной сети без интернета.

Данный репозиторий содержит расширенную версию LocalSend CLI со встроенным **интерактивным меню настроек прямо внутри запущенного приложения**, а также готовые установочные пакеты:
* **Windows**: инсталляторы `.exe` (Inno Setup) с добавлением в `PATH`, контекстным меню Проводника и деинсталлятором.
* **Linux**: пакеты `.deb` для **amd64**, **arm64** и **aarch64** с защитой от конфликта портов, шаблоном конфигурации и `systemd`-сервисом.

---

## ⚙️ Меню настроек внутри запущенного приложения (TUI)

В запущенном приложении нажмите клавишу **`S`** (или **`C`** / **`O`**), чтобы открыть интерактивное окно настроек:

```text
┌────────────────────── LocalSend Settings ──────────────────────┐
│                                                                │
│  ▶ 1. Device Alias:          [ My-Linux-Server ]               │
│    2. HTTP Server Port:      [ 53318 ]                         │
│    3. Destination Folder:    [ /root/Downloads ]               │
│    4. Auto-Accept Transfers: [ Paired Devices Only ]           │
│    5. Organize Downloads:    [ By Sender Device ]              │
│    6. Multicast Discovery:   [ Active ]                        │
│    7. Transfer History:      [ Enabled ]                       │
│                                                                │
│       [ SAVE AND APPLY SETTINGS ]                              │
│       [ Cancel / Return ]                                      │
│                                                                │
│  ↑/↓: Навигация  Enter/Space: Изменить/Переключить  S: Сохранить
└────────────────────────────────────────────────────────────────┘
```

### Доступные настройки:
1. **Device Alias** — изменение имени вашего устройства в сети на лету.
2. **HTTP Server Port** — смена порта (например, с 53317 на 53318), если порт занят другим процессом.
3. **Destination Folder** — выбор папки для сохранения входящих файлов.
4. **Auto-Accept Transfers** — автоприём файлов:
   * `Off (Always Ask)` — спрашивать подтверждение для каждой передачи.
   * `Paired Devices Only` — автоматически принимать только от сопряжённых устройств.
   * `Everyone (Always)` — принимать файлы от всех в локальной сети без лишних вопросов.
5. **Organize Downloads** — автоматическая раскладка принятых файлов:
   * `Flat (Disabled)` — сохранять прямо в общую папку.
   * `By Sender Device` — создавать подпапку с именем отправителя.
   * `By Date` — раскладывать по папкам дат `YYYY-MM-DD`.
6. **Multicast Discovery** — переключение режима обнаружения: `Active` (обычный) или `Stealth (Off)` (скрытый режим).
7. **Transfer History** — ведение журнала переданных файлов.

Нажмите **`S`** или выберите пункт **`[ SAVE AND APPLY SETTINGS ]`**, чтобы применить изменения и автоматически записать их в файл `~/.config/localsend-cli/config.toml`.

---

## 📥 Скачать установщики (Releases)

Все готовые пакеты доступны в разделе [Releases v1.0](https://github.com/sshremoteh/localsend-cli/releases/tag/v1.0):

| Платформа | Архитектура | Формат | Ссылка |
| :--- | :--- | :--- | :--- |
| **Windows** | x86_64 (64-bit) | `.exe` Setup | [LocalSend-CLI-1.0.0-windows-x86_64-setup.exe](https://github.com/sshremoteh/localsend-cli/releases/download/v1.0/LocalSend-CLI-1.0.0-windows-x86_64-setup.exe) |
| **Windows** | ARM64 | `.exe` Setup | [LocalSend-CLI-1.0.0-windows-arm64-setup.exe](https://github.com/sshremoteh/localsend-cli/releases/download/v1.0/LocalSend-CLI-1.0.0-windows-arm64-setup.exe) |
| **Linux** | ARM64 / AArch64 | `.deb` | [localsend-cli_1.0.0_arm64.deb](https://github.com/sshremoteh/localsend-cli/releases/download/v1.0/localsend-cli_1.0.0_arm64.deb) |
| **Linux** | AArch64 (alias) | `.deb` | [localsend-cli_1.0.0_aarch64.deb](https://github.com/sshremoteh/localsend-cli/releases/download/v1.0/localsend-cli_1.0.0_aarch64.deb) |
| **Linux** | amd64 (x86_64) | `.deb` | [localsend-cli_1.0.0_amd64.deb](https://github.com/sshremoteh/localsend-cli/releases/download/v1.0/localsend-cli_1.0.0_amd64.deb) |
| **Linux** | AArch64 (ELF) | Binary | [localsend-cli-1.0.0-linux-aarch64](https://github.com/sshremoteh/localsend-cli/releases/download/v1.0/localsend-cli-1.0.0-linux-aarch64) |

Хэши файлов: [SHA256SUMS.txt](https://github.com/sshremoteh/localsend-cli/releases/download/v1.0/SHA256SUMS.txt).

---

## 🚀 Установка

### Linux (AArch64 / ARM64):
> [!IMPORTANT]
> При скачивании через `curl` обязательно используйте флаг **`-L`** (или `-fsSL`), чтобы следовать редиректам GitHub Assets.

```bash
# Скачивание пакета с переходом по редиректу:
curl -fsSL -o localsend-cli.deb "https://github.com/sshremoteh/localsend-cli/releases/download/v1.0/localsend-cli_1.0.0_arm64.deb"

# Установка:
sudo apt install ./localsend-cli.deb
# или:
sudo dpkg -i ./localsend-cli.deb
```

### Windows (EXE):
1. Скачайте `LocalSend-CLI-1.0.0-windows-x86_64-setup.exe`.
2. Запустите установщик (он автоматически добавит программу в `PATH` и добавит пункт контекстного меню).
3. Введите `localsend` в командной строке.

---

## ⌨️ Горячие клавиши в интерактивном режиме

* **`S`** / **`C`** — открыть меню интерактивных настроек.
* **`D`** — список сопряжённых и обнаруженных устройств.
* **`1`–`9`** — отправить файлы устройству с указанным номером.
* **`W` + `S`** — раздать файлы по ссылке (скачивание через браузер).
* **`W` + `R`** — принять файлы по ссылке (загрузка из браузера).
* **`Y` / `N` / `P`** — принять / отклонить / принять и сопрячь устройство.
* **`Ctrl+C`** — отмена операции или выход.
