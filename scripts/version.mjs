import * as fs from "fs/promises";
import * as path from "path";
import { promisify } from "util";
import { exec as execCB } from "child_process";
import { CWD, GAME_DATA_FOLDER } from "./env.mjs";

const exec = promisify(execCB);

const VERSION_LINE_REGEX = /Currently updated for game version: \*\*(.*)\*\*/g;

/**
 * @returns {Promise<string>}
 */
async function getGameDataVersion() {
  const { stdout } = await exec("git show -s --format=%B HEAD", {
    cwd: GAME_DATA_FOLDER,
  });
  return stdout.trim();
}

export async function updateReadmeVersion() {
  const gameDataVersion = await getGameDataVersion();
  const readmeFile = path.join(CWD, "README.md");
  const readmeContents = await fs.readFile(readmeFile, "utf-8");
  const updatedReadmeContents = readmeContents.replace(
    VERSION_LINE_REGEX,
    `Currently updated for game version: **${gameDataVersion}**`
  );
  return fs.writeFile(readmeFile, updatedReadmeContents);
}
