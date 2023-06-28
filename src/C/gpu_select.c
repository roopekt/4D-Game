#include "Windows.h"

//from here: https://stackoverflow.com/a/27881472
__declspec(dllexport) DWORD NvOptimusEnablement = 1;
__declspec(dllexport) int AmdPowerXpressRequestHighPerformance = 1;

int request_for_best_gpu_made() {
    return 1;
}
