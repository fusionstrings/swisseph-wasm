
#include "swephexp.h"
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <time.h>

// Driver to generate reference values for precision check
// Uses Moshier mode (SEFLG_MOSEPH) and explicitly DOES NOT set ephe path
// to match Wasm environment.

double get_random_jd(double min, double max) {
    double scale = rand() / (double) RAND_MAX;
    return min + scale * (max - min);
}

int main(int argc, char *argv[]) {
    char serr[AS_MAXCH];
    double xx[6];
    int ipl;
    double tjd;
    int32 iflag = SEFLG_MOSEPH | SEFLG_SPEED; 
    
    // Seed for deterministic results
    srand(12345);

    // Initialize Swiss Ephemeris
    // DO NOT SET PATH to simulate Wasm 'no file access'
    // swe_set_ephe_path("vendor/swisseph/ephe");
    
    // Print JSON start
    printf("[\n");
    
    int first = 1;
    int i;
    // Generate 1000 random dates
    for (i = 0; i < 1000; i++) {
        // Range 1900-2100 (approx JD 2415020 - 2488070)
        tjd = get_random_jd(2415020.5, 2488070.5);
        
        // Test all main planets: Sun(0) to True Node(11)
        for (ipl = SE_SUN; ipl <= SE_TRUE_NODE; ipl++) {
            // swe_calc (ET) effectively, since tjd is passed as is.
            if (swe_calc(tjd, ipl, iflag, xx, serr) >= 0) {
                 if (!first) printf(",\n");
                 // Output JSON object
                 // Use %.15f to capture full double precision
                 printf("  {\"jd\": %.15f, \"planet\": %d, \"lon\": %.15f, \"lat\": %.15f, \"dist\": %.15f}", 
                        tjd, ipl, xx[0], xx[1], xx[2]);
                 first = 0;
            } else {
                 // Should not happen for Moshier mode usually
                 fprintf(stderr, "Error for JD %f planet %d: %s\n", tjd, ipl, serr);
            }
        }
    }
    
    printf("\n]\n");
    
    swe_close();
    return 0;
}
