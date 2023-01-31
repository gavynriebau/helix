#[derive(Debug)]
pub struct FallbackProvider {
    buf: String,
    primary_buf: String,
}

impl FallbackProvider {
    pub fn new() -> Self {
        log::warn!(
            "No native clipboard provider found! Yanking and pasting will be internal to Helix"
        );
        Self {
            buf: String::new(),
            primary_buf: String::new(),
        }
    }
}

impl Default for FallbackProvider {
    fn default() -> Self {
        Self::new()
    }
}

impl ClipboardProvider for FallbackProvider {
    fn name(&self) -> Cow<str> {
        Cow::Borrowed("none")
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
        // Set our internal variables to use in get_content regardless of using OSC 52
        match clipboard_type {
            ClipboardType::Clipboard => self.buf = content,
            ClipboardType::Selection => self.primary_buf = content,
        }
        Ok(())
    }
}