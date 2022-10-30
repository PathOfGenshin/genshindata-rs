# This script uses quicktype to generate serde_json compatible Rust structs from GenshinData's ExcelBinOutput folder JSON files
# Quicktype tool for reference: https://github.com/quicktype
Get-ChildItem -Path .\GenshinData\ExcelBinOutput\ -Filter *.json |
ForEach-Object {
    $file = [System.IO.Path]::GetFileNameWithoutExtension($_)
    Write-Host "Running quicktype $($_.FullName) -o .\src\excelbinoutput\$file.rs --density normal --visibility public" -ForegroundColor green
    quicktype $_.FullName -o .\src\excelbinoutput\$file.rs --density normal --visibility public
}
