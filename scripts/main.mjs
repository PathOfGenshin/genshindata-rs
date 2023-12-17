import { generateExcelBinOutput } from "./generate/excelbinoutput.mjs";
import { generateLanguageEnum } from "./generate/languages.mjs";
import { updateVersions } from "./version.mjs";

async function main() {
  await Promise.all([
    generateExcelBinOutput(),
    generateLanguageEnum(),
    updateVersions(),
  ]);
  console.log("Done!");
  // TODO: generate BinOutput Avatar/ConfigAvatar_*.json structs
}

main();
