#[derive(Clone, Debug)]
pub struct KeyModifiers {
    pub shift: bool,
    pub control: bool,
    pub alt: bool,
}

#[derive(Clone, Debug)]
pub enum MouseButton {
    Left,
    Right,
    Middle,
}

#[derive(Clone, Debug)]
pub enum MouseEventKind {
    Down(MouseButton),
    Up(MouseButton),
    Drag(MouseButton),
    Moved,
    ScrollDown,
    ScrollUp,
}

#[derive(Clone, Debug)]
pub struct MouseEvent {
    pub kind: MouseEventKind,
    pub column: u16,
    pub row: u16,
    pub modifiers: KeyModifiers,
}

#[derive(Clone, Debug)]
pub enum KeyCode {
    Backspace,
    Enter,
    Left,
    Right,
    Up,
    Down,
    Home,
    End,
    PageUp,
    PageDown,
    Tab,
    BackTab,
    Delete,
    Insert,
    F(u8),
    Char(char),
    Null,
    Esc,
}

#[derive(Clone, Debug)]
pub struct KeyEvent {
    pub code: KeyCode,
    pub modifiers: KeyModifiers,
}

impl From<crossterm::event::KeyCode> for KeyCode {
    fn from(key_code: crossterm::event::KeyCode) -> Self {
        return match key_code {
            crossterm::event::KeyCode::Backspace => KeyCode::Backspace,
            crossterm::event::KeyCode::Enter => KeyCode::Enter,
            crossterm::event::KeyCode::Left => KeyCode::Left,
            crossterm::event::KeyCode::Right => KeyCode::Right,
            crossterm::event::KeyCode::Up => KeyCode::Up,
            crossterm::event::KeyCode::Down => KeyCode::Down,
            crossterm::event::KeyCode::Home => KeyCode::Home,
            crossterm::event::KeyCode::End => KeyCode::End,
            crossterm::event::KeyCode::PageUp => KeyCode::PageUp,
            crossterm::event::KeyCode::PageDown => KeyCode::PageDown,
            crossterm::event::KeyCode::Tab => KeyCode::Tab,
            crossterm::event::KeyCode::BackTab => KeyCode::BackTab,
            crossterm::event::KeyCode::Delete => KeyCode::Delete,
            crossterm::event::KeyCode::Insert => KeyCode::Insert,
            crossterm::event::KeyCode::F(n) => KeyCode::F(n),
            crossterm::event::KeyCode::Char(c) => KeyCode::Char(c),
            crossterm::event::KeyCode::Null => KeyCode::Null,
            crossterm::event::KeyCode::Esc => KeyCode::Esc,
        };
    }
}

impl From<crossterm::event::KeyEvent> for KeyEvent {
    fn from(key_event: crossterm::event::KeyEvent) -> Self {
        let code = KeyCode::from(key_event.code);
        let modifiers = KeyModifiers::from(key_event.modifiers);

        KeyEvent { code, modifiers }
    }
}

impl From<crossterm::event::KeyModifiers> for KeyModifiers {
    fn from(modifiers: crossterm::event::KeyModifiers) -> Self {
        let shift = modifiers.contains(crossterm::event::KeyModifiers::SHIFT);
        let control = modifiers.contains(crossterm::event::KeyModifiers::CONTROL);
        let alt = modifiers.contains(crossterm::event::KeyModifiers::ALT);

        KeyModifiers {
            shift,
            control,
            alt,
        }
    }
}

impl From<crossterm::event::MouseButton> for MouseButton {
    fn from(mouse_button: crossterm::event::MouseButton) -> Self {
        return match mouse_button {
            crossterm::event::MouseButton::Left => MouseButton::Left,
            crossterm::event::MouseButton::Right => MouseButton::Right,
            crossterm::event::MouseButton::Middle => MouseButton::Middle,
        };
    }
}

impl From<crossterm::event::MouseEventKind> for MouseEventKind {
    fn from(mouse_event_kind: crossterm::event::MouseEventKind) -> Self {
        return match mouse_event_kind {
            crossterm::event::MouseEventKind::Down(button) => {
                MouseEventKind::Down(MouseButton::from(button))
            }
            crossterm::event::MouseEventKind::Up(button) => {
                MouseEventKind::Up(MouseButton::from(button))
            }
            crossterm::event::MouseEventKind::Drag(button) => {
                MouseEventKind::Drag(MouseButton::from(button))
            }
            crossterm::event::MouseEventKind::Moved => MouseEventKind::Moved,
            crossterm::event::MouseEventKind::ScrollDown => MouseEventKind::ScrollDown,
            crossterm::event::MouseEventKind::ScrollUp => MouseEventKind::ScrollUp,
        };
    }
}

impl From<crossterm::event::MouseEvent> for MouseEvent {
    fn from(mouse_event: crossterm::event::MouseEvent) -> Self {
        let kind = MouseEventKind::from(mouse_event.kind);
        let column = mouse_event.column;
        let row = mouse_event.row;
        let modifiers = KeyModifiers::from(mouse_event.modifiers);

        MouseEvent {
            kind,
            column,
            row,
            modifiers,
        }
    }
}
