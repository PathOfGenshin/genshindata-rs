# This script creates a mod.rs file containing all the generated serde_json structs for ExcelBinOutput JSON files
$files = [System.Collections.ArrayList]::new()

Get-ChildItem -Path ".\genshindata-rs\src\excelbinoutput\" -Exclude "mod.rs" |
ForEach-Object {
    $file = [System.IO.Path]::GetFileNameWithoutExtension($_)
    [void]$files.Add($file)
}

$mod_entry = { "pub mod $($_);" }

@"
#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
$(($files | ForEach-Object $mod_entry) -join "`n")
"@ | Out-File ".\genshindata-rs\src\excelbinoutput\mod.rs"
