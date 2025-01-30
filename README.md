# vanity-miner

Ethereum(hex) and Conflux core space(base32) address generator.

## Download

You can download the latest release [here](https://github.com/iosh/vanity-miner/releases/latest).

## Usage

```bash
vanity-miner  --help
Usage: vanity-miner [OPTIONS] <--use-mnemonic|--use-private-key>

Options:
  -k, --use-private-key
          Use a private key to generate the address
  -a, --max-attempts <MAX_ATTEMPTS>
          Max attempts to generate addresses (default: unlimited)
  -l, --limit <LIMIT>
          Max matching addresses to return (default: unlimited)
  -t, --threads <THREADS>
          Number of threads to use (default: number of CPU cores)
  -c, --contains <CONTAINS>
          Required substring(s) in the address (case-insensitive)
  -p, --prefix <PREFIX>
          Required prefix for the address
  -s, --suffix <SUFFIX>
          Required suffix for the address
  -r, --regex <REGEX>
          Regex for the address (Rust regex syntax). Example: "^[a-zA-Z0-9]{4}.*\\d{2}$"
  -m, --use-mnemonic
          Use a random mnemonic to generate the address
  -d, --derivation-path <DERIVATION_PATH>
          Derivation path for mnemonic-based address generation [default: m/44'/60'/0'/0/0]
  -w, --mnemonic-words <MNEMONIC_WORDS>
          Number of words in the mnemonic (12, 15, 18, 21, or 24)
  -f, --address-format <ADDRESS_FORMAT>
          Address format: HEX (default) or BASE32 [default: HEX]
  -n, --cfx-network <CFX_NETWORK>
          Conflux network ID for address generation [default: 1029]
          - 1029: Mainnet
          - 1: Testnet
          - Other: Custom network
  -o, --output-file <OUTPUT_FILE>
          Output CSV file path [default: vanity-addresses.csv]
  -h, --help
          Print help
  -V, --version
          Print version
```

### Quick Start

Here are some common use cases:

1. Generate Ethereum address with prefix:
```bash
vanity-miner --use-private-key --prefix 0000 --threads 4
```

2. Generate Conflux address with prefix:
```bash
vanity-miner --use-private-key --address-format BASE32 --prefix aa --threads 4
```

3. Generate with mnemonic and specific word count:
```bash
vanity-miner --use-mnemonic --mnemonic-words 12 --prefix 0000
```

### Detailed Examples

1. Generate address from a private key:

```bash
vanity-miner --use-private-key --max-attempts 10000000 --limit 100 --threads 4 --prefix 0000 --output-file eth_addresses.csv
```

This command will:
- Use 4 threads (--threads 4)
- Attempt up to 10,000,000 addresses (--max-attempts 10000000)
- Return up to 100 matches (--limit 100)
- Find addresses starting with "0000" (--prefix 0000)
- Save results to eth_addresses.csv file (--output-file eth_addresses.csv)

CSV file content example:
```csv
address,private_key
0x0000d5b34d86f06659412c30143840e14b106c52,f625874f4f8ebf2a8c3deafd705d0fdd45b12e4fc16c590164911b3d3db619e9
0x000007f94cc0e698a3a0388a038d8cc2fcc161e5,d4fea502a8d6442257b0e879ac85e9109bd67b73df910bcb5b9cae059b705d82
```

2. Generate address from a mnemonic:

```bash
vanity-miner --use-mnemonic --mnemonic-words 12 --max-attempts 100000 --prefix 0000 --threads 5
```

This command will:
- Use mnemonic with 12 words (--use-mnemonic --mnemonic-words 12)
- Use 5 threads (--threads 5)
- Attempt up to 100,000 addresses (--max-attempts 100000)
- Find addresses starting with "0000" (--prefix 0000)

Output example:
```
found address: 0x000044b3c579c3617d3ee24c5898f41604da5725, secret: annual spend legend mix fatal bone valley fold buffalo hobby undo negative
```

3. Generate Conflux BASE32 address:

```bash
vanity-miner --use-private-key --address-format BASE32 --cfx-network 1029 --max-attempts 10000000 --prefix aammmm
```

This command will:
- Use BASE32 format (--address-format BASE32)
- Use Conflux mainnet (--cfx-network 1029)
- Find addresses starting with "aammmm" (--prefix aammmm)

Output example:
```
found address: cfx:aammmmft805zu26h0bg3nngk18sz85w7wyvuntatuy, secret: d09ccd59339418c71a22fe37f8a265a518bf6ca2a47ea9c61a36887fd96cc661
```

Note: The Conflux base32 address has special rules. The third digit of a personal address can only be one of `jkmnprst`.

### Advanced Features

1. Multiple Pattern Matching:
```bash
vanity-miner --use-private-key --contains 123 --contains 456 --contains 789  # Address must contain all patterns
```

2. Regular Expression:
```bash
vanity-miner --use-private-key --regex "^[0-9]{4}.*[a-f]{4}$"  # Match specific pattern
```

3. Custom Network:
```bash
vanity-miner --use-private-key --address-format BASE32 --cfx-network 1  # Use testnet
```

### Performance Note

Generating addresses from a private key is significantly faster than generating from a mnemonic (approximately 32 times faster based on tests with an i5-12500H CPU).

### File Output

Program automatically saves found addresses to CSV file:
- Default save to current directory `vanity-addresses.csv`
- Use `-o` or `--output-file` to specify custom output file path
- CSV file contains addresses and corresponding private key/mnemonic

### Disclaimer

This tool is provided as-is, without any warranty. Use it at your own risk.
