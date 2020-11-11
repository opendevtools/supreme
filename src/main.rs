mod commands;
mod utils;

use commands::{add, github_actions};
use structopt::StructOpt;
use utils::helpers::Result;

#[derive(Debug, StructOpt)]
enum AddCommand {
    /// Add gitignore files
    Git,
    /// Add nvmrc with current node version
    Nvm,
    /// Add Husky setup
    Husky,
    /// Add Prettier
    Prettier,
}

/// Supreme
#[derive(Debug, StructOpt)]
enum Cli {
    /// Add packages and config files
    Add(AddCommand),

    /// Add GitHub actions
    GithubActions {
        /// Remove release to npm
        #[structopt(long = "no-npm", short = "n")]
        no_npm: bool,
    },
}

fn main() -> Result<()> {
    let opt = Cli::from_args();

    match opt {
        Cli::Add(AddCommand::Git) => add::git()?,
        Cli::Add(AddCommand::Nvm) => add::nvm()?,
        Cli::Add(AddCommand::Husky) => add::husky()?,
        Cli::Add(AddCommand::Prettier) => add::prettier()?,
        Cli::GithubActions { no_npm } => github_actions::run(no_npm)?,
    };

    Ok(())
}
