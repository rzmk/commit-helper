use super::parser::Cli;
use std::process::Command;

// Run the commands in succession
pub fn orchestrate_commit(cli: &Cli, message: &str) {
    if cli.add {
        println!("Running git add -A");
        run(cli, "git", &["add", "-A"]);
    }

    println!("Running git commit -m \"{}\"", message);
    run(cli, "git", &["commit", "-m", message]);

    if cli.push {
        println!("Running git push");
        run(cli, "git", &["push"]);
    }

    println!("Done 🎉");
}

pub fn print_debug_info(output: std::process::Output) {
    println!("Debug info:");
    println!("stdout:\n{}", String::from_utf8_lossy(&output.stdout));
    println!("stderr:\n{}", String::from_utf8_lossy(&output.stderr));
    println!("Exit status:\n{}", output.status);
}

pub fn run(cli: &Cli, command: &str, args: &[&str]) {
    if !cli.dry_run {
        let output = Command::new(command)
            .args(args)
            .output()
            .expect("failed to execute process");

        if cli.debug {
            print_debug_info(output);
        }
    }
}
