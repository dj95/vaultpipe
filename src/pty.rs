use miette::{bail, miette, IntoDiagnostic, Result};
use portable_pty::PtySystem;
use portable_pty::{CommandBuilder, NativePtySystem, PtySize};
use terminal_size::{terminal_size, Height, Width};

use std::collections::BTreeMap;
use std::env;
use std::io::{BufRead, BufReader};

fn build_command(
    command: &[String],
    env: BTreeMap<String, String>,
    clear_env: bool,
) -> Result<CommandBuilder> {
    let executable = command.first().unwrap();
    let args = &command[1..];

    let mut builder = CommandBuilder::new(executable);
    builder.args(args);

    let cwd = env::current_dir().into_diagnostic()?;
    tracing::debug!("{:?}", cwd);
    builder.cwd(cwd);

    if clear_env {
        builder.env_clear();

        if let Ok(path) = env::var("PATH") {
            builder.env("PATH", path);
        }
    }

    for (key, value) in &env {
        builder.env(key, value);
    }

    Ok(builder)
}

pub fn run(command: &[String], env: BTreeMap<String, String>, clear_env: bool) -> Result<u32> {
    let (Width(w), Height(h)) = match terminal_size() {
        Some(s) => s,
        None => bail!("cannot get terminal size"),
    };

    tracing::debug!("terminal width: {}, height: {}", w, h);

    let pty_system = NativePtySystem::default();
    let pair = pty_system
        .openpty(PtySize {
            rows: w,
            cols: h,
            pixel_width: 0,
            pixel_height: 0,
        })
        .map_err(|e| miette!(e))?;

    let mut child = pair
        .slave
        .spawn_command(build_command(command, env, clear_env)?)
        .map_err(|e| miette!(e))?;

    // Both stdout and stderr are merged here!
    let mut reader = BufReader::new(pair.master.try_clone_reader().map_err(|e| miette!(e))?);

    let mut line = String::new();
    while reader.read_line(&mut line).into_diagnostic()? > 0 {
        print!("{}", line); // Already interleaved output
        line.clear();
    }

    let status = child.wait().into_diagnostic()?;

    Ok(status.exit_code())
}
