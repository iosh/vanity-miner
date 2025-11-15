# vanity-miner

Ethereum (hex) and Conflux Core Space (base32) vanity address generator.

## Download

You can download the latest release [here](https://github.com/iosh/vanity-miner/releases/latest).

## Usage

Run `vanity-miner --help` to see all options. Example output:

```bash
Usage: vanity-miner [OPTIONS] --private-key | --mnemonic

Options:
      --chain <CHAIN>
          Target blockchain id. [default: ethereum]

  -k, --private-key
          Use randomly generated private keys to generate addresses

  -m, --mnemonic
          Use randomly generated mnemonics to generate addresses

  -a, --max-attempts <MAX_ATTEMPTS>
          Max attempts to generate addresses (0 = unlimited)

  -l, --limit <LIMIT>
          Max matching addresses to return (0 = unlimited)

  -t, --threads <THREADS>
          Number of threads to use (default: number of CPU cores)

  -c, --contains <CONTAINS>...
          Required substring(s) in the address (case-insensitive)

  -p, --prefix <PREFIX>
          Required prefix for the address

  -s, --suffix <SUFFIX>
          Required suffix for the address

  -r, --regex <REGEX>
          Regex for the address (Rust regex syntax). Example: "^[0-9a-f]{4}.*\\d{2}$"

  -d, --derivation-path <DERIVATION_PATH>
          Derivation path for mnemonic-based address generation
          [default: m/44'/60'/0'/0/0]

  -w, --mnemonic-words <MNEMONIC_WORDS>
          Number of words in the mnemonic (12, 15, 18, 21, or 24)

      --cfx-network <CFX_NETWORK>
          Conflux network ID for address generation [default: 1029]
            - 1029: Mainnet
            - 1: Testnet
            - Other: Custom network (net<id>)

  -o, --output <OUTPUT>
          Output CSV file path [default: vanity-addresses.csv]

      --console
          Also print each found address to the console

      --no-file
          Do not write results to a CSV file

  -h, --help
          Print help

  -V, --version
          Print version
```

### Address formats

- --chain ethereum
  - Addresses are formatted as hex with EIP-55 checksum, e.g. 0x61B5ccbBee....
  - Matching (--prefix, --contains, --regex, etc.) is applied to the lowercase hex body without 0x.
- --chain conflux
  - Addresses are formatted as Conflux base32, e.g. cfx:acbzt35h1wgpv6rb7....
  - Matching is applied to the lowercase base32 body (the part after cfx: / cfxtest:).
  - Base32 encoding and checksum follow the same rules as the official Conflux JS SDK (@conflux-dev/
    conflux-address-js).

## Quick Start

Here are some common use cases:

1. Generate Ethereum address with prefix:

```bash
vanity-miner --chain ethereum --private-key --prefix 0000 --threads 4
```

2. Generate Conflux address with prefix:

```bash
vanity-miner --chain conflux --private-key --cfx-network 1029 --prefix aa --threads 4
```

3. Generate with mnemonic and specific word count (Ethereum):

```bash
vanity-miner --chain ethereum --mnemonic --mnemonic-words 12 --prefix 0000
```

## Detailed Examples

### 1. Generate Ethereum addresses from private keys

```bash
vanity-miner \
 --chain ethereum \
 --private-key \
 --max-attempts 10000000 \
 --limit 100 \
 --threads 4 \
 --prefix 0000 \
 --output eth_addresses.csv
```

This command will:

- Use 4 threads (--threads 4)
- Attempt up to 10,000,000 addresses (--max-attempts 10000000)
- Return up to 100 matches (--limit 100)
- Find addresses starting with 0000 (on the lowercase hex body, without 0x)
- Save results to eth_addresses.csv (--output eth_addresses.csv)

CSV file example:

```csv
address,secret
0x0000d5b34d86f06659412c30143840e14b106c52,f625874f4f8ebf2a8c3deafd705d0fdd45b12e4fc16c590164911b3d3db619e9
0x000007f94cc0e698a3a0388a038d8cc2fcc161e5,d4fea502a8d6442257b0e879ac85e9109bd67b73df910bcb5b9cae059b705d82
```

For private-key mode, the secret column contains the hex-encoded private key.

### 2. Generate Ethereum addresses from mnemonics

```bash
vanity-miner \
 --chain ethereum \
 --mnemonic \
 --mnemonic-words 12 \
 --max-attempts 100000 \
 --prefix 0000 \
 --threads 5
```

This command will:

- Use a random mnemonic with 12 words (--mnemonic --mnemonic-words 12)
- Use 5 threads (--threads 5)
- Attempt up to 100,000 addresses (--max-attempts 100000)
- Find addresses starting with 0000

Output example:

```csv
0x0000C2Ca1723c6f03cfb33C9704D63B26be9CD79,mnemonic:lamp assist sort symbol ritual perfect blouse clean layer right evidence upper|path:m/44'/60'/0'/0/0
```

### 3. Generate Conflux Core Space base32 addresses

```bash
vanity-miner \
 --chain conflux \
 --private-key \
 --cfx-network 1029 \
 --max-attempts 10000000 \
 --prefix aammmm \
 --threads 4
```

This command will:

- Use Conflux Core Space mainnet (--cfx-network 1029)
- Output base32 addresses with cfx: prefix
- Find addresses whose base32 body starts with aammmm (--prefix aammmm)

Output example:

```csv
cfx:aammmmj2tnxh1hrz73vs1g46bxnpwzam1a3tn0mzur,9ed03e47929bcc1d3ae594e22888079069039ac671c5e4e116574384a7a87100
```

### 4. Use Conflux testnet

```bash
vanity-miner \
 --chain conflux \
 --private-key \
 --cfx-network 1 \
 --prefix aa
```

This will generate addresses with cfxtest: prefix for Conflux testnet.

## Advanced Features

1. Multiple pattern matching (all must match):

```bash
vanity-miner \
 --chain ethereum \
 --private-key \
 --contains 123 \
 --contains 456 \
 --contains 789
```

2. Regular expression:

```bash
vanity-miner \
 --chain ethereum \
 --private-key \
 --regex "^[0-9]{4}.\*[a-f]{4}$"
```

Matching is always done on the normalized address string:

- Ethereum: lowercase hex body without 0x
- Conflux: lowercase base32 body without the cfx: / cfxtest: prefix

## File Output

The program can automatically save found addresses to a CSV file:

- By default, results are written to vanity-addresses.csv in the current directory.
- Use -o or --output to specify a custom output file path.
- Use --console to also print each found address to stdout.
- Use --no-file to disable CSV output and only print to the console.

The CSV file contains the address and the corresponding secret (private key or mnemonic/path).

## Disclaimer

This tool is provided as-is, without any warranty. Use it at your own risk. Always keep your private keys
and mnemonics secure.
