# vanity-miner

Ethereum(hex) and Conflux core space(base32) address generator.

## Download

You can download the latest release [here](https://github.com/iosh/vanity-miner/releases/latest).

## Usage

```bash
vanity-miner  --help
Usage: vanity-miner [OPTIONS] <--use-mnemonic|--use-private-key>

Options:
      --use-private-key
          Use a private key to generate the address
      --max-attempts <MAX_ATTEMPTS>
          Max attempts to generate addresses (default: unlimited)
      --limit <LIMIT>
          Max matching addresses to return (default: unlimited)
      --threads <THREADS>
          Number of threads to use (default: number of CPU cores)
      --contains <CONTAINS>
          Required substring(s) in the address (case-insensitive)
      --prefix <PREFIX>
          Required prefix for the address
      --suffix <SUFFIX>
          Required suffix for the address
      --regex <REGEX>
          Regex for the address (Rust regex syntax). Example: "^[a-zA-Z0-9]{4}.*\\d{2}$"
      --use-mnemonic
          Use a random mnemonic to generate the address
      --derivation-path <DERIVATION_PATH>
          Derivation path for mnemonic-based address generation [default: m/44'/60'/0'/0/0]
      --mnemonic-words <MNEMONIC_WORDS>
          Number of words in the mnemonic (12, 15, 18, 21, or 24)
      --address-format <ADDRESS_FORMAT>
          Address format: HEX (default) or BASE32 [default: HEX]
      --cfx-network-id <CFX_NETWORK_ID>
          If you want to use base32 for Conflux core space, you need to specify the network id mainnet: 1029 testnet: 1028 default: 1029 [default: 1029]
  -h, --help
          Print help
  -V, --version
          Print version
```

### Examples

1. Generate address from a private key:

```bash
vanity-miner --use-private-key --max-attempts 10000000 --limit 100 --threads 4 --prefix 0000
```

This command will:

Use 4 threads.

- Attempt to generate up to 10,000,000 addresses.
- Return up to 100 matching addresses that start with "0000".
- Output the found addresses and their corresponding private keys, like this:

```
found address: 0x0000d5b34d86f06659412c30143840e14b106c52, secret: f625874f4f8ebf2a8c3deafd705d0fdd45b12e4fc16c590164911b3d3db619e9
found address: 0x000007f94cc0e698a3a0388a038d8cc2fcc161e5, secret: d4fea502a8d6442257b0e879ac85e9109bd67b73df910bcb5b9cae059b705d82
```

2. Generate address from a mnemonic:

```bash
vanity-miner --use-mnemonic --max-attempts 100000 --prefix 0000 --threads 5
```

This command will:

- Use 5 threads.
- Attempt to generate up to 100,000 addresses.
- Return the first matching address that starts with "0000".
- Output the found address and its corresponding mnemonic phrase, like this:

```bash
found address: 0x000044b3c579c3617d3ee24c5898f41604da5725, secret: annual spend legend mix fatal bone valley fold buffalo hobby undo negative
```

3. Generate address with Conflux BASE32 format:

```bash
vanity-miner --use-private-key --address-format BASE32 --cfx-network-id 1029 --max-attempts 10000000 --prefix aammmm
```

This command will:

- Attempt to generate up to 10,000,000 addresses.
- Return up to 100 matching addresses that start with "aammmm".
- Output the found addresses and their corresponding private keys, like this:

```
found address: cfx:aammmmft805zu26h0bg3nngk18sz85w7wyvuntatuy, secret: d09ccd59339418c71a22fe37f8a265a518bf6ca2a47ea9c61a36887fd96cc661
```

Note: The conflux base32 address has special rules, so you can't get the address start with aaa The third digit of personal address can always only be one of `jkmnprst`

### Performance Note:

Generating addresses from a private key is significantly faster than generating from a mnemonic (approximately 32 times faster based on tests with an i5-12500H CPU).

### Disclaimer:

This tool is provided as-is, without any warranty. Use it at your own risk.
