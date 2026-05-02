use log::{error, info};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::OnceLock;
use tauri::{AppHandle, Emitter};
// removed LRESULT, LPARAM, WPARAM
use windows_sys::Win32::UI::Input::KeyboardAndMouse::{VK_CONTROL, VK_RCONTROL};
use windows_sys::Win32::UI::WindowsAndMessaging::{
    CallNextHookEx, DispatchMessageW, GetMessageW, SetWindowsHookExW, TranslateMessage,
    MSG, WM_KEYDOWN, WM_KEYUP, WM_SYSKEYDOWN, WM_SYSKEYUP,
    WH_KEYBOARD_LL, KBDLLHOOKSTRUCT
};

static APP_HANDLE: OnceLock<AppHandle> = OnceLock::new();
static RCTRL_PRESSED: AtomicBool = AtomicBool::new(false);

unsafe extern "system" fn hook_callback(code: i32, wparam: usize, lparam: isize) -> isize {
    if code >= 0 {
        let kb_struct = *(lparam as *const KBDLLHOOKSTRUCT);
        // LLKHF_EXTENDED is 1. Some keyboards send VK_CONTROL with the extended flag for Right Control.
        let is_rctrl = kb_struct.vkCode == VK_RCONTROL as u32 || 
            (kb_struct.vkCode == VK_CONTROL as u32 && (kb_struct.flags & 1) != 0);

        if is_rctrl {
            let msg = wparam as u32;
            let is_down = msg == WM_KEYDOWN || msg == WM_SYSKEYDOWN;
            let is_up = msg == WM_KEYUP || msg == WM_SYSKEYUP;

            if is_down {
                if !RCTRL_PRESSED.swap(true, Ordering::Relaxed) {
                    if let Some(app) = APP_HANDLE.get() {
                        let _ = app.emit("native-shortcut-pressed", "RightControl");
                    }
                }
            } else if is_up {
                if RCTRL_PRESSED.swap(false, Ordering::Relaxed) {
                    if let Some(app) = APP_HANDLE.get() {
                        let _ = app.emit("native-shortcut-released", "RightControl");
                    }
                }
            }
        }
    }
    CallNextHookEx(std::ptr::null_mut(), code, wparam, lparam)
}

pub fn init(app: AppHandle) {
    if APP_HANDLE.set(app).is_err() {
        error!("Keyboard hook already initialized");
        return;
    }

    std::thread::spawn(|| unsafe {
        // hMod = 0 usually works for WH_KEYBOARD_LL when the callback is in the same EXE.
        let hook = SetWindowsHookExW(WH_KEYBOARD_LL, Some(hook_callback), std::ptr::null_mut(), 0);
        if hook == std::ptr::null_mut() {
            error!("Failed to install keyboard hook");
            return;
        }
        info!("Keyboard hook installed successfully");

        let mut msg: MSG = std::mem::zeroed();
        // The message loop is required to keep the hook alive and to process hook messages.
        while GetMessageW(&mut msg, std::ptr::null_mut(), 0, 0) > 0 {
            TranslateMessage(&msg);
            DispatchMessageW(&msg);
        }
    });
}
