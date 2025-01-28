use std::{
    io::{self, stderr, stdout, Write},
    process::Output,
};

use colored::Colorize;

pub fn smallify_command(cmd_text: &str) -> String {
    let replaced = cmd_text.replace("\n", "\\n");
    let max_len = 50;

    if replaced.len() > max_len {
        format!("{}...", &replaced[..max_len - 3])
    } else {
        replaced
    }
}

pub fn prettify_output(input: &str) -> String {
    let mut lines: Vec<_> = input
        .lines()
        .map(|l| format!("{}   {}", "│".bright_black(), l))
        .collect();
    let start = if lines.len() > 10 {
        let start = lines.len() - 11;
        lines[start] = format!("{}   ...", "│".bright_black());
        lines.push(format!(
            "{}   Warning: This output is shortened",
            "│".bright_black()
        ));
        start
    } else {
        0
    };

    lines[start..].join("\n")
}

pub fn is_cmd_passes(output: &Output) -> anyhow::Result<bool> {
    if !output.status.success() {
        stderr().write_all(
            format!(
                "{}\n{}   Error: Command failed with output\n{}\n",
                "│".bright_black(),
                "│".bright_black(),
                prettify_output(&String::from_utf8(output.stderr.clone())?)
            )
            .as_bytes(),
        )?;

        let mut input = String::new();

        print!(
            "{}\n{}   Would you like to proceed to other tasks after this one failed? (y/N) ",
            "│".bright_black(),
            "│".bright_black(),
        );
        io::stdout().flush()?;

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        let input = input.trim().to_lowercase();

        if input != "y" {
            println!("Exiting...");
            return Err(anyhow::anyhow!("Exit after failed command"));
        }

        println_std("Continuing...")?;
        return Ok(false);
    }

    Ok(true)
}

pub fn println_no_space_std<T: AsRef<str>>(value: T) -> anyhow::Result<()> {
    let s = "│".bright_black().to_string() + value.as_ref() + "\n";

    stdout().write_all(s.as_bytes())?;

    Ok(())
}

pub fn println_std<T: AsRef<str>>(value: T) -> anyhow::Result<()> {
    let s = format!("{}   ", "│".bright_black()) + value.as_ref() + "\n";

    stdout().write_all(s.as_bytes())?;

    Ok(())
}
