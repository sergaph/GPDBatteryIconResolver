use winapi::um::cfgmgr32::{CM_Reenumerate_DevNode, CR_SUCCESS};
use winapi::um::setupapi::{SetupDiGetClassDevsA, SetupDiEnumDeviceInfo, SetupDiDestroyDeviceInfoList, DIGCF_PRESENT, DIGCF_ALLCLASSES, SP_DEVINFO_DATA};
use winapi::um::handleapi::INVALID_HANDLE_VALUE;
use std::ptr;

/// Starts hardware configuration refresh in Device Manager
/// Uses SetupDi API to enumerate all devices and refresh their configuration
fn refresh_hardware_configuration() -> Result<(), String> {
    unsafe {
        // Get list of all devices in the system
        let device_info_set = SetupDiGetClassDevsA(
            ptr::null(),
            ptr::null_mut(),
            ptr::null_mut(),
            DIGCF_PRESENT | DIGCF_ALLCLASSES
        );
        
        if device_info_set == INVALID_HANDLE_VALUE {
            return Err("Failed to get device list".to_string());
        }
        
        // Enumerate all devices and refresh their configuration
        let mut device_index = 0u32;
        let mut updated_count = 0;
        let mut error_count = 0;
        
        loop {
            let mut device_info_data: SP_DEVINFO_DATA = std::mem::zeroed();
            device_info_data.cbSize = std::mem::size_of::<SP_DEVINFO_DATA>() as u32;
            
            // SetupDiEnumDeviceInfo returns 0 on error or end of list
            if SetupDiEnumDeviceInfo(device_info_set, device_index, &mut device_info_data) == 0 {
                break;
            }
            
            // Get device instance handle from device_info_data
            let dev_inst = device_info_data.DevInst;
            
            // Refresh device configuration
            let flags = 0x00000001u32; // CM_REENUMERATE_RETRY_INSTALLATION
            let result = CM_Reenumerate_DevNode(dev_inst, flags);
            
            if result == CR_SUCCESS {
                updated_count += 1;
            } else {
                error_count += 1;
            }
            
            device_index += 1;
        }
        
        // Close handle
        SetupDiDestroyDeviceInfoList(device_info_set);
        
        if updated_count > 0 {
            Ok(())
        } else if error_count > 0 {
            Err(format!("Failed to refresh device configuration. Try running the program as administrator."))
        } else {
            Err("No devices found to refresh".to_string())
        }
    }
}

fn main() {
    println!("1. Starting hardware configuration refresh...");
    
    match refresh_hardware_configuration() {
        Ok(_) => println!(">>> Hardware configuration successfully refreshed!"),
        Err(e) => eprintln!(">>> Error: {}", e),
    }

    // Pause before restarting Explorer
    std::thread::sleep(std::time::Duration::from_secs(3));

    // Restart Explorer
    println!("2. Restarting Explorer...");
    
    // Terminate explorer.exe process
    let _ = std::process::Command::new("taskkill")
        .args(&["/F", "/IM", "explorer.exe"])
        .output();
    
    // Wait a bit for the process to fully terminate
    std::thread::sleep(std::time::Duration::from_secs(2));
    
    // Start explorer.exe again
    match std::process::Command::new("explorer.exe").spawn() {
        Ok(_) => println!(">>> Explorer successfully restarted!"),
        Err(e) => eprintln!(">>> Error starting Explorer: {}", e),
    }

    // Pause before program termination
    println!("3. Terminating program...");
    std::thread::sleep(std::time::Duration::from_secs(3));
}
