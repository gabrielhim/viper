# Viper :snake:

Viper is a simple, yet fast and precise command-line tool for pairwise global alignment of biological sequences.

## Installation

Installation requires cloning the project and running `cargo install`:

```bash
git clone https://github.com/gabrielhim/viper.git
cd viper
cargo install --path .
```

## Usage

Specify the sequences to be aligned in the parameters `-1` (or `--sequence1`) and `-2` (or `--sequence2`):

```bash
viper -1 AGGTGTAGAGGT -2 AAAGGAGTTGAAG
```

Output:
```
A--GGTGTAGA-GGT
|  || || || |
AAAGGAGTTGAAG--
```
