use gpui::{actions, KeyBinding};

actions!(
    editor,
    [
        Backspace,
        Delete,
        DeleteToBeginningOfLine,
        DeleteToEndOfLine,
        Enter,
        Up,
        Down,
        Left,
        Right,
        SelectUp,
        SelectDown,
        SelectLeft,
        SelectRight,
        SelectAll,
        Home,
        End,
        SelectToStartOfLine,
        SelectToEndOfLine,
        SelectToStart,
        SelectToEnd,
        ShowCharacterPalette,
        Copy,
        Cut,
        Paste,
        Undo,
        Redo,
        MoveToStartOfLine,
        MoveToEndOfLine,
        MoveToStart,
        MoveToEnd,
        TextChanged,
    ]
);

pub const CONTEXT: &str = "editor";

pub(crate) fn init(cx: &mut gpui::App) {
  cx.bind_keys([
    KeyBinding::new("backspace", Backspace, Some(CONTEXT)),
    KeyBinding::new("delete", Delete, Some(CONTEXT)),
    #[cfg(target_os = "macos")]
    KeyBinding::new("cmd-backspace", DeleteToBeginningOfLine, Some(CONTEXT)),
    #[cfg(target_os = "macos")]
    KeyBinding::new("cmd-delete", DeleteToEndOfLine, Some(CONTEXT)),
    KeyBinding::new("enter", Enter, Some(CONTEXT)),
    KeyBinding::new("up", Up, Some(CONTEXT)),
    KeyBinding::new("down", Down, Some(CONTEXT)),
    KeyBinding::new("left", Left, Some(CONTEXT)),
    KeyBinding::new("right", Right, Some(CONTEXT)),
    KeyBinding::new("shift-left", SelectLeft, Some(CONTEXT)),
    KeyBinding::new("shift-right", SelectRight, Some(CONTEXT)),
    KeyBinding::new("shift-up", SelectUp, Some(CONTEXT)),
    KeyBinding::new("shift-down", SelectDown, Some(CONTEXT)),
    KeyBinding::new("home", Home, Some(CONTEXT)),
    KeyBinding::new("end", End, Some(CONTEXT)),
    KeyBinding::new("shift-home", SelectToStartOfLine, Some(CONTEXT)),
    KeyBinding::new("shift-end", SelectToEndOfLine, Some(CONTEXT)),
    #[cfg(target_os = "macos")]
    KeyBinding::new("ctrl-shift-a", SelectToStartOfLine, Some(CONTEXT)),
    #[cfg(target_os = "macos")]
    KeyBinding::new("ctrl-shift-e", SelectToEndOfLine, Some(CONTEXT)),
    #[cfg(target_os = "macos")]
    KeyBinding::new("shift-cmd-left", SelectToStartOfLine, Some(CONTEXT)),
    #[cfg(target_os = "macos")]
    KeyBinding::new("shift-cmd-right", SelectToEndOfLine, Some(CONTEXT)),
    #[cfg(target_os = "macos")]
    KeyBinding::new("ctrl-cmd-space", ShowCharacterPalette, Some(CONTEXT)),
    #[cfg(target_os = "macos")]
    KeyBinding::new("cmd-a", SelectAll, Some(CONTEXT)),
    #[cfg(not(target_os = "macos"))]
    KeyBinding::new("ctrl-a", SelectAll, Some(CONTEXT)),
    #[cfg(target_os = "macos")]
    KeyBinding::new("cmd-c", Copy, Some(CONTEXT)),
    #[cfg(not(target_os = "macos"))]
    KeyBinding::new("ctrl-c", Copy, Some(CONTEXT)),
    #[cfg(target_os = "macos")]
    KeyBinding::new("cmd-x", Cut, Some(CONTEXT)),
    #[cfg(not(target_os = "macos"))]
    KeyBinding::new("ctrl-x", Cut, Some(CONTEXT)),
    #[cfg(target_os = "macos")]
    KeyBinding::new("cmd-v", Paste, Some(CONTEXT)),
    #[cfg(not(target_os = "macos"))]
    KeyBinding::new("ctrl-v", Paste, Some(CONTEXT)),
    #[cfg(target_os = "macos")]
    KeyBinding::new("ctrl-a", Home, Some(CONTEXT)),
    #[cfg(target_os = "macos")]
    KeyBinding::new("cmd-left", Home, Some(CONTEXT)),
    #[cfg(target_os = "macos")]
    KeyBinding::new("ctrl-e", End, Some(CONTEXT)),
    #[cfg(target_os = "macos")]
    KeyBinding::new("cmd-right", End, Some(CONTEXT)),
    #[cfg(target_os = "macos")]
    KeyBinding::new("cmd-z", Undo, Some(CONTEXT)),
    #[cfg(target_os = "macos")]
    KeyBinding::new("cmd-shift-z", Redo, Some(CONTEXT)),
    #[cfg(target_os = "macos")]
    KeyBinding::new("cmd-up", MoveToStart, Some(CONTEXT)),
    #[cfg(target_os = "macos")]
    KeyBinding::new("cmd-down", MoveToEnd, Some(CONTEXT)),
    #[cfg(target_os = "macos")]
    KeyBinding::new("cmd-shift-up", SelectToStart, Some(CONTEXT)),
    #[cfg(target_os = "macos")]
    KeyBinding::new("cmd-shift-down", SelectToEnd, Some(CONTEXT)),
    #[cfg(not(target_os = "macos"))]
    KeyBinding::new("ctrl-z", Undo, Some(CONTEXT)),
    #[cfg(not(target_os = "macos"))]
    KeyBinding::new("ctrl-y", Redo, Some(CONTEXT))
  ]);
}