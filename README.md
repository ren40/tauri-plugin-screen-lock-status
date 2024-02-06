<h1 align="center">Плагин для отслеживания блокировки сессии</h1>
Моя реализация плагина для Tauri

## Использование
```Rust
//main.rc
fn main() {
  tauri::Builder::default().setup(|app| {
              let _ =
                  tauri_plugin_screen_lock_status::WINDOW_TAURI.set(app.get_window("main").unwrap());
              Ok(())
          })
}
```

```JS
 await listen("windows_screen_lock_status://change_session_status", (result) => {
    msgStatus.value.push(`[${new Date().toLocaleString()} ${result.payload}]`)
  })
```

## ОС
| Название ОС | Статус | 
| ----------  | ------ | 
| Windows     |  :white_check_mark:   | 
| Linux       |  :white_check_mark:    | 
| Unix        |  :negative_squared_cross_mark:      |
