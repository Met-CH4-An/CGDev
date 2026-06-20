// SPDX-License-Identifier: None
// Copyright (c) 2026 None

fn main() {
    println!("пример: создание WvkLibrary\n");
    
    let _wvk_library = match wvk::WvkLibrary::new() {
        Ok(wvk_library) => wvk_library,
        Err(wvk_error) => {
            println!("{}", wvk_error.getMessage());
            return ;
        }
    };

    println!("пример: получение свойств слоёв\n");
    {
        let properties_ = match _wvk_library.wvkEnumerateInstanceLayerProperties() {
            Ok(v) => v,
            Err(v) => {
                println!("{}", v.getMessage());
                return;
            }
        };
        
        for property in &properties_ {
            let layer_name_ = unsafe { 
                std::ffi::CStr::from_ptr(property.layerName.as_ptr())
                .to_string_lossy()
                .into_owned()
            };
            let spec_version_ = property.specVersion;
            let implementation_version_ = property.implementationVersion;
            let description_cstr_ = unsafe { 
                std::ffi::CStr::from_ptr(property.description.as_ptr())
                .to_string_lossy()
                .into_owned()
            };

            println!("layerName: {}", layer_name_);
            println!("specVersion: {}", spec_version_);
            println!("implementationVersion: {}", implementation_version_);
            println!("description: {}\n", description_cstr_);
        }
    }

    println!("пример: получение свойств расширений\n");
    {
        let properties_ = match _wvk_library.wvkEnumerateInstanceExtensionProperties(None) {
            Ok(v) => v,
            Err(v) => {
                println!("{}", v.getMessage());
                return;
            }
        };

        for property in &properties_ {
            let extension_name_ = unsafe { 
                std::ffi::CStr::from_ptr(property.extensionName.as_ptr())
                .to_string_lossy()
                .into_owned()
            };
            let spec_version_ = property.specVersion;

            println!("layerName: {}", extension_name_);
            println!("specVersion: {}\n", spec_version_);
        }

    }

    println!("пример: ошибка при получении свойств расширений\n");
    {
        match _wvk_library.wvkEnumerateInstanceExtensionProperties(Some("Не существующий слой")) {
            Ok(_) => (),
            Err(v) => {
                println!("{}", v.getMessage());
                ()
            }
        };

    }

    println!("пример: получение версий инстанса вулкана\n");
    {
        let version_result = _wvk_library.wvkEnumerateInstanceVersion();
        
        if let Err(wvk_error) = &version_result {
            println!("Не удалось получить версии инстанса вулкана: {}", wvk_error.getMessage());
        }

        if let Ok(version) = &version_result {
            wvk::svk::VK_API_VERSION_MAJOR(*version);
            println!("Версия вулкана поддерживаемая системой: {}.{}.{}", wvk::svk::VK_API_VERSION_MAJOR(*version), wvk::svk::VK_API_VERSION_MINOR(*version), wvk::svk::VK_API_VERSION_PATCH(*version));
        }
    }   

    let _ = std::io::stdin().read_line(&mut String::new());
}