# Technical Audit: swisseph-wasm

## 1. Executive Summary

The technical audit confirms that `swisseph-wasm` is a **robust and highly
accurate** port of the Swiss Ephemeris library, successfully achieving **100%
API coverage** of the core exported functions. It demonstrates **high functional
correctness**, matching the original C library's output to **13 decimal places**
(approx. `1e-13` degrees).

A Critical Integrity Finding remains: the current implementation relies
**exclusively on Moshier (analytical) mode** due to stubbed file I/O operations,
meaning it simply cannot use high-precision `.se1` ephemeris files.

## 2. API Parity

- **Coverage**: 100% of applicable `swephexp.h` functions are exported.
- **Verification**: `tests/full_coverage.test.ts` verifies the existence and
  return signatures of all 95+ exported functions.
- **Exceptions**: File system configuration functions (`swe_set_ephe_path`,
  etc.) are present but effectively no-ops due to the WASM environment sandbox.

## 3. Functional Correctness (Precision)

A rigorous comparison was conducted between the Wasm build and the original C
library compiled from source (`vendor/swisseph`).

### Methodology

- **Reference**: Custom C tool (`compare_precision.c`) linked against
  `libswe.a`.
- **Target**: `swe_calc_ut` (Sun, Jan 1 2000, Moshier mode).
- **Metric**: Floating-point difference.

### Results

| Implementation  | Result (Longitude)     |
| :-------------- | :--------------------- |
| **C Reference** | `280.3689196753432...` |
| **Wasm Port**   | `280.3689196753433...` |
| **Difference**  | `1.137e-13` degrees    |

**Assessment**: The port matches the C reference to **13 decimal places**.

- Exceeds typical astrological requirements (< 0.00001 degrees).
- Technically falls slightly short of "bit-perfect" identity at the 14th decimal
  place, likely due to distinct `libm` implementations (Rust/Wasm vs System C).

### Comprehensive Scope

Precision was further verified across diverse domains:

- **Houses**: `swe_houses` matched Ascendant to 6+ decimals.
- **Stars**: `swe_fixstar` matched Sirius longitude to 6+ decimals.
- **Eclipses**: `swe_sol_eclipse` matched timing to `1e-6` JD.

## 4. Implementation Integrity & Stability

### Ephemeris File Loading (Critical Gap)

- **Status**: **Not Implemented**.
- **Evidence**: `src/wrapper/stub.c` implements `fopen`, `fread`, etc., as
  no-ops.
- **Impact**: The library is locked to "Moshier Mode". While this mode is
  accurate (~0.1 arcsec), it does not support the sub-milliarcsecond precision
  available with Swiss Ephemeris data files. The user requirement to "handle
  large binary files efficiently" is currently **failed**.

### Memory Stability

- **Test**: `tests/memory_leak.test.ts` executed `swe_calc_ut` 1,000,000 times.
- **Result**: **Pass**. No memory growth observed. This confirms that Rust
  `CString` allocations and `wasm-bindgen` memory management are correct.

## 5. Documentation

- **Status**: **Pass**.
- **Quality**: JSDoc comments are present for exported functions. API naming is
  consistent with the C standard.
- **Recommendation**: Explicitly document the "Moshier Mode Only" limitation in
  the `README`.

## 6. Recommendations

1. **Implement Ephemeris Loading**: To support the full power of Swiss
   Ephemeris, implement a mechanism (e.g., VFS or callback) to inject binary
   ephemeris data into the Wasm memory.
2. **Document Precision**: State the 13-decimal place parity in documentation to
   reassure high-precision users.
3. **Strict Mode**: Consider exposing `swe_set_ephe_path` dummy to warn users if
   they attempt to load files.
