# Executable launcher

A simple executable file to wrap a call to the target executable file.
The target executable file is in the text file named `<exe_filename>.link`.

## Build
- Debug
The following command create a Windows exe file in `./target/debug/launcher.exe`
```
cargo build
```
- Release
The following command create a Windows exe file in `./target/release/launcher.exe`
```
cargo build --release
```

## Test
- Create a `launcher.exe.link` which content the full path of another executor file
```
C:\Program Files\nodejs\node.exe
```
- Lauch it
```
./launcher.exe
```

## Debug

You can add a first parameter `--debug-launcher` to add some logs.

- Without param
```
$ target/debug/launcher.exe --debug-launcher
[WRAPPER] Params: []
[WRAPPER] Config file: C:\Users\me\prog\launchexe\launcher\target\debug\launcher.exe.link
[WRAPPER] Link to: C:\Program Files\nodejs\node.exe
[WRAPPER] Process ID: 8624
Welcome to Node.js v16.15.0.      
Type ".help" for more information.
>
[WRAPPER] Status code: 0
```
*Note:* interractive mode is managed.

- With param
```
$ target/debug/launcher.exe --debug-launcher --version
[WRAPPER] Params: ["--version"]
[WRAPPER] Config file: C:\Users\antoi\prog\launchexe\launcher\target\debug\launcher.exe.link
[WRAPPER] Link to: C:\Program Files\nodejs\node.exe
[WRAPPER] Process ID: 14920
v16.15.0
[WRAPPER] Status code: 0
```
