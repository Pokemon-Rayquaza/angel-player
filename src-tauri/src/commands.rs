use serde::Serialize;
use std::fs;
use std::path::PathBuf;
use std::process::Command;

#[derive(Serialize)]
pub struct Track {
    name: String,
    path: String,
}

#[tauri::command]
pub fn pick_folder() -> Option<String> {
    rfd::FileDialog::new()
        .pick_folder()
        .map(|p| p.to_string_lossy().to_string())
}

#[tauri::command]
pub fn pick_program() -> Option<String> {
    rfd::FileDialog::new()
        .add_filter("Executable", &["exe", "bat", "cmd"])
        .pick_file()
        .map(|p| p.to_string_lossy().to_string())
}

/// 扫描文件夹里的 FLAC + MP3
#[tauri::command]
pub fn scan_flac_folder(folder: String) -> Result<Vec<Track>, String> {
    let dir = PathBuf::from(&folder);
    if !dir.exists() {
        return Err("文件夹不存在".into());
    }

    let mut tracks = Vec::new();
    let entries = fs::read_dir(&dir).map_err(|e| e.to_string())?;

    for entry in entries {
        let entry = match entry {
            Ok(e) => e,
            Err(_) => continue,
        };
        let path = entry.path();

        let ext_ok = path.extension()
            .map_or(false, |e| {
                let e = e.to_string_lossy().to_lowercase();
                e == "flac" || e == "mp3"
            });

        if ext_ok {
            let name = path
                .file_stem()
                .map(|s| s.to_string_lossy().to_string())
                .unwrap_or_default();
            let path_str = path.to_string_lossy().to_string();

            tracks.push(Track {
                name,
                path: path_str,
            });
        }
    }

    tracks.sort_by(|a, b| a.name.cmp(&b.name));
    Ok(tracks)
}

/// 读取默认音乐目录
/// 默认使用用户音乐目录下的 Music 文件夹，可通过环境变量 ANGEL_PLAYER_MUSIC_DIR 覆盖
#[tauri::command]
pub fn scan_default_music() -> Result<Vec<Track>, String> {
    let default_dir = std::env::var("ANGEL_PLAYER_MUSIC_DIR")
        .unwrap_or_else(|_| {
            // 尝试用用户音乐目录
            dirs_next()
                .map(|p| p.join("Music").to_string_lossy().to_string())
                .unwrap_or_else(|| ".".to_string())
        });
    scan_flac_folder(default_dir)
}

/// 获取用户音乐目录
fn dirs_next() -> Option<PathBuf> {
    #[cfg(target_os = "windows")]
    {
        std::env::var("USERPROFILE")
            .ok()
            .map(PathBuf::from)
    }
    #[cfg(target_os = "macos")]
    {
        std::env::var("HOME")
            .ok()
            .map(PathBuf::from)
    }
    #[cfg(target_os = "linux")]
    {
        std::env::var("HOME")
            .ok()
            .map(PathBuf::from)
    }
}

#[tauri::command]
pub fn launch_program(path: String) -> Result<(), String> {
    Command::new(&path)
        .spawn()
        .map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
pub fn open_in_explorer(path: String) -> Result<(), String> {
    #[cfg(target_os = "windows")]
    {
        Command::new("explorer")
            .arg(&path)
            .spawn()
            .map_err(|e| e.to_string())?;
    }
    #[cfg(target_os = "macos")]
    {
        Command::new("open")
            .arg(&path)
            .spawn()
            .map_err(|e| e.to_string())?;
    }
    #[cfg(target_os = "linux")]
    {
        Command::new("xdg-open")
            .arg(&path)
            .spawn()
            .map_err(|e| e.to_string())?;
    }
    Ok(())
}