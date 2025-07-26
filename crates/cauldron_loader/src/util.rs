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
