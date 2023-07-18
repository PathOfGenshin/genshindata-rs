import { generateExcelBinOutput } from "./generate/excelbinoutput.mjs";
import { generateLanguageEnum } from "./generate/languages.mjs";

async function main() {
  await Promise.all([generateExcelBinOutput(), generateLanguageEnum()]);
  console.log("Done!");
  // TODO: generate BinOutput Avatar/ConfigAvatar_*.json structs
}

main();
