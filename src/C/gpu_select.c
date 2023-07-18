#ifdef _WIN32
#include "Windows.h"

//Windows only, from here: https://stackoverflow.com/a/27881472
__declspec(dllexport) DWORD NvOptimusEnablement = 1;
__declspec(dllexport) int AmdPowerXpressRequestHighPerformance = 1;
#endif
