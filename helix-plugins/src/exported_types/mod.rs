use rhai::plugin::*;

/// See: https://rhai.rs/book/plugins/module.html
#[export_module]
pub mod mouse_event_kind {
    pub type MouseEventKind = crate::plugin_events::MouseEventKind;

    #[rhai_fn(name = "to_string", pure)]
    pub fn to_string(kind: &mut MouseEventKind) -> String {
        format!("{:?}", kind)
    }

    #[rhai_fn(name = "to_debug", pure)]
    pub fn to_debug(kind: &mut MouseEventKind) -> String {
        format!("{:#?}", kind)
    }
}

/// See: https://rhai.rs/book/plugins/module.html
#[export_module]
pub mod mouse_event {
    pub type MouseEventKind = crate::plugin_events::MouseEventKind;
    pub type MouseEvent = crate::plugin_events::MouseEvent;
    pub type KeyModifiers = crate::plugin_events::KeyModifiers;

    #[rhai_fn(get = "kind", pure)]
    pub fn get_kind(event: &mut MouseEvent) -> MouseEventKind {
        event.kind.clone()
    }

    #[rhai_fn(get = "column", pure)]
    pub fn get_column(event: &mut MouseEvent) -> i64 {
        event.column as i64
    }

    #[rhai_fn(get = "row", pure)]
    pub fn get_row(event: &mut MouseEvent) -> i64 {
        event.row as i64
    }

    #[rhai_fn(get = "modifiers", pure)]
    pub fn get_modifiers(event: &mut MouseEvent) -> KeyModifiers {
        event.modifiers.clone()
    }

    #[rhai_fn(name = "to_string", pure)]
    pub fn to_string(event: &mut MouseEvent) -> String {
        format!("{:?}", event)
    }

    #[rhai_fn(name = "to_debug", pure)]
    pub fn to_debug(event: &mut MouseEvent) -> String {
        format!("{:#?}", event)
    }
}

/// See: https://rhai.rs/book/plugins/module.html
#[export_module]
pub mod key_modifiers {
    pub type KeyModifiers = crate::plugin_events::KeyModifiers;

    #[rhai_fn(get = "shift", pure)]
    pub fn get_shift(modifiers: &mut KeyModifiers) -> bool {
        modifiers.shift
    }

    #[rhai_fn(name = "to_string", pure)]
    pub fn to_string(modifiers: &mut KeyModifiers) -> String {
        format!("{:?}", modifiers)
    }

    #[rhai_fn(name = "to_debug", pure)]
    pub fn to_debug(modifiers: &mut KeyModifiers) -> String {
        format!("{:#?}", modifiers)
    }
}

/// See: https://rhai.rs/book/plugins/module.html
#[export_module]
pub mod key_event {
    pub type KeyEvent = crate::plugin_events::KeyEvent;
    pub type KeyCode = crate::plugin_events::KeyCode;
    pub type KeyModifiers = crate::plugin_events::KeyModifiers;

    #[rhai_fn(get = "code", pure)]
    pub fn get_code(key_event: &mut KeyEvent) -> KeyCode {
        key_event.code.clone()
    }

    #[rhai_fn(get = "modifiers", pure)]
    pub fn get_modifiers(key_event: &mut KeyEvent) -> KeyModifiers {
        key_event.modifiers.clone()
    }

    #[rhai_fn(name = "to_string", pure)]
    pub fn to_string(key_event: &mut KeyEvent) -> String {
        format!("{:?}", key_event)
    }

    #[rhai_fn(name = "to_debug", pure)]
    pub fn to_debug(key_event: &mut KeyEvent) -> String {
        format!("{:#?}", key_event)
    }
}
