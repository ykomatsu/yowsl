# YoWSL

Yet another Windows Subsystem for Linux tweaker.

**This project is in the very very early stage of development.**

## Features

* Register a WSL distro from a `.tar.gz` file
* Unregister a WSL distro
* Get the configuration of a registered WSL distro
* Set the configuration of a registered WSL distro

## Building

```
$ cargo build --release
```

## Running

```
> # To register:
> New-Item -Path . -Name MyUbuntu -ItemType Directory
> yowsl.exe register MyUbuntu -s install.tar.gz -d MyUbuntu
> wslconfig.exe /setdefault MyUbuntu
```

```
> # To get the configuration
> yowsl.exe get-configuration MyUbuntu
[MyUbuntu]
version = 1
default_uid = 0
distro_flags = 7 # 0b111
```

```
> # To set the configuration
> yowsl.exe set-configuration MyUbuntu -u 1000 -f 111
```

```
> # To unregister:
> yowsl.exe unregister MyUbuntu
```

## License

YoWSL is distributed under the terms of the Apache License, Version 2.0.
See `LICENSE-APACHE-2-0` for details.
