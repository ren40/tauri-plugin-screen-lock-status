use tauri::{
    plugin::{TauriPlugin},
    Runtime, Window,
};
#[cfg(target_os = "linux")]
mod linux;

#[cfg(target_os = "windows")]
mod windows;

/// Initializes the plugin.
pub fn init<R: Runtime>() -> TauriPlugin<R> {
    #[cfg(target_os = "windows")]
    {
        windows::init()
    }
}
