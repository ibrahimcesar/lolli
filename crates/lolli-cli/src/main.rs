//! Lolli - Linear Logic Workbench CLI
//!
//! A toolkit for working with linear logic — parsing formulas, searching for proofs,
//! extracting computational content, and compiling to Rust.

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "lolli")]
#[command(author = "Ibrahim Cesar")]
#[command(version)]
#[command(about = "Linear Logic Workbench - Parse, prove, extract, and compile linear logic", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Parse and pretty-print a formula
    Parse {
        /// Formula to parse
        formula: String,
    },

    /// Check if a sequent is provable
    Prove {
        /// Sequent to prove (e.g., "A, B |- A * B")
        sequent: String,

        /// Maximum search depth
        #[arg(short, long, default_value = "100")]
        depth: usize,

        /// Output format: tree, latex, dot
        #[arg(short, long, default_value = "tree")]
        format: String,
    },

    /// Extract a term from a proof
    Extract {
        /// Sequent to prove
        sequent: String,

        /// Normalize the extracted term
        #[arg(short, long)]
        normalize: bool,
    },

    /// Generate Rust code from a proof
    Codegen {
        /// Sequent to prove
        sequent: String,

        /// Output file
        #[arg(short, long)]
        output: Option<String>,
    },

    /// Visualize a proof
    Viz {
        /// Sequent to prove
        sequent: String,

        /// Output format: tree, latex, dot, svg
        #[arg(short, long, default_value = "tree")]
        format: String,

        /// Output file (stdout if not specified)
        #[arg(short, long)]
        output: Option<String>,
    },

    /// Run interactive REPL
    Repl,
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Parse { formula } => {
            println!("Parsing: {}", formula);
            println!("(Parser not yet implemented - see Issue #6)");
        }

        Commands::Prove {
            sequent,
            depth,
            format,
        } => {
            println!("Proving: {} (depth: {}, format: {})", sequent, depth, format);
            println!("(Prover not yet implemented - see Issues #8-11)");
        }

        Commands::Extract { sequent, normalize } => {
            println!(
                "Extracting from: {} (normalize: {})",
                sequent, normalize
            );
            println!("(Extractor not yet implemented - see Issues #12-14)");
        }

        Commands::Codegen { sequent, output } => {
            println!(
                "Generating code for: {} (output: {:?})",
                sequent, output
            );
            println!("(Codegen not yet implemented - see Issues #15-17)");
        }

        Commands::Viz {
            sequent,
            format,
            output,
        } => {
            println!(
                "Visualizing: {} (format: {}, output: {:?})",
                sequent, format, output
            );
            println!("(Visualization not yet implemented - see Issues #18-20)");
        }

        Commands::Repl => {
            println!("Lolli Linear Logic Workbench REPL");
            println!("(REPL not yet implemented - see Issue #22)");
            println!();
            println!("Commands:");
            println!("  :prove <sequent>  - Prove a sequent");
            println!("  :parse <formula>  - Parse and display a formula");
            println!("  :extract <sequent> - Extract term from proof");
            println!("  :codegen <sequent> - Generate Rust code");
            println!("  :help             - Show help");
            println!("  :quit             - Exit REPL");
        }
    }
}
