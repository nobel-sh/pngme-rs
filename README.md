# Pngme-rs
## Hide secret messages in PNG Files.
### Implementation of https://picklenerd.github.io/pngme_book/introduction.html


# 
## usage:
```
Usage: pngme-rs <COMMAND>

Commands:
  encode  Hide message in a PNG File
  decode  Decode hidden message from a PNG File
  remove  Remove the hidden message from a PNG File
  print   Print all chunks in a PNG File
  help    Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version
```

### Can be run with cargo
```
git clone https://github.com/nobel-sh/pngme-rs.git
cd pngme-rs
cargo run
```