# vanity-miner

Ethereum(conflux eSpace) address generator

## Download

You can download the latest release [here](https://github.com/iosh/vanity-miner/releases/latest).

## Usage

```bash
vanity-miner  --help
Usage: vanity-miner [OPTIONS] <--from-mnemonic|--from-private-key>

Options:
      --from-private-key
          Generate address from a private key
  -a, --max-attempts <MAX_ATTEMPTS>
          Maximum number of attempts to generate addresses (default: unlimited)
  -l, --limit <LIMIT>
          Maximum number of matching addresses to return (default: 1)
  -t, --threads <THREADS>
          Number of concurrent threads to use (default: number of CPU cores)
  -c, --contains <CONTAINS>
          Substrings that the address must contain (case-insensitive)
  -p, --prefix <PREFIX>
          Prefix that the address must start with
  -s, --suffix <SUFFIX>
          Suffix that the address must end with
  -r, --regex <REGEX>
          Regular expression that the address must match (supports Rust regex syntax). Example: "^[a-zA-Z0-9]{4}.*\\d{2}$"
      --from-mnemonic
          Generate address from a random mnemonic phrase
      --derivation-path <DERIVATION_PATH>
          Derivation path to use when generating a mnemonic phrase address [default: m/44'/60'/0'/0/0]
  -m, --mnemonic-words <MNEMONIC_WORDS>
          Number of mnemonic words to use when generating a mnemonic phrase address. Only
  -h, --help
          Print help
  -V, --version
          Print version
```
1. generate random address by private key

```bash
vanity-miner --from-private-key -a 10000000 -l 100 -t 4 -p 0000

found address: 0x0000d5b34d86f06659412c30143840e14b106c52, secret: f625874f4f8ebf2a8c3deafd705d0fdd45b12e4fc16c590164911b3d3db619e9
found address: 0x000007f94cc0e698a3a0388a038d8cc2fcc161e5, secret: d4fea502a8d6442257b0e879ac85e9109bd67b73df910bcb5b9cae059b705d82

```
This will use 4 threads, generate 10000000 addresses, and return 100 matching addresses

2. generate random address by mnemonic

```bash

vanity-miner  --from-mnemonic  -a 100000 -p 0000 -t 5

found address: 0x0000939f6d88f30ef85d59ab6c621b6426f54700, secret: major ensure ask agree hip hill owner over pen wagon kingdom flame
```
This will use 5 threads, generate 100000 addresses, and return addresses that start with 0000 
