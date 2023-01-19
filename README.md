A strongly-typed Rust based library wrapper for game resource JSON files.

Uses [quicktype](https://github.com/quicktype/quicktype) to generate `serde` compatible
Rust structs.

## Development

Note that this is still a work-in-progress. Currently only generates serde structs for
`ExcelBinOutput` JSON files.

### Generating `serde` structs from JSON

This assumes that we are working in a Windows environment with PowerShell installed.

1. Ensure that `quicktype` is globally installed via `npm`/`pnpm`/`yarn`
2. Ensure that the `GAME_DATA_PATH` environment variable is set to the path containing
   the game's JSON `ExcelBinOutput` resource folder (we don't provide that here, you'll
   have to get that yourself).
3. Run `.\scripts\Sync-ExcelBinOutput.ps1` in PowerShell
4. Double-check generated `.rs` files in `genshindata-rs/src/excelbinoutput/`

---

Currently updated for game version: **3.4.0**
