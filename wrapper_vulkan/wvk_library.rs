// SPDX-License-Identifier: None
// Copyright (c) 2026 None

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
/// зависимости
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
use {
    crate::wvk_call_with_check,
    crate::WvkError,
    crate::WvkErrorType,
};

//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
///
//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
pub struct WvkLibrary {
    // vulkan 1.0
    vkGetInstanceProcAddr : crate::svk::PFN_vkGetInstanceProcAddr,
    vkEnumerateInstanceLayerProperties : crate::svk::PFN_vkEnumerateInstanceLayerProperties,
    vkEnumerateInstanceExtensionProperties : crate::svk::PFN_vkEnumerateInstanceExtensionProperties,
    vkCreateInstance : crate::svk::PFN_vkCreateInstance,    
    
    // vulkan 1.1
    vkEnumerateInstanceVersion : crate::svk::PFN_vkEnumerateInstanceVersion,
}

//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
///
//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
impl WvkLibrary {
    //~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    ///
    //~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    pub fn new() -> Result<Self, crate::WvkError> {
        // получаем первичную функцию PFN_vkGetInstanceProcAddr
        let _vkGetInstanceProcAddr = Self::loadVkGetInstanceProcAddr().map_err(|wvk_err| {
            wvk_err.addMessage("Не удалось получить библиотеку вулкана.")
        })?;

        return Ok(Self{
            // vulkan 1.0
            vkGetInstanceProcAddr : _vkGetInstanceProcAddr,            
            vkEnumerateInstanceLayerProperties : Self::loadCommand::<crate::svk::PFN_vkEnumerateInstanceLayerProperties>(_vkGetInstanceProcAddr, "vkEnumerateInstanceLayerProperties\0")?,
            vkEnumerateInstanceExtensionProperties : Self::loadCommand::<crate::svk::PFN_vkEnumerateInstanceExtensionProperties>(_vkGetInstanceProcAddr, "vkEnumerateInstanceExtensionProperties\0")?,
            vkCreateInstance : Self::loadCommand::<crate::svk::PFN_vkCreateInstance>(_vkGetInstanceProcAddr, "vkCreateInstance\0")?,

            // vulkan 1.1
            vkEnumerateInstanceVersion : Self::loadCommand::<crate::svk::PFN_vkEnumerateInstanceVersion>(_vkGetInstanceProcAddr, "vkEnumerateInstanceVersion\0")?,
        });
    }

    //~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    ///
    //~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    fn loadVkGetInstanceProcAddr() -> Result<crate::svk::PFN_vkGetInstanceProcAddr, crate::WvkError> {
        // поддержка платформы MSWindows
        #[cfg(target_os = "windows")]
        {
            // пробуем загрузить vulkan-1.dll
            let _hmodule = unsafe {
                windows::Win32::System::LibraryLoader::LoadLibraryA(windows::core::PCSTR(b"vulkan-1.dll\0".as_ptr()))
                    .map_err(|err| {
                        WvkError::newWithMessage(
                            WvkErrorType::WVK_RUNTIME_VULKAN_LIBRARY_LOAD_FAILED,
                            &format!("vulkan-1.dll не найдена. LoadLibraryA вернула {}", &err.message())
                        )
                    })?
            };

            // пробуем получить vkGetInstanceProcAddr
            let _proc = unsafe {
                windows::Win32::System::LibraryLoader::GetProcAddress(_hmodule, windows::core::PCSTR(b"vkGetInstanceProcAddr\0".as_ptr()))
                    .ok_or_else(|| {
                        WvkError::newWithMessage(
                            WvkErrorType::WVK_RUNTIME_VULKAN_LIBRARY_LOAD_FAILED,
                            &format!("vkGetInstanceProcAddr не найдена.")
                        )
                    })?
            };

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

    //~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    ///
    //~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    fn loadCommand<T>(vkGetInstanceProcAddr : crate::svk::PFN_vkGetInstanceProcAddr, name : &str) -> Result<T, crate::WvkError> {
        // загружаем команду через vkGetInstanceProcAddr
        let _command_raw = unsafe {
            (vkGetInstanceProcAddr)(std::ptr::null_mut(), name.as_ptr() as *const i8)
        };

        // если не удалось
        if _command_raw.is_null() {
            return Err(WvkError::newWithMessage(
                WvkErrorType::WVK_RUNTIME_VULKAN_COMMAND_LOAD_FAILED, 
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

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
///
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
impl WvkLibrary {
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    ///
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    #[cfg(feature = "vulkan_1_0")]
    pub fn wvkEnumerateInstanceLayerProperties(&self) -> Result<Vec<svk::VkLayerProperties>, WvkError> {
        let mut properties_ : Vec<svk::VkLayerProperties> = Vec::new();
       
        // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
        // получаем количество
        // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

        let mut count_ : u32 = 0;
        wvk_call_with_check!(
            (self.vkEnumerateInstanceLayerProperties)(&mut count_, std::ptr::null_mut())
        );
        
        // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
        // получаем свойства
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
        // в вулкане если нужны все расширения, команда PFN_vkEnumerateInstanceExtensionProperties
        // должна принимать нулевой указатель в качестве имени
        // если нужны расширения для конкретного слоя, указывается соответственно имя

        let mut properties_ : Vec<svk::VkExtensionProperties> = Vec::new();

        let layer_name_cstring_ = layer_name.map(|v| {
            std::ffi::CString::new(v)
                .map_err(|_| {
                    WvkError::new(WvkErrorType::WVK_INPUT_PARAMETER_FAILED)
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
        // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

        let mut count_ : u32 = 0;

        wvk_call_with_check!(
            (self.vkEnumerateInstanceExtensionProperties)(layer_name_ptr_, &mut count_, std::ptr::null_mut())
        );

        // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
        // получаем свойства
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

    //~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    ///
    //~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
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
        
        let allocator_ptr_ = allocator.map_or_else(std::ptr::null, |value| {value as *const _});
        
        let mut vk_instance_ : svk::VkInstance = std::ptr::null_mut();

        wvk_call_with_check!(
            (self.vkCreateInstance)(create_info, allocator_ptr_, &mut vk_instance_)
        );

        return Ok(vk_instance_);
    }

    //~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    ///
    //~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    #[cfg(feature = "vulkan_1_1")]
    pub fn wvkEnumerateInstanceVersion(&self) -> Result<u32, WvkError> {
        let mut version_ : u32 = 0;
        
        wvk_call_with_check!(
            (self.vkEnumerateInstanceVersion)(&mut version_)
        );
        
        return Ok(version_);
    }
}