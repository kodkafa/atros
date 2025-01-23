use std::process::Command;

use crate::{get_home_var, step::Step};
mod get;

#[derive(Default)]
pub struct Executor {
    pub cmd: &'static str,
    pub parser_file: &'static str,
}

impl Executor {
    pub fn parse(&self, step_path: &str) -> anyhow::Result<Step> {
        let home = get_home_var()?;
        let parser_path = format!("{home}/.config/atros/.atros/parsers/{}", self.parser_file);

        let out = Command::new("sh")
            .arg("-c")
            .arg(format!(
                "stepPath={} {} {}",
                step_path, self.cmd, parser_path
            ))
            .output()?;

        if !out.stderr.is_empty() {
            return Err(anyhow::anyhow!(format!(
                "Parse command failed: Stderr is not empty:\n{}",
                String::from_utf8(out.stderr)?
            )));
        }

        let parsed_step: Step = serde_json::from_str(&String::from_utf8(out.stdout)?)?;

        Ok(parsed_step)
    }

    pub fn try_get() -> anyhow::Result<Self> {
        get::get_executor()
    }
}
