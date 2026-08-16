//! Process spawning and desktop-entry Exec expansion.

use std::{
    env,
    ffi::OsString,
    path::Path,
    process::{Command, Stdio},
};

use super::{Error, domain::Entry};

pub(super) fn start_entry(entry: &Entry) -> Result<(), Error> {
    if command_exists("gtk-launch") {
        let mut command = Command::new("gtk-launch");
        command
            .arg(entry.id.as_str())
            .stdin(Stdio::null())
            .stdout(Stdio::null())
            .stderr(Stdio::null());

        if command.spawn().is_ok() {
            return Ok(());
        }
    }

    let argv = expand_exec(entry)?;
    let (program, arguments) = argv.split_first().ok_or_else(|| Error::InvalidExec {
        id: entry.id.clone(),
        exec: entry.exec.clone(),
    })?;

    let mut command = if entry.terminal {
        let terminal = terminal_command()?;
        let mut command = Command::new(&terminal.program);
        command.args(&terminal.arguments);
        command.arg(program);
        command.args(arguments);
        command
    } else {
        let mut command = Command::new(program);
        command.args(arguments);
        command
    };

    if let Some(directory) = &entry.working_dir {
        command.current_dir(directory);
    }

    command
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null());
    command.spawn().map_err(|source| Error::Spawn {
        command: launch_command_debug(entry, program, arguments),
        source,
    })?;
    Ok(())
}

fn launch_command_debug(entry: &Entry, program: &str, arguments: &[String]) -> String {
    if entry.terminal {
        format!("terminal -> {} {}", program, arguments.join(" "))
    } else if arguments.is_empty() {
        program.to_string()
    } else {
        format!("{program} {}", arguments.join(" "))
    }
}

fn expand_exec(entry: &Entry) -> Result<Vec<String>, Error> {
    let tokens = shlex::split(&entry.exec).ok_or_else(|| Error::InvalidExec {
        id: entry.id.clone(),
        exec: entry.exec.clone(),
    })?;
    let mut expanded = Vec::new();

    for token in tokens {
        let token = expand_token(&token, entry)?;
        if token.is_empty() {
            continue;
        }
        expanded.push(token);
    }

    if expanded.is_empty() {
        return Err(Error::InvalidExec {
            id: entry.id.clone(),
            exec: entry.exec.clone(),
        });
    }

    Ok(expanded)
}

fn expand_token(token: &str, entry: &Entry) -> Result<String, Error> {
    let mut output = String::new();
    let mut chars = token.chars().peekable();

    while let Some(character) = chars.next() {
        if character != '%' {
            output.push(character);
            continue;
        }

        let Some(code) = chars.next() else {
            output.push('%');
            break;
        };

        match code {
            '%' => output.push('%'),
            'c' => output.push_str(&entry.name),
            'k' => output.push_str(&entry.desktop_path.to_string_lossy()),
            'i' => {}
            'f' | 'F' | 'u' | 'U' | 'd' | 'D' | 'n' | 'N' | 'v' | 'm' => {}
            other => {
                return Err(Error::UnsupportedExecPlaceholder {
                    id: entry.id.clone(),
                    placeholder: other,
                });
            }
        }
    }

    Ok(output)
}

fn terminal_command() -> Result<TerminalCommand, Error> {
    if let Some(command) =
        env::var_os("TERMINAL").and_then(|value| parse_terminal_command(value).ok())
    {
        return Ok(command);
    }

    for (program, arguments) in [
        ("x-terminal-emulator", &["-e"][..]),
        ("kitty", &["-e"][..]),
        ("alacritty", &["-e"][..]),
        ("foot", &["-e"][..]),
        ("wezterm", &["start", "--"][..]),
        ("gnome-terminal", &["--"][..]),
        ("konsole", &["-e"][..]),
    ] {
        if command_exists(program) {
            return Ok(TerminalCommand {
                program: program.to_string(),
                arguments: arguments
                    .iter()
                    .map(|argument| argument.to_string())
                    .collect(),
            });
        }
    }

    Err(Error::MissingTerminalEmulator)
}

fn parse_terminal_command(value: OsString) -> Result<TerminalCommand, Error> {
    let command = value.to_string_lossy().into_owned();
    let mut parts = shlex::split(&command).ok_or_else(|| Error::InvalidTerminalCommand {
        command: command.clone(),
    })?;
    let program = parts
        .first()
        .cloned()
        .ok_or_else(|| Error::InvalidTerminalCommand { command })?;
    if !command_exists(&program) {
        return Err(Error::MissingConfiguredTerminal { command: program });
    }
    let _ = parts.remove(0);

    Ok(TerminalCommand {
        program,
        arguments: parts,
    })
}

struct TerminalCommand {
    program: String,
    arguments: Vec<String>,
}

pub(super) fn command_exists(command: &str) -> bool {
    let candidate = command_name(command);
    let candidate_path = Path::new(&candidate);
    if candidate_path.components().count() > 1 {
        return candidate_path.exists();
    }

    env::var_os("PATH").is_some_and(|path| {
        env::split_paths(&path).any(|directory| directory.join(&candidate).exists())
    })
}

pub(super) fn command_name(command: &str) -> String {
    shlex::split(command)
        .and_then(|parts| parts.first().cloned())
        .unwrap_or_else(|| command.to_string())
}

#[cfg(test)]
mod tests {
    use std::path::{Path, PathBuf};

    use super::{command_name, expand_token};
    use crate::launcher::domain::{Entry, SearchFields, normalize};

    fn entry(id: &str, name: &str, exec: &str) -> Entry {
        Entry {
            id: super::super::domain::Id::new(id).expect("test entry ID should be non-empty"),
            name: name.to_string(),
            subtitle: Some("Browser".to_string()),
            icon_name: Some("firefox".to_string()),
            icon_path: Some(PathBuf::from(
                "/usr/share/icons/hicolor/scalable/apps/firefox.svg",
            )),
            icon_resolved: true,
            terminal: false,
            desktop_path: Path::new("/usr/share/applications").join(id),
            exec: exec.to_string(),
            working_dir: None,
            normalized: SearchFields {
                name: normalize(name),
                exec: normalize(exec),
                subtitle: normalize("Browser"),
                keywords: vec![normalize("web")],
            },
        }
    }

    #[test]
    fn expand_token_replaces_supported_placeholders() {
        let entry = entry("firefox.desktop", "Firefox", "firefox %u");
        assert_eq!(
            expand_token("%c", &entry).expect("placeholder should expand"),
            "Firefox"
        );
        assert_eq!(
            expand_token("%k", &entry).expect("placeholder should expand"),
            "/usr/share/applications/firefox.desktop"
        );
    }

    #[test]
    fn command_name_uses_first_exec_token() {
        assert_eq!(command_name("firefox --new-window"), "firefox");
        assert_eq!(command_name("\"code\" --reuse-window"), "code");
    }
}
