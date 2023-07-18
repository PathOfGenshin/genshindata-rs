import * as path from "path";

export const CWD = process.cwd();
export const GAME_DATA_FOLDER =
  process.env.GAME_DATA_FOLDER || path.join(CWD, "..", "AnimeGameData");
export const EXCEL_BIN_OUTPUT_FOLDER = path.join(GAME_DATA_FOLDER, "ExcelBinOutput");
export const TEXTMAP_FOLDER = path.join(GAME_DATA_FOLDER, "TextMap");
export const RUST_OUTPUT_SRC_FOLDER = path.join(CWD, "genshindata-rs", "src");
