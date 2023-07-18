import * as fs from "fs/promises";
import * as path from "path";
import { CWD, RUST_OUTPUT_SRC_FOLDER, TEXTMAP_FOLDER } from "../env.mjs";
import { generateSimpleEnumSrc } from "../rust/enum.mjs";

const RUST_LANGUAGES_OUTPUT_FOLDER = path.join(RUST_OUTPUT_SRC_FOLDER, "language");
const LANGUAGE_REGEX = /^TextMap([A-Z]+)\.json$/;

/**
 * @param {string} fileName The file name including the ".json" extension
 * @returns {string}
 */
function extractLanguageCodeFromFileName(fileName) {
  return LANGUAGE_REGEX.exec(fileName)[1];
}

/**
 * @returns {Promise<string[]>}
 */
async function getTextMapLanguages() {
  return (await fs.readdir(TEXTMAP_FOLDER)).map(extractLanguageCodeFromFileName).sort();
}

export async function generateLanguageEnum() {
  console.log("Generating Language enum file...");
  const languages = await getTextMapLanguages();
  const rustSrc = generateSimpleEnumSrc("Language", languages);
  const outputFilePath = path.join(RUST_LANGUAGES_OUTPUT_FOLDER, "mod.rs");
  console.log(`Generating file ${path.relative(CWD, outputFilePath)}`);
  await fs.writeFile(outputFilePath, rustSrc);
}
