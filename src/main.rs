#![windows_subsystem = "windows"]

use std::thread::sleep;
use std::time::Duration;
use std::fs;
use windows::{
    core::*,
    Win32::UI::WindowsAndMessaging::*,
};

// ウィンドウのサイズを保持する構造体
struct WindowSize {
    min: (i32, i32),
    max: (i32, i32),
}

// ウィンドウの状態を保持する構造体
struct WindowState {
    state_file: &'static str,
    min_mode: &'static str,
    max_mode: &'static str,
}

// 定数定義
const SLEEP_DURATION: Duration = Duration::from_secs(30);

// 構造体のインスタンスを生成
const WINDOW_SIZE: WindowSize = WindowSize {
    min: (50, 50),
    max: (1380, 1040),
};

const WINDOW_STATE: WindowState = WindowState {
    state_file: "mvrc.dat",
    min_mode: "1",
    max_mode: "0",
};

fn main() -> Result<()> {
    // VRChatのウィンドウを探す
    let window = loop {
        unsafe {
            if let Ok(window) = FindWindowW(w!("UnityWndClass"), w!("VRChat")) {
                break window;
            }
        }
        sleep(SLEEP_DURATION);
    };

    // 現在のモードに基づいてウィンドウサイズを設定
    let is_min_mode = fs::read_to_string(WINDOW_STATE.state_file).unwrap_or_default() == WINDOW_STATE.min_mode;
    let (width, height) = if is_min_mode { WINDOW_SIZE.min } else { WINDOW_SIZE.max };

    // ウィンドウの位置とサイズを設定
    unsafe {
        SetWindowPos(
            window,
            HWND_TOP,
            0,
            0,
            width,
            height,
            SWP_NOZORDER | SWP_NOACTIVATE,
        )?;
    }

    // 新しい状態を書き込む
    let state_to_write = if is_min_mode { WINDOW_STATE.max_mode } else { WINDOW_STATE.min_mode };
    fs::write(WINDOW_STATE.state_file, state_to_write)?;

    Ok(())
}
