fn main() {
    if std::path::Path::new("icon.ico").exists() {
        winres::WindowsResource::new()
            .set_icon("icon.ico")
            .set("ProductName", "WinBatteryIconChecker")
            .set("FileDescription", "Battery Icon Checker")
            .set("LegalCopyright", "Copyright (c) 2024")
            .compile()
            .unwrap();
    }
}

