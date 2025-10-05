
use std::io::{self, BufRead, BufReader};
use std::process::{ChildStdout, Command, Stdio};
use std::env;
use std::path::Path;

pub fn execute_command(command: &str) -> String {
    if command.starts_with("cd") {
        return handle_cd_command(command);
    }
    
    let commands = parse_input(command);
    match execute_analysed_commands(&commands) {
        Ok(output) => output,
        Err(e) => format!("Error: {}", e),
    }
}

fn handle_cd_command(command: &str) -> String {
    let args: Vec<&str> = command.split_whitespace().collect();
    let new_dir = args.get(1).copied().unwrap_or("~");
    let expanded_dir = navigate_to_directory(new_dir);

    match env::set_current_dir(Path::new(&expanded_dir)) {
        Ok(_) => format!("Changed directory to: {}", expanded_dir),
        Err(e) => format!("Error: {}", e),
    }
}

fn navigate_to_directory(path: &str) -> String {
    if path == "~" || path.starts_with("~/") {
        if let Ok(home) = env::var("HOME") {
            return path.replacen("~", &home, 1);
        }
    }
    path.to_string()
}

fn parse_input(command: &str) -> Vec<Vec<String>> {
    command.split('|')
        .map(|seg| {
            seg.trim()
                .split_whitespace()
                .map(|s| s.to_string())
                .collect()
        })
        .collect()
}

fn execute_analysed_commands(commands: &[Vec<String>]) -> io::Result<String> {
    let mut children = Vec::new();
    let mut previous_stdout: Option<ChildStdout> = None;
    let mut output = String::new();
    
    for (i, cmd) in commands.iter().enumerate() {
        if cmd.is_empty() {
            continue;
        }

        let mut c = Command::new(&cmd[0]);
        if cmd.len() > 1 {
            c.args(&cmd[1..]);
        }

        let stdin = match previous_stdout.take() {
            Some(out) => Stdio::from(out),
            None => Stdio::inherit(),
        };
        c.stdin(stdin);

        if i == commands.len() - 1 {
            c.stdout(Stdio::piped());
        } else {
            c.stdout(Stdio::inherit());
        }

        let mut child = c.spawn()?;

        if i == commands.len() - 1 {
            if let Some(stdout) = child.stdout.take() {
                let reader = BufReader::new(stdout);
                for line in reader.lines() {
                    if let Ok(line) = line {
                        output.push_str(&line);
                        output.push('\n');
                    }
                }
            }
        } else {
            previous_stdout = child.stdout.take();
        }

        children.push(child);
    }

    for mut child in children {
        child.wait()?;
    }

    Ok(output)
}
