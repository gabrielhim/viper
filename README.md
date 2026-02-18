# Viper :snake:

Viper is a simple, yet fast and precise command-line tool for pairwise alignment of biological sequences.

## Installation

Installation requires cloning the project and running `cargo install`:

```bash
git clone https://github.com/gabrielhim/viper.git
cd viper
cargo install --path .
```

## Usage

Sequences to be aligned should be provided in FASTA files. Specify them in the parameters `-1` (or `--fasta1`) and `-2` (or `--fasta2`):

```bash
echo ">seq1\nAGGTGTAGAGAT" > seq1.fa
echo ">seq2\nAAGGAGTATGAAG" > seq2.fa

viper -1 seq1.fa -2 seq2.fa
```

The output is printed in stdout. It shows the conserved positions as well as insertions and deletions in both sequences:
```
A-GGTGTA-GA-GAT
| || ||| || |
AAGGAGTATGAAG--
```

Viper default alignment mode is global. It expects sequences similar in size and performs alignment along their entire lengths. Alternatively, viper can receive a `--local` flag for local alignment, in which case it will locate the substring of both sequences with the maximum similarity. This option is useful for comparing short conserved regions in DNA sequences of different species, for example.

**Global mode**
```bash
echo ">seq1\nGCCCGGTTACGCTAGGGGGCACGAGCATGCAG" > seq1.fa
echo ">seq2\nGCCGGGGTCGTTTTCAGCGGTTACGCTAGTTA" > seq2.fa

viper -1 seq1.fa -2 seq2.fa
```

Output:
```
GCCCGG--TTACG----CTAG-GG--GGCACGAGCATGCAG---
||| ||  |  ||    | || ||     |||  | |  ||
GCC-GGGGT--CGTTTTC-AGCGGTT---ACG--C-T--AGTTA
```

**Local mode**
```bash
viper -1 seq1.fa -2 seq2.fa --local
```

Output:
```
              GCCCGGTTACGCTAGGGGGCACGAGCATGCAG
                 ||||||||||||
GCCGGGGTCGTTTTCAGCGGTTACGCTAGTTA
```
