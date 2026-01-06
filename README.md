# swisseph-wasm 🌌

**High-precision, WebAssembly bindings for the Swiss Ephemeris.**

> 🚀 **Best-in-Class**: Faster, smaller, and more accurate than pure JS
> implementations. 🛡️ **Safe**: Rust-based memory safety for FFI interactions.
> 📦 **Universal**: Works in Node.js, Deno, Bun, and Browsers.

[![JSR](https://jsr.io/badges/@fusionstrings/swisseph-wasm)](https://jsr.io/@fusionstrings/swisseph-wasm)
[![CI](https://github.com/fusionstrings/swisseph-wasm/actions/workflows/ci.yml/badge.svg)](https://github.com/fusionstrings/swisseph-wasm/actions/workflows/ci.yml)

## Features

- **Full Swiss Ephemeris**: Wraps the complete C library (v2.10.03).
- **Zero Dependencies**: Generated WASM has no external runtime deps.
- **Type-Safe**: Complete TypeScript definitions.
- **Isomorphic**: Runs everywhere WebAssembly runs.

## Installation

### Deno / JSR

```bash
deno add jsr:@fusionstrings/swisseph-wasm
```

**Advanced**: Import raw WASM (Deno):

```typescript
import wasmBytes from "@fusionstrings/swisseph-wasm/wasm";
const module = new WebAssembly.Module(wasmBytes);
```

### NPM / Node.js

```bash
npm install swisseph-wasm
```

## Quick Start

```typescript
import {
  SE_GREG_CAL,
  SE_MOON,
  SE_SUN,
  swe_calc_ut,
  swe_julday,
} from "@fusionstrings/swisseph-wasm";

// 1. Calculate Julian Day for Jan 1, 2000, 12:00 UTC
const jd = swe_julday(2000, 1, 1, 12.0, SE_GREG_CAL);
console.log(`Julian Day: ${jd}`); // 2451545.0

// 2. Calculate Sun Position
const sun = swe_calc_ut(jd, SE_SUN, 0);
console.log("Sun Longitude:", sun.longitude);

// 3. Calculate Moon Position
const moon = swe_calc_ut(jd, SE_MOON, 0);
console.log("Moon Longitude:", moon.longitude);
```

## Building from Source

### Prerequisites

- **Rust**: `rustup target add wasm32-unknown-unknown`
- **Deno**: For task management.
- **LLVM**: Required for compiling the C library to WASM.

**macOS Users**:

```bash
brew install llvm
```

### Build Command

The build script automatically detects Homebrew LLVM.

```bash
deno task build
```

## License

MIT License. Based on the Swiss Ephemeris (GPL/Commercial dual license). _Note:
Using this library implies compliance with the Swiss Ephemeris license terms._
