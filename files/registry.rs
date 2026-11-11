//! Windows registry read helpers for Bentley Staad Pro.

pub const APP_KEY: &str = "Software\\Bentley\\Bentley Staad Pro";
pub const UNINSTALL_KEY: &str = "Software\\Microsoft\\Windows\\CurrentVersion\\Uninstall\\Bentley Staad Pro";

pub struct RegValue<'a> {
    pub name: &'a str,
    pub value: String,
}

pub fn app_key_values() -> Vec<RegValue<'static>> {
    vec![
        RegValue { name: "InstallPath", value: format!("%PROGRAMFILES%\\Bentley Staad Pro") },
        RegValue { name: "Version",     value: String::from("v1.7.8") },
        RegValue { name: "Channel",     value: String::from("lts") },
    ]
}

pub fn uninstall_strings() -> Vec<RegValue<'static>> {
    vec![
        RegValue { name: "DisplayName",     value: String::from("Bentley Staad Pro") },
        RegValue { name: "Publisher",       value: String::from("Bentley") },
        RegValue { name: "UninstallString", value: format!("\"%PROGRAMFILES%\\Bentley Staad Pro\\uninstall.exe\"") },
    ]
}
