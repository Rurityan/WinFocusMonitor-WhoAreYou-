use windows::Win32::Foundation::{HWND, MAX_PATH};
use windows::Win32::UI::Accessibility::{SetWinEventHook, HWINEVENTHOOK};
use windows::Win32::System::Threading::{OpenProcess, PROCESS_QUERY_INFORMATION, PROCESS_VM_READ};
use windows::Win32::System::ProcessStatus::K32GetModuleFileNameExW;
use windows::Win32::UI::WindowsAndMessaging::{
    DispatchMessageW, GetMessageW, GetWindowTextW, GetWindowThreadProcessId,
    EVENT_SYSTEM_FOREGROUND, MSG, WINEVENT_OUTOFCONTEXT,
};

/// 根据 PID 获取进程的可执行文件路径
fn get_process_path(pid: u32) -> Option<String> {
    unsafe {
        // 1. 获取进程句柄
        let handle = OpenProcess(PROCESS_QUERY_INFORMATION | PROCESS_VM_READ, false, pid).ok()?;

        let mut buffer = [0u16; MAX_PATH as usize];
        // 2. 获取路径
        let len = K32GetModuleFileNameExW(handle, None, &mut buffer);

        if len > 0 {
            Some(String::from_utf16_lossy(&buffer[..len as usize]))
        } else {
            None
        }
    }
}

unsafe extern "system" fn win_event_proc(
    _h_win_event_hook: HWINEVENTHOOK,
    event: u32,
    hwnd: HWND,
    _id_object: i32,
    _id_child: i32,
    _id_event_thread: u32,
    _dw_ms_event_time: u32,
) {
    if event == EVENT_SYSTEM_FOREGROUND && !hwnd.is_invalid() {
        // 获取窗口标题
        let mut title_buffer = [0u16; 512];
        let len = GetWindowTextW(hwnd, &mut title_buffer);
        let title = String::from_utf16_lossy(&title_buffer[..len as usize]);

        // 获取 PID
        let mut pid: u32 = 0;
        GetWindowThreadProcessId(hwnd, Some(&mut pid));

        // 获取路径
        let path = get_process_path(pid).unwrap_or_else(|| "Unknown Path (Access Denied: Try running as Administrator)".to_string());

        if !title.is_empty() {
            println!("--- Focus Switched ---");
            println!("Title: {}", title);
            println!("PID : {}", pid);
            println!("Path: {}", path);
            println!("----------------\n");
        }
    }
}

fn main() {
    println!("Focusing...");

    unsafe {
        let _hook = SetWinEventHook(
            EVENT_SYSTEM_FOREGROUND,
            EVENT_SYSTEM_FOREGROUND,
            None,
            Some(win_event_proc),
            0,
            0,
            WINEVENT_OUTOFCONTEXT,
        );

        let mut msg = MSG::default();
        while GetMessageW(&mut msg, None, 0, 0).as_bool() {
            DispatchMessageW(&msg);
        }
    }
}