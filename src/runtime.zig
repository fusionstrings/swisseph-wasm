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





const PI = 3.14159265358979323846;
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
    if (str == null or format == null) return 0;
    const fmt = std.mem.span(format.?);
    var va = @cVaStart();
    defer @cVaEnd(&va);

    var i: usize = 0; // index in format
    var j: usize = 0; // index in output str
    while (i < fmt.len) {
        if (fmt[i] == '%' and i + 1 < fmt.len) {
            i += 1;
            // Handle padding like %02d
            var width: usize = 0;
            if (fmt[i] == '0') {
                i += 1;
                while (fmt[i] >= '0' and fmt[i] <= '9') {
                    width = width * 10 + (fmt[i] - '0');
                    i += 1;
                }
            }
            
            if (fmt[i] == 'd') {
                const val = @cVaArg(&va, i32);
                var buf: [32]u8 = undefined;
                const s = std.fmt.bufPrint(&buf, "{d}", .{val}) catch "ERR";
                // Pad with zeros if needed
                if (width > s.len) {
                    for (0..(width - s.len)) |_| {
                        str.?[j] = '0';
                        j += 1;
                    }
                }
                for (s) |c| {
                    str.?[j] = c;
                    j += 1;
                }
                i += 1;
            } else if (fmt[i] == 's') {
                const s_ptr = @cVaArg(&va, [*:0]const u8);
                const s = std.mem.span(s_ptr);
                for (s) |c| {
                    str.?[j] = c;
                    j += 1;
                }
                i += 1;
            } else if (fmt[i] == 'f') { // Some floats used in house names
                const val = @cVaArg(&va, f64);
                var buf: [64]u8 = undefined;
                const s = std.fmt.bufPrint(&buf, "{d:.2}", .{val}) catch "ERR";
                 for (s) |c| {
                    str.?[j] = c;
                    j += 1;
                }
                i += 1;
            } else {
                // Unknown/unsupported format, just copy literally
                str.?[j] = '%';
                j += 1;
            }
        } else {
            str.?[j] = fmt[i];
            i += 1;
            j += 1;
        }
    }
    str.?[j] = 0;
    return @as(c_int, @intCast(j));
}

export fn fwrite(_: ?*anyopaque, _: usize, nmemb: usize, _: ?*anyopaque) callconv(.c) usize {
    return nmemb;
}

export fn fflush(_: ?*anyopaque) callconv(.c) c_int {
    return 0;
}

export fn fgets(str: ?[*:0]u8, size: c_int, stream: ?*anyopaque) callconv(.c) ?[*:0]u8 {
    if (stream == null or str == null) return null;
    const handle = @as(*OpenFileHandle, @ptrCast(@alignCast(stream)));
    if (!handle.in_use) return null;

    const file = &vfs_files[handle.file_idx];
    var i: usize = 0;
    while (i < @as(usize, @intCast(size - 1)) and handle.cursor < file.data.len) {
        const ch = file.data[handle.cursor];
        str.?[i] = ch;
        handle.cursor += 1;
        i += 1;
        if (ch == '\n') break;
    }
    if (i == 0) return null;
    str.?[i] = 0;
    return str;
}

export fn fseeko(stream: ?*anyopaque, offset: i64, origin: c_int) callconv(.c) c_int {
    return fseek(stream, @as(c_long, @intCast(offset)), origin);
}

export fn ftello(stream: ?*anyopaque) callconv(.c) i64 {
    return @as(i64, @intCast(ftell(stream)));
}

// --- Virtual File System ---

const MAX_FILES = 32;
const MAX_OPEN_FILES = 32;

const VirtualFile = struct {
    name: []u8,
    data: []u8,
    in_use: bool,
};

const OpenFileHandle = struct {
    file_idx: usize,
    cursor: usize,
    in_use: bool,
};

var vfs_files: [MAX_FILES]VirtualFile = undefined;
var vfs_open_handles: [MAX_OPEN_FILES]OpenFileHandle = undefined;
var vfs_initialized = false;

fn vfs_init() void {
    if (vfs_initialized) return;
    for (0..MAX_FILES) |i| {
        vfs_files[i].in_use = false;
    }
    for (0..MAX_OPEN_FILES) |i| {
        vfs_open_handles[i].in_use = false;
    }
    vfs_initialized = true;
}

export fn vfs_add_file(name_ptr: [*]const u8, name_len: usize, data_ptr: [*]const u8, data_len: usize) callconv(.c) void {
    vfs_init();
    // Copy name and data to owned memory?
    // Actually, let's assume `data_ptr` is the pointer to the permanently allocated buffer data for the file.
    // But name needs to be copied or stored.
    // Simplest: Find free slot.
    var slot_idx: ?usize = null;
    for (0..MAX_FILES) |i| {
        if (!vfs_files[i].in_use) {
            slot_idx = i;
            break;
        }
    }
    if (slot_idx) |idx| {
        const name_slice = allocator.dupe(u8, name_ptr[0..name_len]) catch return; // Leaks intentionally (global FS)
        const data_slice = allocator.dupe(u8, data_ptr[0..data_len]) catch return; // Copy data to own it
        vfs_files[idx] = VirtualFile{
            .name = name_slice,
            .data = data_slice,
            .in_use = true,
        };
    }
}

// Helper to find file by name
fn vfs_find_file(name: []const u8) ?usize {
    vfs_init();
    // Helper: simplistic name matching (ignore directory prefix if strict match fails?)
    // swisseph often passes full paths. we should match if the end of path matches our filename?
    // Or just exact match?
    // Let's do loose match: checks if `name` ends with `file.name` or `file.name` ends with `name`?
    // Safer: exact match. The user should mount files with correct paths.
    // Actually swisseph calls `swe_set_ephe_path`. If we provide `.`, it looks for `./sepl_18.se1`.
    for (0..MAX_FILES) |i| {
        if (vfs_files[i].in_use) {
            // Check for exact match or suffix match (to handle path differences)
            const f_name = vfs_files[i].name;
            if (std.mem.eql(u8, f_name, name)) return i;
            // Also try matching basename if full path provided
            if (std.mem.endsWith(u8, name, f_name) and (name.len > f_name.len and name[name.len - f_name.len - 1] == '/')) return i;
             // Opposite: if vfs has full path and we ask for basename? Unlikely.
        }
    }
    return null;
}

export fn fopen(filename: ?[*:0]const u8, mode: ?[*:0]const u8) callconv(.c) ?*anyopaque {
    _ = mode;
    if (filename == null) return null;
    const name = std.mem.span(filename.?);
    
    // Check VFS
    if (vfs_find_file(name)) |file_idx| {
         // Find open handle slot
         for (0..MAX_OPEN_FILES) |i| {
             if (!vfs_open_handles[i].in_use) {
                 vfs_open_handles[i] = OpenFileHandle{
                     .file_idx = file_idx,
                     .cursor = 0,
                     .in_use = true,
                 };
                 // Return pointer to the handle index (offset by some magic number to distinguish vs actual pointers?)
                 // Actually, returning address of structs in `vfs_open_handles` is safer.
                 return @ptrCast(&vfs_open_handles[i]);
             }
         }
    }
    return null;
}

export fn fclose(stream: ?*anyopaque) callconv(.c) c_int {
    if (stream == null) return 0;
    const handle = @as(*OpenFileHandle, @ptrCast(@alignCast(stream)));
    handle.in_use = false;
    return 0;
}

export fn fread(ptr: ?*anyopaque, size: usize, nmemb: usize, stream: ?*anyopaque) callconv(.c) usize {
    if (stream == null or ptr == null) return 0;
    const handle = @as(*OpenFileHandle, @ptrCast(@alignCast(stream)));
    if (!handle.in_use) return 0;
    
    const file = &vfs_files[handle.file_idx];
    const total_bytes = size * nmemb;
    const available = file.data.len - handle.cursor;
    const to_read = @min(total_bytes, available);
    
    const dest = @as([*]u8, @ptrCast(ptr))[0..to_read];
    @memcpy(dest, file.data[handle.cursor..handle.cursor + to_read]);
    
    handle.cursor += to_read;
    return to_read / size; // Return number of elements read
}

export fn fseek(stream: ?*anyopaque, offset: c_long, origin: c_int) callconv(.c) c_int {
    if (stream == null) return -1;
    const handle = @as(*OpenFileHandle, @ptrCast(@alignCast(stream)));
    if (!handle.in_use) return -1;
    
    const file_len = @as(c_long, @intCast(vfs_files[handle.file_idx].data.len));
    const current = @as(c_long, @intCast(handle.cursor));
    
    var new_pos: c_long = 0;
    switch (origin) {
        0 => new_pos = offset, // SEEK_SET
        1 => new_pos = current + offset, // SEEK_CUR
        2 => new_pos = file_len + offset, // SEEK_END
        else => return -1,
    }
    
    if (new_pos < 0) new_pos = 0;
    if (new_pos > file_len) new_pos = file_len; // Or allow seeking past end? C allows it. But we just clamp for SAFETY.
    
    handle.cursor = @as(usize, @intCast(new_pos));
    return 0;
}

export fn ftell(stream: ?*anyopaque) callconv(.c) c_long {
    if (stream == null) return -1;
    const handle = @as(*OpenFileHandle, @ptrCast(@alignCast(stream)));
    return @as(c_long, @intCast(handle.cursor));
}

export fn rewind(stream: ?*anyopaque) callconv(.c) void {
    if (stream == null) return;
    const handle = @as(*OpenFileHandle, @ptrCast(@alignCast(stream)));
    handle.cursor = 0;
}

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
