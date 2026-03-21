//! `cst` — Claude Sentinel CLI
//!
//! Intelligent Claude Code account, profile, and session manager.

use anyhow::Result;
use clap::{CommandFactory, Parser, Subcommand};
use clap_complete::{generate, Shell};

mod commands;
use commands::{profile as profile_cmd, session as session_cmd, shell as shell_cmd};

#[derive(Parser)]
#[command(
    name = "cst",
    about = "🛡 Claude Sentinel — intelligent Claude Code account manager",
    version,
    propagate_version = true
)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Switch to a profile:session (shell function wraps this).
    /// Without args, opens the interactive TUI.
    Use {
        /// Profile name, optionally with session: "work" or "work:backend"
        profile_session: Option<String>,
    },

    /// Show current profile:session, auth type, and quota status.
    Status,

    /// List all profiles and their sessions.
    List,

    /// Show quota used %, tokens today, and time to reset.
    Remaining,

    /// Show switch history with reasons.
    History,

    /// Explain why the current profile is active.
    Why,

    /// Create a new profile.
    New {
        name: String,
        /// Auth type: oauth, api, bedrock, vertex
        #[arg(long, default_value = "oauth")]
        auth: String,
        /// Base on a template: pro, max, api, bedrock, vertex
        #[arg(long)]
        template: Option<String>,
    },

    /// Import current ~/.claude.json as a named profile.
    Import {
        #[arg(long)]
        r#as: Option<String>,
    },

    /// Clone a profile.
    Clone { source: String, destination: String },

    /// Delete a profile.
    Rm { name: String },

    /// Rename a profile.
    Rename { old: String, new: String },

    /// Re-run OAuth login for a profile.
    Login { profile: Option<String> },

    /// Add an API key to a profile's key pool.
    AddKey {
        profile: String,
        #[arg(long, default_value = "1")]
        slot: u8,
    },

    /// Session management subcommands.
    Session {
        #[command(subcommand)]
        action: SessionCommands,
    },

    /// Auto-switch daemon management.
    Daemon {
        #[command(subcommand)]
        action: DaemonCommands,
    },

    /// Auto-switch configuration.
    AutoSwitch {
        #[command(subcommand)]
        action: AutoSwitchCommands,
    },

    /// Pause auto-switching temporarily.
    Pause {
        #[arg(long)]
        minutes: Option<u64>,
    },

    /// Run a command with a specific profile (no persistent switch).
    Run {
        profile_session: String,
        #[arg(last = true)]
        cmd: Vec<String>,
    },

    /// Rebuild symlinks from ~/.claude/ to all sessions.
    Sync,

    /// Show usage statistics.
    Stats {
        profile_session: Option<String>,
    },

    /// Health check — validate all profiles, symlinks, and credentials.
    Doctor,

    /// Output shell init code (add `eval "$(cst shell-init)"` to your rc).
    ShellInit {
        #[arg(long)]
        shell: Option<String>,
    },

    /// Output env var exports for a profile:session (used by shell function).
    #[command(name = "_env", hide = true)]
    Env { profile_session: String },

    /// List available profile templates.
    Templates,

    /// Validate a profile's config and credentials.
    Validate { profile: String },

    /// Generate shell tab completions.
    Completions {
        /// Shell: bash, zsh, fish, powershell
        shell: Shell,
    },

    /// Open the interactive TUI.
    #[command(alias = "tui")]
    Tui,

    /// First-run setup wizard.
    Init {
        /// Accept all defaults without prompting.
        #[arg(long)]
        yes: bool,
        /// Shell to configure: zsh, bash, fish
        #[arg(long)]
        shell: Option<String>,
        /// Skip starting the daemon.
        #[arg(long)]
        no_daemon: bool,
    },
}

#[derive(Subcommand)]
enum SessionCommands {
    /// Create a new session for the current profile.
    New {
        name: String,
        #[arg(long)]
        tag: Option<String>,
    },
    /// List sessions for a profile.
    List { profile: Option<String> },
    /// Delete a session.
    Rm { name: String },
    /// Add a description tag to a session.
    Tag { name: String, description: String },
    /// Archive a session (hidden from list, history kept).
    Archive { name: String },
}

#[derive(Subcommand)]
enum DaemonCommands {
    Start,
    Stop,
    Restart,
    Status,
    Logs,
}

#[derive(Subcommand)]
enum AutoSwitchCommands {
    /// Interactively configure fallback chain and reset estimate.
    Configure { profile: String },
    /// Show auto-switch event log.
    Log,
    /// Dry-run the fallback chain.
    Test { profile: String },
}

#[tokio::main]
async fn main() -> Result<()> {
    // Initialise tracing subscriber (respects RUST_LOG)
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::from_default_env()
                .add_directive("cst=info".parse()?),
        )
        .with_target(false)
        .init();

    let cli = Cli::parse();

    match cli.command {
        None => {
            // No subcommand → open TUI
            commands::tui::run().await
        }
        Some(Commands::Tui) => commands::tui::run().await,
        Some(Commands::ShellInit { shell }) => shell_cmd::shell_init(shell),
        Some(Commands::Env { profile_session }) => shell_cmd::env_cmd(&profile_session),
        Some(Commands::Status) => commands::status::run(),
        Some(Commands::List) => commands::list::run(),
        Some(Commands::Remaining) => commands::quota::remaining(),
        Some(Commands::History) => commands::history::run(),
        Some(Commands::Why) => commands::history::why(),
        Some(Commands::New { name, auth, template }) => {
            profile_cmd::new(&name, &auth, template.as_deref()).await
        }
        Some(Commands::Import { r#as: alias }) => profile_cmd::import(alias.as_deref()),
        Some(Commands::Clone { source, destination }) => profile_cmd::clone(&source, &destination),
        Some(Commands::Rm { name }) => profile_cmd::remove(&name),
        Some(Commands::Rename { old, new }) => profile_cmd::rename(&old, &new),
        Some(Commands::Login { profile }) => profile_cmd::login(profile.as_deref()).await,
        Some(Commands::AddKey { profile, slot }) => profile_cmd::add_key(&profile, slot),
        Some(Commands::Session { action }) => session_cmd::dispatch(action).await,
        Some(Commands::Daemon { action }) => commands::daemon::dispatch(action).await,
        Some(Commands::AutoSwitch { action }) => commands::auto_switch::dispatch(action).await,
        Some(Commands::Pause { minutes }) => commands::auto_switch::pause(minutes),
        Some(Commands::Run { profile_session, cmd }) => {
            commands::run::run_with_profile(&profile_session, &cmd).await
        }
        Some(Commands::Sync) => commands::sync::run(),
        Some(Commands::Stats { profile_session }) => {
            commands::stats::run(profile_session.as_deref())
        }
        Some(Commands::Doctor) => commands::doctor::run(),
        Some(Commands::Validate { profile }) => commands::doctor::validate(&profile),
        Some(Commands::Templates) => commands::templates::list(),
        Some(Commands::Init { yes, shell, no_daemon }) => {
            commands::init::run(yes, shell.as_deref(), !no_daemon).await
        }
        Some(Commands::Completions { shell }) => {
            let mut cmd = Cli::command();
            let name = cmd.get_name().to_string();
            generate(shell, &mut cmd, name, &mut std::io::stdout());
            Ok(())
        }
        Some(Commands::Use { profile_session }) => {
            // `cst use` is normally handled by the shell function.
            // If called directly (not via eval), just print the env vars.
            let ps = profile_session.unwrap_or_else(|| "default:default".to_string());
            shell_cmd::env_cmd(&ps)
        }
    }
}
