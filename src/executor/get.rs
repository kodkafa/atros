use super::Executor;
use std::process::Command;

pub fn get_executor() -> anyhow::Result<Executor> {
    let mut executor = Executor::default();

    for exec in ["bun", "npx", "deno"] {
        let output = Command::new(exec).arg("--version").output();

        if let Ok(output) = output {
            if output.status.success() {
                executor.parser_file = match exec {
                    "deno" => "deno.ts",
                    _ => "default.ts",
                };

                executor.cmd = match exec {
                    "npx" => "npx ts-node",
                    "bun" => "bun run",
                    "deno" => "deno run --allow-env --allow-read",
                    _ => panic!("Unreachable code"),
                };

                break;
            }
        }
    }

    if executor.cmd.is_empty() {
        return Err(anyhow::anyhow!("No JS Executor Is Found"));
    }

    Ok(executor)
}
