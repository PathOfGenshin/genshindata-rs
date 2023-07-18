A strongly-typed Rust based library wrapper for game resource JSON files.

Uses [quicktype](https://github.com/quicktype/quicktype) to generate `serde` compatible
Rust structs.

## Development

Note that this is still a work-in-progress. Currently only generates serde structs for
`ExcelBinOutput` JSON files.

### Generating `serde` structs from JSON

Ensure that `pnpm` and a `node` environment is installed (tested on `node v20.4.0`).

1. `pnpm install` to install the node dependencies needed to generate Rust code.
2. Ensure that the `GAME_DATA_FOLDER` environment variable is set to the path containing
   the game's JSON `ExcelBinOutput` resource folder (we don't provide that here, you'll
   have to get that yourself).
   If not set, this defaults to `../AnimeGameData/`
3. `pnpm generate` to generate the Rust `serde` structs code.
4. Double-check generated `.rs` files in `genshindata-rs/src/excelbinoutput/`

---

Currently updated for game version: **3.8.0**
