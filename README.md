<h1 align="center" id="rus">Плагин для отслеживания блокировки сессии |<a href="#eng">eng</a>|</h1>

Данный плагин помогает отслеживать статус блокировки для текущей сессии.  Текущая версия плагина подходит для `Tauri 1.5.4`.

## Установка

- [ 1 ] Для начала работы нужно подключить в проект плагин
- [ 2 ] Для этого в `src-tauri/Cargo.toml` добавить:

```toml
[dependencies]
  tauri-plugin-screen-lock-status = { git = "https://github.com/ren40/tauri-plugin-screen-lock-status" }
```

- [ 3 ] Далее в исполняемом файле `src-tauri/src/main.rc` подключаем:

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

 - [ 4 ] Для прослушивания надо подключить следующий код:

```JS
import { listen } from '@tauri-apps/api/event'

await listen("window_screen_lock_status://change_session_status", (result) => {
    console.log(`[${new Date().toLocaleString()} ${result.payload}]`)
  })
```

- [ 4.1 ]Событие изменения статуса прослушиваются по каналу `window_screen_lock_status://change_session_status`.

## ОС

| Название ОС | Статус |
| ----------  | ------ |
| Windows     |  :white_check_mark:   |
| Linux       |  :white_check_mark:    |
| Unix        |  :negative_squared_cross_mark:      |
| Android     | :negative_squared_cross_mark:  |
| IOS         | :negative_squared_cross_mark: |

## Цели

- [ ] Реализовать и проверить для Unix
- [ ] Реализовать и проверить для Android
- [ ] Реализовать и проверить для IOS
- [ ] Проверить работоспособность для версии Tauri v2 и выше (При условии что он станет стабильным)
  
## Сотрудничество

PR принимаются.

-----

<h1 align="center" id="eng">Plugin for tracking session blocking |<a href="#rus">rus</a>|</h1>

This plugin helps you control the lock status for the current session. The current version of the plugin is suitable for `Tauri 1.5.4`.

## Installation

- [ 1 ] To get started, you need to connect a plugin to the project
- [ 2 ] To do this, add to `src-tauri/Cargo.toml`:

```toml
[dependencies]
  tauri-plugin-screen-lock-status = { git = "https://github.com/ren40/tauri-plugin-screen-lock-status" }
```

- [ 3 ] Next, in the executable file `src-tauri/src/main.rc`, we connect:

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

- [ 4 ] To listen, you need to connect the following code:

```JS
import { listen } from '@tauri-apps/api/event'

await listen("window_screen_lock_status://change_session_status", (result) => {
    console.log(`[${new Date().toLocaleString()} ${result.payload}]`)
  })
```

- [ 4.1 ]The status change event is listened to via the channel `window_screen_lock_status://change_session_status'.

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
  
