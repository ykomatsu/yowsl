# YoWSL

[YoWSL](https://yowsl.akaumiga.me/) is yet another Windows Subsystem for Linux tweaker.

The Windows 10 Fall Creators Update has a great feature called
[the Windows Subsystem for Linux (WSL)](https://msdn.microsoft.com/en-us/commandline/wsl/about).
YoWSL enables you to register a WSL distro in an arbitrary folder using your
`.tar.gz` archive.

**This project is in a very early stage of development.**

## Features

* Register a WSL distro from your `.tar.gz` archive
* Unregister a WSL distro
* Get a configuration of a registered WSL distro
* Set a configuration of a registered WSL distro
* Launch a registered WSL distro

## Prerequisites

* The Windows 10 Fall Creators Update (x64)
* The Windows Subsystem for Linux optional feature. You can install it in
  PowerShell as Administrator:

```
> Enable-WindowsOptionalFeature -Online -FeatureName Microsoft-Windows-Subsystem-Linux
```

* A `.tar.gz` archive that contains all files using a WSL distro. If you
  installed some WSL distro from the Microsoft Store, you may find some
  `install.tar.gz` in `C:\Program Files\WindowsApps`

## Installation

### Pre-built binaries

You can download pre-built binaries from
[the downloads page](https://bitbucket.org/ykomatsu/yowsl/downloads).
You will need to extract a `.zip` archive and put `yowsl.exe` in your paths.

### Building

If you have [the Rust toolchain](https://www.rustup.rs/), you can build YoWSL
with [Cargo](https://crates.io/):

```
> cargo install yowsl
```

## References

* [Windows Subsystem for Linux (Windows)](https://msdn.microsoft.com/en-us/library/windows/desktop/mt811415(v=vs.85).aspx)

## License

YoWSL is distributed under the terms of the Apache License, Version 2.0.
See `LICENSE-APACHE-2-0` for details.
