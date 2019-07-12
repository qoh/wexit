#[cfg(windows)]
pub fn has_own_console() -> bool {
    use winapi::{
        shared::minwindef::DWORD,
        um::{
            processthreadsapi::GetCurrentProcessId, wincon::GetConsoleWindow,
            winuser::GetWindowThreadProcessId,
        },
    };
    let window = unsafe { GetConsoleWindow() };
    // TODO: If `GetWindowThreadProcessId` sets `window_pid = 0` when
    // `hwnd.is_null()` then this check can be removed entirely.
    if window.is_null() {
        return false;
    }
    let mut window_pid: DWORD = 0;
    unsafe { GetWindowThreadProcessId(window, &mut window_pid) };
    window_pid == unsafe { GetCurrentProcessId() }
}

#[cfg(not(windows))]
pub fn has_own_console() -> bool {
    false
}

pub fn prompt_enter_to_exit(code: i32) -> ! {
    prompt_enter_to_exit_custom(code, "Press Enter to exit.")
}

pub fn prompt_enter_to_exit_custom(code: i32, msg: impl AsRef<str>) -> ! {
    use std::io::{self, Write};
    if has_own_console() {
        println!("{}", msg.as_ref());
        io::stdout().flush().unwrap_or(());
        io::stdin().read_line(&mut String::new()).unwrap();
    }
    std::process::exit(code)
}
