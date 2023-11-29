use inquire::{Confirm, Select};
use std::{env, process::Command};

fn main() {
    // Check if -a flag is passed to run git add -A before commit
    let args: Vec<String> = env::args().collect();
    let run_git_add = args.len() > 1 && args.contains(&String::from("-a"));
    // Check if -d or --dry-run flag is passed to run in dry run mode
    let dry_run = args.len() > 1
        && (args.contains(&String::from("-d")) || args.contains(&String::from("--dry-run")));
    if dry_run {
        println!("Running in dry run mode\n");
    }

    // Commit Type
    let commit_type_options = vec![
        "build", "ci", "docs", "feat", "fix", "perf", "refactor", "test",
    ];
    let commit_type = Select::new("Type:", commit_type_options).prompt();
    let commit_type = match commit_type {
        Ok(commit_type) => String::from(commit_type),
        Err(_) => {
            println!("No commit type selected, exiting");
            return;
        }
    };

    // Summary
    let commit_summary = inquire::Text::new("Summary:").prompt();
    let commit_summary = match commit_summary {
        Ok(commit_summary) => commit_summary,
        Err(_) => {
            println!("No commit summary entered, exiting");
            return;
        }
    };

    let result_message = format!("{}: {}", commit_type, commit_summary);

    // Confirm
    let confirm =
        Confirm::new(format!("Result:\n\n{}\n\nCommit? (y/n):", result_message,).as_str()).prompt();

    match confirm {
        Ok(true) => {
            if run_git_add {
                println!("Running git add -A");
                if !dry_run {
                    Command::new("git")
                        .args(["add", "-A"])
                        .output()
                        .expect("failed to execute process");
                }
            }

            println!("Running git commit -m \"{}\"", result_message);
            if !dry_run {
                Command::new("git")
                    .args(["commit", "-m", result_message.as_str()])
                    .output()
                    .expect("failed to execute process");
            }
        }
        _ => {
            println!("Exiting");
            return;
        }
    }

    println!("Done 🎉");
}
