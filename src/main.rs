#![windows_subsystem = "windows"]

use std::thread::sleep;
use std::time::Duration;
use windows::{
    core::*,
    Win32::Foundation::*,
    Win32::UI::WindowsAndMessaging::*,
};

fn main() -> Result<()> {
    const MIN_WIDTH: i32 = 50;
    const MIN_HEIGHT: i32 = 50;
    const MAX_WIDHT: i32 = 1380;
    const MAX_HEIGHT: i32 = 1040;
    const SLEEP_DURATION: u64 = 30_000;

    let window = loop {
        unsafe {
            // ウィンドウを探す
            if let Ok(window) = FindWindowW(w!("UnityWndClass"), w!("VRChat")) {
                // 見つかった場合はループを抜ける
                break window;
            }
        }
        // 見つからなかった場合、少し待機して再試行
        sleep(Duration::from_millis(SLEEP_DURATION));
    };

    unsafe {
        // 現在のウィンドウ位置とサイズを取得
        let mut rect = RECT::default();
        GetWindowRect(window, &mut rect)?;

        let current_width = rect.right - rect.left;
        let current_height = rect.bottom - rect.top;

        let (new_width, new_height) = if current_width == MIN_WIDTH && current_height == MIN_HEIGHT {
            // 50x50 の場合、1380x1040 に変更
            (MAX_WIDHT, MAX_HEIGHT)
        } else if current_width == MAX_WIDHT && current_height == MAX_HEIGHT {
            // 1380x1040 の場合、50x50 に変更
            (MIN_WIDTH, MIN_HEIGHT)
        } else {
            // それ以外の場合は変更しない
            (current_width, current_height)
        };

        // ウィンドウサイズと位置を変更 (左上を (0, 0) に設定)
        SetWindowPos(
            window,
            HWND_TOP,
            0,
            0,
            new_width,
            new_height,
            SWP_NOZORDER | SWP_NOACTIVATE,
        )?;
    }
    Ok(())
}
