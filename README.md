# Playing with C/C++ callbacks in Rust

### How to compile ?

#### Build embed-lib rust library:

```bash
cd embed-lib
cargo build
```

#### Compile with VS2019 command prompt 

```powershell
cl.exe /Zi src\main.cc embed-lib\target\debug\embed.lib
```

#### Compile with clang on MacOS X (c++17: nullptr) 

```bash
clang++ -g -std=c++17 src/main.cc embed-lib/target/debug/libembed.a -pthread -ldl
```

#### Compile with gcc on Linux 

```bash
g++ -g src/main.cc embed-lib/target/debug/libembed.a -pthread -ldl
```

#### Compile with clang on Linux 

```bash
clang++ -g src/main.cc embed-lib/target/debug/libembed.a -pthread -ldl
```
