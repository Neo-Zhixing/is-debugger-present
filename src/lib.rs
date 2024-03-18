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

    #[cfg(target_os = "macos")]
    unsafe {
        let mut mib = [
            libc::CTL_KERN,
            libc::KERN_PROC,
            libc::KERN_PROC_PID,
            libc::getpid(),
        ];
        #[allow(non_camel_case_types)]
        #[repr(C)]
        struct kinfo_proc {
            padding: [usize; 4],
            p_flag: libc::c_int,
            more_padding: i32,
            rest: [u8; 608],
        }
        let mut info: kinfo_proc = std::mem::zeroed();
        let mut size = std::mem::size_of::<kinfo_proc>();
        assert_eq!(size, 648);
        libc::sysctl(
            mib.as_mut_ptr(),
            4,
            &mut info as *mut kinfo_proc as *mut _,
            &mut size,
            std::ptr::null_mut(),
            0,
        );
        const P_TRACED: libc::c_int = 0x00000800;
        return (info.p_flag & P_TRACED) != 0;
    }

    #[allow(unreachable_code)]
    false
}

pub fn breakpoint() {
    #![allow(unreachable_code)]
    if !is_debugger_present() {
        return;
    }
    #[cfg(target_os = "windows")]
    {
        unsafe {
            windows_sys::Win32::System::Diagnostics::Debug::DebugBreak();
        }
        return;
    }
    #[cfg(target_arch = "x86_64")]
    {
        unsafe {
            std::arch::asm!("int3");
        }
        return;
    }

    #[cfg(not(target_os = "windows"))]
    {
        unsafe {
            libc::raise(libc::SIGTRAP);
        }
    }
}
