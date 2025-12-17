use clap::{Parser, Subcommand};

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
pub struct Cli {
    #[clap(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Adds a new expense
    AddExpense {
        /// Description of the expense
        #[clap(short, long)]
        description: String,

        /// Amount of the expense
        #[clap(short, long)]
        amount: f64,

        /// Payer of the expense
        #[clap(short, long)]
        payer: String,

        /// Participants of the expense (comma-separated)
        #[clap(short = 'P', long)]
        participants: String,
    },
    /// Shows the balances of the group
    ShowBalances,
    /// Starts the web server
    Serve,
}
