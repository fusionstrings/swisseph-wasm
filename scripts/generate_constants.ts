const headerPath = "./vendor/swisseph/swephexp.h";
const outPath = "./src/typescript/constants.ts";

let text = await Deno.readTextFile(headerPath);

// improved comment stripping:
// 1. Remove /* ... */ across lines
text = text.replace(/\/\*[\s\S]*?\*\//g, "");
// 2. Remove // ... end of line
text = text.replace(/\/\/.*$/gm, "");

const lines = text.split("\n");

let output = `// Auto-generated from swephexp.h
`;

for (const line of lines) {
  const match = line.match(/^\s*#define\s+(\w+)\s+(.+)$/);
  if (match) {
    const key = match[1];
    let val = match[2].trim();

    // Skip macro functions
    if (key.includes("(")) continue;

    // Remove trailing 'L' from integers (e.g. 123L)
    val = val.replace(/(\d)L\b/gi, "$1");

    // Remove empty definitions
    if (!val) continue;

    output += `export const ${key} = ${val};\n`;
  }
}

await Deno.writeTextFile(outPath, output);
console.log("Generated constants.ts");
