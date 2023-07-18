/**
 *
 * @param {string} enumName
 * @param {string[]} enumEntries
 * @returns {string}
 */
export function generateSimpleEnumSrc(enumName, enumEntries) {
  return `use strum::{Display, EnumCount, EnumIter, EnumString};

#[derive(Copy, Clone, Hash, PartialEq, Eq, Debug, Display, EnumCount, EnumIter, EnumString)]
pub enum ${enumName} {
${enumEntries.map((entry) => `    ${entry},`).join("\n")}
}\n`;
}
