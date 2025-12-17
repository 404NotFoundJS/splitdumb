use clap::{Parser, Subcommand};

#[derive(Parser)]
#[clap(author, version, about = "Splitdumb - Expense sharing made simple")]
pub struct Cli {
    #[clap(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Starts the web server
    Serve {
        /// Port to listen on
        #[clap(short, long, default_value = "3000")]
        port: u16,

        /// Path to the data file
        #[clap(short, long, default_value = "app_data.json")]
        data_file: String,
    },

    /// Adds a new expense
    AddExpense {
        /// Description of the expense
        #[clap(short, long)]
        description: String,

        /// Amount of the expense
        #[clap(short, long)]
        amount: f64,

        /// Payer of the expense
        #[clap(short = 'P', long)]
        payer: String,

        /// Participants of the expense (comma-separated)
        #[clap(short = 'u', long)]
        participants: String,

        /// Path to the data file
        #[clap(long, default_value = "app_data.json")]
        data_file: String,
    },

    /// Shows the balances of the group
    ShowBalances {
        /// Path to the data file
        #[clap(long, default_value = "app_data.json")]
        data_file: String,
    },

    /// Shows suggested settlements
    ShowSettlements {
        /// Path to the data file
        #[clap(long, default_value = "app_data.json")]
        data_file: String,
    },
}
