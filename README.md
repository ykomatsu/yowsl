# YoWSL

Yet another Windows Subsystem for Linux tweaker.

**This project is in the very early stage of development.**

## Building

```
cargo build --release
```

## Running

To register a WSL distro:

**Warning: YoWSL cannot choose a directory for `rootfs` yet.**
**It always creates `rootfs` in a directory where `yowsl.exe` exists.**

```
yowsl.exe-register
Registers a WSL distro

USAGE:
    yowsl.exe register <DISTRO_NAME> --src <SOURCE>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -s, --src <SOURCE>    A source .tar.gz file

ARGS:
    <DISTRO_NAME>    A WSL distro name to register
```

To unregister a WSL distro:

```
yowsl.exe-unregister
Unregisters a WSL distro

USAGE:
    yowsl.exe unregister <DISTRO_NAME>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

ARGS:
    <DISTRO_NAME>    A WSL distro name to unregister
```

## License

YoWSL is distributed under the terms of the Apache License, Version 2.0.
See `LICENSE-APACHE-2-0` for details.
