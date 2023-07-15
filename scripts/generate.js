import * as fs from "fs/promises";
import * as path from "path";
import { InputData, jsonInputForTargetLanguage, quicktype } from "quicktype-core";

const GAME_DATA_FOLDER =
  process.env.GAME_DATA_FOLDER || path.join(process.cwd(), "..", "AnimeGameData");
const EXCEL_BIN_OUTPUT_FOLDER = path.join(GAME_DATA_FOLDER, "ExcelBinOutput");

const RUST_OUTPUT_FOLDER = path.join(
  process.cwd(),
  "genshindata-rs",
  "src",
  "excelbinoutput"
);

const RUST_QUICKTYPE_OPTIONS = {
  density: "dense",
  visibility: "public",
  "derive-debug": true,
  "derive-clone": true,
  "edition-2018": true,
  "leading-comments": false,
};

async function quicktypeJSON(typeName, jsonString) {
  const jsonInput = jsonInputForTargetLanguage("rust");

  // Generate a single type from a single JSON file input
  await jsonInput.addSource({
    name: typeName,
    samples: [jsonString],
  });

  const inputData = new InputData();
  inputData.addInput(jsonInput);

  return await quicktype({
    inputData,
    lang: "rust",
    leadingComments: undefined,
    rendererOptions: RUST_QUICKTYPE_OPTIONS,
  });
}

async function getExcelBinOutputJsonFiles() {
  return (await fs.readdir(EXCEL_BIN_OUTPUT_FOLDER)).map(
    (fileName) => fileName.split(".json")[0]
  );
}

async function generateExcelBinOutputStructs(filesNoExt) {
  const generateStructs = filesNoExt.map(async (fileNameNoExt) => {
    const fileContents = fs.readFile(
      path.join(EXCEL_BIN_OUTPUT_FOLDER, `${fileNameNoExt}.json`),
      "utf8"
    );
    const { lines: rustFile } = await quicktypeJSON(fileNameNoExt, await fileContents);

    const content = [
      "/// This file was automatically generated by quicktype",
      "/// DO NOT MANUALLY EDIT THIS FILE!",
      "",
      "#[allow(unused_imports)]",
      rustFile.join("\n"),
    ].join("\n");
    const outputFilePath = path.join(RUST_OUTPUT_FOLDER, `${fileNameNoExt}.rs`);
    console.log(`Generating file ${path.relative(process.cwd(), outputFilePath)}`);
    return fs.writeFile(outputFilePath, content);
  });
  await Promise.all(generateStructs);
}

async function generateExcelBinOutputModFile(filesNoExt) {
  const pubMods = filesNoExt
    .map((fileNameNoExt) => `pub mod ${fileNameNoExt};`)
    .join("\n");
  const modFileContents = `${[
    "#![cfg_attr(rustfmt, rustfmt_skip)]",
    "#![allow(non_snake_case)]",
    "#![allow(non_camel_case_types)]",
    pubMods,
  ].join("\n")}\n`;
  const modFilePath = path.join(RUST_OUTPUT_FOLDER, "mod.rs");
  console.log(`Generating file ${path.relative(process.cwd(), modFilePath)}`);
  await fs.writeFile(modFilePath, modFileContents);
}

async function generateSerdeStructs() {
  const filesNoExt = await getExcelBinOutputJsonFiles();
  await Promise.all([
    generateExcelBinOutputStructs(filesNoExt),
    generateExcelBinOutputModFile(filesNoExt),
  ]);
}

async function main() {
  console.log(
    `Generating Rust files from ExcelBinOutput folder located at: ${EXCEL_BIN_OUTPUT_FOLDER}`
  );
  await Promise.all([generateSerdeStructs()]);
  console.log("Done!");
  // TODO: generate language enums
  // TODO: generate BinOutput Avatar/ConfigAvatar_*.json structs
}

main();
