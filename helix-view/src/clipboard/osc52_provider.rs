use std::borrow::Cow;
use super::{ClipboardProvider, ClipboardType};
use anyhow::Result;

mod osc52 {
    use {super::ClipboardType, crate::base64, crossterm};

    #[derive(Debug)]
    pub struct SetClipboardCommand {
        encoded_content: String,
        clipboard_type: ClipboardType,
    }

    impl SetClipboardCommand {
        pub fn new(content: &str, clipboard_type: ClipboardType) -> Self {
            Self {
                encoded_content: base64::encode(content.as_bytes()),
                clipboard_type,
            }
        }
    }

    impl crossterm::Command for SetClipboardCommand {
        fn write_ansi(&self, f: &mut impl std::fmt::Write) -> std::fmt::Result {
            let kind = match &self.clipboard_type {
                ClipboardType::Clipboard => "c",
                ClipboardType::Selection => "p",
            };
            // Send an OSC 52 set command: https://terminalguide.namepad.de/seq/osc-52/
            write!(f, "\x1b]52;{};{}\x1b\\", kind, &self.encoded_content)
        }
    }
}

#[derive(Debug)]
pub struct Osc52Provider {
    buf: String,
    primary_buf: String,
}

impl Osc52Provider {
    pub fn new() -> Self {
        log::debug!(
            "No native clipboard provider found. Yanking by OSC 52 and pasting will be internal to Helix"
        );
        
        Self {
            buf: String::new(),
            primary_buf: String::new(),
        }
    }
}

impl Default for Osc52Provider {
    fn default() -> Self {
        Self::new()
    }
}

impl ClipboardProvider for Osc52Provider {
    fn name(&self) -> Cow<str> {
        Cow::Borrowed("termcode")
    }

    fn get_contents(&self, clipboard_type: ClipboardType) -> Result<String> {
        // This is the same noop if term is enabled or not.
        // We don't use the get side of OSC 52 as it isn't often enabled, it's a security hole,
        // and it would require this to be async to listen for the response
        let value = match clipboard_type {
            ClipboardType::Clipboard => self.buf.clone(),
            ClipboardType::Selection => self.primary_buf.clone(),
        };

        Ok(value)
    }

    fn set_contents(&mut self, content: String, clipboard_type: ClipboardType) -> Result<()> {
        crossterm::execute!(
            std::io::stdout(),
            osc52::SetClipboardCommand::new(&content, clipboard_type)
        )?;
        // Set our internal variables to use in get_content regardless of using OSC 52
        match clipboard_type {
            ClipboardType::Clipboard => self.buf = content,
            ClipboardType::Selection => self.primary_buf = content,
        }
        Ok(())
    }
}

