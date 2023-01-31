use super::{ClipboardProvider, ClipboardType};
use anyhow::Result;
use std::borrow::Cow;

#[cfg(not(target_arch = "wasm32"))]
pub mod command {
    use super::*;
    use anyhow::{bail, Context as _, Result};

    #[cfg(not(any(windows, target_os = "macos")))]
    pub fn is_exit_success(program: &str, args: &[&str]) -> bool {
        std::process::Command::new(program)
            .args(args)
            .output()
            .ok()
            .and_then(|out| out.status.success().then(|| ())) // TODO: use then_some when stabilized
            .is_some()
    }

    #[derive(Debug)]
    pub struct Config {
        pub prg: &'static str,
        pub args: &'static [&'static str],
    }

    impl Config {
        fn execute(&self, input: Option<&str>, pipe_output: bool) -> Result<Option<String>> {
            use std::io::Write;
            use std::process::{Command, Stdio};

            let stdin = input.map(|_| Stdio::piped()).unwrap_or_else(Stdio::null);
            let stdout = pipe_output.then(Stdio::piped).unwrap_or_else(Stdio::null);

            let mut command: Command = Command::new(self.prg);

            let mut command_mut: &mut Command = command
                .args(self.args)
                .stdin(stdin)
                .stdout(stdout)
                .stderr(Stdio::null());

            // Fix for https://github.com/helix-editor/helix/issues/5424
            if cfg!(unix) {
                use std::os::unix::process::CommandExt;

                unsafe {
                    command_mut = command_mut.pre_exec(|| match libc::setsid() {
                        -1 => Err(std::io::Error::last_os_error()),
                        _ => Ok(()),
                    });
                }
            }

            let mut child = command_mut.spawn()?;

            if let Some(input) = input {
                let mut stdin = child.stdin.take().context("stdin is missing")?;
                stdin
                    .write_all(input.as_bytes())
                    .context("couldn't write in stdin")?;
            }

            // TODO: add timer?
            let output = child.wait_with_output()?;

            if !output.status.success() {
                bail!("clipboard provider {} failed", self.prg);
            }

            if pipe_output {
                Ok(Some(String::from_utf8(output.stdout)?))
            } else {
                Ok(None)
            }
        }
    }

    #[derive(Debug)]
    pub struct Provider {
        pub get_cmd: Config,
        pub set_cmd: Config,
        pub get_primary_cmd: Option<Config>,
        pub set_primary_cmd: Option<Config>,
    }

    impl ClipboardProvider for Provider {
        fn name(&self) -> Cow<str> {
            if self.get_cmd.prg != self.set_cmd.prg {
                Cow::Owned(format!("{}+{}", self.get_cmd.prg, self.set_cmd.prg))
            } else {
                Cow::Borrowed(self.get_cmd.prg)
            }
        }

        fn get_contents(&self, clipboard_type: ClipboardType) -> Result<String> {
            match clipboard_type {
                ClipboardType::Clipboard => Ok(self
                    .get_cmd
                    .execute(None, true)?
                    .context("output is missing")?),
                ClipboardType::Selection => {
                    if let Some(cmd) = &self.get_primary_cmd {
                        return cmd.execute(None, true)?.context("output is missing");
                    }

                    Ok(String::new())
                }
            }
        }

        fn set_contents(&mut self, value: String, clipboard_type: ClipboardType) -> Result<()> {
            let cmd = match clipboard_type {
                ClipboardType::Clipboard => &self.set_cmd,
                ClipboardType::Selection => {
                    if let Some(cmd) = &self.set_primary_cmd {
                        cmd
                    } else {
                        return Ok(());
                    }
                }
            };
            cmd.execute(Some(&value), false).map(|_| ())
        }
    }
}
