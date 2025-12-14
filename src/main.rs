use winapi::um::cfgmgr32::{CM_Reenumerate_DevNode, CR_SUCCESS};
use winapi::um::setupapi::{SetupDiGetClassDevsA, SetupDiEnumDeviceInfo, SetupDiDestroyDeviceInfoList, DIGCF_PRESENT, DIGCF_ALLCLASSES, SP_DEVINFO_DATA};
use winapi::um::handleapi::INVALID_HANDLE_VALUE;
use std::ptr;

/// Запускает обновление конфигурации оборудования в Диспетчере устройств
/// Использует SetupDi API для перечисления всех устройств и обновления их конфигурации
fn refresh_hardware_configuration() -> Result<(), String> {
    unsafe {
        // Получаем список всех устройств в системе
        let device_info_set = SetupDiGetClassDevsA(
            ptr::null(),
            ptr::null_mut(),
            ptr::null_mut(),
            DIGCF_PRESENT | DIGCF_ALLCLASSES
        );
        
        if device_info_set == INVALID_HANDLE_VALUE {
            return Err("Не удалось получить список устройств".to_string());
        }
        
        // Перечисляем все устройства и обновляем их конфигурацию
        let mut device_index = 0u32;
        let mut updated_count = 0;
        let mut error_count = 0;
        
        loop {
            let mut device_info_data: SP_DEVINFO_DATA = std::mem::zeroed();
            device_info_data.cbSize = std::mem::size_of::<SP_DEVINFO_DATA>() as u32;
            
            // SetupDiEnumDeviceInfo возвращает 0 при ошибке или конце списка
            if SetupDiEnumDeviceInfo(device_info_set, device_index, &mut device_info_data) == 0 {
                break;
            }
            
            // Получаем дескриптор устройства из device_info_data
            let dev_inst = device_info_data.DevInst;
            
            // Обновляем конфигурацию устройства
            let flags = 0x00000001u32; // CM_REENUMERATE_RETRY_INSTALLATION
            let result = CM_Reenumerate_DevNode(dev_inst, flags);
            
            if result == CR_SUCCESS {
                updated_count += 1;
            } else {
                error_count += 1;
            }
            
            device_index += 1;
        }
        
        // Закрываем handle
        SetupDiDestroyDeviceInfoList(device_info_set);
        
        if updated_count > 0 {
            Ok(())
        } else if error_count > 0 {
            Err(format!("Не удалось обновить конфигурацию устройств. Попробуйте запустить программу с правами администратора."))
        } else {
            Err("Не найдено устройств для обновления".to_string())
        }
    }
}

fn main() {
    println!("Запуск обновления конфигурации оборудования...");
    
    match refresh_hardware_configuration() {
        Ok(_) => println!("Конфигурация оборудования успешно обновлена!"),
        Err(e) => eprintln!("Ошибка: {}", e),
    }

    // Добавить паузу на 10 секунд
    std::thread::sleep(std::time::Duration::from_secs(3));

    // Презагрузка "Проводник"
    std::process::Command::new("explorer.exe").spawn().unwrap();
}
