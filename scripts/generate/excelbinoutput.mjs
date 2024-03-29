import * as fs from "fs/promises";
import * as path from "path";
import { quicktypeJSON } from "../quicktype.mjs";
import { CWD, EXCEL_BIN_OUTPUT_FOLDER, RUST_OUTPUT_SRC_FOLDER } from "../env.mjs";

const RUST_EXCEL_BIN_OUTPUT_FOLDER = path.join(
  RUST_OUTPUT_SRC_FOLDER,
  "excelbinoutput"
);

/**
 * @returns {Promise<string[]>}
 */
async function getExcelBinOutputJsonFiles() {
  return (await fs.readdir(EXCEL_BIN_OUTPUT_FOLDER)).map(
    (fileName) => fileName.split(".json")[0]
  );
}

/**
 * Replaces any instance of quicktype generating a struct with the name "Vec" to "GVec"
 * @param {string[]} lines
 * @returns {string[]}
 */
function replaceVecStruct(lines) {
  return lines.map((line) =>
    line.replace("pub struct Vec {", "pub struct GVec {").replace("<Vec>", "<GVec>")
  );
}

/**
 * @param {string[]} filesNoExt
 */
async function generateExcelBinOutputStructs(filesNoExt) {
  const generateStructs = filesNoExt.map(async (fileNameNoExt) => {
    const fileContents = fs.readFile(
      path.join(EXCEL_BIN_OUTPUT_FOLDER, `${fileNameNoExt}.json`),
      "utf8"
    );
    const { lines } = await quicktypeJSON(fileNameNoExt, await fileContents);

    lines.unshift(
      "/// This file was automatically generated by quicktype",
      "/// DO NOT MANUALLY EDIT THIS FILE!",
      "",
      "#[allow(unused_imports)]"
    );
    const rustSrc = replaceVecStruct(lines).join("\n");
    const outputFilePath = path.join(
      RUST_EXCEL_BIN_OUTPUT_FOLDER,
      `${fileNameNoExt}.rs`
    );
    console.log(`Generating file ${path.relative(CWD, outputFilePath)}`);
    return fs.writeFile(outputFilePath, rustSrc);
  });
  await Promise.all(generateStructs);
}

/**
 * @param {string[]} filesNoExt
 */
async function generateExcelBinOutputModFile(filesNoExt) {
  const pubMods = filesNoExt
    .sort()
    .map((fileNameNoExt) => `pub mod ${fileNameNoExt};`)
    .join("\n");
  const modFileContents = `${[
    "#![cfg_attr(rustfmt, rustfmt_skip)]",
    "#![allow(non_snake_case)]",
    "#![allow(non_camel_case_types)]",
    pubMods,
  ].join("\n")}\n`;
  const modFilePath = path.join(RUST_EXCEL_BIN_OUTPUT_FOLDER, "mod.rs");
  console.log(`Generating file ${path.relative(CWD, modFilePath)}`);
  await fs.writeFile(modFilePath, modFileContents);
}

/**
 * @returns {Promise<void>}
 */
export async function generateExcelBinOutput() {
  console.log("Generating ExcelBinOutput files...");
  const filesNoExt = await getExcelBinOutputJsonFiles();
  await Promise.all([
    generateExcelBinOutputStructs(filesNoExt),
    generateExcelBinOutputModFile(filesNoExt),
  ]);
}
