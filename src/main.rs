mod cli;
mod intent;
mod merge;
mod worktree;

use clap::Parser;
use cli::{Cli, Commands};

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Switch {
            branch,
            create,
            agent,
            intent,
            scope,
            exec,
            risk,
        } => {
            worktree::switch(branch, create, agent, intent, scope, exec, risk)?;
        }
        Commands::List { long, json } => {
            worktree::list(long, json)?;
        }
        Commands::Merge {
            target,
            squash,
            dry_run,
        } => {
            merge::run(target, squash, dry_run)?;
        }
        Commands::Clean { all } => {
            worktree::clean(all)?;
        }
    }

    Ok(())
}
