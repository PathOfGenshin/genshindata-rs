# genshindata-rs

A strongly-typed Rust based library wrapper for
[GenshinData](https://github.com/Dimbreath/GenshinData) JSON files.

Uses [quicktype](https://github.com/quicktype/quicktype) to generate `serde` compatible
Rust structs.

## Development

Note that this is still a work-in-progress. Currently only generates serde structs for
`ExcelBinOutput` JSON files.

### Generating `serde` structs from GenshinData JSON

This assumes that we are working in a Windows environment with PowerShell installed.

1. Ensure that `quicktype` is globally installed via `npm` or `yarn`
2. Ensure that `GenshinData` submodule is cloned and up-to-date
3. Run `.\scripts\Sync-ExcelBinOutput.ps1` in PowerShell
4. Double-check generated `.rs` files in `genshindata-rs/src/excelbinoutput/`

---

Currently updated for game version: **3.1.0**
