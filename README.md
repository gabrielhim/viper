# Viper :snake:

Viper is a simple, yet fast and precise command-line tool for pairwise alignment of biological sequences.

## Installation

One way to install viper is to clone the project and run `cargo install`. This requires Rust and the Cargo package, follow the instructions in the [Rust Book](https://doc.rust-lang.org/book/ch01-01-installation.html) to install them.
```bash
git clone https://github.com/gabrielhim/viper.git
cd viper
cargo install --path .
viper --help
```

Alternatively, you can build a Docker image with viper by running `docker build` in the root of the cloned project:
```bash
docker build -t viper:v2.0 .
docker run --rm -it viper:v2.0 bash
viper --help
```

Finally, if you are using Linux, you can download the binary directly from a release and copy it to a directory in $PATH:
```bash
wget https://github.com/gabrielhim/viper/releases/download/v2.0.0/viper
chmod +x viper
cp viper /usr/local/bin/
viper --help
```

## Usage

Sequences to be aligned should be provided in FASTA files. Specify them in the parameters `-1` (or `--fasta1`) and `-2` (or `--fasta2`):
```bash
echo -e ">seq1\nAGGTGTAGAGAT" > seq1.fa
echo -e ">seq2\nAAGGAGTATGAAG" > seq2.fa

viper -1 seq1.fa -2 seq2.fa
```

The output is printed in stdout. It shows the conserved positions as well as insertions and deletions in both sequences:
```
A-GGTGTA-GA-GAT
| || ||| || |
AAGGAGTATGAAG--

Alignment score: 6
```

Viper applies a score for matched positions and penalties for mismatches, gap openings and gap extensions. Check the values in the tool's help output. These values can be changed on the command line, if you wish, for example, to penalize gap openings more than substitutions:
```bash
viper -1 seq1.fa -2 seq2.fa --gap-penalty 3 --mismatch-penalty 1
```

In this example, viper reduces the number of indels in favor of substitutions:
```
A-GGTGTAGAGAT
| || |||   |
AAGGAGTATGAAG

Alignment score: 7
```

The default alignment mode is global. It expects sequences similar in size and performs alignment along their entire lengths. Alternatively, viper can receive a `--local` flag for local alignment, in which case it will locate the substring of both sequences with the maximum similarity. This option is useful for comparing short conserved regions in DNA sequences of different species, for example.

**Global mode**
```bash
echo -e ">seq1\nGCCCGGTTACGCTAGGGGGCACGAGCATGCAG" > seq1.fa
echo -e ">seq2\nGCCGGGGTCGTTTTCAGCGGTTACGCTAGTTA" > seq2.fa

viper -1 seq1.fa -2 seq2.fa
```

Output:
```
GCCCGG--TTACG----CTAG-GG--GGCACGAGCATGCAG---
||| ||  |  ||    | || ||     |||  | |  ||
GCC-GGGGT--CGTTTTC-AGCGGTT---ACG--C-T--AGTTA

Alignment score: 4
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

Alignment score: 24
```
