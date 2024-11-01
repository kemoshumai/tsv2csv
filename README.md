# tsv2csv

tsv2csv is a simple command-line tool to convert TSV (Tab-Separated Values) files to CSV (Comma-Separated Values) format.

## Installation

To install tsv2csv, you need to have Rust and Cargo installed. Then, you can build the project using the following commands:

```sh
cargo install --git https://github.com/kemoshumai/tsv2csv.git
```

## Usage

You can use tsv2csv by specifying the input and output files. If no input or output file is specified, it will read from stdin and write to stdout respectively.

```sh
tsv2csv --input <input_file> --output <output_file>
```

### Examples

Convert a TSV file to CSV and save the output to a file:

```sh
tsv2csv --input data.tsv --output data.csv
```

Convert a TSV file to CSV and print the output to stdout:

```sh
tsv2csv --input data.tsv
```

Read from stdin and write to stdout:

```sh
cat data.tsv | tsv2csv
```

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

## Contributing

Contributions are welcome! Please open an issue or submit a pull request for any improvements or bug fixes.

## Contact

For any questions or suggestions, please contact [Kemoshumai](https://x.com/kemoshumai).
