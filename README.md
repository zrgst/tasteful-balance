# Mifare UID Converter for ARX

When reading Mifare cards with my Flipper Zero you only get 
UID as a 4-byte HEX number. That does not help much when you want to add this access-card to
an ARX access-control system. So i made this easy converter in Rust to help out.

It takes the 4-byte HEX number as an argument in the command line and gives you the correct
card-number to put in your ARX system.

Remember if you type it with spaces like this "02 3D A2 B8" you need to include "" or ''.
But you can also put it in without spaces like this: 023DA2B8

## Usage:
```bash
> arxmf "02 3D A2 B8"
# or:
> arxmf 023DA2B8
```

```

## Installation:
```bash
git clone https://github.com/zrgst/tasteful-balance.git
#
cd tasteful-balance
#
cargo build --release
#
# Then copy it to a directory in your Path to be able to run it directly from your terminal.
```

