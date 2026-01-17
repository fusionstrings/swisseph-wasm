const std = @import("std");
const builtin = @import("builtin");
const math = std.math;

// --- Memory Management ---
// Use page_allocator for freestanding wasm.
const allocator = std.heap.wasm_allocator;

const Header = struct {
    size: usize,
    padding: u32,
};

export fn malloc(size: usize) ?*anyopaque {
    // 8-byte alignment for doubles
    const total_size = size + @sizeOf(Header);
    // Alignment must be comptime
    const alignment = comptime std.mem.Alignment.fromByteUnits(8);
    
    const slice = allocator.alignedAlloc(u8, alignment, total_size) catch return null;
    
    const header_ptr: *Header = @ptrCast(slice.ptr);
    header_ptr.size = size;
    
    // Return pointer after header
    return @as(?*anyopaque, @ptrCast(slice.ptr + @sizeOf(Header)));
}

export fn free(ptr: ?*anyopaque) void {
    if (ptr) |p| {
        const data_ptr: [*]u8 = @ptrCast(p);
        const header_ptr: *Header = @ptrCast(@alignCast(data_ptr - @sizeOf(Header)));
        const size = header_ptr.size;
        const total_size = size + @sizeOf(Header);
        
        const slice_ptr = data_ptr - @sizeOf(Header);
        const slice = slice_ptr[0..total_size];
        
        allocator.free(slice);
    }
}

export fn wasm_alloc(len: usize) [*]u8 {
    const ptr = malloc(len);
    if (ptr) |p| return @ptrCast(p);
    @panic("wasm_alloc OOM");
}

export fn wasm_free(ptr: [*]u8) void {
    free(ptr);
}

export fn realloc(ptr: ?*anyopaque, size: usize) callconv(.c) ?*anyopaque {
    if (ptr == null) return malloc(size);
    if (size == 0) {
        free(ptr);
        return null;
    }

    const data_ptr: [*]u8 = @ptrCast(ptr.?);
    const header_ptr: *Header = @ptrCast(@alignCast(data_ptr - @sizeOf(Header)));
    const old_size = header_ptr.size;
    const old_total = old_size + @sizeOf(Header);
    const slice_ptr = data_ptr - @sizeOf(Header);
    const old_slice = slice_ptr[0..old_total];

    const new_total = size + @sizeOf(Header);

    const new_slice = allocator.realloc(old_slice, new_total) catch return null;

    const new_header_ptr: *Header = @ptrCast(@alignCast(new_slice.ptr));
    new_header_ptr.size = size;

    return @as(?*anyopaque, @ptrCast(new_slice.ptr + @sizeOf(Header)));
}

// --- String Functions ---
export fn strlen(s: ?[*:0]const u8) callconv(.c) usize {
    if (s == null) return 0;
    return std.mem.len(s.?);
}

export fn strcpy(dest: ?[*:0]u8, src: ?[*:0]const u8) callconv(.c) ?[*:0]u8 {
    if (dest == null or src == null) return dest;
    var i: usize = 0;
    while (src.?[i] != 0) : (i += 1) {
        dest.?[i] = src.?[i];
    }
    dest.?[i] = 0;
    return dest;
}

export fn strncpy(dest: ?[*:0]u8, src: ?[*:0]const u8, n: usize) callconv(.c) ?[*:0]u8 {
    if (dest == null or src == null) return dest;
    var i: usize = 0;
    while (i < n and src.?[i] != 0) : (i += 1) {
        dest.?[i] = src.?[i];
    }
    while (i < n) : (i += 1) {
        dest.?[i] = 0;
    }
    return dest;
}

export fn strcat(dest: ?[*:0]u8, src: ?[*:0]const u8) callconv(.c) ?[*:0]u8 {
    if (dest == null or src == null) return dest;
    const dest_len = std.mem.len(dest.?);
    var i: usize = 0;
    while (src.?[i] != 0) : (i += 1) {
        dest.?[dest_len + i] = src.?[i];
    }
    dest.?[dest_len + i] = 0;
    return dest;
}

export fn strchr(s: ?[*:0]const u8, c: c_int) callconv(.c) ?[*:0]const u8 {
    if (s == null) return null;
    var i: usize = 0;
    const char_val = @as(u8, @intCast(c & 0xff));
    while (true) : (i += 1) {
        if (s.?[i] == char_val) return @ptrCast(&s.?[i]);
        if (s.?[i] == 0) return null;
    }
}

export fn strcmp(s1: ?[*:0]const u8, s2: ?[*:0]const u8) callconv(.c) c_int {
    if (s1 == null and s2 == null) return 0;
    if (s1 == null) return -1;
    if (s2 == null) return 1;
    var i: usize = 0;
    while (s1.?[i] == s2.?[i]) : (i += 1) {
        if (s1.?[i] == 0) return 0;
    }
    return @as(c_int, s1.?[i]) - @as(c_int, s2.?[i]);
}

export fn strncmp(s1: ?[*:0]const u8, s2: ?[*:0]const u8, n: usize) callconv(.c) c_int {
    if (n == 0) return 0;
    if (s1 == null and s2 == null) return 0;
    var i: usize = 0;
    while (i < n) : (i += 1) {
        if (s1.?[i] != s2.?[i]) return @as(c_int, s1.?[i]) - @as(c_int, s2.?[i]);
        if (s1.?[i] == 0) return 0;
    }
    return 0;
}

export fn strstr(haystack: ?[*:0]const u8, needle: ?[*:0]const u8) callconv(.c) ?[*:0]const u8 {
    if (haystack == null or needle == null) return null;
    const h_len = std.mem.len(haystack.?);
    const n_len = std.mem.len(needle.?);
    if (n_len == 0) return haystack;
    if (n_len > h_len) return null;
    
    var i: usize = 0; 
    while (i <= h_len - n_len) : (i += 1) {
        var match = true;
        var j: usize = 0;
        while (j < n_len) : (j += 1) {
            if (haystack.?[i+j] != needle.?[j]) {
                match = false;
                break;
            }
        }
        if (match) return @ptrCast(&haystack.?[i]);
    }
    return null;
}

export fn memset(s: ?*anyopaque, c: c_int, n: usize) callconv(.c) ?*anyopaque {
    if (s == null) return null;
    const ptr: [*]u8 = @ptrCast(s);
    const val = @as(u8, @intCast(c & 0xff)); // Correct cast
    @memset(ptr[0..n], val);
    return s;
}

export fn memcpy(dest: ?*anyopaque, src: ?*const anyopaque, n: usize) callconv(.c) ?*anyopaque {
    if (dest == null or src == null) return dest;
    const d: [*]u8 = @ptrCast(dest);
    const s: [*]const u8 = @ptrCast(src);
    @memcpy(d[0..n], s[0..n]);
    return dest;
}

export fn strdup(s: ?[*:0]const u8) callconv(.c) ?[*:0]u8 {
    if (s == null) return null;
    const len = std.mem.len(s.?);
    const ptr = malloc(len + 1);
    if (ptr == null) return null;
    const dest: [*:0]u8 = @ptrCast(ptr);
    return strcpy(dest, s);
}

// --- Ctype ---
export fn isspace(c: c_int) callconv(.c) c_int {
    const ch = @as(u8, @intCast(c & 0xff));
    return if (std.ascii.isWhitespace(ch)) 1 else 0;
}

export fn isalnum(c: c_int) callconv(.c) c_int {
    const ch = @as(u8, @intCast(c & 0xff));
    return if (std.ascii.isAlphanumeric(ch)) 1 else 0;
}

export fn isdigit(c: c_int) callconv(.c) c_int {
     const ch = @as(u8, @intCast(c & 0xff));
     return if (std.ascii.isDigit(ch)) 1 else 0;
}

export fn isalpha(c: c_int) callconv(.c) c_int {
    const ch = @as(u8, @intCast(c & 0xff));
    return if (std.ascii.isAlphabetic(ch)) 1 else 0;
}

export fn toupper(c: c_int) callconv(.c) c_int {
     const ch = @as(u8, @intCast(c & 0xff));
     return std.ascii.toUpper(ch);
}

export fn tolower(c: c_int) callconv(.c) c_int {
     const ch = @as(u8, @intCast(c & 0xff));
     return std.ascii.toLower(ch);
}

// --- Stdlib ---
export fn atoi(s: ?[*:0]const u8) callconv(.c) c_int {
    if (s == null) return 0;
    const span = std.mem.span(s.?);
    return std.fmt.parseInt(c_int, span, 10) catch 0;
}

export fn atof(s: ?[*:0]const u8) callconv(.c) f64 {
    if (s == null) return 0.0;
    const span = std.mem.span(s.?);
    return std.fmt.parseFloat(f64, span) catch 0.0;
}

export fn abs(j: c_int) callconv(.c) c_int {
    return if (j < 0) -j else j;
}

export fn getenv(_: ?[*:0]const u8) callconv(.c) ?[*:0]u8 {
    return null;
}

export fn exit(_: c_int) callconv(.c) noreturn {
    while (true) {}
}

extern fn __wasm_call_ctors() void;

export fn wasm_start() callconv(.c) void {
    __wasm_call_ctors();
}





// --- Math ---
// Software implementations to avoid recursion (Zig's std.math calls exported symbols)
const PI: f64 = 3.14159265358979323846;
const TWO_PI: f64 = 6.28318530717958647692;

fn normalize_angle(x: f64) f64 {
    // Efficiently reduce angle to [-PI, PI] using FMod equivalence
    // x - 2PI * floor((x + PI) / 2PI)
    return x - TWO_PI * @floor((x + PI) / TWO_PI);
}

export fn sin(x: f64) callconv(.c) f64 {
    const a = normalize_angle(x);
    // Taylor series: sin(x) = x - x^3/3! + x^5/5! - x^7/7! + ...
    const x2 = a * a;
    var result = a;
    var term = a;
    var n: f64 = 1.0;
    // Iterate until term is negligible
    while (term > 1e-16 or term < -1e-16) {
        term *= -x2 / ((2.0 * n) * (2.0 * n + 1.0));
        result += term;
        n += 1.0;
        if (n > 100.0) break; // Safety break
    }
    return result;
}

export fn cos(x: f64) callconv(.c) f64 {
    const a = normalize_angle(x);
    // Taylor series: cos(x) = 1 - x^2/2! + x^4/4! - x^6/6! + ...
    const x2 = a * a;
    var result: f64 = 1.0;
    var term: f64 = 1.0;
    var n: f64 = 1.0;
    while (term > 1e-16 or term < -1e-16) {
        term *= -x2 / ((2.0 * n - 1.0) * (2.0 * n));
        result += term;
        n += 1.0;
        if (n > 100.0) break;
    }
    return result;
}

export fn tan(x: f64) callconv(.c) f64 { 
    const s = sin(x);
    const c = cos(x);
    return s / c;
}

// Software asin using Newton-Raphson/Taylor
export fn asin(x: f64) callconv(.c) f64 {
    if (x >= 1.0) return PI / 2.0;
    if (x <= -1.0) return -PI / 2.0;
    // For x > 0.7, use identity asin(x) = PI/2 - 2*asin(sqrt((1-x)/2)) to avoid slow convergence
    // But self-recursion?
    // Let's just use massive iterations for now.
    var result = x;
    var term = x;
    const x2 = x * x;
    var n: f64 = 1.0;
    // Convergence loop
    while (term > 1e-16 or term < -1e-16) {
        term *= x2 * (2.0 * n - 1.0) * (2.0 * n - 1.0) / ((2.0 * n) * (2.0 * n + 1.0));
        result += term;
        n += 1.0;
        if (n > 5000.0) break; 
    }
    return result;
}

export fn acos(x: f64) callconv(.c) f64 {
    return PI / 2.0 - asin(x);
}

export fn atan(x: f64) callconv(.c) f64 {
    // Range reduction
    if (x < 0) return -atan(-x);
    if (x == 0) return 0;
    if (x > 1.0) return PI / 2.0 - atan(1.0 / x);
    // Now 0 <= x <= 1
    // If x is close to 1, series is slow. Use atan(x) = PI/4 + atan((x-1)/(1+x))
    // 0.414... is tan(pi/8). If x > tan(pi/8), transform reduces argument magnitude.
    if (x > 0.4142135623730950) {
        return PI / 4.0 + atan((x - 1.0) / (1.0 + x));
    }
    
    // Taylor series for |x| <= 0.414 converges nicely
    var result = x;
    var term = x;
    const x2 = x * x;
    var n: f64 = 1.0;
    while (term > 1e-16 or term < -1e-16) {
        n += 1.0;
        term *= -x2;
        result += term / (2.0 * n - 1.0);
        if (n > 2000.0) break;
    }
    return result;
}

export fn atan2(y: f64, x: f64) callconv(.c) f64 {
    if (x > 0) return atan(y / x);
    if (x < 0 and y >= 0) return atan(y / x) + PI;
    if (x < 0 and y < 0) return atan(y / x) - PI;
    if (x == 0 and y > 0) return PI / 2.0;
    if (x == 0 and y < 0) return -PI / 2.0;
    return 0.0; // undefined
}

export fn sqrt(x: f64) callconv(.c) f64 {
    if (x <= 0) return 0;
    // Newton-Raphson
    var guess = x / 2.0;
    var prev: f64 = 0.0;
    // Iterate until stable
    var i: usize = 0;
    while (i < 100) : (i += 1) {
        prev = guess;
        guess = (guess + x / guess) / 2.0;
        const diff = guess - prev;
        if (diff < 1e-16 and diff > -1e-16) break;
    }
    return guess;
}

export fn fabs(x: f64) callconv(.c) f64 { 
    return if (x < 0) -x else x; 
}

export fn ceil(x: f64) callconv(.c) f64 {
    const i: i64 = @intFromFloat(x);
    const f: f64 = @floatFromInt(i);
    return if (x > f) f + 1.0 else f;
}

export fn floor(x: f64) callconv(.c) f64 {
    const i: i64 = @intFromFloat(x);
    const f: f64 = @floatFromInt(i);
    return if (x < f) f - 1.0 else f;
}

export fn fmod(x: f64, y: f64) callconv(.c) f64 { 
    return x - floor(x / y) * y; 
}

export fn pow(base: f64, exponent: f64) callconv(.c) f64 {
    return exp(exponent * log(base));
}

export fn exp(x: f64) callconv(.c) f64 {
    // Taylor series: e^x = 1 + x + x^2/2! + x^3/3! + ...
    var result: f64 = 1.0;
    var term: f64 = 1.0;
    var n: f64 = 1.0;
    while (term > 1e-16 or term < -1e-16) {
        term *= x / n;
        result += term;
        n += 1.0;
        if (n > 2000.0) break; // Handle large x
    }
    return result;
}

export fn log(x: f64) callconv(.c) f64 {
    if (x <= 0) return -1e308; // -infinity approximation
    
    // Range reduction: log(M * 2^k) = log(M) + k * ln2
    // Reduce x to [0.5, 1.0] or similar for fast convergence.
    // Simple reduction: loop divide by 2
    var val = x;
    var k: f64 = 0.0;
    const LN2 = 0.6931471805599453;
    
    while (val > 1.5) {
        val *= 0.5;
        k += 1.0;
    }
    while (val < 0.5) {
        val *= 2.0;
        k -= 1.0;
    }

    // Reduce to log((1+y)/(1-y)) where y = (val-1)/(val+1)
    // Then use series: 2*(y + y^3/3 + y^5/5 + ...)
    const y = (val - 1.0) / (val + 1.0);
    const y2 = y * y;
    var result = y;
    var term = y;
    var n: f64 = 3.0;
    while (term > 1e-16 or term < -1e-16) {
        term *= y2;
        result += term / n;
        n += 2.0;
        if (n > 5000.0) break;
    }
    return 2.0 * result + k * LN2;
}

export fn log10(x: f64) callconv(.c) f64 {
    return log(x) / 2.302585092994046; // ln(10)
}

// --- Stdio ---
export fn fprintf(_: ?*anyopaque, _: ?[*:0]const u8, ...) callconv(.c) c_int {
    return 0;
}

export fn printf(_: ?[*:0]const u8, ...) callconv(.c) c_int {
    return 0;
}

export fn sprintf(str: ?[*:0]u8, format: ?[*:0]const u8, ...) callconv(.c) c_int {
    if (str != null and format != null) {
        var i: usize = 0;
        while (format.?[i] != 0) : (i += 1) {
            str.?[i] = format.?[i];
        }
        str.?[i] = 0;
    }
    return 0;
}

export fn fopen(_: ?[*:0]const u8, _: ?[*:0]const u8) callconv(.c) ?*anyopaque {
    return null;
}

export fn fclose(_: ?*anyopaque) callconv(.c) c_int {
    return 0;
}

export fn fread(_: ?*anyopaque, _: usize, _: usize, _: ?*anyopaque) callconv(.c) usize {
    return 0;
}

export fn fwrite(_: ?*anyopaque, _: usize, nmemb: usize, _: ?*anyopaque) callconv(.c) usize {
    return nmemb;
}

export fn fseek(_: ?*anyopaque, _: c_long, _: c_int) callconv(.c) c_int {
    return 0;
}

export fn ftell(_: ?*anyopaque) callconv(.c) c_long {
    return 0;
}

export fn fflush(_: ?*anyopaque) callconv(.c) c_int {
    return 0;
}

export fn fgets(_: ?[*:0]u8, _: c_int, _: ?*anyopaque) callconv(.c) ?[*:0]u8 {
    return null;
}

export fn fseeko(_: ?*anyopaque, _: i64, _: c_int) callconv(.c) c_int {
    return 0;
}

export fn ftello(_: ?*anyopaque) callconv(.c) i64 {
    return 0;
}

export fn rewind(_: ?*anyopaque) callconv(.c) void {}

// --- Additional Strings/Stdlib ---
export fn calloc(nmemb: usize, size: usize) callconv(.c) ?*anyopaque {
    const total = nmemb * size;
    const ptr = malloc(total);
    if (ptr) |p| {
         _ = memset(p, 0, total);
    }
    return ptr;
}

export fn strrchr(s: ?[*:0]const u8, c: c_int) callconv(.c) ?[*:0]const u8 {
    if (s == null) return null;
    var last: ?[*:0]const u8 = null;
    var i: usize = 0;
    const ch = @as(u8, @intCast(c & 0xff));
    while (true) : (i += 1) {
        if (s.?[i] == ch) last = @ptrCast(&s.?[i]);
        if (s.?[i] == 0) break;
    }
    return last;
}

export fn strpbrk(s: ?[*:0]const u8, accept: ?[*:0]const u8) callconv(.c) ?[*:0]const u8 {
    if (s == null or accept == null) return null;
    var i: usize = 0;
    while (s.?[i] != 0) : (i += 1) {
        if (strchr(accept, s.?[i]) != null) return @ptrCast(&s.?[i]);
    }
    return null;
}

export fn atol(s: ?[*:0]const u8) callconv(.c) c_long {
    return @intCast(atoi(s));
}

// --- Dynamic Linking / System ---
// dlfcn.h stubs
export fn dladdr(_: ?*const anyopaque, _: ?*anyopaque) callconv(.c) c_int {
    return 0;
}

// unistd.h stubs
export fn readlink(_: ?[*:0]const u8, _: ?[*:0]u8, _: usize) callconv(.c) isize {
    return -1; // Error
}
