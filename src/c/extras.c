#include <stddef.h>

// Simple swap function
static void swap(char *a, char *b, size_t size) {
    char temp;
    while (size--) {
        temp = *a;
        *a++ = *b;
        *b++ = temp;
    }
}

// Simple quicksort implementation
void qsort(void *base, size_t nmemb, size_t size, int (*compar)(const void *, const void *)) {
    char *pivot, *run_ptr, *left_ptr;
    
    if (nmemb < 2) return;
    
    pivot = (char *)base + (nmemb - 1) * size;
    left_ptr = (char *)base;
    run_ptr = (char *)base;
    
    for (; run_ptr < pivot; run_ptr += size) {
        if (compar(run_ptr, pivot) < 0) {
            if (run_ptr != left_ptr) {
                swap(run_ptr, left_ptr, size);
            }
            left_ptr += size;
        }
    }
    
    swap(pivot, left_ptr, size);
    
    qsort(base, (left_ptr - (char *)base) / size, size, compar);
    qsort(left_ptr + size, nmemb - ((left_ptr - (char *)base) / size) - 1, size, compar);
}

// Binary search implementation
void *bsearch(const void *key, const void *base, size_t nmemb, size_t size, int (*compar)(const void *, const void *)) {
    const char *pivot;
    int result;
    
    while (nmemb > 0) {
        pivot = (const char *)base + (nmemb >> 1) * size;
        result = compar(key, pivot);
        if (result == 0) return (void *)pivot;
        if (result > 0) {
            base = pivot + size;
            nmemb--;
        }
        nmemb >>= 1;
    }
    return NULL;
}
