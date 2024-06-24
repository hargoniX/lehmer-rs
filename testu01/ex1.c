// Adapted from TestU01 manual, Figure 2.2, Figure 2.4

#include <stdio.h>
#include <stdlib.h>
#include <stdbool.h>
#include <unistd.h>
#include <getopt.h>
#include "TestU01.h"

// Example PRNG: Xorshift 32
static unsigned int y = 2463534242U;
unsigned int xorshift_32 (void) {
    y ^= (y << 13);
    y ^= (y >> 17);
    return y ^= (y << 5);
}

void usage(const char* progname) {
    printf("%s: [-v] [-s] [-m] [-b] [-f seed]\n", progname);
    exit(1);
}

int main (int argc, char** argv)
{
    const char* progname = argv[0];

    // Config options for TestU01:
    swrite_Basic = FALSE;

    // Config options for tests
    bool testSmallCrush = false;
    bool testCrush = false;
    bool testBigCrush = false;

    int opt;
    while((opt = getopt(argc, argv, "smbhvf:")) != -1) {
        switch(opt) {
            case 's':
                testSmallCrush = true;
                break;
            case 'm':
                testCrush = true;
                break;
            case 'b':
                testBigCrush = true;
                break;
            case 'v':
                swrite_Basic = TRUE;
                break;
            case 'f':
                printf("seed: %s\n", optarg);
                break;
            case '?':
            case 'h':
            default :
                printf("Help/Usage Example\n");
                usage(progname);
                break;
            case -1:
                break;
        }
    }

    // Run tests.
    unif01_Gen* gen = unif01_CreateExternGenBits("Xorshift 32", xorshift_32);
    if (testSmallCrush) {
        bbattery_SmallCrush(gen);
    }
    if (testCrush) {
        bbattery_Crush(gen);
    }
    if (testBigCrush) {
        bbattery_BigCrush(gen);
    }
    fflush(stdout);

    // Clean up.
    unif01_DeleteExternGenBits(gen);

    return 0;
}
