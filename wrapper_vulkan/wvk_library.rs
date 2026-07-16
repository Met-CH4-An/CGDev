// SPDX-License-Identifier: None
// Copyright (c) 2026 None

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
/// зависимости
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
use {
    std::marker::PhantomData,

    crate::wvk_call_with_check,
    crate::WvkError,
    crate::WvkErrorType,
};
use crate::WvkFeature_0_1_0_0;

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
///
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
pub struct WvkLibrary<TWvkVersion> {
    phantom : PhantomData<TWvkVersion>,

    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    // команды вулкана
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    
    // vulkan 1.0
    vkGetInstanceProcAddr : crate::svk::PFN_vkGetInstanceProcAddr,
    vkEnumerateInstanceLayerProperties : crate::svk::PFN_vkEnumerateInstanceLayerProperties,
    vkEnumerateInstanceExtensionProperties : crate::svk::PFN_vkEnumerateInstanceExtensionProperties,
    vkCreateInstance : crate::svk::PFN_vkCreateInstance,    
    
    // vulkan 1.1
    vkEnumerateInstanceVersion : crate::svk::PFN_vkEnumerateInstanceVersion,
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
/// публичные методы
/// public methods
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
impl<TWvkVersion> WvkLibrary<TWvkVersion>
where TWvkVersion : WvkFeature_0_1_0_0 {
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    ///
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    pub fn create() -> Result<Self, crate::WvkError> {
        // получаем первичную функцию PFN_vkGetInstanceProcAddr
        let _vkGetInstanceProcAddr = Self::loadVkGetInstanceProcAddr().map_err(|wvk_error| {
            wvk_error.addError(WvkErrorType::WVK_LIBRARY_CREATE_FAILED, "Не удалось выполнить loadVkGetInstanceProcAddr.")
        })?;

        return Ok(Self{
            phantom : PhantomData,
            // vulkan 1.0
            vkGetInstanceProcAddr : _vkGetInstanceProcAddr,            
            vkEnumerateInstanceLayerProperties : Self::loadCommand::<crate::svk::PFN_vkEnumerateInstanceLayerProperties>(_vkGetInstanceProcAddr, "vkEnumerateInstanceLayerProperties\0")?,
            vkEnumerateInstanceExtensionProperties : Self::loadCommand::<crate::svk::PFN_vkEnumerateInstanceExtensionProperties>(_vkGetInstanceProcAddr, "vkEnumerateInstanceExtensionProperties\0")?,
            vkCreateInstance : Self::loadCommand::<crate::svk::PFN_vkCreateInstance>(_vkGetInstanceProcAddr, "vkCreateInstance\0")?,

            // vulkan 1.1
            vkEnumerateInstanceVersion : Self::loadCommand::<crate::svk::PFN_vkEnumerateInstanceVersion>(_vkGetInstanceProcAddr, "vkEnumerateInstanceVersion\0")?,
        });
    }
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
/// публичные методы
/// public methods
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
impl<TWvkVersion> WvkLibrary<TWvkVersion>
where TWvkVersion : WvkFeature_0_1_0_0 {
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    ///
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    #[cfg(feature = "vulkan_1_0")]
    pub fn wvkEnumerateInstanceLayerProperties(&self) -> Result<Vec<svk::VkLayerProperties>, WvkError> {
        let mut properties_ : Vec<svk::VkLayerProperties> = Vec::new();
       
        // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
        // получаем количество
        // get the quantity
        // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

        let mut count_ : u32 = 0;
        wvk_call_with_check!(
            (self.vkEnumerateInstanceLayerProperties)(&mut count_, std::ptr::null_mut())
        );
        
        // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
        // выделяем память под свойства и запрашиваем свойства через vkEnumerateInstanceLayerProperties
        // allocate memory for properties and request properties via vkEnumerateInstanceLayerProperties
        // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
        
        //properties.resize_with(count_ as usize, || unsafe { std::mem::zeroed()});
        properties_.reserve(count_ as usize);
        unsafe {
            properties_.set_len(count_ as usize);
        };

        wvk_call_with_check!(
            (self.vkEnumerateInstanceLayerProperties)(&mut count_, properties_.as_mut_ptr())
        );

        return Ok(properties_);
    }

    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    ///
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    #[cfg(feature = "vulkan_1_0")]
    pub fn wvkEnumerateInstanceExtensionProperties(&self, layer_name : Option<&str>) -> Result<Vec<svk::VkExtensionProperties>, WvkError> {
        let mut properties_ = Vec::<svk::VkExtensionProperties>::new();

        // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
        // вулкан апи использует си строки. конвертируем str в CString
        // The Vulcan API uses C strings. Converting str to CString
        // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

        let layer_name_cstring_ = layer_name.map(|layer_name_str| {
            std::ffi::CString::new(layer_name_str).map_err(|std_ffi_nul_error| {
                WvkError::createWithDescription(WvkErrorType::WVK_INPUT_PARAMETER_INVALID, &format!("Не удалось получить CString: в &str обнаружен нулевой байт {}", std_ffi_nul_error.nul_position()))
            })
        }).transpose()?;

        let layer_name_ptr_ = layer_name_cstring_
            .as_ref()
            .map_or_else(
                std::ptr::null, 
                |v| {
                    v.as_ptr()
        });

        // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
        // получаем количество
        // get the quantity
        // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

        let mut count_ : u32 = 0;

        wvk_call_with_check!(
            (self.vkEnumerateInstanceExtensionProperties)(layer_name_ptr_, &mut count_, std::ptr::null_mut())
        );

        // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
        // выделяем память под свойства и запрашиваем свойства через vkEnumerateInstanceLayerProperties
        // allocate memory for properties and request properties via vkEnumerateInstanceLayerProperties
        // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

        properties_.reserve(count_ as usize);
        unsafe {
            properties_.set_len(count_ as usize);
        }
        
        wvk_call_with_check!(
            (self.vkEnumerateInstanceExtensionProperties)(layer_name_ptr_, &mut count_, properties_.as_mut_ptr())
        );

        return Ok(properties_);
    }

    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    ///
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    #[cfg(feature = "vulkan_1_0")]
    pub fn wvkCreateInstance(&self, create_info : &svk::VkInstanceCreateInfo, allocator : Option<&svk::VkAllocationCallbacks>) -> Result<svk::VkInstance, WvkError> {
        /*let allocator_ptr_ = match allocator {
            Some(value) => {
                value as *const svk::VkAllocationCallbacks
            }
            None => std::ptr::null()
        };
        let allocator_ptr_ = allocator.map(|value| {value as *const svk::VkAllocationCallbacks}).unwrap_or(std::ptr::null());
        let allocator_ptr_ = allocator.map_or(std::ptr::null(), |value| {value as *const svk::VkAllocationCallbacks});
        let allocator_ptr_ = allocator.map_or_else(std::ptr::null, |value| {value as *const svk::VkAllocationCallbacks});*/
        
        // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
        // если аалокатор задан - превращает в *const svk::VkAllocationCallbacks
        // если не задан - std::ptr::null
        // If the allocation callback is specified, it converts to *const svk::VkAllocationCallbacks
        // If not specified, it converts to std::ptr::null
        // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

        let allocator_ptr_ = allocator.map_or_else(std::ptr::null, |value| {value as *const _});
        
        let mut vk_instance_ : svk::VkInstance = std::ptr::null_mut();

        // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
        // создаем svk::VkInstance
        // create svk::VkInstance
        // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

        wvk_call_with_check!(
            (self.vkCreateInstance)(create_info, allocator_ptr_, &mut vk_instance_)
        );

        return Ok(vk_instance_);
    }

    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    ///
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    #[cfg(feature = "vulkan_1_1")]
    pub fn wvkEnumerateInstanceVersion(&self) -> Result<u32, WvkError> {
        let mut version_ : u32 = 0;
        
        wvk_call_with_check!(
            (self.vkEnumerateInstanceVersion)(&mut version_)
        );

        return Ok(version_);
    }
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
/// защищённые методы
/// protected methods
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
/// приватные методы
/// private methods
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
impl<TWvkVersion> WvkLibrary<TWvkVersion>
where TWvkVersion : WvkFeature_0_1_0_0 {
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    ///
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    fn loadVkGetInstanceProcAddr() -> Result<crate::svk::PFN_vkGetInstanceProcAddr, crate::WvkError> {
        // поддержка платформы MSWindows
        #[cfg(target_os = "windows")]
        {
            // пробуем загрузить vulkan-1.dll
            let _hmodule = unsafe {
                windows::Win32::System::LibraryLoader::LoadLibraryA(windows::core::PCSTR(b"vulkan-1.dll\0".as_ptr()))
                    .map_err(|windows_core_error| {
                        WvkError::createWithDescription(
                            WvkErrorType::WVK_LIBRARY_VULKAN_LIBRARY_LOAD_FAILED, 
                            &format!("Не удалось загрузить vulkan-1.dll: LoadLibraryA вернула {}.", &windows_core_error.message())
                        )
                    }
                )?
            };

            // пробуем получить vkGetInstanceProcAddr
            let _proc = unsafe {
                windows::Win32::System::LibraryLoader::GetProcAddress(_hmodule, windows::core::PCSTR(b"vkGetInstanceProcAddr\0".as_ptr()))
                    .ok_or_else(|| {
                        WvkError::createWithDescription(
                            WvkErrorType::WVK_LIBRARY_VULKAN_LIBRARY_LOAD_FAILED,
                            &format!("Не удалось получить адрес vkGetInstanceProcAddr: не найдена в vulkan-1.dll.")
                        )
                    }
                )?
            };

            // преобразовываем в памяти в нужный тип
            let vkGetInstanceProcAddr = unsafe {
                std::mem::transmute::<
                    _,
                    crate::svk::PFN_vkGetInstanceProcAddr
                >(_proc)
            };

            return Ok(vkGetInstanceProcAddr);
        }

        // никакая платформа не поддерживается
        #[cfg(not(any(target_os = "windows")))]
        compile_error!("Платформа не поддерживается.");        
    }

    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    ///
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    fn loadCommand<T>(vkGetInstanceProcAddr : crate::svk::PFN_vkGetInstanceProcAddr, name : &str) -> Result<T, crate::WvkError> {
        // загружаем команду через vkGetInstanceProcAddr
        let _command_raw = unsafe {
            (vkGetInstanceProcAddr)(std::ptr::null_mut(), name.as_ptr() as *const i8)
        };

        // если не удалось
        if _command_raw.is_null() {
            return Err(WvkError::createWithDescription(
                WvkErrorType::WVK_LIBRARY_VULKAN_COMMAND_LOAD_FAILED, 
                &format!("Не удалось загрузить команду вулкана: {}", name))
            );
        };

        // превращаем в конкретный тип команды
        let _command = unsafe {
            std::mem::transmute_copy::<*mut std::ffi::c_void, T>(
                &_command_raw
            )            
        };         

        return Ok(_command);
    }
}

#[cfg(test)]
mod tests {
    use crate::WVK_0_1_4_0;
    use super::*;

    #[test]
    fn wvk_library__create__ok() {
        let wvk_library_ = WvkLibrary::<WVK_0_1_4_0>::create().ok();
        assert!(wvk_library_.is_some());
    }

    #[test]
    fn wvk_library__wvkEnumerateInstanceExtensionProperties__null_byte_parameter() {
        let wvk_library_ = WvkLibrary::<WVK_0_1_4_0>::create().ok().unwrap();
        
        let wvk_error_ = wvk_library_.wvkEnumerateInstanceExtensionProperties(Some("layer\0name")).err();
        println!("{}", wvk_error_.as_ref().unwrap().getMessage());
        assert!(wvk_error_.is_some(), "message");
    }
}