# IPS

An implementation of the [IPS](http://justsolve.archiveteam.org/wiki/IPS_(binary_patch_format)) binary patching format.

- [spec](https://zerosoft.zophar.net/ips.php) consulted for this implementation

## Requirements

- Cargo >=1.63.0
- Rust >=1.63.0

## Usage

Compile and call via the command line.

Expects three arguments:

- `-p | --patch` path to an IPS patch file
- `-r | --rom` path to a ROM file to patch
- `-t | --target` path to a target file to write the patched ROM to

```sh
cargo run -- --patch ./my-patch.ips --rom ./my-rom.rom --target ./my-patched-rom.rom
```

Or use as a lib.

```rs
use std::fs::File;

use ips_rs::{Error, apply_patch};

fn main() {
  let patch_file = File::open("./my-patch.ips").unwrap();
  let rom_file = File::open("./my-rom.rom").unwrap();
  let out_file = File::create("./my-patched-rom.rom").unwrap();

  match ips_rs::apply_patch(&mut patch_file, &mut rom_file, &mut out_file) {
    Ok(()) => println!("patch completed"),
    Err(e) => eprintln!("{e}"),
  }
}
```
