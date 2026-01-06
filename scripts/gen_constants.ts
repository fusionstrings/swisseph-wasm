// Script to generate constants from C header

const HEADER_PATH = "vendor/swisseph/swephexp.h";
const TARGET_TS = "src/constants.ts";

async function main() {
  const text = await Deno.readTextFile(HEADER_PATH);
  const lines = text.split("\n");

  const constants: { name: string; value: string }[] = [];

  // Regex to match #define NAME VALUE (allowing space after #)
  const regex = /^\s*#\s*define\s+([A-Z][A-Z0-9_]+)\s+(.+?)(\s*\/\*.*)?$/;

  // Prefixes to include
  const ALLOWED_PREFIXES = ["SE_", "SEFLG_", "SEMOD_", "TJD_", "SIMULATE_"];
  const IGNORED = [
    "SE_EPHE_PATH",
    "SE_FNAME_",
    "SE_STARFILE",
    "SE_ASTNAMFILE",
    "SE_FICTFILE",
    "MALLOC",
    "CALLOC",
    "FREE",
  ];

  // Buffer to handle multi-line defines (though swephexp.h usually doesn't have them for constants)

  for (const line of lines) {
    const match = line.match(regex);
    if (!match) continue;

    const name = match[1].trim();
    let value = match[2].trim();

    // Filter
    const isAllowed = ALLOWED_PREFIXES.some((p) => name.startsWith(p));
    if (!isAllowed) continue;
    if (IGNORED.some((i) => name.startsWith(i))) continue;

    // Cleanup Value
    // Remove trailing comments that were not caught
    value = value.replace(/\/\*.*\*\//g, "").trim();
    value = value.replace(/\/\/.*/g, "").trim();

    // Attempt to handle casts like (double) or similar if present (swephexp usually clean)
    // Remove wrapping parens if it's a simple number inside, but keep for expressions
    // e.g. (1.0/20.0) -> (1.0/20.0) is fine.

    // Special handling for bit shifts 1 << 11
    // TS/JS handles << fine.

    constants.push({ name, value });
  }

  // Generate TS Content
  let tsContent =
    `// Auto-generated from ${HEADER_PATH}\n// DO NOT EDIT DIRECTLY\n\n`;
  for (const { name, value } of constants) {
    tsContent += `export const ${name} = ${value};\n`;
  }

  await Deno.writeTextFile(TARGET_TS, tsContent);

  console.log(`Generated ${constants.length} constants.`);
  console.log(`- ${TARGET_TS}`);
}

main();
