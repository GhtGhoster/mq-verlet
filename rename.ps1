#!/bin/bash
echo `# <#`

# Bash goes here

# PROJECT_NAME=$(find . -type d -links 2)
PROJECT_NAME=${PWD/*\//}

sed -i "s/mq-wbg-template/${PROJECT_NAME}/" src/main.rs
sed -i "s/mq-wbg-template/${PROJECT_NAME}/" Cargo.toml
sed -i "s/mq-wbg-template/${PROJECT_NAME}/" intex.html

cargo clean

exit
#> > $null

# PowerShell goes here

$NewName = Split-Path -Leaf $PWD

$Content = Get-Content .\src\main.rs | ForEach-Object{$_ -replace [regex]::Escape("mq-wbg-template"), $NewName}
[System.IO.File]::WriteAllLines(".\src\main.rs", $Content)

$Content = Get-Content .\Cargo.toml | ForEach-Object{$_ -replace [regex]::Escape("mq-wbg-template"), $NewName}
[System.IO.File]::WriteAllLines(".\Cargo.toml", $Content)

$Content = Get-Content .\index.html | ForEach-Object{$_ -replace [regex]::Escape("mq-wbg-template"), $NewName}
[System.IO.File]::WriteAllLines(".\index.html", $Content)

cargo clean
