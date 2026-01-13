
#include "swephexp.h"
#include <stdio.h>
#include <stdlib.h>
#include <time.h>
#include <string.h>

// Simple random double generator
double random_jd() {
    double min = 2415020.5;
    double max = 2488070.5;
    double scale = rand() / (double) RAND_MAX;
    return min + scale * (max - min); 
}

int main() {
    char serr[256];
    double xx[6];
    int ipl;
    double tjd;
    int32 iflag = SEFLG_SWIEPH | SEFLG_SPEED;
    
    // Initialize random seed
    srand(time(NULL));

    printf("Starting Memory Audit (Native Sanitizers)...\n");

    // Close any previous open (good practice)
    swe_close();

    // Loop 1000 times
    for (int i = 0; i < 1000; i++) {
        tjd = random_jd();
        
        // Test all main planets
        for (ipl = SE_SUN; ipl <= SE_TRUE_NODE; ipl++) {
            if (swe_calc_ut(tjd, ipl, iflag, xx, serr) < 0) {
                // It's okay if it fails (e.g. missing ephe file), we just want to check for crashes/leaks
                // printf("Error: %s\n", serr);
            }
        }
        
        // Test a fixed star
        if (i % 10 == 0) {
            if (swe_fixstar_ut("Sirius", tjd, iflag, xx, serr) < 0) {
                // ignore error
            }
        }
    }

    swe_close();
    printf("Memory Audit Completed Successfully.\n");
    return 0;
}
