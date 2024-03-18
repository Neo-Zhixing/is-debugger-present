#[track_caller]
pub fn is_debugger_present() -> bool {
    #![allow(unreachable_code)]
    #[cfg(target_os = "windows")]
    {
        use windows_sys::Win32::Foundation::TRUE;
        unsafe {
            return windows_sys::Win32::System::Diagnostics::Debug::IsDebuggerPresent() == TRUE;
        }
    }
    panic!("Not implemented for this platform");
}

pub fn breakpoint() {
    if !is_debugger_present() {
        return;
    }
    #[cfg(target_os = "windows")]
    {
        unsafe {
            windows_sys::Win32::System::Diagnostics::Debug::DebugBreak();
        }
    }
}
