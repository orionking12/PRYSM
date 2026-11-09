use clap::{ArgAction, Parser, Subcommand, ValueEnum};
use serde::{Deserialize, Serialize};

#[derive(Parser)]
#[command(name = "prysm")]
#[command(about = "Parallel Git Worktrees for the AI Agent Era", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Open or create an isolated agent worktree
    Switch {
        /// Branch name
        branch: String,

        /// Create branch + worktree
        #[arg(short = 'c', long)]
        create: bool,

        /// Agent identifier (e.g., claude, gpt, codex)
        #[arg(short = 'a', long)]
        agent: Option<String>,

        /// Human goal or task intent
        #[arg(short = 'i', long)]
        intent: Option<String>,

        /// Logical scopes/modules affected (comma-separated)
        #[arg(short = 's', long, value_delimiter = ',')]
        scope: Vec<String>,

        /// Command to execute right after switching
        #[arg(short = 'x', long)]
        exec: Option<String>,

        /// Optional risk assessment hint
        #[arg(long, value_enum)]
        risk: Option<RiskLevel>,
    },

    /// List active agent realities and intents
    List {
        /// Display extended intent and scope details
        #[arg(long)]
        long: bool,

        /// Format output as JSON
        #[arg(long)]
        json: bool,
    },

    /// Converge branch safely into target with intent checks
    Merge {
        /// Target branch to merge into (e.g., main)
        target: String,

        /// Squash commits on merge (use --squash=false to preserve history)
        #[arg(long, action = ArgAction::Set, num_args = 0..=1, require_equals = true, default_missing_value = "true", default_value_t = true)]
        squash: bool,

        /// Perform dry-run intent collision analysis only
        #[arg(long)]
        dry_run: bool,
    },

    /// Clean and remove finished realities
    Clean {
        /// Remove clean PRYSM worktrees whose branches are merged into the primary branch
        #[arg(long)]
        all: bool,
    },
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum RiskLevel {
    Low,
    Med,
    High,
}
