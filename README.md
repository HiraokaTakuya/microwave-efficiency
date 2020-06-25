# Microwave-Efficiency

Calculate microwave efficiency.

## Require

This is a Rust project.
You should install Rust developping environment.
Do the following to install Rust environment.
```
curl https://sh.rustup.rs -sSf | sh
```

## Install

```bash
cargo install --git https://github.com/HiraokaTakuya/microwave-efficiency.git
```

## Uninstall

```bash
cargo uninstall microwave-efficiency
```

## Usage

```
microwave-efficiency 1.0.0

USAGE:
    microwave-efficiency <microwave-wattage> <heating-time> <water-weight> <temperature-before-heating> <temperature-after-heating>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

ARGS:
    <microwave-wattage>             [W]
    <heating-time>                  [s]
    <water-weight>                  [g]
    <temperature-before-heating>    [℃]
    <temperature-after-heating>     [℃]
```

## License

MIT
