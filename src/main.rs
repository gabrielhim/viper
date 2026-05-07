use anstyle::{AnsiColor, Color, Style};
use clap::{Parser, builder::Styles};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::process;
use viper::align_sequences;

fn define_styles() -> Styles {
    Styles::styled()
        .usage(
            Style::new()
                .bold()
                .fg_color(Some(Color::Ansi(AnsiColor::BrightGreen))),
        )
        .header(
            Style::new()
                .bold()
                .fg_color(Some(Color::Ansi(AnsiColor::BrightGreen))),
        )
        .literal(Style::new().fg_color(Some(Color::Ansi(AnsiColor::Cyan))))
        .placeholder(Style::new().fg_color(Some(Color::Ansi(AnsiColor::Cyan))))
}

fn read_fasta_file(file_path: &str) -> Vec<char> {
    let mut record_chars: Vec<char> = Vec::new();

    let file_content = match File::open(file_path) {
        Err(e) => panic!("Couldn't open file {}: {}", file_path, e),
        Ok(content) => content,
    };
    let reader = BufReader::new(file_content);

    for line in reader.lines() {
        let row = match line {
            Err(e) => panic!("Couldn't read line: {}", e),
            Ok(record) => record,
        };
        if !row.starts_with(">") && !row.is_empty() {
            let chars: Vec<char> = row.trim().chars().collect();
            record_chars.extend(chars);
        }
    }

    record_chars
}

#[derive(Parser)]
#[command(
    version,
    about = "Pairwise alignment tool for biological sequences.",
    styles = define_styles(),
    arg_required_else_help = true
)]
struct Args {
    /// FASTA file with the first sequence
    #[arg(short = '1', long)]
    fasta1: String,

    /// FASTA file with the second sequence
    #[arg(short = '2', long)]
    fasta2: String,

    /// Whether viper should perform local alignment
    #[arg(short, long, action)]
    local: bool,

    /// Score for matching positions
    #[arg(short, long, default_value_t = 2)]
    match_score: i32,

    /// Penalization for every substitution
    #[arg(short = 's', long, default_value_t = 3)]
    mismatch_penalty: i32,

    /// Penalization for every new gap opened in an indel
    #[arg(short, long, default_value_t = 2)]
    gap_penalty: i32,

    /// Penalization for every position extended in an existing gap
    #[arg(short = 'e', long, default_value_t = 1)]
    gap_extension_penalty: i32,
}

fn main() {
    let args = Args::parse();

    if args.gap_extension_penalty >= args.gap_penalty {
        eprintln!("Gap extension penalty should be lower than gap penalty.");
        process::exit(1);
    };

    let seq1_chars = read_fasta_file(&args.fasta1);
    let seq2_chars = read_fasta_file(&args.fasta2);

    align_sequences(
        seq1_chars,
        seq2_chars,
        args.match_score,
        args.mismatch_penalty,
        args.gap_penalty,
        args.gap_extension_penalty,
        args.local,
    );
}
