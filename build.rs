fn main() {
    if std::path::Path::new("icon.ico").exists() {
        winres::WindowsResource::new()
            .set_icon("icon.ico")
            .set("ProductName", "GPDBatteryIconResolver")
            .set("FileDescription", "Battery icon resolver for GPD devices ")
            .set("LegalCopyright", "Copyright (c) 2025")
            .compile()
            .unwrap();
    }
}

