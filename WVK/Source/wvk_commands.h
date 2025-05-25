#ifndef CGDEV_SOURCE_GPU_PRIVATE_VULKAN__VKN_COMMANDS_H
#define CGDEV_SOURCE_GPU_PRIVATE_VULKAN__VKN_COMMANDS_H
////////////////////////////////////////////////////////////////
// секция форвард-декларации
////////////////////////////////////////////////////////////////
#include "_wvk.h"
////////////////////////////////////////////////////////////////
// секция имплементации
////////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////
// секция родительского класса
////////////////////////////////////////////////////////////////
#include "wvk_base_commands.h"
////////////////////////////////////////////////////////////////
// секция для остального
////////////////////////////////////////////////////////////////
#include "wvk_instance.h"

namespace CGDev {

	//namespace GPU {

		//namespace Private {

			namespace wvk {

				struct VknVulkanFunctionInfo {
					const char* name;       ///< Название функции Vulkan, например "vkDestroySurfaceKHR"
					void** targetPtr;       ///< Указатель на переменную, куда будет сохранён загруженный адрес
					VknVulkanFunctionInfo(const char* n, void** ptr)
						: name(n), targetPtr(ptr) {
					}
				};

				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				/*!	\brief
				*/
				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				struct WvkCommandsCreateInfo {
					WvkLoaderPtr				wvk_loader = nullptr;
					WvkInstancePtr				wvk_instance = nullptr;
					VknLogicalDevicePtr			wvk_logical_device = nullptr;
				
				}; // struct WvkCommandsCreateInfo





				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				/*!	\brief
				*/
				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				class WvkGlobalCommands : public VknBaseCommands {

				public:

					WvkStatus tryLoadFunction(std::vector<VknVulkanFunctionInfo> wvk_vulkan_procedure_collection1) const noexcept;
					// Хелпер-функция для получения адреса Vulkan-функции
					//template <typename FuncType>
					//inline WvkStatus loadInstanceProcAddress(FuncType& out_func, const char* func_name) {
					//	WvkStatus _status;

					//	void* ptr = reinterpret_cast<void*>(vknGetInstanceProcAddr(func_name));
					//	out_func = reinterpret_cast<FuncType>(ptr);

					//	if (!out_func) {
					//		_status.m_code = VknStatusCode::FAIL;
					//		_status.message_old = std::string{ "Failed to load function: " } + func_name;
					//		return _status;
					//	}

					//	_status.m_code = VknStatusCode::SUCCESSFUL;
					//	return _status;
					//}

				public:

					inline VkResult vknEnumerateInstanceVersion(uint32_t* pApiVersion) const noexcept;
					inline VkResult vknEnumerateInstanceLayerProperties(uint32_t* pPropertyCount, VkLayerProperties* pProperties) const noexcept;
					inline VkResult vknEnumerateInstanceExtensionProperties(const char* pLayerName, uint32_t* pPropertyCount, VkExtensionProperties* pProperties) const noexcept;
					inline VkResult vknCreateInstance(const VkInstanceCreateInfo* pCreateInfo, const VkAllocationCallbacks* pAllocator, VkInstance* pInstance) const noexcept;

					inline PFN_vkVoidFunction vknGetInstanceProcAddr(const char* pName) const noexcept;
					inline void vknDestroyInstance(const VkAllocationCallbacks* pAllocator) const noexcept;

					inline VkResult vknEnumeratePhysicalDevices(uint32_t* pPhysicalDeviceCount, VkPhysicalDevice* pPhysicalDevices) const noexcept;
					inline void vknGetPhysicalDeviceProperties(VkPhysicalDevice physicalDevice, VkPhysicalDeviceProperties* pProperties) const noexcept;
					inline void vknGetPhysicalDeviceQueueFamilyProperties(VkPhysicalDevice physicalDevice, uint32_t* pQueueFamilyPropertyCount, VkQueueFamilyProperties* pQueueFamilyProperties) const noexcept;
					
					inline VkResult vknEnumeratePhysicalDeviceGroups(uint32_t* pPhysicalDeviceGroupCount, VkPhysicalDeviceGroupProperties* pPhysicalDeviceGroupProperties) const noexcept;
					inline void vknGetPhysicalDeviceProperties2(VkPhysicalDevice physicalDevice, VkPhysicalDeviceProperties2* pProperties) const noexcept;
					inline void vknGetPhysicalDeviceQueueFamilyProperties2(VkPhysicalDevice physicalDevice, uint32_t* pQueueFamilyPropertyCount, VkQueueFamilyProperties2* pQueueFamilyProperties) const noexcept;

					inline VkResult vknCreateDevice(VkPhysicalDevice vk_physical_device, const VkDeviceCreateInfo* pCreateInfo, const VkAllocationCallbacks* pAllocator, VkDevice* pDevice) const noexcept;	
					inline void vknDestroyDevice(VkDevice device, const VkAllocationCallbacks* pAllocator) const noexcept;
					inline PFN_vkVoidFunction vknGetDeviceProcAddr(VkDevice device, const char* pName) const noexcept;

					inline VkResult vknCreateCommandPool(VkDevice device, const VkCommandPoolCreateInfo* pCreateInfo, const VkAllocationCallbacks* pAllocator, VkCommandPool* pCommandPool) const noexcept;
					inline void vknDestroyCommandPool(VkDevice device, VkCommandPool commandPool, const VkAllocationCallbacks* pAllocator) const noexcept;

					inline VkResult vknAllocateCommandBuffers(VkDevice device, const VkCommandBufferAllocateInfo* pAllocateInfo, VkCommandBuffer* pCommandBuffers) const noexcept;
					inline void vknFreeCommandBuffers(VkDevice device, VkCommandPool commandPool, uint32_t commandBufferCount, const VkCommandBuffer* pCommandBuffers) const noexcept;

					inline void vknCmdBeginRenderPass(VkCommandBuffer commandBuffer, const VkRenderPassBeginInfo* pRenderPassBegin, VkSubpassContents contents) const noexcept;
					inline void vknCmdEndRenderPass(VkCommandBuffer commandBuffer) const noexcept;

					inline void vknCmdBeginRendering(VkCommandBuffer commandBuffer, const VkRenderingInfo* pRenderingInfo) const noexcept;
					inline void vknCmdEndRendering(VkCommandBuffer commandBuffer) const noexcept;
					
					// ~~~~~~~~~~~~~~~~
					// VkSurface. 1.1
					// ~~~~~~~~~~~~~~~~
					inline VkResult wvkGetPhysicalDeviceSurfaceCapabilities2KHR(VkPhysicalDevice physicalDevice, const VkPhysicalDeviceSurfaceInfo2KHR* pSurfaceInfo, VkSurfaceCapabilities2KHR* pSurfaceCapabilities) const noexcept;
					inline VkResult wvkGetPhysicalDeviceSurfaceFormats2KHR(VkPhysicalDevice physicalDevice, const VkPhysicalDeviceSurfaceInfo2KHR* pSurfaceInfo, uint32_t* pSurfaceFormatCount, VkSurfaceFormat2KHR* pSurfaceFormats) const noexcept;

				private:

					// ~~~~~~~~~~~~~~~~
					// 
					// ~~~~~~~~~~~~~~~~

					PFN_vkGetInstanceProcAddr													m_vkGetInstanceProcAddr = nullptr;

					// ~~~~~~~~~~~~~~~~
					// VkInstance
					// 1.0
					// ~~~~~~~~~~~~~~~~					

					PFN_vkEnumerateInstanceLayerProperties										m_vkEnumerateInstanceLayerProperties = nullptr;
					PFN_vkEnumerateInstanceExtensionProperties									m_vkEnumerateInstanceExtensionProperties = nullptr;
					PFN_vkCreateInstance														m_vkCreateInstance = nullptr;
					PFN_vkDestroyInstance														m_vkDestroyInstance = nullptr;

					// ~~~~~~~~~~~~~~~~
					// VkInstance
					// 1.1
					// ~~~~~~~~~~~~~~~~

					PFN_vkEnumerateInstanceVersion												m_vkEnumerateInstanceVersion = nullptr;

					// ~~~~~~~~~~~~~~~~
					// VkPhysicalDevice
					// 1.0
					// ~~~~~~~~~~~~~~~~

					PFN_vkEnumeratePhysicalDevices												m_vkEnumeratePhysicalDevices = nullptr;
					PFN_vkGetPhysicalDeviceProperties											m_vkGetPhysicalDeviceProperties = nullptr;
					PFN_vkGetPhysicalDeviceQueueFamilyProperties								m_vkGetPhysicalDeviceQueueFamilyProperties = nullptr;

					// ~~~~~~~~~~~~~~~~
					// VkPhysicalDevice
					// 1.1
					// ~~~~~~~~~~~~~~~~

					PFN_vkEnumeratePhysicalDeviceGroups											m_vkEnumeratePhysicalDeviceGroups = nullptr;
					PFN_vkGetPhysicalDeviceProperties2											m_vkGetPhysicalDeviceProperties2 = nullptr;
					PFN_vkGetPhysicalDeviceQueueFamilyProperties2								m_vkGetPhysicalDeviceQueueFamilyProperties2 = nullptr;

					// ~~~~~~~~~~~~~~~~
					// VkDevice
					// 1.0
					// ~~~~~~~~~~~~~~~~

					PFN_vkCreateDevice															m_vkCreateDevice = nullptr;
					PFN_vkDestroyDevice															m_vkDestroyDevice = nullptr;
					PFN_vkGetDeviceProcAddr														m_vkGetDeviceProcAddr = nullptr;

					// ~~~~~~~~~~~~~~~~
					// vkCommandPool
					// 1.0
					// ~~~~~~~~~~~~~~~~

					PFN_vkCreateCommandPool														m_vkCreateCommandPool = nullptr;
					PFN_vkDestroyCommandPool													m_vkDestroyCommandPool = nullptr;
					PFN_vkAllocateCommandBuffers												m_vkAllocateCommandBuffers = nullptr;
					PFN_vkFreeCommandBuffers													m_vkFreeCommandBuffers = nullptr;

					// ~~~~~~~~~~~~~~~~
					// vkCommandBuffer
					// 1.0
					// ~~~~~~~~~~~~~~~~

					PFN_vkCmdBeginRenderPass													m_vkCmdBeginRenderPass = nullptr;
					PFN_vkCmdEndRenderPass														m_vkCmdEndRenderPass = nullptr;

					// ~~~~~~~~~~~~~~~~
					// vkCommandBuffer
					// 1.3
					// ~~~~~~~~~~~~~~~~

					PFN_vkCmdBeginRendering														m_vkCmdBeginRendering = nullptr;
					PFN_vkCmdEndRendering														m_vkCmdEndRendering = nullptr;

					// ~~~~~~~~~~~~~~~~
					// VkSurface. 1.1
					// ~~~~~~~~~~~~~~~~
					PFN_vkGetPhysicalDeviceSurfaceCapabilities2KHR m_vkGetPhysicalDeviceSurfaceCapabilities2KHR = VK_NULL_HANDLE;
					PFN_vkGetPhysicalDeviceSurfaceFormats2KHR m_vkGetPhysicalDeviceSurfaceFormats2KHR = VK_NULL_HANDLE;

				public:

					//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					/*!	\brief
					*/
					//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					WvkGlobalCommands(void) noexcept;

					//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					/*!	\brief
					*/
					//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					~WvkGlobalCommands(void) noexcept;

					//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					/*!	\brief
					*/
					//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					WvkStatus create(const WvkCommandsCreateInfo* create_info) noexcept;

					//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					/*!	\brief
					*/
					//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					void destroy(void) noexcept;

				public:

					//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					/*!	\brief
					*/
					//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					inline const WvkCommandsCreateInfo& getCreateInfo(void) const noexcept {
						return m_create_info;
					}

					//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					/*!	\brief
					*/
					//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					inline const VkInstance& getVkInstance(void) const noexcept {
						return m_create_info.wvk_instance->getVkInstance();
					}

				public:

					template<typename Method, typename Object, typename... Args>
					inline std::enable_if_t<
						std::is_invocable_v<Method, Object, VkPhysicalDevice, Args...>&&
						std::is_void_v<std::invoke_result_t<Method, Object, VkPhysicalDevice, Args...>>,
						void
					>
						invokeWithVkInstance(Method&& method, Object&& object, Args&&... args) {
						// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
						// Шаг 1. Вызов переданного метода (указателя или лямбды),
						//        подставляя объект, VkPhysicalDevice и остальные аргументы
						// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
						std::invoke(std::forward<Method>(method),
							std::forward<Object>(object),
							m_create_info.wvk_instance->getVkInstance(),
							std::forward<Args>(args)...);
					}

					//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

					template<typename Method, typename Object, typename... Args>
					inline std::enable_if_t<
						std::is_invocable_v<Method, Object, VkPhysicalDevice, Args...> &&
						!std::is_void_v<std::invoke_result_t<Method, Object, VkPhysicalDevice, Args...>>,
						std::invoke_result_t<Method, Object, VkPhysicalDevice, Args...>
					>
						invokeWithVkInstance(Method&& method, Object&& object, Args&&... args) {
						return std::invoke(std::forward<Method>(method),
							std::forward<Object>(object),
							m_create_info.wvk_instance->getVkInstance(),
							std::forward<Args>(args)...);
					}

					template<typename Method, typename Object, typename... Args>
					inline std::enable_if_t<
						!std::is_invocable_v<Method, Object, VkPhysicalDevice, Args...>,
						void
					>
						invokeWithVkInstance(Method&&, Object&&, Args&&...) {
						static_assert(dependent_false <Method>::value,
							"\n[WVK Error] Метод невозможно вызвать как method(object, VkPhysicalDevice, ...)\n");
					}

				private:

					//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					/*!	\brief
					*/
					//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					template<typename Command>
					inline void reinterpreted(Command& command, const std::string& name_command, const VkInstance& vk_instance) const noexcept;

				private:

					//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					/*!	\brief
					*/
					//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					WvkStatus validationCreateInfo(const WvkCommandsCreateInfo* create_info) const noexcept;

					//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					/*!	\brief
					*/
					//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					WvkStatus loadVulkanCommandLoaderLevel(void) noexcept;

					//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					/*!	\brief
					*/
					//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					WvkStatus loadVulkanCommandInstanceLevel(void) noexcept;
				
				private:

					WvkCommandsCreateInfo					m_create_info = {};
					
				}; // class WvkGlobalCommands

			} // namespace wvk

		//} // namespace Private

	//} // namespace GPU

} // namespace CGDev

#include "wvk_commands.hpp"

#endif // CGDEV_SOURCE_GPU_PRIVATE_VULKAN__VKN_COMMANDS_H
