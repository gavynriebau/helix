// Implementation reference: https://github.com/neovim/neovim/blob/f2906a4669a2eef6d7bf86a29648793d63c98949/runtime/autoload/provider/clipboard.vim#L68-L152

use anyhow::Result;
use std::borrow::Cow;

#[derive(Clone, Copy, Debug)]
pub enum ClipboardType {
    Clipboard,
    Selection,
}

pub trait ClipboardProvider: std::fmt::Debug {
    fn name(&self) -> Cow<str>;
    fn get_contents(&self, clipboard_type: ClipboardType) -> Result<String>;
    fn set_contents(&mut self, contents: String, clipboard_type: ClipboardType) -> Result<()>;
}

#[cfg(not(windows))]
macro_rules! command_provider {
    (paste => $get_prg:literal $( , $get_arg:literal )* ; copy => $set_prg:literal $( , $set_arg:literal )* ; ) => {{
        log::debug!(
            "Using {} to interact with the system clipboard",
            if $set_prg != $get_prg { format!("{}+{}", $set_prg, $get_prg)} else { $set_prg.to_string() }
        );
        Box::new(provider::command::Provider {
            get_cmd: provider::command::Config {
                prg: $get_prg,
                args: &[ $( $get_arg ),* ],
            },
            set_cmd: provider::command::Config {
                prg: $set_prg,
                args: &[ $( $set_arg ),* ],
            },
            get_primary_cmd: None,
            set_primary_cmd: None,
        })
    }};

    (paste => $get_prg:literal $( , $get_arg:literal )* ;
     copy => $set_prg:literal $( , $set_arg:literal )* ;
     primary_paste => $pr_get_prg:literal $( , $pr_get_arg:literal )* ;
     primary_copy => $pr_set_prg:literal $( , $pr_set_arg:literal )* ;
    ) => {{
        log::info!(
            "Using {} to interact with the system and selection (primary) clipboard",
            if $set_prg != $get_prg { format!("{}+{}", $set_prg, $get_prg)} else { $set_prg.to_string() }
        );
        Box::new(provider::command::Provider {
            get_cmd: provider::command::Config {
                prg: $get_prg,
                args: &[ $( $get_arg ),* ],
            },
            set_cmd: provider::command::Config {
                prg: $set_prg,
                args: &[ $( $set_arg ),* ],
            },
            get_primary_cmd: Some(provider::command::Config {
                prg: $pr_get_prg,
                args: &[ $( $pr_get_arg ),* ],
            }),
            set_primary_cmd: Some(provider::command::Config {
                prg: $pr_set_prg,
                args: &[ $( $pr_set_arg ),* ],
            }),
        })
    }};
}

#[cfg(windows)]
pub fn get_clipboard_provider() -> Box<dyn ClipboardProvider> {
    Box::new(provider::WindowsProvider::default())
}

#[cfg(target_os = "macos")]
pub fn get_clipboard_provider() -> Box<dyn ClipboardProvider> {
    use crate::env::binary_exists;

    if binary_exists("pbcopy") && binary_exists("pbpaste") {
        command_provider! {
            paste => "pbpaste";
            copy => "pbcopy";
        }
    } else {
        Box::new(provider::FallbackProvider::new())
    }
}

#[cfg(target_os = "wasm32")]
pub fn get_clipboard_provider() -> Box<dyn ClipboardProvider> {
    // TODO:
    Box::new(provider::FallbackProvider::new())
}

#[cfg(not(any(windows, target_os = "wasm32", target_os = "macos")))]
pub fn get_clipboard_provider() -> Box<dyn ClipboardProvider> {
    use crate::env::{binary_exists, env_var_is_set};
    use provider::command::is_exit_success;
    // TODO: support for user-defined provider, probably when we have plugin support by setting a
    // variable?

    if env_var_is_set("WAYLAND_DISPLAY") && binary_exists("wl-copy") && binary_exists("wl-paste") {
        command_provider! {
            paste => "wl-paste", "--no-newline";
            copy => "wl-copy", "--type", "text/plain";
            primary_paste => "wl-paste", "-p", "--no-newline";
            primary_copy => "wl-copy", "-p", "--type", "text/plain";
        }
    } else if env_var_is_set("DISPLAY") && binary_exists("xclip") {
        command_provider! {
            paste => "xclip", "-o", "-selection", "clipboard";
            copy => "xclip", "-i", "-selection", "clipboard";
            primary_paste => "xclip", "-o";
            primary_copy => "xclip", "-i";
        }
    } else if env_var_is_set("DISPLAY")
        && binary_exists("xsel")
        && is_exit_success("xsel", &["-o", "-b"])
    {
        // FIXME: check performance of is_exit_success
        command_provider! {
            paste => "xsel", "-o", "-b";
            copy => "xsel", "-i", "-b";
            primary_paste => "xsel", "-o";
            primary_copy => "xsel", "-i";
        }
    } else if binary_exists("win32yank.exe") {
        command_provider! {
            paste => "win32yank.exe", "-o", "--lf";
            copy => "win32yank.exe", "-i", "--crlf";
        }
    } else if binary_exists("termux-clipboard-set") && binary_exists("termux-clipboard-get") {
        command_provider! {
            paste => "termux-clipboard-get";
            copy => "termux-clipboard-set";
        }
    } else if env_var_is_set("TMUX") && binary_exists("tmux") {
        command_provider! {
            paste => "tmux", "save-buffer", "-";
            copy => "tmux", "load-buffer", "-w", "-";
        }
    } else {
        Box::new(provider::FallbackProvider::new())
    }
}

#[cfg_attr(target_os = "windows", path = "windows_provider.rs")]
#[cfg_attr(target_os = "wasm32", path = "fallback_provider.rs")]
#[cfg_attr(all(not(target_os = "windows"), feature = "term"), path = "osc52_provider.rs")]
#[cfg_attr(all(not(target_os = "windows"), not(feature = "term")), path = "other_provider.rs")]
pub mod provider;