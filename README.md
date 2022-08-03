# upload_my_image

Rewrite <https://github.com/YieldRay/upload-my-image> using `Rust`

A cli tool which could run with `Typora`

Build-in config set in `src/servers.rs`

# Build

```sh
cargo build --release
```

# Usage

```sh
USAGE:
    upload_my_image.exe [OPTIONS] [PATH]...

ARGS:
    <PATH>...    Path of the file to upload

OPTIONS:
    -c, --config <CONFIG>    Use a config file
    -d, --debug              Turn debugging information on
    -h, --help               Print help information
    -l, --list               show all avaliable servers
    -s, --server <SERVER>    Select server
    -V, --version            Print version information
```
