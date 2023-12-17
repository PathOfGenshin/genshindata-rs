import * as fs from "fs/promises";
import * as path from "path";
import { promisify } from "util";
import { exec as execCB } from "child_process";
import { CWD, GAME_DATA_FOLDER } from "./env.mjs";

const exec = promisify(execCB);

const VERSION_LINE_REGEX = /Currently updated for game version: \*\*(.*)\*\*/g;
const VERSION_GIT_COMMIT_REGEX = /OSREL[a-zA-Z]+([\d.]+)[a-zA-Z\d_.]+/g;
const FIRST_VERSION_LINE_JSON_REGEX = /"version" = "(.*)"/;
const FIRST_VERSION_LINE_TOML_REGEX = /version = "(.*)"/;

/**
 * Tries to find the version string in the git commit messages within the past 20
 * commits of the game data folder.
 *
 * @param {number} headCount
 * @returns {Promise<{ fullVersionString: string, versionString: string }>}
 * @throws {Error} If the version string could not be found in the past 20 commits
 */
async function getGameDataVersion(headCount) {
  if (headCount > 20) {
    throw Error("Could not find version string from the past 20 commits!");
  }

  const head = headCount === 0 ? "HEAD" : `HEAD~${headCount}`;
  const { stdout } = await exec(`git show -s --format=%B ${head}`, {
    cwd: GAME_DATA_FOLDER,
  });

  const match = VERSION_GIT_COMMIT_REGEX.exec(stdout.trim());
  if (match && match[0] && match[1]) {
    return { fullVersionString: match[0], versionString: match[1] };
  }

  return await getGameDataVersion(headCount + 1);
}

/**
 * Updates the "README.md" version string line
 * @param {{ fullVersionString: string, versionString: string }} version
 * @returns {Promise<void>}
 */
async function updateReadmeVersion(version) {
  const readmeFile = path.join(CWD, "README.md");
  const readmeContents = await fs.readFile(readmeFile, "utf-8");
  const updatedReadmeContents = readmeContents.replace(
    VERSION_LINE_REGEX,
    `Currently updated for game version: **${version.fullVersionString}**`
  );
  return fs.writeFile(readmeFile, updatedReadmeContents);
}

/**
 * Updates the "package.json" package version string line
 *
 * @param {{ fullVersionString: string, versionString: string }} version
 * @returns {Promise<void>}
 */
async function updatePackageJsonVersion(version) {
  const packageJsonFile = path.join(CWD, "package.json");
  const packageJsonContents = await fs.readFile(packageJsonFile, "utf-8");
  const updatedPackageJsonContents = packageJsonContents.replace(
    FIRST_VERSION_LINE_JSON_REGEX,
    `"version": "${version.versionString}"`
  );
  return fs.writeFile(packageJsonFile, updatedPackageJsonContents);
}

/**
 * Updates the Cargo.toml version string line for "genshindata-rs'
 *
 * @param {{ fullVersionString: string, versionString: string }} version
 * @returns {Promise<void>}
 */
async function updateRustGenshinDataRsCargoTomlVersion(version) {
  const file = path.join(CWD, "genshindata-rs", "Cargo.toml");
  const contents = await fs.readFile(file, "utf-8");
  const updatedContents = contents.replace(
    FIRST_VERSION_LINE_TOML_REGEX,
    `version = "${version.versionString}"`
  );
  return fs.writeFile(file, updatedContents);
}

/**
 * Updates the Cargo.toml version string line for "genshindata-processed'
 *
 * @param {{ fullVersionString: string, versionString: string }} version
 * @returns {Promise<void>}
 */
async function updateRustGenshinDataProcessedCargoTomlVersion(version) {
  const file = path.join(CWD, "genshindata-processed", "Cargo.toml");
  const contents = await fs.readFile(file, "utf-8");
  const updatedContents = contents.replace(
    FIRST_VERSION_LINE_TOML_REGEX,
    `version = "${version.versionString}"`
  );
  return fs.writeFile(file, updatedContents);
}

/**
 * Updates the version strings for the README, package.json, and Cargo.toml files
 *
 * @returns {Promise<void>}
 */
export async function updateVersions() {
  const version = await getGameDataVersion(0);
  console.log("Updating versions to: ", version);
  await Promise.all([
    updateReadmeVersion(version),
    updatePackageJsonVersion(version),
    updateRustGenshinDataRsCargoTomlVersion(version),
    updateRustGenshinDataProcessedCargoTomlVersion(version),
  ]);
}
