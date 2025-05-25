////////////////////////////////////////////////////////////////
// секция форвард-декларации
////////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////
// секция заголовочного файла
////////////////////////////////////////////////////////////////
#include "vkn_validation.h"
////////////////////////////////////////////////////////////////
// секция имплементации
////////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////
// секция для остального
////////////////////////////////////////////////////////////////
#include "vkn_loader.h"
#include "vkn_instance.h"
#include "Layers/vkn_layer_khronos_validation.h"
#include "Extensions/vkn_ext_debug_utils_commands.h"
#include "Extensions/vkn_ext_debug_utils.h"

namespace CGDev {

	namespace GPU {

		namespace Private {

			namespace Vulkan {

				std::vector<std::string> VknValidation::s_layer_name_collection = { Layers::VknLayerKhronosValidation::s_getName() };
				std::vector<std::string> VknValidation::s_extension_name_collection = { Extensions::VknExtDebugUtils::s_getName() };

				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

				bool VknValidationCreateInfo::isValid(void) const noexcept {

					if (log == nullptr) {

						return false;
					}

					// ~~~~~~~~~~~~~~~~
					// VknValidationCreateInfo::loader
					// ~~~~~~~~~~~~~~~~

					//if (loader == nullptr || loader->isValid() == false) {

					//	CGDev::Tools::addEntry(log, CGDev::Tools::LogEntryError("VknValidationCreateInfo::loader fail"));

					//	return false;
					//}

					// ~~~~~~~~~~~~~~~~
					// VknInstance
					// ~~~~~~~~~~~~~~~~

					//if (vkn_instance == nullptr || vkn_instance->isValid() == false) {

					//	CGDev::Tools::addEntry(log, CGDev::Tools::LogEntryError("VknValidationCreateInfo::vkn_instance fail"));

					//	return false;
					//}

					return true;
				}

				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

				void VknValidation::s_requestNextForVkInstance(void*& next_for_vk_instance, CGDev::Tools::LogSptr& log) noexcept {

					//Extensions::VknExtDebugUtils::s_requestNextForVkInstance(next_for_vk_instance, log);
				
				}

				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

				void VknValidation::s_requestLastValidationForVkInstance(bool& validation) noexcept {

					//Extensions::VknExtDebugUtils::s_requestLastValidationForVkInstance(validation);
				}

				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

				VknValidation::VknValidation(void) noexcept {
				}

				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

				VknValidation::~VknValidation(void) noexcept {
				}

				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

				void VknValidation::create(const VknValidationCreateInfo& create_info) noexcept {

					// ~~~~~~~~~~~~~~~~
					// отладочный код
					// проверяем VknValidationCreateInfo на валидность
					// ~~~~~~~~~~~~~~~~

					if constexpr (Private::Build::type_build == Private::Build::TypeBuild::DEBUG) {

						if (create_info.isValid() == false) {

							if (create_info.log != nullptr) Tools::addEntry(create_info.log, Tools::LogEntryError("VknValidationCreateInfo fail"));

							return;
						}

					}

					m_create_info = create_info;
			
					// ~~~~~~~~~~~~~~~~
					// создаём VknExtDebugUtils
					// ~~~~~~~~~~~~~~~~

					if (createVknExtDebugUtils() == false) {

						Tools::addEntry(m_create_info.log, Tools::LogEntryError("createVknExtDebugUtils fail"));

						return;
					}

					// ~~~~~~~~~~~~~~~~
					// подготавливаем имена для слоёв
					// ~~~~~~~~~~~~~~~~

					prepareLayerNames();

					// ~~~~~~~~~~~~~~~~
					// подготавливаем имена для расширений
					// ~~~~~~~~~~~~~~~~

					prepareExtensionNames();

					//m_valid = true;
				}

				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

				void VknValidation::destroy(void) noexcept {

					m_ext_debug_utils->destroy();

					// ~~~~~~~~~~~~~~~~
					// очистка данных
					// ~~~~~~~~~~~~~~~~

					m_create_info = {};

					//m_valid = false;
				}

				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

				//void VknValidation::requestLastValidation(bool& validation) noexcept {

				//	m_ext_debug_utils->requestLastValidation(validation);
				//}

				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

				void VknValidation::searchSupportedLayersAndExtensions(void) noexcept {

					// ~~~~~~~~~~~~~~~~
					// получаем количество VkLayerProperties
					// ~~~~~~~~~~~~~~~~

					//uint32_t _count = 0;

					//auto _vk_result = m_create_info.vkn_functions->vkEnumerateInstanceLayerProperties(&_count, VK_NULL_HANDLE);

					//if (_vk_result != VK_SUCCESS) {

					//	if (_vk_result == VK_ERROR_OUT_OF_HOST_MEMORY) Tools::addEntry(m_create_info.log, Tools::LogEntryError("VK_ERROR_OUT_OF_HOST_MEMORY"));
					//	if (_vk_result == VK_ERROR_OUT_OF_DEVICE_MEMORY) Tools::addEntry(m_create_info.log, Tools::LogEntryError("VK_ERROR_OUT_OF_DEVICE_MEMORY"));

					//	m_valid = false;

					//	return;
					//}

					// ~~~~~~~~~~~~~~~~
					// получаем список VkLayerProperties
					// ~~~~~~~~~~~~~~~~

					//VkLayerPropertiesArr _vk_layer_properties_collection = {};
					//_vk_layer_properties_collection.resize(_count);

					//_vk_result = m_create_info.vkn_functions->vkEnumerateInstanceLayerProperties(&_count, _vk_layer_properties_collection.data());

					//if (_vk_result != VK_SUCCESS &&
					//	_vk_result != VK_INCOMPLETE) {

					//	if (_vk_result == VK_ERROR_OUT_OF_HOST_MEMORY) Tools::addEntry(m_create_info.log, Tools::LogEntryError("VK_ERROR_OUT_OF_HOST_MEMORY"));
					//	if (_vk_result == VK_ERROR_OUT_OF_DEVICE_MEMORY) Tools::addEntry(m_create_info.log, Tools::LogEntryError("VK_ERROR_OUT_OF_DEVICE_MEMORY"));

					//	m_valid = false;

					//	return;
					//}
				}

				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

				bool VknValidation::createVknExtDebugUtils(void) noexcept {

					// ~~~~~~~~~~~~~~~~
					// описываем VknExtCommandsCreateInfo
					// ~~~~~~~~~~~~~~~~

					Extensions::VknExtDebugUtilsCommandsCreateInfo _vkn_ext_commands_create_info					= {};
					//_vkn_ext_commands_create_info.log													= m_create_info.log;
					//_vkn_ext_commands_create_info.loader												= m_create_info.loader;
					//_vkn_ext_commands_create_info.vkn_instance												= m_create_info.vkn_instance;

					// ~~~~~~~~~~~~~~~~
					// создаём VknExtCommands
					// ~~~~~~~~~~~~~~~~

					auto _vkn_ext_commands = std::make_shared<Extensions::VknExtDebugUtilsCommands>();

					_vkn_ext_commands->create(_vkn_ext_commands_create_info);

					

					// ~~~~~~~~~~~~~~~~
					// описываем VknExtDebugUtilsCreateInfo
					// ~~~~~~~~~~~~~~~~

					Extensions::VknExtDebugUtilsCreateInfo _vkn_ext_debug_utils_create_info			= {};
					//_vkn_ext_debug_utils_create_info.log											= m_create_info.log;
					//_vkn_ext_debug_utils_create_info.commands										= std::move(_vkn_ext_commands);
					//_vkn_ext_debug_utils_create_info.vkn_instance										= m_create_info.vkn_instance;

					// ~~~~~~~~~~~~~~~~
					// создаём VknExtDebugUtils
					// ~~~~~~~~~~~~~~~~

					m_ext_debug_utils = std::make_shared<Extensions::VknExtDebugUtils>();

					m_ext_debug_utils->create(_vkn_ext_debug_utils_create_info);

					

					return true;
				}

				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

				void VknValidation::prepareLayerNames(void) noexcept {

					//std::vector<std::string> _layer_name_collection = { "VK_LAYER_KHRONOS_validation" };

					// ~~~~~~~~~~~~~~~~
					// получаем количество VkLayerProperties
					// ~~~~~~~~~~~~~~~~

					//uint32_t _count = 0;

					//auto _vk_result = m_create_info.vkn_functions->vkEnumerateInstanceLayerProperties(&_count, VK_NULL_HANDLE);

					//if (_vk_result != VK_SUCCESS) {

					//	if (_vk_result == VK_ERROR_OUT_OF_HOST_MEMORY) Tools::addEntry(m_create_info.log, Tools::LogEntryError("VK_ERROR_OUT_OF_HOST_MEMORY"));
					//	if (_vk_result == VK_ERROR_OUT_OF_DEVICE_MEMORY) Tools::addEntry(m_create_info.log, Tools::LogEntryError("VK_ERROR_OUT_OF_DEVICE_MEMORY"));

					//	m_valid = false;

					//	return;
					//}

					// ~~~~~~~~~~~~~~~~
					// получаем список VkLayerProperties
					// ~~~~~~~~~~~~~~~~

					//VkLayerPropertiesArr _vk_layer_properties_collection = {};
					//_vk_layer_properties_collection.resize(_count);

					//_vk_result = m_create_info.vkn_functions->vkEnumerateInstanceLayerProperties(&_count, _vk_layer_properties_collection.data());

					//if (_vk_result != VK_SUCCESS &&
					//	_vk_result != VK_INCOMPLETE) {

					//	if (_vk_result == VK_ERROR_OUT_OF_HOST_MEMORY) Tools::addEntry(m_create_info.log, Tools::LogEntryError("VK_ERROR_OUT_OF_HOST_MEMORY"));
					//	if (_vk_result == VK_ERROR_OUT_OF_DEVICE_MEMORY) Tools::addEntry(m_create_info.log, Tools::LogEntryError("VK_ERROR_OUT_OF_DEVICE_MEMORY"));

					//	m_valid = false;

					//	return;
					//}

					// ~~~~~~~~~~~~~~~~
					// проверяем наличие требуемых слоёв
					// ~~~~~~~~~~~~~~~~

					//for (size_t ct_0 = 0; ct_0 < _vk_layer_properties_collection.size(); ++ct_0) {
					//	const auto& it_property = _vk_layer_properties_collection[ct_0];

					//	for (size_t ct_1 = 0; ct_1 < _layer_name_collection.size(); ++ct_1) {
					//		const auto& it_name = _layer_name_collection[ct_1];

					//		if (it_property.layerName == it_name) {

					//			m_layer_name_collection.push_back(it_name);

					//		} // if (it_property.layerName == it_name)

					//	} // for (size_t ct_1 = 0;

					//} // for (size_t ct_0 = 0;
				}

				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

				void VknValidation::prepareExtensionNames(void) noexcept {

					//std::vector<std::string> _extension_name_collection = { "VK_EXT_debug_utils" };

					// ~~~~~~~~~~~~~~~~
					// получаем количество VkExtensionProperties
					// ~~~~~~~~~~~~~~~~

					//uint32_t _count = 0;

					//auto _vk_result = m_create_info.vkn_functions->vkEnumerateInstanceExtensionProperties(VK_NULL_HANDLE, &_count, VK_NULL_HANDLE);

					//if (_vk_result != VK_SUCCESS) {

					//	if (_vk_result == VK_ERROR_OUT_OF_HOST_MEMORY) Tools::addEntry(m_create_info.log, Tools::LogEntryError("VK_ERROR_OUT_OF_HOST_MEMORY"));
					//	if (_vk_result == VK_ERROR_OUT_OF_DEVICE_MEMORY) Tools::addEntry(m_create_info.log, Tools::LogEntryError("VK_ERROR_OUT_OF_DEVICE_MEMORY"));

					//	m_valid = false;

					//	return;
					//}

					// ~~~~~~~~~~~~~~~~
					// получаем список VkExtensionProperties
					// ~~~~~~~~~~~~~~~~

					//VkExtensionPropertiesArr _vk_extension_properties_collection = {};
					//_vk_extension_properties_collection.resize(_count);

					//_vk_result = m_create_info.vkn_functions->vkEnumerateInstanceExtensionProperties(VK_NULL_HANDLE, &_count, _vk_extension_properties_collection.data());

					//if (_vk_result != VK_SUCCESS &&
					//	_vk_result != VK_INCOMPLETE) {

					//	if (_vk_result == VK_ERROR_OUT_OF_HOST_MEMORY) Tools::addEntry(m_create_info.log, Tools::LogEntryError("VK_ERROR_OUT_OF_HOST_MEMORY"));
					//	if (_vk_result == VK_ERROR_OUT_OF_DEVICE_MEMORY) Tools::addEntry(m_create_info.log, Tools::LogEntryError("VK_ERROR_OUT_OF_DEVICE_MEMORY"));

					//	m_valid = false;

					//	return;
					//}

					// ~~~~~~~~~~~~~~~~
					// проверяем наличие требуемых расширений
					// ~~~~~~~~~~~~~~~~

					//for (size_t ct_0 = 0; ct_0 < _vk_extension_properties_collection.size(); ++ct_0) {
					//	const auto& it_property = _vk_extension_properties_collection[ct_0];

					//	for (size_t ct_1 = 0; ct_1 < _extension_name_collection.size(); ++ct_1) {
					//		const auto& it_name = _extension_name_collection[ct_1];

					//		if (it_property.extensionName == it_name) {

					//			m_extension_name_collection.push_back(it_name);

					//		} // if (it_property.extensionName == it_name)

					//	} // for (size_t ct_1 = 0;

					//} // for (size_t ct_0 = 0;
				}

				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

				VkBool32 VknValidation::callbackForVkDebugUtilsMessengerCreateInfoEXT(VkDebugUtilsMessageSeverityFlagBitsEXT messageSeverity, VkDebugUtilsMessageTypeFlagsEXT messageTypes, const VkDebugUtilsMessengerCallbackDataEXT* pCallbackData, void* pUserData) noexcept {

					// ~~~~~~~~~~~~~~~~
					// 
					// ~~~~~~~~~~~~~~~~

					//if (m_validation_bits[static_cast<unsigned>(VknValidationFlags::PROCEED)] == true) {
						
					//	m_validation_bits[static_cast<unsigned>(VknValidationFlags::PROCEED)] = false;
					//	m_validation_bits[static_cast<unsigned>(VknValidationFlags::INFO)] = true;
					//	m_validation_bits[static_cast<unsigned>(VknValidationFlags::WARNING)] = true;
					//	m_validation_bits[static_cast<unsigned>(VknValidationFlags::ERRO)] = true;						
						
					//}

					// ~~~~~~~~~~~~~~~~
					// 
					// ~~~~~~~~~~~~~~~~

					//if (messageSeverity & VK_DEBUG_UTILS_MESSAGE_SEVERITY_INFO_BIT_EXT) {

						//Tools::addEntry(m_create_info.log, Tools::LogEntryInfo(pCallbackData->pMessage));

					//	m_validation_bits[static_cast<unsigned>(VknValidationFlags::INFO)] = false;
					//}

					// ~~~~~~~~~~~~~~~~
					// 
					// ~~~~~~~~~~~~~~~~

					//if (messageSeverity & VK_DEBUG_UTILS_MESSAGE_SEVERITY_WARNING_BIT_EXT) {

					//	Tools::addEntry(m_create_info.log, Tools::LogEntryWarning(pCallbackData->pMessage));

					//	m_validation_bits[static_cast<unsigned>(VknValidationFlags::WARNING)] = false;
					//}

					// ~~~~~~~~~~~~~~~~
					// 
					// ~~~~~~~~~~~~~~~~

					//if (messageSeverity & VK_DEBUG_UTILS_MESSAGE_SEVERITY_ERROR_BIT_EXT) {

					//	Tools::addEntry(m_create_info.log, Tools::LogEntryError(pCallbackData->pMessage));

					//	m_validation_bits[static_cast<unsigned>(VknValidationFlags::ERRO)] = false;
					//}

					return VK_FALSE;
				}

				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

				VkBool32 VknValidation::s_callbackForVkDebugUtilsMessengerCreateInfoEXT(VkDebugUtilsMessageSeverityFlagBitsEXT messageSeverity, VkDebugUtilsMessageTypeFlagsEXT messageTypes, const VkDebugUtilsMessengerCallbackDataEXT* pCallbackData, void* pUserData) noexcept {
		
					return static_cast<VknValidation*>(pUserData)->callbackForVkDebugUtilsMessengerCreateInfoEXT(messageSeverity, messageTypes, pCallbackData, pUserData);
				}

				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

			} // namespace Vulkan

		} // namespace Private

	} // namespace GPU

} // namespace CGDev
