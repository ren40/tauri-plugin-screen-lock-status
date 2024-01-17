use windows::{
    core::*,
    Win32::Foundation::*,
    Win32::System::{
        LibraryLoader::*,
        RemoteDesktop::{
            WTSRegisterSessionNotification, WTSUnRegisterSessionNotification,
            NOTIFY_FOR_ALL_SESSIONS,
        },
    },
    Win32::UI::Input::{KeyboardAndMouse::GetActiveWindow, *},
    Win32::UI::WindowsAndMessaging::*,
};

use std::sync::OnceLock;
use std::thread;
use std::time::Duration;
use tauri::{
    plugin::{Builder, TauriPlugin},
    Runtime, Window, Manager
  };

fn register_session_notification(hwnd: HWND) {
    unsafe {
        let _ = WTSRegisterSessionNotification(hwnd, NOTIFY_FOR_ALL_SESSIONS);
    }
}

fn unregister_session_notification(hwnd: HWND) {
    unsafe {
        let _ = WTSUnRegisterSessionNotification(hwnd);
    }
}

pub static WINDOW_TAURI: OnceLock<Window> = OnceLock::new();

pub fn init<R: Runtime>() -> TauriPlugin<R> {
    #[cfg(target_os = "windows")]
    {
        println!("Start new thread...");
        thread::spawn(|| unsafe {
            let instance = GetModuleHandleA(None).unwrap();
            debug_assert!(instance.0 != 0);

            let window_class = s!("window");

            let wc = WNDCLASSA {
                hCursor: LoadCursorW(None, IDC_ARROW).unwrap(),
                hInstance: instance.into(),
                lpszClassName: window_class,

                style: CS_HREDRAW | CS_VREDRAW,
                lpfnWndProc: Some(wndproc),
                ..Default::default()
            };

            let atom = RegisterClassA(&wc);
            debug_assert!(atom != 0);

            CreateWindowExA(
                WINDOW_EX_STYLE::default(),
                window_class,
                s!("Window"),
                WS_OVERLAPPEDWINDOW | WS_VISIBLE,
                CW_USEDEFAULT,
                CW_USEDEFAULT,
                CW_USEDEFAULT,
                CW_USEDEFAULT,
                None,
                None,
                instance,
                None,
            );

            let hwnd = GetActiveWindow();

            ShowWindow(*&hwnd, SW_HIDE);

            let mut message = MSG::default();
            register_session_notification(hwnd);
            while GetMessageA(&mut message, HWND(0), 0, 0).into() {
                if message.message == WM_WTSSESSION_CHANGE {
                    TranslateMessage(&message);
                    DispatchMessageW(&message);

                    match message.wParam.0 as u32 {
                        WTS_SESSION_LOCK => {
                            let _ = WINDOW_TAURI
                                .get()
                                .expect("")
                                .emit_all("windows_screen_lock_status://change_session_status", "lock");
                            println!("Locked");
                        }
                        WTS_SESSION_UNLOCK => {
                            let _ = WINDOW_TAURI
                                .get()
                                .expect("")
                                .emit_all("windows_screen_lock_status://change_session_status", "unlock");
                            println!("Unlocked");
                        }
                        _ => {}
                    }
                }
            }
            thread::sleep(Duration::from_millis(100));
        });
    }
    Builder::new("windows_screen_lock_status")
        .build()
}

extern "system" fn wndproc(window: HWND, message: u32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
    unsafe {
        match message as u32 {
            _ => DefWindowProcA(window, message, wparam, lparam),
        }
    }
}
