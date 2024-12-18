// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
    tauri_app_lib::run()
}

pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]
mod tests {
    use super::*;
    // 引入父模块中的内容

    #[test]
    fn test_add() {
        assert_eq!(add(2, 3), 5); // 测试通过
        assert_eq!(add(-1, 1), 0); // 测试通过
    }
}
