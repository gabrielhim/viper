use clap::Parser;
use viper::align_sequences;

#[derive(Parser)]
#[command(
    version,
    about = "Pairwise alignment tool for biological sequences.",
    arg_required_else_help = true
)]
struct Args {
    /// First sequence
    #[arg(short = '1', long)]
    sequence1: String,

    /// Second sequence
    #[arg(short = '2', long)]
    sequence2: String,
}

fn main() {
    let args = Args::parse();

    let sequence1 = args.sequence1.trim();
    let sequence2 = args.sequence2.trim();

    let alignment = align_sequences(sequence1, sequence2);

    alignment.print_alignment();
}
