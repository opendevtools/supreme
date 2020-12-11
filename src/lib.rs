mod commands;
mod utils;

use commands::{add, github_actions, graphql, rescript};
use structopt::StructOpt;
use utils::{helpers::Result, project::ProjectType};

#[derive(Debug, StructOpt)]
enum AddCommand {
    /// Create a base setup for config files
    Config,
    /// Add gitignore files
    Git,
    /// Add GraphQL Codegen
    GraphqlCodegen,
    /// Add Husky setup
    Husky,
    /// Add Jest setup with typeahead
    Jest,
    /// Add nvmrc with current node version
    Nvm,
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
        #[structopt(long, short, parse(from_flag= std::ops::Not::not))]
        no_npm: bool,

        /// Project type
        #[structopt(long, short, possible_values = &ProjectType::variants(), case_insensitive = true)]
        project: Option<ProjectType>,
    },

    /// Create GraphQL API
    Graphql { name: String },

    /// Create a ReScript project
    Rescript { name: String },
}

pub fn run() -> Result<()> {
    let opt = Cli::from_args();

    match opt {
        Cli::Add(AddCommand::Config) => add::config()?,
        Cli::Add(AddCommand::Git) => add::git()?,
        Cli::Add(AddCommand::GraphqlCodegen) => add::graphql_codegen()?,
        Cli::Add(AddCommand::Husky) => add::husky()?,
        Cli::Add(AddCommand::Jest) => add::jest()?,
        Cli::Add(AddCommand::Nvm) => add::nvm()?,
        Cli::Add(AddCommand::Prettier) => add::prettier()?,
        Cli::GithubActions { no_npm, project } => github_actions::run(no_npm, project)?,
        Cli::Graphql { name } => graphql::run(name)?,
        Cli::Rescript { name } => rescript::run(name)?,
    };

    Ok(())
}
