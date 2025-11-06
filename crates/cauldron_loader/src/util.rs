#[cfg(windows)]
pub(crate) fn message_box(title: &str, text: &str, icon: u32) {
    use windows::Win32::UI::WindowsAndMessaging::{MESSAGEBOX_STYLE, MessageBoxW};
    use windows::core::{HSTRING, PCWSTR};
    unsafe {
        MessageBoxW(
            None,
            PCWSTR::from_raw(HSTRING::from(text).as_ptr()),
            PCWSTR::from_raw(HSTRING::from(title).as_ptr()),
            MESSAGEBOX_STYLE(icon),
        );
    }
}

#[cfg(not(windows))]
pub(crate) fn message_box(title: &str, text: &str, icon: u32) {
    unimplemented!()
}

#[cfg(windows)]
pub(crate) fn alloc_console(title: &str) {
    use windows::Win32::System::Console::SetConsoleTitleW;
    use windows::core::{HSTRING, PCWSTR};
    use windows_sys::Win32::System::Console::{ATTACH_PARENT_PROCESS, AllocConsole, AttachConsole};
    unsafe {
        AllocConsole();
        AttachConsole(ATTACH_PARENT_PROCESS);
        SetConsoleTitleW(PCWSTR::from_raw(HSTRING::from(title).as_ptr())).unwrap();
    }
}

#[cfg(not(windows))]
pub(crate) fn alloc_console(title: &str) {
    unimplemented!()
}
