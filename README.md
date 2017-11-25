# YoWSL

YoWSL is yet another Windows Subsystem for Linux tweaker.

The Windows 10 Fall Creators Update has a great feature called
[the Windows Subsystem for Linux (WSL)](https://msdn.microsoft.com/en-us/commandline/wsl/about).
YoWSL enables you to register a WSL distro in an arbitrary folder using your
`.tar.gz` archive.

**This project is in the very very early stage of development.**

## Features

* Register a WSL distro from your `.tar.gz` archive
* Unregister a WSL distro
* Get a configuration of a registered WSL distro
* Set a configuration of a registered WSL distro
* Launch a registered WSL distro

## Prerequisites

* The Windows 10 Fall Creators Update
* The Windows Subsystem for Linux optional feature. You can install it in
  PowerShell as Administrator:

```
> Enable-WindowsOptionalFeature -Online -FeatureName Microsoft-Windows-Subsystem-Linux
```

* A `.tar.gz` archive that contains all files using a WSL distro. If you
  installed some WSL distro from the Microsoft Store, you may find some
  `install.tar.gz` archive in `C:\\Program Files\\WindowsApps` folder

## Pre-built binaries

You can download pre-built binaries from
[the downloads page](https://bitbucket.org/ykomatsu/yowsl/downloads).
You will need to extract a `.zip` file and put `yowsl.exe` in your paths.

## Building

If you have [the Rust toolchain](https://www.rustup.rs/), you can build YoWSL
with [Cargo](https://crates.io/):

```
> cargo install yowsl
```

## Running

```
> # To register:
> New-Item -Name MyUbuntu -ItemType Directory
> yowsl.exe register MyUbuntu -s install.tar.gz -d MyUbuntu
```

```
> # To launch:
> yowsl.exe launch MyUbuntu
```

```
> # To get the configuration:
> yowsl.exe get-configuration MyUbuntu
[MyUbuntu]
version = 1
default_uid = 0
flags = 7 # 0b111 -> ENABLE_INTEROP (1) | APPEND_NT_PATH (2) | ENABLE_DRIVE_MOUNTING (4)
default_environment_values = ["HOSTTYPE=x86_64", "LANG=en_US.UTF-8", "PATH=/usr/local/sbin:/usr/local/bin:/usr/sbin:/usr/bin:/sbin:/bin:/usr/games:/usr/local/games", "TERM=xterm-256color"]
```

```
> # To set the configuration:
> yowsl.exe set-configuration MyUbuntu -d 1000
```

```
> # To unregister:
> yowsl.exe unregister MyUbuntu
```

## License

YoWSL is distributed under the terms of the Apache License, Version 2.0.
See `LICENSE-APACHE-2-0` for details.
