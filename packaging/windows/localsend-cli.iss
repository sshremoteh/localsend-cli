; LocalSend CLI Inno Setup Script
; Generates Windows Installer executable (.exe)

#define MyAppName "LocalSend CLI"
#define MyAppVersion "1.0.0"
#define MyAppPublisher "LocalSend Team"
#define MyAppURL "https://localsend.org"
#define MyAppExeName "localsend-cli.exe"

[Setup]
AppId={{5E4B2892-7A9C-4CD6-981F-D8B4D16F712A}}
AppName={#MyAppName}
AppVersion={#MyAppVersion}
AppPublisher={#MyAppPublisher}
AppPublisherURL={#MyAppURL}
AppSupportURL={#MyAppURL}
AppUpdatesURL={#MyAppURL}
DefaultDirName={autopf}\{#MyAppName}
DisableProgramGroupPage=yes
PrivilegesRequiredOverridesAllowed=dialog
OutputDir=..\..\dist
OutputBaseFilename=LocalSend-CLI-1.0.0-windows-x86_64-setup
SetupIconFile=logo.ico
UninstallDisplayIcon={app}\{#MyAppExeName}
Compression=lzma2/ultra
SolidCompression=yes
WizardStyle=modern
ChangesEnvironment=yes
ArchitecturesInstallIn64BitMode=x64compatible
ArchitecturesAllowed=x64compatible

[Languages]
Name: "english"; MessagesFile: "compiler:Default.isl"
Name: "russian"; MessagesFile: "compiler:Languages\Russian.isl"

[CustomMessages]
english.AddToPath=Add to PATH environment variable (recommended)
russian.AddToPath=Добавить в переменную среды PATH (рекомендуется)
english.ContextMenu=Add "Send via LocalSend CLI" to Explorer context menu
russian.ContextMenu=Добавить «Отправить через LocalSend CLI» в контекстное меню Проводника
english.SendWith=Send via LocalSend CLI
russian.SendWith=Отправить через LocalSend CLI

[Tasks]
Name: "addtopath"; Description: "{cm:AddToPath}"; GroupDescription: "PATH:"
Name: "contextmenu"; Description: "{cm:ContextMenu}"; GroupDescription: "{cm:AdditionalIcons}"
Name: "desktopicon"; Description: "{cm:CreateDesktopIcon}"; GroupDescription: "{cm:AdditionalIcons}"; Flags: unchecked

[Files]
Source: "localsend-cli.exe"; DestDir: "{app}"; Flags: ignoreversion
Source: "localsend.exe"; DestDir: "{app}"; Flags: ignoreversion
Source: "logo.ico"; DestDir: "{app}"; Flags: ignoreversion
Source: "README.txt"; DestDir: "{app}"; Flags: ignoreversion
Source: "LICENSE.txt"; DestDir: "{app}"; Flags: ignoreversion

[Icons]
Name: "{autoprograms}\{#MyAppName}"; Filename: "{app}\{#MyAppExeName}"; IconFilename: "{app}\logo.ico"
Name: "{autodesktop}\{#MyAppName}"; Filename: "{app}\{#MyAppExeName}"; IconFilename: "{app}\logo.ico"; Tasks: desktopicon

[Registry]
; Context menu for files
Root: HKA; Subkey: "Software\Classes\*\shell\LocalSendCLI"; ValueType: string; ValueData: "{cm:SendWith}"; Flags: uninsdeletekey; Tasks: contextmenu
Root: HKA; Subkey: "Software\Classes\*\shell\LocalSendCLI"; ValueType: string; ValueName: "Icon"; ValueData: """{app}\logo.ico"""; Flags: uninsdeletekey; Tasks: contextmenu
Root: HKA; Subkey: "Software\Classes\*\shell\LocalSendCLI\command"; ValueType: string; ValueData: """{app}\{#MyAppExeName}"" -f ""%1"""; Flags: uninsdeletekey; Tasks: contextmenu

; Context menu for directories
Root: HKA; Subkey: "Software\Classes\Directory\shell\LocalSendCLI"; ValueType: string; ValueData: "{cm:SendWith}"; Flags: uninsdeletekey; Tasks: contextmenu
Root: HKA; Subkey: "Software\Classes\Directory\shell\LocalSendCLI"; ValueType: string; ValueName: "Icon"; ValueData: """{app}\logo.ico"""; Flags: uninsdeletekey; Tasks: contextmenu
Root: HKA; Subkey: "Software\Classes\Directory\shell\LocalSendCLI\command"; ValueType: string; ValueData: """{app}\{#MyAppExeName}"" -f ""%1"""; Flags: uninsdeletekey; Tasks: contextmenu

[Run]
Filename: "{app}\{#MyAppExeName}"; Description: "{cm:LaunchProgram,{#StringChange(MyAppName, '&', '&&')}}"; Flags: nowait postinstall skipifsilent

[Code]
const
  EnvironmentKey = 'SYSTEM\CurrentControlSet\Control\Session Manager\Environment';

procedure CurStepChanged(CurStep: TSetupStep);
var
  AppDir, OldPath, NewPath: string;
begin
  if (CurStep = ssPostInstall) and WizardIsTaskSelected('addtopath') then
  begin
    AppDir := ExpandConstant('{app}');
    if IsAdminInstallMode then
    begin
      if RegQueryStringValue(HKEY_LOCAL_MACHINE, EnvironmentKey, 'Path', OldPath) then
      begin
        if Pos(';' + AppDir + ';', ';' + OldPath + ';') = 0 then
        begin
          NewPath := OldPath + ';' + AppDir;
          RegWriteStringValue(HKEY_LOCAL_MACHINE, EnvironmentKey, 'Path', NewPath);
        end;
      end;
    end
    else
    begin
      if not RegQueryStringValue(HKEY_CURRENT_USER, 'Environment', 'Path', OldPath) then
        OldPath := '';
      if Pos(';' + AppDir + ';', ';' + OldPath + ';') = 0 then
      begin
        if OldPath = '' then
          NewPath := AppDir
        else
          NewPath := OldPath + ';' + AppDir;
        RegWriteStringValue(HKEY_CURRENT_USER, 'Environment', 'Path', NewPath);
      end;
    end;
  end;
end;

procedure CurUninstallStepChanged(CurUninstallStep: TUninstallStep);
var
  AppDir, OldPath: string;
  P, L: Integer;
begin
  if CurUninstallStep = usUninstall then
  begin
    AppDir := ExpandConstant('{app}');
    if IsAdminInstallMode then
    begin
      if RegQueryStringValue(HKEY_LOCAL_MACHINE, EnvironmentKey, 'Path', OldPath) then
      begin
        P := Pos(';' + AppDir, ';' + OldPath);
        if P > 0 then
        begin
          OldPath := ';' + OldPath;
          L := Length(AppDir) + 1;
          Delete(OldPath, P, L);
          if (Length(OldPath) > 0) and (OldPath[1] = ';') then
            Delete(OldPath, 1, 1);
          RegWriteStringValue(HKEY_LOCAL_MACHINE, EnvironmentKey, 'Path', OldPath);
        end;
      end;
    end
    else
    begin
      if RegQueryStringValue(HKEY_CURRENT_USER, 'Environment', 'Path', OldPath) then
      begin
        P := Pos(';' + AppDir, ';' + OldPath);
        if P > 0 then
        begin
          OldPath := ';' + OldPath;
          L := Length(AppDir) + 1;
          Delete(OldPath, P, L);
          if (Length(OldPath) > 0) and (OldPath[1] = ';') then
            Delete(OldPath, 1, 1);
          RegWriteStringValue(HKEY_CURRENT_USER, 'Environment', 'Path', OldPath);
        end;
      end;
    end;
  end;
end;
