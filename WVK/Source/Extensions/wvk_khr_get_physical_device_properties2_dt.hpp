#ifndef CGDEV_WVK_SOURCE_EXTENSIONS__WVK_KHR_GET_PHYSICAL_DEVICE_PROPERTIES2_DT_H
#define CGDEV_WVK_SOURCE_EXTENSIONS__WVK_KHR_GET_PHYSICAL_DEVICE_PROPERTIES2_DT_H
////////////////////////////////////////////////////////////////
// секция форвард-декларации
////////////////////////////////////////////////////////////////
#include "_extensions.h"
////////////////////////////////////////////////////////////////
// секция имплементации
////////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////
// секция родительского класса
////////////////////////////////////////////////////////////////
#include "../wvk_extension.h"
////////////////////////////////////////////////////////////////
// секция для остального
////////////////////////////////////////////////////////////////
#include "../wvk_loader.h"
#include "../wvk_instance.h"

namespace CGDev {

	namespace wvk {

		namespace Extensions {

			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			/*!	\brief
			*/
			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			struct WvkKhrGetPhysicalDeviceProperties2DTCreateInfo {
				WvkLoaderPtr wvk_loader = nullptr;
				WvkInstancePtr wvk_instance = nullptr;
			}; // WvkKhrGetPhysicalDeviceProperties2DTCreateInfo

			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			/*!	\brief
			*/
			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			class WvkKhrGetPhysicalDeviceProperties2DT : public VknExtension {

			public:

				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				/*!	\brief
				*/
				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				inline static constexpr std::string_view s_getName(void) noexcept {
					return s_name;
				}

			private:

				inline static constexpr std::string_view s_name = "VK_KHR_get_physical_device_properties2";

			public:

				inline void wvkGetPhysicalDeviceFeatures2KHR(VkPhysicalDevice physicalDevice, VkPhysicalDeviceFeatures2* pFeatures) const noexcept {
					m_vkGetPhysicalDeviceFeatures2KHR(physicalDevice, pFeatures);}
				inline void wvkGetPhysicalDeviceProperties2KHR(VkPhysicalDevice physicalDevice, VkPhysicalDeviceProperties2* pProperties) const noexcept {
					m_vkGetPhysicalDeviceProperties2KHR(physicalDevice, pProperties);}
				inline VkResult wvkGetPhysicalDeviceFormatProperties2KHR(VkPhysicalDevice physicalDevice, VkFormat format, VkFormatProperties2* pFormatProperties) const noexcept {
					m_vkGetPhysicalDeviceFormatProperties2KHR(physicalDevice, format, pFormatProperties);}
				inline void wvkGetPhysicalDeviceImageFormatProperties2KHR(VkPhysicalDevice physicalDevice, const VkPhysicalDeviceImageFormatInfo2* pImageFormatInfo, VkImageFormatProperties2* pImageFormatProperties) const noexcept {
					m_vkGetPhysicalDeviceImageFormatProperties2KHR(physicalDevice, pImageFormatInfo, pImageFormatProperties);}
				inline void wvkGetPhysicalDeviceQueueFamilyProperties2KHR(VkPhysicalDevice physicalDevice, uint32_t* pQueueFamilyPropertyCount, VkQueueFamilyProperties2* pQueueFamilyProperties) const noexcept {
					m_vkGetPhysicalDeviceQueueFamilyProperties2KHR(physicalDevice, pQueueFamilyPropertyCount, pQueueFamilyProperties);}
				inline void wvkGetPhysicalDeviceMemoryProperties2KHR(VkPhysicalDevice physicalDevice, VkPhysicalDeviceMemoryProperties2* pMemoryProperties) const noexcept {
					m_vkGetPhysicalDeviceMemoryProperties2KHR(physicalDevice, pMemoryProperties);}
				inline void wvkGetPhysicalDeviceSparseImageFormatProperties2KHR(VkPhysicalDevice physicalDevice, const VkPhysicalDeviceSparseImageFormatInfo2* pFormatInfo, uint32_t* pPropertyCount, VkSparseImageFormatProperties2* pProperties) const noexcept {
					m_vkGetPhysicalDeviceSparseImageFormatProperties2KHR(physicalDevice, pFormatInfo, pPropertyCount, pProperties);}

			private:

				PFN_vkGetPhysicalDeviceFeatures2KHR m_vkGetPhysicalDeviceFeatures2KHR = VK_NULL_HANDLE;
				PFN_vkGetPhysicalDeviceProperties2KHR m_vkGetPhysicalDeviceProperties2KHR = VK_NULL_HANDLE;
				PFN_vkGetPhysicalDeviceFormatProperties2KHR m_vkGetPhysicalDeviceFormatProperties2KHR = VK_NULL_HANDLE;
				PFN_vkGetPhysicalDeviceImageFormatProperties2KHR m_vkGetPhysicalDeviceImageFormatProperties2KHR = VK_NULL_HANDLE;
				PFN_vkGetPhysicalDeviceQueueFamilyProperties2KHR m_vkGetPhysicalDeviceQueueFamilyProperties2KHR = VK_NULL_HANDLE;
				PFN_vkGetPhysicalDeviceMemoryProperties2KHR m_vkGetPhysicalDeviceMemoryProperties2KHR = VK_NULL_HANDLE;
				PFN_vkGetPhysicalDeviceSparseImageFormatProperties2KHR m_vkGetPhysicalDeviceSparseImageFormatProperties2KHR = VK_NULL_HANDLE;
				
			public:

				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				/*!	\brief
				*/
				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				inline WvkKhrGetPhysicalDeviceProperties2DT(void) : VknExtension(s_name) {}

				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				/*!	\brief
				*/
				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				inline ~WvkKhrGetPhysicalDeviceProperties2DT(void) noexcept {}

				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				/*!	\
				*/
				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				inline WvkStatus create(const WvkKhrGetPhysicalDeviceProperties2DTCreateInfo& create_info) noexcept {
					WvkStatus _status;

					// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					// Шаг 1. Проверка на повторную инициализацию
					// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					if (m_valid) {
						return _status.set(VknStatusCode::ALREADY_INITIALIZED);
					}

					// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					// Шаг 2. Сохраняем переданную структуру создания.
					// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					m_create_info = create_info;

					// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					// Шаг 3. Если включена сборка с валидацией, запускаем проверку данных.
					// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					if constexpr (wvk::Build::ValidationBuildInfo::enable == true) {
						_status = validationCreateInfo();

						if (_status.m_code != VknStatusCode::SUCCESSFUL) {
							reset();
							return _status.set(VknStatusCode::FAIL, "\n\tWvkKhrSurfaceDispatchTable::validationCreateInfo() - fail.");
						}
					}

					// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					// Шаг 4. Загружаем указатели на Vulkan-функции, связанные с Surface.
					// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					_status = loadVulkanCommand();

					if (_status.m_code != VknStatusCode::SUCCESSFUL) {
						reset();
						return _status.set(VknStatusCode::FAIL, "\n\tWvkKhrSurfaceDispatchTable::loadVulkanCommand() - fail.");
					}

					// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					// Шаг 5. Успешное завершение создания.
					// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					m_valid = true;
					return _status.setOk();
				}

				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				/*!	\brief
				*/
				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				inline void destroy(void) noexcept {}

				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				/*!	\brief
				*/
				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				inline void loadInto(std::vector<WvkVulkanProcedureInfo>& procedures) noexcept {
					for (auto& it : procedures) {
						if (it.name == "vkGetPhysicalDeviceFeatures2") {
							*it.targetPtr = reinterpret_cast<void*>(m_vkGetPhysicalDeviceFeatures2KHR); }
						if (it.name == "vkGetPhysicalDeviceProperties2") {
							*it.targetPtr = reinterpret_cast<void*>(m_vkGetPhysicalDeviceProperties2KHR); }
						if (it.name == "vkGetPhysicalDeviceFormatProperties2") {
							*it.targetPtr = reinterpret_cast<void*>(m_vkGetPhysicalDeviceFormatProperties2KHR); }
						if (it.name == "vkGetPhysicalDeviceImageFormatProperties2") {
							*it.targetPtr = reinterpret_cast<void*>(m_vkGetPhysicalDeviceImageFormatProperties2KHR); }
						if (it.name == "vkGetPhysicalDeviceQueueFamilyProperties2") {
							*it.targetPtr = reinterpret_cast<void*>(m_vkGetPhysicalDeviceQueueFamilyProperties2KHR); }
						if (it.name == "vkGetPhysicalDeviceMemoryProperties2") {
							*it.targetPtr = reinterpret_cast<void*>(m_vkGetPhysicalDeviceMemoryProperties2KHR); }
						if (it.name == "vkGetPhysicalDeviceSparseImageFormatProperties2") {
							*it.targetPtr = reinterpret_cast<void*>(m_vkGetPhysicalDeviceSparseImageFormatProperties2KHR); }
					}
				}

			private:

				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				/*!	\
				*/
				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				inline WvkStatus validationCreateInfo(void) const noexcept {
					WvkStatus _status; 
					return _status.setOk();
				}

				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				/*!	\
				*/
				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				inline WvkStatus loadVulkanCommand(void) noexcept {
					WvkStatus _status;

					// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					// Шаг 1. Подготовка списка процедур, которые необходимо загрузить
					// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					std::vector<WvkVulkanProcedureInfo> _procedures = {
						{ "vkGetPhysicalDeviceFeatures2KHR", reinterpret_cast<void**>(&m_vkGetPhysicalDeviceFeatures2KHR) },
						{ "vkGetPhysicalDeviceProperties2KHR", reinterpret_cast<void**>(&m_vkGetPhysicalDeviceProperties2KHR) },
						{ "vkGetPhysicalDeviceFormatProperties2KHR", reinterpret_cast<void**>(&m_vkGetPhysicalDeviceFormatProperties2KHR) },
						{ "vkGetPhysicalDeviceImageFormatProperties2KHR", reinterpret_cast<void**>(&m_vkGetPhysicalDeviceImageFormatProperties2KHR) },
						{ "vkGetPhysicalDeviceQueueFamilyProperties2KHR", reinterpret_cast<void**>(&m_vkGetPhysicalDeviceQueueFamilyProperties2KHR) },
						{ "vkGetPhysicalDeviceMemoryProperties2KHR", reinterpret_cast<void**>(&m_vkGetPhysicalDeviceMemoryProperties2KHR) },
						{ "vkGetPhysicalDeviceSparseImageFormatProperties2KHR", reinterpret_cast<void**>(&m_vkGetPhysicalDeviceSparseImageFormatProperties2KHR) }
					};

					// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					// Шаг 2. Вызов метода загрузки через обёртку с VkInstance
					// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					_status = m_create_info.wvk_instance->invokeWithVkInstanceMethod(
						&WvkLoader::loadProcedure,
						m_create_info.wvk_loader,
						_procedures
					);

					// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					// Шаг 3. Проверка результата загрузки
					// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					if (!_status) {
						return _status.set(VknStatusCode::FAIL,
							"\n\tWvkInstance::invokeWithVkInstanceMethod - fail.");
					}

					// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					// Шаг 4. Возврат успешного статуса
					// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					return _status.setOk();
				}

				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				/*!	\brief
				*/
				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				inline void reset(void) noexcept {
					m_vkGetPhysicalDeviceFeatures2KHR = VK_NULL_HANDLE;
					m_vkGetPhysicalDeviceProperties2KHR = VK_NULL_HANDLE;
					m_vkGetPhysicalDeviceFormatProperties2KHR = VK_NULL_HANDLE;
					m_vkGetPhysicalDeviceImageFormatProperties2KHR = VK_NULL_HANDLE;
					m_vkGetPhysicalDeviceQueueFamilyProperties2KHR = VK_NULL_HANDLE;
					m_vkGetPhysicalDeviceMemoryProperties2KHR = VK_NULL_HANDLE;
					m_vkGetPhysicalDeviceSparseImageFormatProperties2KHR = VK_NULL_HANDLE;

					m_create_info = {};

					m_valid = false;
				}

			private:

				WvkKhrGetPhysicalDeviceProperties2DTCreateInfo m_create_info = {};
			}; // class WvkKhrGetPhysicalDeviceProperties2DT

		} // namespace Extensions

	} // namespace wvk

} // namespace CGDev

#endif // CGDEV_WVK_SOURCE_EXTENSIONS__WVK_KHR_GET_PHYSICAL_DEVICE_PROPERTIES2_DT_H
