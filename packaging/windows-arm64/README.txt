LocalSend CLI
=============
Version: 1.18.2
Website: https://localsend.org
Repository: https://github.com/localsend/localsend

LocalSend is an open-source cross-platform tool to securely share files
and messages with nearby devices over your local network.

Command-line Usage:
-------------------
1. Run interactive dashboard:
     localsend-cli
   or:
     localsend

2. Send a file to nearby devices:
     localsend-cli -f "C:\path\to\file.ext"
     localsend-cli -f "file1.txt" -f "file2.jpg"

3. Options:
     --alias <NAME>             Device name shown to other devices
     --port <PORT>              HTTP server port (default: 53317)
     --destination <DIR>        Directory where received files are saved
     -f, --file <PATH>          File to send (repeatable)
     -h, --help                 Show help message
     -V, --version              Show version information

Keyboard shortcuts in interactive mode:
--------------------------------------
  1-9    Send files to the device with that number
  D      Show paired and discovered devices
  W+S    Toggle share via link (serve files to web browsers over HTTP)
  W+R    Toggle receive via link (allow browsers to upload files)
  Y/N/P  Accept / Decline / Accept-and-Pair an incoming transfer
  Ctrl+C Cancel transfer or quit when idle

Environment variables:
----------------------
  LOCALSEND_ALIAS
  LOCALSEND_PORT
  LOCALSEND_DESTINATION
  XDG_CONFIG_HOME
