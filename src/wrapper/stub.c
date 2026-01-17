#include <stdio.h>
#include <stdarg.h>
#include <string.h>

// Stubs for variadic functions that can't be implemented in Rust easily
// These satisfy link dependencies for Swiss Ephemeris when compiling to Wasm

int fprintf(FILE *stream, const char *format, ...) {
    return 0;
}

int printf(const char *format, ...) {
    return 0;
}

int sprintf(char *str, const char *format, ...) {
    // PRELIMINARY FIX: Copy just the format string to the buffer.
    // This provides "file %s not found" instead of "" on error.
    if (str && format) {
        char *d = str;
        const char *s = format;
        while (*s != '\0') {
            *d++ = *s++;
        }
        *d = '\0';
    }
    return 0;
}

int sscanf(const char *str, const char *format, ...) {
    return 0;
}

// --- File I/O Stubs ---
FILE *fopen(const char *filename, const char *mode) { return NULL; }
int fclose(FILE *stream) { return 0; }
size_t fread(void *ptr, size_t size, size_t nmemb, FILE *stream) { return 0; }
size_t fwrite(const void *ptr, size_t size, size_t nmemb, FILE *stream) { return nmemb; }
int fseek(FILE *stream, long offset, int whence) { return 0; }
long ftell(FILE *stream) { return 0; }
int fflush(FILE *stream) { return 0; }
char *fgets(char *s, int size, FILE *stream) { return NULL; }
int fseeko(FILE *stream, off_t offset, int whence) { return 0; }
off_t ftello(FILE *stream) { return 0; }
void rewind(FILE *stream) { }

// --- Stdlib/Env Stubs ---
char *getenv(const char *name) { return NULL; }
void exit(int status) { while(1); } // Trap

// --- IO Stubs ---
int puts(const char *s) { return 0; }

// --- Ctype Stubs ---
int isspace(int c) { return (c == ' ' || c == '\t' || c == '\n' || c == '\r' || c == '\f' || c == '\v'); }
int isdigit(int c) { return (c >= '0' && c <= '9'); }
int isalpha(int c) { return ((c >= 'a' && c <= 'z') || (c >= 'A' && c <= 'Z')); }
int isalnum(int c) { return (isdigit(c) || isalpha(c)); }
int isupper(int c) { return (c >= 'A' && c <= 'Z'); }
int tolower(int c) { return (c >= 'A' && c <= 'Z') ? (c + 32) : c; }
int toupper(int c) { return (c >= 'a' && c <= 'z') ? (c - 32) : c; }




extern double rust_fmod(double, double);
double fmod(double x, double y) {
    return rust_fmod(x, y);
}


// --- Memory Stubs ---
// Forwarding to Rust-exported allocators to ensure single heap management
extern void *rust_malloc(size_t);
extern void rust_free(void *);
extern void *rust_calloc(size_t, size_t);
extern void *rust_realloc(void *, size_t);

void *malloc(size_t size) { return rust_malloc(size); }
void free(void *ptr) { rust_free(ptr); }
void *calloc(size_t nmemb, size_t size) { return rust_calloc(nmemb, size); }
void *realloc(void *ptr, size_t size) { return rust_realloc(ptr, size); }



