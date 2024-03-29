mod commands;
mod config;
mod utils;

use clap::{ArgAction, Parser};
use commands::*;
use utils::{helpers::Result, project::ProjectType};

#[derive(Parser)]
enum AddCommand {
    /// Create a base setup for config files
    Config,
    /// Add gitignore files
    Git {
        /// Project type
        #[clap(value_enum, long, short, ignore_case = true)]
        project: Option<ProjectType>,
    },
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
    /// Add Tailwind CSS
    Tailwind,
}

#[derive(Parser)]
enum RemoveCommand {
    /// Remove config setup
    Config,
    /// Remove gitignore files
    Git,
    /// Remove GraphQL Codegen
    GraphqlCodegen,
    /// Remove Husky setup
    Husky,
    /// Remove Jest setup
    Jest,
    /// Remove nvmrc
    Nvm,
    /// Remove Prettier
    Prettier,
}

#[derive(Parser)]
enum Config {
    /// List current configuration
    List,
    /// Update Supreme configuration
    Set {
        /// Set which Node installer you want to use
        #[clap(value_enum, long, ignore_case = true)]
        node: config::NodeInstaller,
    },
}

#[derive(Parser)]
enum Workspace {
    /// Add package to workspace
    Add {
        /// The name of the package
        #[clap(num_args=0..)]
        packages: Vec<String>,
        /// Install as devDependency
        #[clap(long, short)]
        dev: bool,
    },
    /// Remove package from workspace
    Remove {
        /// The name of the package
        #[clap(num_args=0..)]
        packages: Vec<String>,
    },
}

/// Supreme
#[derive(Parser)]
#[clap(version, name = "Supreme")]
enum Cli {
    /// Add packages and config files
    #[clap(subcommand)]
    Add(AddCommand),

    /// List or update Supreme configuration
    #[clap(subcommand)]
    Config(Config),

    /// Add GitHub actions
    GithubActions {
        /// Remove release to npm
        #[clap(long, short, action = ArgAction::SetFalse)]
        no_npm: bool,

        /// Project type
        #[clap(value_enum, long, short, ignore_case = true)]
        project: Option<ProjectType>,
    },

    /// Create GraphQL API
    Graphql { name: String },

    /// Install a Node package
    Install {
        /// The name of the package(s)
        #[clap(num_args=0..)]
        packages: Vec<String>,
        /// Install as devDependency
        #[clap(long, short)]
        dev: bool,
        /// Run install with --lockfile-only (npm only)
        #[clap(long, short)]
        sync_lockfile: bool,
        /// Install globally
        #[clap(long, short)]
        global: bool,
    },

    /// Remove any setup from add command
    #[clap(subcommand)]
    Remove(RemoveCommand),

    /// Create a ReScript project
    Rescript { name: String },

    /// Display and execute available package.json scripts
    Run { name: Option<String> },

    /// Uninstall a Node package
    Uninstall {
        /// The name of the package(s)
        #[clap(num_args=0..)]
        name: Vec<String>,

        /// Uninstall globally
        #[clap(long, short)]
        global: bool,
    },

    /// Update dependencies in a npm or yarn project
    UpdateDependencies,

    /// Add or remove packages from a yarn workspace
    #[clap(subcommand)]
    Workspace(Workspace),
}

pub fn run() -> Result<()> {
    let opt = Cli::parse();

    match opt {
        Cli::Add(AddCommand::Config) => add::config()?,
        Cli::Add(AddCommand::Git { project }) => add::git(project)?,
        Cli::Add(AddCommand::GraphqlCodegen) => add::graphql_codegen()?,
        Cli::Add(AddCommand::Husky) => add::husky()?,
        Cli::Add(AddCommand::Jest) => add::jest()?,
        Cli::Add(AddCommand::Nvm) => add::nvm()?,
        Cli::Add(AddCommand::Prettier) => add::prettier()?,
        Cli::Add(AddCommand::Tailwind) => add::tailwind()?,

        Cli::Config(Config::List) => config::list(),
        Cli::Config(Config::Set { node }) => config::set(node)?,

        Cli::GithubActions { no_npm, project } => github_actions::run(no_npm, project)?,
        Cli::Graphql { name } => graphql::run(name)?,

        Cli::Install {
            dev,
            packages,
            sync_lockfile,
            global,
        } => install::run(packages, dev, sync_lockfile, global)?,

        Cli::Remove(RemoveCommand::Config) => remove::config()?,
        Cli::Remove(RemoveCommand::Git) => remove::git()?,
        Cli::Remove(RemoveCommand::GraphqlCodegen) => remove::graphql_codegen()?,
        Cli::Remove(RemoveCommand::Husky) => remove::husky()?,
        Cli::Remove(RemoveCommand::Jest) => remove::jest()?,
        Cli::Remove(RemoveCommand::Nvm) => remove::nvm()?,
        Cli::Remove(RemoveCommand::Prettier) => remove::prettier()?,

        Cli::Rescript { name } => rescript::run(name)?,
        Cli::Run { name } => run::run(name)?,

        Cli::Uninstall { global, name } => uninstall::run(name, global)?,
        Cli::UpdateDependencies => update_dependencies::run()?,

        Cli::Workspace(Workspace::Add { packages, dev }) => workspace::add(packages, dev)?,
        Cli::Workspace(Workspace::Remove { packages }) => workspace::remove(packages)?,
    };

    Ok(())
}
