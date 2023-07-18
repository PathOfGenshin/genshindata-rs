import { generateExcelBinOutput } from "./generate/excelbinoutput.mjs";
import { generateLanguageEnum } from "./generate/languages.mjs";
import { updateReadmeVersion } from "./version.mjs";

async function main() {
  await Promise.all([generateExcelBinOutput(), generateLanguageEnum()]);
  console.log("Done!");
  await updateReadmeVersion();
  // TODO: generate BinOutput Avatar/ConfigAvatar_*.json structs
}

main();
