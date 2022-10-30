# This script creates a mod.rs file containing all the generated serde_json structs for ExcelBinOutput JSON files
$files = [System.Collections.ArrayList]@(
    "#![cfg_attr(rustfmt, rustfmt_skip)]",
    "#![allow(non_snake_case)]",
    "#![allow(non_camel_case_types)]"
)

Get-ChildItem -Path .\src\excelbinoutput\ -Exclude "mod.rs" |
ForEach-Object {
    $file = [System.IO.Path]::GetFileNameWithoutExtension($_)
    [void]$files.Add("pub mod $file;")
}

$files -join "`n" | Out-File .\src\excelbinoutput\mod.rs
