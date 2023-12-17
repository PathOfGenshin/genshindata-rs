import * as fs from "fs/promises";
import * as path from "path";
import { promisify } from "util";
import { exec as execCB } from "child_process";
import { CWD, GAME_DATA_FOLDER } from "./env.mjs";

const exec = promisify(execCB);

const VERSION_LINE_REGEX = /Currently updated for game version: \*\*(.*)\*\*/g;
const VERSION_GIT_COMMIT_REGEX = /(OSREL[a-zA-Z\d_.]+)/g

/**
 * @param {number} headCount
 * @returns {Promise<string>}
 */
async function getGameDataVersion(headCount) {
  const head = headCount === 0 ? "HEAD" : `HEAD~${headCount}`
  const { stdout } = await exec(`git show -s --format=%B ${head}`, {
    cwd: GAME_DATA_FOLDER,
  });

  const match = stdout.trim().match(VERSION_GIT_COMMIT_REGEX)
  if (match[0]) {
    return match[0];
  }

  return await getGameDataVersion(headCount + 1);
}

export async function updateReadmeVersion() {
  const gameDataVersion = await getGameDataVersion(0);
  const readmeFile = path.join(CWD, "README.md");
  const readmeContents = await fs.readFile(readmeFile, "utf-8");
  const updatedReadmeContents = readmeContents.replace(
    VERSION_LINE_REGEX,
    `Currently updated for game version: **${gameDataVersion}**`
  );
  return fs.writeFile(readmeFile, updatedReadmeContents);
}
