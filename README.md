# NUM.rs

Print a number or ASCII char in various formats. 🤷

## How to install

### Compile *Manually* 😎

For the installation a rust compiler (and cargo) is required.

```bash
git clone https://github.com/El-Wumbus/num.rs.git
cd num.rs
cargo build --release
install -D -m 751 ./target/release/num ~/.local/bin/num
```

> You can install the program to any directory in your `$PATH` with any permissions you desire.

You may, instead, want to install with cargo

```bash
git clone https://github.com/El-Wumbus/num.rs.git
cd num.rs
cargo install --path .
```

## Usage

```output
$ num -help
num.rs 0.1.1

USAGE:
    num [value]...

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

ARGS:
    <value>...    The value, can be hex (0x41), octal (0o101), decimal (65), an ASCII char (A), or a binary number
                  (0b01000001)
```

### Example

```output
$ num A 66 0o102 0x43 0b1000100
+-------+---------+-------+------+-----------+
| ASCII | Decimal | Octal | Hex  | binary    |
+-------+---------+-------+------+-----------+
| A     | 65      | 0101  | 0x41 | 0b1000001 |
+-------+---------+-------+------+-----------+
| B     | 66      | 0102  | 0x42 | 0b1000010 |
+-------+---------+-------+------+-----------+
| B     | 66      | 0102  | 0x42 | 0b1000010 |
+-------+---------+-------+------+-----------+
| C     | 67      | 0103  | 0x43 | 0b1000011 |
+-------+---------+-------+------+-----------+
| D     | 68      | 0104  | 0x44 | 0b1000100 |
+-------+---------+-------+------+-----------+
```

## Contributing

* Check the issues (if there are any), it's a good place to start when you don't know what to do.
* Fork the repository and create pull requests to this repository.
* Don’t change the formatting - Dont reformat or otherwise change the formatting of source code or documentation in the repo. Use the same formatting as the rest of the codebase. Format with rustfmt using the `rustfmt.toml` configuration file.
* Make documentation - If adding features or otherwise changing the user experience create documentation regarding the added or changed features.
* Use space only indentation in all source code files - Do not use tabs or any form of indentation other than spaces. Use 2 space indentation.
