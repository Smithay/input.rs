# Rust libinput bindings

[![Build Status](https://img.shields.io/github/workflow/status/Smithay/input.rs/Rust?logo=github-actions&logoColor=white&style=for-the-badge)](https://github.com/Smithay/input.rs/actions/workflows/ci.yml)
[![Crates.io](https://img.shields.io/crates/v/input.svg?logo=rust&style=for-the-badge)](https://crates.io/crates/input)
[![License](https://img.shields.io/crates/l/input.svg?style=for-the-badge)](https://crates.io/crates/input)
[![Docs](https://img.shields.io/docsrs/input?style=for-the-badge)](https://docs.rs/input)

[libinput](https://wayland.freedesktop.org/libinput/doc/latest/) bindings for [Rust](https://www.rust-lang.org)

These bindings closely follow libinput's concepts and it's original API.
Please refer to the [libinput documentation](https://wayland.freedesktop.org/libinput/doc/latest/) to understand the general structure and concepts.

## Usage

Add to your `Cargo.toml`:

```toml
input = "0.5"
```

Configure and run event loop:

```rust
use std::fs::{File, OpenOptions};
use std::os::unix::{fs::OpenOptionsExt, io::{RawFd, FromRawFd, IntoRawFd}};
use std::path::Path;

use input::{Libinput, LibinputInterface};
use libc::{O_RDONLY, O_RDWR, O_WRONLY};

struct Interface;

impl LibinputInterface for Interface {
    fn open_restricted(&mut self, path: &Path, flags: i32) -> Result<RawFd, i32> {
        OpenOptions::new()
            .custom_flags(flags)
            .read((flags & O_RDONLY != 0) | (flags & O_RDWR != 0))
            .write((flags & O_WRONLY != 0) | (flags & O_RDWR != 0))
            .open(path)
            .map(|file| file.into_raw_fd())
            .map_err(|err| err.raw_os_error().unwrap())
    }
    fn close_restricted(&mut self, fd: RawFd) {
        unsafe {
            File::from_raw_fd(fd);
        }
    }
}

fn main() {
    let mut input = Libinput::new_with_udev(Interface);
    input.udev_assign_seat("seat0").unwrap();
    loop {
        input.dispatch().unwrap();
        for event in &mut input {
            println!("Got event: {:?}", event);
        }
    }
}
```
