use anstyle::{AnsiColor, Color, Style};
use clap::{Parser, builder::Styles};
use viper::align_sequences;

fn define_styles() -> Styles {
    Styles::styled()
        .usage(Style::new().fg_color(Some(Color::Ansi(AnsiColor::Green))))
        .header(Style::new().fg_color(Some(Color::Ansi(AnsiColor::Green))))
        .literal(Style::new().fg_color(Some(Color::Ansi(AnsiColor::Blue))))
        .placeholder(Style::new().fg_color(Some(Color::Ansi(AnsiColor::BrightBlue))))
}

#[derive(Parser)]
#[command(
    version,
    about = "Pairwise alignment tool for biological sequences.",
    styles = define_styles(),
    arg_required_else_help = true
)]
struct Args {
    /// First sequence
    #[arg(short = '1', long)]
    sequence1: String,

    /// Second sequence
    #[arg(short = '2', long)]
    sequence2: String,

    /// Whether viper should perform local alignment
    #[arg(short, long, action)]
    local: bool,
}

fn main() {
    let args = Args::parse();

    let alignment = align_sequences(&args.sequence1, &args.sequence2, args.local);

    alignment.print_alignment();
}
