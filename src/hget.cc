#include <chrono>
#include <thread>
#include <iostream>
#include <mutex>
#include <condition_variable>

// Compile with VS2019 command prompt
// cl /EHsc /Zi src/hget.cc embed-http-lib/target/debug/embedhttp.lib

#ifdef _WIN32
#pragma comment(lib, "ncrypt") // hyper-tls
#pragma comment(lib, "crypt32") // hyper-tls
#pragma comment(lib, "secur32") // hyper-tls
#pragma comment(lib, "ws2_32")
#pragma comment(lib, "bcrypt")
#pragma comment(lib, "userenv")
#pragma comment(lib, "advapi32")
#pragma comment(lib, "ntdll")
#endif

// waitgroup: thank you golang
class waitgroup {
    std::mutex mutex;
    std::condition_variable condition;
    unsigned long counter = 0;

public:
    void add() {
        std::lock_guard<std::mutex> lock(this->mutex);
        this->counter++;
        std::cout << "wg::add: counter=" << this->counter << std::endl;
    }

    void done() {
        std::lock_guard<std::mutex> lock(this->mutex);
        this->counter--;
        std::cout << "wg::done: counter=" << this->counter << std::endl;
        this->condition.notify_all();
        std::cout << "wg::done: notify_all called\n";
    }

    void wait() {
        std::unique_lock<std::mutex> lock(this->mutex);
        std::cout << "wg::wait: calling condition.wait\n";
        this->condition.wait(lock, [this] () { return this->counter == 0; });
        std::cout << "wg::wait: condition.wait finished\n";
    }
};

auto wait = false;
waitgroup wg;

// Rust exported function
extern "C" void hget(const char *name, void (*cb)(const char *res), bool wait);

// C++ asynchronous callback
extern "C" void hget_cb(const char *res)
{
    std::cout << "cb: res=" << res << std::endl;
    if (!wait) wg.done();
}

void usage()
{
    std::cout << "usage: hget [options] (<urls>...)\n";
    std::cout << "options:\n"
              << "  -?, -h:      show this help\n"
              << "  -w:          do synchronous operation\n"
              << std::endl;
}

int main(int argc, char **argv)
{
        if (argc == 1)
    {
        std::cerr << "usage: hget.exe [<urls>...]\n";
        return 0;
    }

    int i = 1;
    for (; i < argc; i++)
    {
        std::string arg = argv[i];

        if (arg == "-?" || arg == "-h")
        {
            usage();
            return 0;
        }
        if (arg == "-w")
        {
            wait = true;
            continue;
        }
        break;
    }

    std::cout << "processing:\n";

    int n = 0;
    for (; i < argc; i++, n++)
    {
        if (!wait) wg.add();
        std::string arg = argv[i];
        std::cout << "url ptr: " << 
            static_cast<const void*>(argv[i]) << "/" <<
            static_cast<const void*>(arg.c_str()) << std::endl;
        hget(arg.c_str(), hget_cb, wait);
    }

    if (!wait) {
        std::cout << "waiting for all threads\n";
        wg.wait();
        std::cout << "all thread have finished\n";
    }

    if (n == 0)
        usage();
    return 0;
}
