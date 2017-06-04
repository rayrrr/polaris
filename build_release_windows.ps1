Get-ChildItem "Cargo.toml" | % {
  $conf = $_ | Get-Content -raw
  $conf -match 'version\s+=\s+"(.*)"' | out-null
  $POLARIS_VERSION = $matches[1]
}

"Compiling resource file"
RC /fo res\windows\application\application.res res\windows\application\application.rc

""
"Compiling executable"
cargo rustc --release --features "ui" -- -C link-args="/SUBSYSTEM:WINDOWS /ENTRY:mainCRTStartup res\windows\application\application.res"

""
"Creating output directory"
New-Item .\release\tmp -type directory -Force | Out-Null
Remove-Item -Recurse .\release\tmp\*

""
"Copying to output directory"
Copy-Item .\res\windows\installer\license.rtf	.\release\tmp\
Copy-Item .\res\windows\installer\banner.bmp	.\release\tmp\
Copy-Item .\res\windows\installer\dialog.bmp	.\release\tmp\
Copy-Item .\target\release\polaris.exe 			.\release\tmp\
Copy-Item .\res\default_config.toml 			.\release\tmp\polaris.toml
Copy-Item .\web\ 								.\release\tmp\ -recurse

""
"Creating installer"
candle -wx -ext WixUtilExtension -arch x64							-out .\release\tmp\installer.wixobj 			.\res\windows\installer\installer.wxs
light  -wx -ext WixUtilExtension -ext WixUIExtension -spdb -sw1076 	-out .\release\Polaris_$POLARIS_VERSION.msi 	.\release\tmp\installer.wixobj

"Cleaning up"
Remove-Item -Recurse .\release\tmp

""
Read-Host -Prompt "All clear! Press Enter to exit"
