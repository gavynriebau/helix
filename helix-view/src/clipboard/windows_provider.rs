use super::{ClipboardProvider, ClipboardType};
use anyhow::Result;
use std::borrow::Cow;

#[derive(Default, Debug)]
pub struct WindowsProvider;

impl ClipboardProvider for WindowsProvider {
    fn name(&self) -> Cow<str> {
        log::info!("Using clipboard-win to interact with the system clipboard");
        Cow::Borrowed("clipboard-win")
    }

    fn get_contents(&self, clipboard_type: ClipboardType) -> Result<String> {
        match clipboard_type {
            ClipboardType::Clipboard => {
                let contents = clipboard_win::get_clipboard(clipboard_win::formats::Unicode)?;
                Ok(contents)
            }
            ClipboardType::Selection => Ok(String::new()),
        }
    }

    fn set_contents(&mut self, contents: String, clipboard_type: ClipboardType) -> Result<()> {
        match clipboard_type {
            ClipboardType::Clipboard => {
                clipboard_win::set_clipboard(clipboard_win::formats::Unicode, contents)?;
            }
            ClipboardType::Selection => {}
        };
        Ok(())
    }
}