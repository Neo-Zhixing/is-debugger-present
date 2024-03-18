#[track_caller]
pub fn is_debugger_present() -> bool {
    #[cfg(target_os = "windows")]
    {
        use windows_sys::Win32::Foundation::TRUE;
        unsafe {
            return windows_sys::Win32::System::Diagnostics::Debug::IsDebuggerPresent() == TRUE;
        }
    }

    #[cfg(target_os = "linux")]
    {
        use std::io::BufRead;
        let Ok(proc_self_status) = std::fs::File::open("/proc/self/status") else {
            return false;
        };
        let mut buf_reader = std::io::BufReader::new(proc_self_status);
        let mut line = String::new();
        while let Ok(bytes_read) = buf_reader.read_line(&mut line) {
            if bytes_read == 0 {
                break;
            }
            if let Some(rest) = line.strip_prefix("TracerPid:") {
                let rest = rest.trim();
                return rest != "0";
            }

            line.clear();
        }
        return false;
    }

    #[allow(unreachable_code)]
    false
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

    #[cfg(not(target_os = "windows"))]
    {
        unsafe {
            libc::raise(libc::SIGTRAP);
        }
    }
}

#[cfg(test)]
mod test {
    use crate::is_debugger_present;

    #[test]
    fn my_test() {
        let present = is_debugger_present();
        println!("Present: {:?}", present);
    }
}
