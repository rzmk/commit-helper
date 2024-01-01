use clap::Parser;
use inquire::{formatter::OptionFormatter, Confirm, InquireError, Select};

pub mod parser;
use parser::{Cli, CommitType};
pub mod util;
use util::orchestrate_commit;

fn main() {
    // Parse command line arguments
    let cli: Cli = Cli::parse();

    // If a message was provided, run the commands in succession
    if let Some(message) = &cli.message {
        orchestrate_commit(&cli, &message);
        return;
    }

    // Otherwise, prompt for a message
    let commit_type_options = vec![
        CommitType::new("build", "build system and dependencies"),
        CommitType::new("ci", "continuous integration"),
        CommitType::new("chore", "misc/maintenance not related to core code"),
        CommitType::new("docs", "documentation changes (e.g., README.md, comments)"),
        CommitType::new("feat", "new feature or significant enhancement"),
        CommitType::new("fix", "bug fix or error correction"),
        CommitType::new("perf", "performance improvement"),
        CommitType::new("refactor", "code restructuring or cleanup"),
        CommitType::new("test", "add or update tests"),
    ];

    // Format the commit type options for display
    let formatter: OptionFormatter<CommitType> =
        &|ct| format!("{}: {}", ct.value.name, ct.value.description);

    // Prompt for the commit type
    let selected_type: Result<CommitType, InquireError> = Select::new("Type:", commit_type_options)
        .with_formatter(formatter)
        .prompt();

    // Get the name of the selected commit type
    let commit_type = match selected_type {
        Ok(ct) => ct.name,
        Err(e) => {
            println!("Error: {:?}", e);
            return;
        }
    };

    // Prompt for the commit summary (the message after the commit type)
    let commit_summary = inquire::Text::new("Summary:").prompt();
    let commit_summary = match commit_summary {
        Ok(commit_summary) => commit_summary,
        Err(_) => {
            println!("No commit summary entered, exiting");
            return;
        }
    };

    // Format the commit message to include the commit type and summary
    let result_message = format!("{}: {}", commit_type, commit_summary);

    // Confirm the commit message
    let confirm =
        Confirm::new(format!("Result:\n\n{}\n\nCommit? (y/n):", result_message).as_str()).prompt();

    // If confirmed, run the commands in succession
    match confirm {
        Ok(true) => {
            orchestrate_commit(&cli, &result_message);
        }
        _ => {
            println!("Exiting");
            return;
        }
    }
}
