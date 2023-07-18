import { InputData, jsonInputForTargetLanguage, quicktype } from "quicktype-core";

const RUST_QUICKTYPE_OPTIONS = {
  density: "dense",
  visibility: "public",
  "derive-debug": true,
  "derive-clone": true,
  "edition-2018": true,
  "leading-comments": false,
};

/**
 *
 * @param {string} typeName
 * @param {string} jsonString
 */
export async function quicktypeJSON(typeName, jsonString) {
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
