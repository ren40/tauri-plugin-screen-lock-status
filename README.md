<h1 align="center" id="rus">Плагин для отслеживания блокировки сессии |<a href="#eng">eng</a>|</h1>

Данный плагин помогает отслеживать статус блокировки для текущей сессии. Реализован на таких технологиях как `WinAPI` и `DBUS`. Текущая версия плагина подходит для `Tauri 1.5.4`.

## Использование

Для работы нужно сначала подключить в проект плагин
Для этого нужно добавить строки в `src-tauri/Cargo.toml`:

```toml
[dependencies]
  tauri-plugin-screen-lock-status = { git = "https://github.com/ren40/tauri-plugin-screen-lock-status" }
```

Далее подключаем в исполняемом файле `src-tauri/src/main.rc`:

```Rust
//main.rc
fn main() {
  tauri::Builder::default()
  .plugin(tauri_plugin_screen_lock_status::init())
  .setup(|app| {
              let _ =
                  tauri_plugin_screen_lock_status::WINDOW_TAURI.set(app.get_window("main").unwrap());
              Ok(())
          })
}
```

Событие изменения статуса прослушиваются по каналу `window_screen_lock_status://change_session_status`. Для прослушивания надо подключить следующий код:

```JS
import { listen } from '@tauri-apps/api/event'

await listen("window_screen_lock_status://change_session_status", (result) => {
    console.log(`[${new Date().toLocaleString()} ${result.payload}]`)
  })
```

## ОС

| Название ОС | Статус |
| ----------  | ------ |
| Windows     |  :white_check_mark:   |
| Linux       |  :white_check_mark:    |
| Unix        |  :negative_squared_cross_mark:      |
| Android     | :negative_squared_cross_mark:  |
| IOS         | :negative_squared_cross_mark: |

## Цели

- [ ] Реализовать и проверить для Unix системе
- [ ] Реализовать и проверить для Android
- [ ] Реализовать и проверить для IOS
- [ ] Проверить работоспособность для версии Tauri v2 и высшее (При условии что он станет стабильным)
  
## Вклад

PR принимаются.

-----

<h1 align="center" id="eng">Plugin for tracking session blocking |<a href="#rus">rus</a>|</h1>

This plugin helps you control the lock status for the current session. Implemented on technologies such as WinAPI and DBUS. The current version of the plugin is suitable for `Tauri 1.5.4`.

## Usage

To work, you must first connect the plugin to the project, this can be done by adding a line to `src-tauri/Cargo.toml`:

```toml
[dependencies]
  tauri-plugin-screen-lock-status = { git = "https://github.com/ren40/tauri-plugin-screen-lock-status" }
```

Next we connect it in the executable file `src-tauri/src/main.rc`:

```Rust
//main.rc
fn main() {
  tauri::Builder::default()
  .plugin(tauri_plugin_screen_lock_status::init())
  .setup(|app| {
              let _ =
                  tauri_plugin_screen_lock_status::WINDOW_TAURI.set(app.get_window("main").unwrap());
              Ok(())
          })
}
```

Status change events are listened to on the `window_screen_lock_status://change_session_status` channel. To listen, you need to connect the following code:

```JS
import { listen } from '@tauri-apps/api/event'

await listen("window_screen_lock_status://change_session_status", (result) => {
    console.log(`[${new Date().toLocaleString()} ${result.payload}]`)
  })
```

## OS

| OS name | Status |
| ----------  | ------ |
| Windows     |  :white_check_mark:   |
| Linux       |  :white_check_mark:    |
| Unix        |  :negative_squared_cross_mark:      |
| Android     | :negative_squared_cross_mark:  |
| IOS         | :negative_squared_cross_mark: |

## TODO

- [ ] Implement and test for Unix system
- [ ] Implement and test for Android
- [ ] Implement and test for iOS
- [ ] Check performance for version Tauri v2 and higher (Assuming it becomes stable)

## Contributing

PRs accepted.
  