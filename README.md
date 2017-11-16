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
    -d, --dest <DESTINATION>    A destination directory
    -s, --src <SOURCE>          A source .tar.gz file

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

To get the configuration of a WSL distro:

```
yowsl.exe-get-configuration
Get the configuration of a WSL distro

USAGE:
    yowsl.exe get-configuration <DISTRO_NAME>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

ARGS:
    <DISTRO_NAME>    A WSL distro name to get the configuration
```

To set the configuration of a WSL distro:

```
yowsl.exe-set-configuration
Set the configuration of a WSL distro

USAGE:
    yowsl.exe set-configuration <DISTRO_NAME> --default-uid <DEFAULT_UID> --distro-flags <DISTRO_FLAGS>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -u, --default-uid <DEFAULT_UID>      The default Linux user ID for this WSL distro
    -f, --distro-flags <DISTRO_FLAGS>    Flags for this WSL distro

ARGS:
    <DISTRO_NAME>    A WSL distro name to set the configuration
```

## License

YoWSL is distributed under the terms of the Apache License, Version 2.0.
See `LICENSE-APACHE-2-0` for details.
