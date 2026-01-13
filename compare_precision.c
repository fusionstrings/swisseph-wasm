#include "vendor/swisseph/swephexp.h"
#include <stdio.h>

int main() {
    double xx[6];
    char serr[256];
    int32 ipl = SE_SUN;
    int32 iflag = SEFLG_MOSEPH | SEFLG_SPEED;
    double tjd = 2451545.0; // Jan 1 2000, 12:00 UT

    // Explicitly set no path to force fallbacks or just Moshier mode
    swe_set_ephe_path(".");

    int32 ret = swe_calc_ut(tjd, ipl, iflag, xx, serr);
    
    if (ret < 0) {
        printf("Error: %s\n", serr);
        return 1;
    }

    // Print longitude with 20 decimals
    printf("%.20f\n", xx[0]);
    return 0;
}
