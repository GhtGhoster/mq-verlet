#!/bin/bash
echo `# <#`

# Bash goes here

# PROJECT_NAME=$(find . -type d -links 2)
PROJECT_NAME=${PWD/*\//}

cargo build --release --target wasm32-unknown-unknown

echo -e "\033[1;34mGenerating WASM Binds"
mkdir -p wbindgen
wasm-bindgen --target web --out-dir "./wbindgen/" "./target/wasm32-unknown-unknown/release/$PROJECT_NAME.wasm"

echo -e "\033[1;34mPatching generated JavaScript"
sed -i "s/import \* as __wbg_star0 from 'env';//" wbindgen/$PROJECT_NAME.js
sed -i "s/let wasm;/let wasm; export const set_wasm = (w) => wasm = w;/" wbindgen/$PROJECT_NAME.js
sed -i "s/imports\['env'\] = __wbg_star0;/return imports.wbg\;/" wbindgen/$PROJECT_NAME.js
sed -i "s/const imports = getImports();/return getImports();/" wbindgen/$PROJECT_NAME.js

echo -e "\033[1;34mMoving files to ./web_res/ directory"
mv wbindgen/$PROJECT_NAME.js web_res/$PROJECT_NAME.js
mv wbindgen/${PROJECT_NAME}_bg.wasm web_res/$PROJECT_NAME.wasm

echo -e "\033[1;34mDone!"
exit
#> > $null

# PowerShell goes here

$ProjectName = Split-Path -Leaf $PWD

cargo build --release --target wasm32-unknown-unknown

if (!($?)) {
    Write-Host -ForegroundColor Red "Build failed!"
    exit 1
}

Write-Host -ForegroundColor Blue "Generating WASM binds"
New-Item -ItemType Directory -Force -Path ".\wbindgen\"
wasm-bindgen --target web --out-dir ".\wbindgen\" ".\target\wasm32-unknown-unknown\release\$ProjectName.wasm"

if (!($?)) {
    Write-Host -ForegroundColor Red "Bind generation failed!"
    exit 1
}

Write-Host -ForegroundColor Blue "Patching generated JavaScript"
$JavaScript = Get-Content ".\wbindgen\$ProjectName.js"
$JavaScript = $JavaScript | ForEach-Object{$_ -replace [regex]::Escape("import * as __wbg_star0 from 'env';")}
$JavaScript = $JavaScript | ForEach-Object{$_ -replace [regex]::Escape("let wasm;") ,"let wasm; export const set_wasm = (w) => wasm = w;"}
$JavaScript = $JavaScript | ForEach-Object{$_ -replace [regex]::Escape("imports['env'] = __wbg_star0;"), "return imports.wbg;"}
$JavaScript = $JavaScript | ForEach-Object{$_ -replace [regex]::Escape("const imports = getImports();"), "return getImports();"}

Write-Host -ForegroundColor Blue "Moving files to .\web_res\ directory"
[System.IO.File]::WriteAllLines(".\web_res\$ProjectName.js", $JavaScript)
Move-Item -Force ".\wbindgen\$($ProjectName)_bg.wasm" ".\web_res\$ProjectName.wasm"

Write-Host -ForegroundColor Blue "Done!"
