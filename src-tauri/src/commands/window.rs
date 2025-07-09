use tauri::{AppHandle, Manager};
use std::path::Path;

#[tauri::command]
pub async fn create_new_window(_app_handle: AppHandle, project_path: Option<String>) -> Result<String, String> {
    // For now, let's use a simple approach - spawn a new instance
    // This avoids the unstable API issues while still providing multiple windows
    use std::process::Command;
    
    // Get the current executable path
    let current_exe = std::env::current_exe()
        .map_err(|e| format!("Failed to get current executable: {}", e))?;
    
    // Set up environment variable for the window title if project path is provided
    let mut cmd = Command::new(current_exe);
    
    if let Some(path) = project_path {
        let project_name = Path::new(&path)
            .file_name()
            .and_then(|name| name.to_str())
            .unwrap_or("Claudia");
        cmd.env("CLAUDIA_PROJECT_PATH", &path);
        cmd.env("CLAUDIA_PROJECT_NAME", project_name);
    }
    
    // Launch a new instance
    cmd.spawn()
        .map_err(|e| format!("Failed to spawn new instance: {}", e))?;
    
    Ok("new-instance".to_string())
}

#[tauri::command]
pub async fn update_window_title(app_handle: AppHandle, project_path: Option<String>) -> Result<(), String> {
    // Get the main window
    let window = app_handle
        .get_webview_window("main")
        .ok_or_else(|| "Main window not found".to_string())?;
    
    let title = if let Some(path) = project_path {
        let project_name = Path::new(&path)
            .file_name()
            .and_then(|name| name.to_str())
            .unwrap_or("Claudia");
        format!("{} - Claudia", project_name)
    } else {
        "Claudia".to_string()
    };
    
    window.set_title(&title)
        .map_err(|e| format!("Failed to update window title: {}", e))?;
    
    Ok(())
}

#[tauri::command]
pub async fn list_windows(app_handle: AppHandle) -> Result<Vec<String>, String> {
    let windows: Vec<String> = app_handle
        .webview_windows()
        .keys()
        .cloned()
        .collect();
    
    Ok(windows)
}

#[tauri::command]
pub async fn close_window(app_handle: AppHandle) -> Result<(), String> {
    // Get the main window and close it
    let window = app_handle
        .get_webview_window("main")
        .ok_or_else(|| "Main window not found".to_string())?;
        
    window.close()
        .map_err(|e| format!("Failed to close window: {}", e))?;
    
    Ok(())
}