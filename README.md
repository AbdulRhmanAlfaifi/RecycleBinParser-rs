# RecycleBinParser

A parser for Windows Recycle Bin $I files written in Rust.

## Library Usage

```rust
use recyclebin_parser::RecycleBinParser;
use std::fs::File;

fn main() {
    let mut f = File::open("samples/$IK9O3HW.txt").expect("Unable to open file");
    let data = RecycleBinParser::from_reader(&mut f).expect("Unable to parse the file");
    println!("{:?}", data);
}
```

## Binary Usage

```verilog
RecycleBinParser-rs 0.1.0
AbdulRhman Alfaifi - @A__ALFAIFI
Parser for Recycle Bin $I files

USAGE:
    recyclebin_parser.exe [OPTIONS]

OPTIONS:
        --extract-sid
            Extract the SID by retriving the parent folder name

    -h, --help
            Print help information

        --no-headers
            Don't print headers when using CSV as the output format

    -o, --output <output>
            The file path to write the output to [default: stdout]

        --output-format <output-format>
            Output format. [default: jsonl] [possible values: csv, jsonl]

    -p, --path <PATH>...
            Path(s) to Recycle Bin $I Files to be Parsed - accepts glob [default:
            C:\$Recycle.Bin\*\$I*]

    -V, --version
            Print version information
```

## References

* https://github.com/libyal/dtformats/blob/main/documentation/Windows%20Recycle.Bin%20file%20formats.asciidoc