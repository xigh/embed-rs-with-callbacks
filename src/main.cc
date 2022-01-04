#include <stdio.h> 

// Compile with VS2019 command prompt 
// cl /Zi src\main.cc embed-lib\target\debug\embed.lib

extern "C" void hello(const char *name);
extern "C" void foo(void (*cb)(const char *msg));
extern "C" void bar(const char *name, void (*cb)(const char *msg));

#ifdef _WIN32
#pragma comment(lib, "ws2_32")
#pragma comment(lib, "bcrypt")
#pragma comment(lib, "userenv")
#pragma comment(lib, "advapi32")
#endif

extern "C" void cb(const char *msg) {
    printf("cb: %s\n", msg);
}

int main(int argc, char **argv) {
    hello("xigh");
    hello(nullptr);
    hello("");

    printf("calling foo at %p\n", cb);
    foo(cb);
    printf("called foo\n");

    printf("calling bar at %p\n", cb);
    bar("Jean", cb);
    printf("called bar\n");
    return 0;
}
