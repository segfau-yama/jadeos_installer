use std::error::Error;
use std::fmt::{Display, Formatter};
use std::io::Write;
use std::process::{Command, Stdio};

use super::command::CommandStrategy;

const DEFAULT_SUPER_USER_PROGRAM: &str = "sudo";
const DEFAULT_SUPER_USER_ARGS: &[&str] = &["--non-interactive"];

#[derive(Debug)]
pub enum CommandError {
    MissingStrategy,
    StartFailed {
        command: String,
        source: std::io::Error,
    },
    Failed {
        command: String,
        stderr: String,
    },
}

impl Display for CommandError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::MissingStrategy => write!(f, "command strategy was not configured"),
            Self::StartFailed { command, source } => {
                write!(f, "failed to start `{command}`: {source}")
            }
            Self::Failed { command, stderr } if stderr.is_empty() => {
                write!(f, "`{command}` failed")
            }
            Self::Failed { command, stderr } => write!(f, "`{command}` failed: {stderr}"),
        }
    }
}

impl Error for CommandError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::StartFailed { source, .. } => Some(source),
            Self::MissingStrategy | Self::Failed { .. } => None,
        }
    }
}

pub struct CommandExecutor {
    strategy: Option<&'static dyn CommandStrategy>,
    super_user_program: &'static str,
    super_user_args: &'static [&'static str],
}

impl CommandExecutor {
    pub fn new() -> Self {
        Self {
            strategy: None,
            super_user_program: DEFAULT_SUPER_USER_PROGRAM,
            super_user_args: DEFAULT_SUPER_USER_ARGS,
        }
    }

    #[cfg(test)]
    fn with_super_user_program(
        super_user_program: &'static str,
        super_user_args: &'static [&'static str],
    ) -> Self {
        Self {
            strategy: None,
            super_user_program,
            super_user_args,
        }
    }

    pub fn set_strategy(&mut self, strategy: &'static dyn CommandStrategy) {
        self.strategy = Some(strategy);
    }

    pub fn execute(&self, args: &[&str]) -> Result<String, CommandError> {
        self.run(false, args, None)
    }

    pub fn super_user_execute(
        &self,
        args: &[&str],
        input: Option<&str>,
    ) -> Result<String, CommandError> {
        self.run(true, args, input)
    }

    pub(crate) fn render_command(
        &self,
        args: &[&str],
        super_user: bool,
    ) -> Result<String, CommandError> {
        let strategy = self.strategy.ok_or(CommandError::MissingStrategy)?;
        Ok(render_command_line(
            strategy.command_name(),
            args,
            super_user,
            self.super_user_program,
            self.super_user_args,
        ))
    }

    fn run(
        &self,
        super_user: bool,
        args: &[&str],
        input: Option<&str>,
    ) -> Result<String, CommandError> {
        let strategy = self.strategy.ok_or(CommandError::MissingStrategy)?;
        let command_name = strategy.command_name();
        let rendered = render_command_line(
            command_name,
            args,
            super_user,
            self.super_user_program,
            self.super_user_args,
        );
        let mut command = if super_user {
            let mut command = Command::new(self.super_user_program);
            command.args(self.super_user_args).arg(command_name);
            command
        } else {
            Command::new(command_name)
        };
        command.args(args);

        if let Some(input) = input {
            let mut child = command
                .stdin(Stdio::piped())
                .stdout(Stdio::piped())
                .stderr(Stdio::piped())
                .spawn()
                .map_err(|source| CommandError::StartFailed {
                    command: rendered.clone(),
                    source,
                })?;

            if let Some(stdin) = child.stdin.as_mut() {
                stdin
                    .write_all(input.as_bytes())
                    .map_err(|source| CommandError::StartFailed {
                        command: rendered.clone(),
                        source,
                    })?;
            }

            let output = child
                .wait_with_output()
                .map_err(|source| CommandError::StartFailed {
                    command: rendered.clone(),
                    source,
                })?;

            return normalize_output(rendered, output);
        }

        let output = command
            .output()
            .map_err(|source| CommandError::StartFailed {
                command: rendered.clone(),
                source,
            })?;
        normalize_output(rendered, output)
    }
}

fn normalize_output(
    rendered: String,
    output: std::process::Output,
) -> Result<String, CommandError> {
    if output.status.success() {
        Ok(String::from_utf8_lossy(&output.stdout).into_owned())
    } else {
        Err(CommandError::Failed {
            command: rendered,
            stderr: String::from_utf8_lossy(&output.stderr).trim().to_string(),
        })
    }
}

fn render_command_line(
    command_name: &str,
    args: &[&str],
    super_user: bool,
    super_user_program: &str,
    super_user_args: &[&str],
) -> String {
    if super_user {
        std::iter::once(super_user_program)
            .chain(super_user_args.iter().copied())
            .chain(std::iter::once(command_name))
            .chain(args.iter().copied())
            .collect::<Vec<_>>()
            .join(" ")
    } else {
        std::iter::once(command_name)
            .chain(args.iter().copied())
            .collect::<Vec<_>>()
            .join(" ")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, Clone, Copy)]
    struct PrintfCommand;

    impl CommandStrategy for PrintfCommand {
        fn command_name(&self) -> &'static str {
            "printf"
        }
    }

    #[derive(Debug, Clone, Copy)]
    struct CatCommand;

    impl CommandStrategy for CatCommand {
        fn command_name(&self) -> &'static str {
            "cat"
        }
    }

    static PRINTF_COMMAND: PrintfCommand = PrintfCommand;
    static CAT_COMMAND: CatCommand = CatCommand;

    #[test]
    fn execute_returns_stdout() {
        let mut executor = CommandExecutor::new();
        executor.set_strategy(&PRINTF_COMMAND);

        let output = executor
            .execute(&["hello"])
            .expect("command should succeed");

        assert_eq!(output, "hello");
    }

    #[test]
    fn super_user_execute_without_input_runs_through_super_user_program() {
        let mut executor = CommandExecutor::with_super_user_program("env", &[]);
        executor.set_strategy(&PRINTF_COMMAND);

        let output = executor
            .super_user_execute(&["hello"], None)
            .expect("command should succeed");

        assert_eq!(output, "hello");
    }

    #[test]
    fn super_user_execute_accepts_input() {
        let mut executor = CommandExecutor::with_super_user_program("env", &[]);
        executor.set_strategy(&CAT_COMMAND);

        let output = executor
            .super_user_execute(&[], Some("jadeos"))
            .expect("command should succeed");

        assert_eq!(output, "jadeos");
    }
}
