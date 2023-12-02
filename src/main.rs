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
    // Check if -p flag is passed to run git push after commit
    let run_git_push = args.len() > 1 && args.contains(&String::from("-p"));
    // Check if --debug flag is passed to run in debug mode
    let debug = args.len() > 1 && args.contains(&String::from("--debug"));

    // Commit Type
    let commit_type_options = vec![
        "build - build system and dependencies",
        "ci - continuous integration",
        "chore - misc/maintenance not related to core code",
        "docs - documentation changes (e.g., README.md, comments)",
        "feat - new feature or significant enhancement",
        "fix - bug fix or error correction",
        "perf - performance improvement",
        "refactor - code restructuring or cleanup",
        "test - add or update tests",
    ];
    let commit_type = Select::new("Type:", commit_type_options).prompt();
    let commit_type = match commit_type {
        Ok(commit_type) =>
        // Get the first word of the commit type
        {
            let commit_type = commit_type.split_whitespace().next().unwrap();
            commit_type
        }
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
                    let output = Command::new("git")
                        .args(["add", "-A"])
                        .output()
                        .expect("failed to execute process");

                    if debug {
                        println!("Debug info:");
                        println!("stdout:\n{}", String::from_utf8_lossy(&output.stdout));
                        println!("stderr:\n{}", String::from_utf8_lossy(&output.stderr));
                        println!("Exit status:\n{}", output.status);
                    }
                }
            }

            println!("Running git commit -m \"{}\"", result_message);
            if !dry_run {
                let output = Command::new("git")
                    .args(["commit", "-m", result_message.as_str()])
                    .output()
                    .expect("failed to execute process");

                if debug {
                    println!("Debug info:");
                    println!("stdout:\n{}", String::from_utf8_lossy(&output.stdout));
                    println!("stderr:\n{}", String::from_utf8_lossy(&output.stderr));
                    println!("Exit status:\n{}", output.status);
                }
            }

            if run_git_push {
                println!("Running git push");
                if !dry_run {
                    let output = Command::new("git")
                        .args(["push"])
                        .output()
                        .expect("failed to execute process");

                    if debug {
                        println!("Debug info:");
                        println!("stdout:\n{}", String::from_utf8_lossy(&output.stdout));
                        println!("stderr:\n{}", String::from_utf8_lossy(&output.stderr));
                        println!("Exit status:\n{}", output.status);
                    }
                }
            }
        }
        _ => {
            println!("Exiting");
            return;
        }
    }

    println!("Done 🎉");
}
