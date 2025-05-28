#ifndef CGDEV_SOURCE_GPU_PRIVATE_VULKAN___VULKAN_H
#define CGDEV_SOURCE_GPU_PRIVATE_VULKAN___VULKAN_H
////////////////////////////////////////////////////////////////
// секция форвард-декларации
////////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////
// секция имплементации
////////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////
// секция родительского класса
////////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////
// секция для остального
////////////////////////////////////////////////////////////////
#include <vector>
#include <memory>
#include <string>
#include <array>
#include <optional>
#include <cstdarg>
#include <functional>
#include <algorithm>
#include <type_traits>
#include "wvk_status.h"
#include "vulkan/vulkan.h"

template<typename>
struct dependent_false : std::false_type {};

// Шаблон для включения битовых операций только для enum'ов, помеченных как поддерживающих флаги
template<typename E>
struct EnableEnumFlags {
	static constexpr bool value = false;
};

// Макрос для удобства активации флагов для нужного enum
#define ENABLE_ENUM_FLAGS(EnumType)                            \
	namespace CGDev::GPU::Private::Vulkan {                    \
		template<>                                              \
		struct EnableEnumFlags<EnumType> {                     \
			static constexpr bool value = true;                \
		};                                                      \
	}

// Основные битовые операторы
template<typename E>
constexpr typename std::enable_if<EnableEnumFlags<E>::value, E>::type
operator|(E lhs, E rhs) {
	using underlying = typename std::underlying_type<E>::type;
	return static_cast<E>(
		static_cast<underlying>(lhs) | static_cast<underlying>(rhs)
		);
}

template<typename E>
constexpr typename std::enable_if<EnableEnumFlags<E>::value, E>::type
operator&(E lhs, E rhs) {
	using underlying = typename std::underlying_type<E>::type;
	return static_cast<E>(
		static_cast<underlying>(lhs) & static_cast<underlying>(rhs)
		);
}

template<typename E>
constexpr typename std::enable_if<EnableEnumFlags<E>::value, E>::type
operator~(E value) {
	using underlying = typename std::underlying_type<E>::type;
	return static_cast<E>(
		~static_cast<underlying>(value)
		);
}

template<typename E>
constexpr typename std::enable_if<EnableEnumFlags<E>::value, E&>::type
operator|=(E& lhs, E rhs) {
	lhs = lhs | rhs;
	return lhs;
}

template<typename E>
constexpr typename std::enable_if<EnableEnumFlags<E>::value, E&>::type
operator&=(E& lhs, E rhs) {
	lhs = lhs & rhs;
	return lhs;
}

template<typename E>
constexpr typename std::enable_if<EnableEnumFlags<E>::value, bool>::type
has_flag(E value, E flag) {
	return (value & flag) == flag;
}

namespace CGDev {

	//namespace GPU {

		//namespace Private {

			namespace wvk {

				namespace Build {

					//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					/*!	\brief
					*/
					//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					enum class VulkanVersion {

						UNKNOWN = 0,

						VERSION_10 = VK_MAKE_API_VERSION(0, 0, 0, 0),
						VERSION_11 = VK_MAKE_API_VERSION(0, 1, 1, 0),
						VERSION_12 = VK_MAKE_API_VERSION(0, 1, 2, 0),
						VERSION_13 = VK_MAKE_API_VERSION(0, 1, 3, 0),

					}; // enum class VulkanVersion

					//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					/*!	\brief
					*/
					//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					enum class PlatformType {

						UNKNOWN = 0,

						MSWindows,
						Linux
					}; // enum class PlatformType

					enum Features {

						UNKNOWN = 0,

						DEBUG						= 1,
						DYNAMIC_RENDERING			= 2

					}; // enum class Features {

					//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					/*!	\brief
					*/
					//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					static const VulkanVersion vulkan_version = VulkanVersion::VERSION_13;

					//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					/*!	\brief
					*/
					//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					extern const std::array<std::string_view, 1> layer_name_collection;			

					//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					/*!	\brief
					*/
					//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					extern const std::vector<std::string> extension_name_collection;

					//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					/*!	\brief
					*/
					//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					//static const bool s_wvk_debug = []() { if constexpr (Build::type_build == Private::Build::TypeBuild::DEBUG) return true; else return false; }();
					static const bool s_wvk_debug = true;

					//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					/*!	\brief Информация о сборке Vulkan-проекта.
					*
					* Структура `WvkBuildInfo` содержит статические параметры, определяющие характеристики текущей сборки.
					* Она предоставляет информацию о версии Vulkan SDK, целевой версии Vulkan API и платформе, под которую производится сборка.
					*
					* Все поля являются `constexpr` и задаются на этапе компиляции.
					*
					* @note Эта структура может использоваться для условий компиляции или логирования в отладочных целях.
					*
					* @code
					* // Пример использования WvkBuildInfo
					* if constexpr (WvkBuildInfo::enable) {
					*     std::cout << "Сборка разрешена для платформы "
					*               << static_cast<int>(WvkBuildInfo::platform_type) << std::endl;
					* }
					* @endcode
					*/
					//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					struct WvkBuildInfo {
						static constexpr bool enable = true;
						static constexpr uint32_t vulkan_sdk_version = VK_HEADER_VERSION;
						static constexpr VulkanVersion vulkan_api_version = VulkanVersion::VERSION_10;
						static constexpr PlatformType platform_type = PlatformType::MSWindows;

						static constexpr bool find(std::string_view value) {
							for (std::string_view item : extensions) {
								if (item == value) {
									return true;
								}
							}
							return false;
						}

					private:
						static constexpr std::array<std::string_view, 1> extensions = {
							"VK_KHR_get_physical_device_properties2"
						};
					};

					//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					/*!	\brief
					*/
					//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					struct ValidationBuildInfo {

						static constexpr const bool enable = true;
						static constexpr std::array<std::string_view, 1> layer_name_collection = { "VK_LAYER_KHRONOS_validation" };
						static constexpr std::array<std::string_view, 1> extension_name_collection = { "VK_EXT_debug_utils" };

					};

					//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					/*!	\brief
					*/
					//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					struct SurfaceBuildInfo {

						static constexpr const bool enable = true;
						static constexpr std::array<std::string_view, 0> layer_name_collection = {};
						static constexpr std::array<std::string_view, 2> extension_name_collection = { "VK_KHR_surface", "VK_KHR_win32_surface" };
						static constexpr PlatformType platform_type = WvkBuildInfo::platform_type;
					};

					//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					/*!	\brief
					*/
					//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					static const bool s_vk_ext_debug_utils = []() { if constexpr (s_wvk_debug == true) return true; else return false; }();

					//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					/*!	\brief
					*/
					//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					//static const bool s_dynamic_rendering = false;
					
					//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					/*!	\brief
					*/
					//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					//constexpr auto get_layer_names() noexcept {

						//constexpr Features _features = Features::DEBUG | Features::DYNAMIC_RENDERING;

						//constexpr uint32_t countBase = 0;

						//constexpr uint32_t _count = countBase
						//	+ ((_features & Features::DEBUG) ? 1 : 0)
						//	+ ((_features & Features::DYNAMIC_RENDERING) ? 1 : 0);

						//std::array<std::string, _count>{};

						//return std::array<int, _count>{};
						//return std::array<std::string_view, 2>{"a", "b"};
						//return std::array<std::string, 1>{
					//	//	layer_name_collection[0]};
					//};

				} //namespace Build





				



				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				/*!	\brief
				*/
				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~				

				using VkLayerPropertiesArr														= std::vector<VkLayerProperties>;
				using VkExtensionPropertiesArr													= std::vector<VkExtensionProperties>;

				using VkPhysicalDeviceArr1														= std::vector<VkPhysicalDevice>;
				using VkPhysicalDeviceArr2														= std::vector<VkPhysicalDeviceArr1>;
				using VkPhysicalDeviceGroupPropertiesArr										= std::vector<VkPhysicalDeviceGroupProperties>;

				using VkQueueFamilyPropertiesArr1												= std::vector<VkQueueFamilyProperties>;
				using VkQueueFamilyProperties2Arr1												= std::vector<VkQueueFamilyProperties2>;
				using VkQueueFamilyGlobalPriorityPropertiesKHRArr1								= std::vector<VkQueueFamilyGlobalPriorityPropertiesKHR>;
				using VkQueueFamilyQueryResultStatusPropertiesKHRArr1							= std::vector<VkQueueFamilyQueryResultStatusPropertiesKHR>;
				using VkQueueFamilyVideoPropertiesKHRArr1										= std::vector<VkQueueFamilyVideoPropertiesKHR>;
				
				using VkDeviceQueueCreateInfoArr1												= std::vector<VkDeviceQueueCreateInfo>;

				using VkCommandBufferArr														= std::vector<VkCommandBuffer>;

				struct VknVulkanFunctionInfo;
				using VknVulkanFunctionInfoArr1 = std::vector<VknVulkanFunctionInfo>;
				using VknVulkanFunctionInfoArr2 = std::vector<VknVulkanFunctionInfoArr1>;
				using VknVulkanFunctionInfoPtr = VknVulkanFunctionInfo * ;
				using VknVulkanFunctionInfoPtrArr1 = std::vector<VknVulkanFunctionInfoPtr>;
				using VknVulkanFunctionInfoPtrArr2 = std::vector<VknVulkanFunctionInfoPtrArr1>;

				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				/*!	\brief
				*/
				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

				class			WvkGlobalCommands;
				using			WvkCommandsPtr													= WvkGlobalCommands * ;
				using			WvkCommandsPtrArr1												= std::vector<WvkCommandsPtr>;
				using			WvkCommandsWptr													= std::weak_ptr<WvkGlobalCommands>;
				using			WvkCommandsWptrArr1												= std::vector<WvkCommandsWptr>;
				using			WvkCommandsSptr													= std::shared_ptr<WvkGlobalCommands>;
				using			WvkCommandsSptrArr1												= std::vector<WvkCommandsSptr>;

				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				/*!	\brief
				*/
				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

				class			VknLayer;
				using			VknLayerPtr												= VknLayer * ;
				using			VknLayerPtrArr											= std::vector<VknLayerPtr>;
				using			VknLayerWptr											= std::weak_ptr<VknLayer>;
				using			VknLayerWptrArr											= std::vector<VknLayerWptr>;
				using			VknLayerSptr											= std::shared_ptr<VknLayer>;
				using			VknLayerSptrArr											= std::vector<VknLayerSptr>;

				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				/*!	\brief
				*/
				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

				class			VknExtension;
				using			VknExtensionPtr											= VknExtension * ;
				using			VknExtensionPtrArr										= std::vector<VknExtensionPtr>;
				using			VknExtensionWptr										= std::weak_ptr<VknExtension>;
				using			VknExtensionWptrArr										= std::vector<VknExtensionWptr>;
				using			VknExtensionSptr										= std::shared_ptr<VknExtension>;
				using			VknExtensionSptrArr										= std::vector<VknExtensionSptr>;

				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				/*!	\brief
				*/
				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

				class WvkLoader;
				using WvkLoaderPtr = WvkLoader * ;
				using WvkLoaderPtrArr1 = std::vector<WvkLoaderPtr>;
				using WvkLoaderSptr = std::shared_ptr<WvkLoader>;
				using WvkLoaderSptrArr1 = std::vector<WvkLoaderSptr>;
				using WvkLoaderUptr = std::unique_ptr<WvkLoader>;
				using WvkLoaderUptrArr1 = std::vector<WvkLoaderUptr>;
				using WvkLoaderWptr = std::weak_ptr<WvkLoader>;
				using WvkLoaderWptrArr1 = std::vector<WvkLoaderWptr>;

				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				/*!	\brief
				*/
				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

				class WvkLoaderDispatchTable;
				using WvkLoaderDispatchTablePtr = WvkLoaderDispatchTable * ;
				using WvkLoaderDispatchTablePtrArr1 = std::vector<WvkLoaderDispatchTablePtr>;
				using WvkLoaderDispatchTableSptr = std::shared_ptr<WvkLoaderDispatchTable>;
				using WvkLoaderDispatchTableSptrArr1 = std::vector<WvkLoaderDispatchTableSptr>;
				using WvkLoaderDispatchTableUptr = std::unique_ptr<WvkLoaderDispatchTable>;
				using WvkLoaderDispatchTableUptrArr1 = std::vector<WvkLoaderDispatchTableUptr>;
				using WvkLoaderDispatchTableWptr = std::weak_ptr<WvkLoaderDispatchTable>;
				using WvkLoaderDispatchTableWptrArr1 = std::vector<WvkLoaderDispatchTableWptr>;

				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				/*!	\brief
				*/
				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

				class WvkInstance;
				using WvkInstancePtr = WvkInstance * ;
				using WvkInstancePtrArr1 = std::vector<WvkInstancePtr>;
				using WvkInstanceSptr = std::shared_ptr<WvkInstance>;
				using WvkInstanceSptrArr1 = std::vector<WvkInstanceSptr>;
				using WvkInstanceUptr = std::unique_ptr<WvkInstance>;
				using WvkInstanceUptrArr1 = std::vector<WvkInstanceUptr>;
				using WvkInstanceWptr = std::weak_ptr<WvkInstance>;
				using WvkInstanceWptrArr1 = std::vector<WvkInstanceWptr>;	

				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				/*!	\brief
				*/
				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

				class WvkInstanceDt;
				using WvkInstanceDtPtr = WvkInstanceDt * ;
				using WvkInstanceDtPtrArr1 = std::vector<WvkInstanceDtPtr>;
				using WvkInstanceDtSptr = std::shared_ptr<WvkInstanceDt>;
				using WvkInstanceDtSptrArr1 = std::vector<WvkInstanceDtSptr>;
				using WvkInstanceDtUptr = std::unique_ptr<WvkInstanceDt>;
				using WvkInstanceDtUptrArr1 = std::vector<WvkInstanceDtUptr>;
				using WvkInstanceDtWptr = std::weak_ptr<WvkInstanceDt>;
				using WvkInstanceDtWptrArr1 = std::vector<WvkInstanceDtWptr>;

				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				/*!	\brief
				*/
				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

				class WvkPhysicalDevice;
				using WvkPhysicalDevicePtr = WvkPhysicalDevice * ;
				using WvkPhysicalDevicePtrArr1 = std::vector<WvkPhysicalDevicePtr>;
				using WvkPhysicalDevicePtrArr2 = std::vector<WvkPhysicalDevicePtrArr1>;
				using WvkPhysicalDeviceSptr = std::shared_ptr<WvkPhysicalDevice>;
				using WvkPhysicalDeviceSptrArr1 = std::vector<WvkPhysicalDeviceSptr>;
				using WvkPhysicalDeviceSptrArr2 = std::vector<WvkPhysicalDeviceSptrArr1>;
				using WvkPhysicalDeviceUptr = std::unique_ptr<WvkPhysicalDevice>;
				using WvkPhysicalDeviceUptrArr1 = std::vector<WvkPhysicalDeviceUptr>;
				using WvkPhysicalDeviceUptrArr2 = std::vector<WvkPhysicalDeviceUptrArr1>;
				using WvkPhysicalDeviceWptr = std::weak_ptr<WvkPhysicalDevice>;
				using WvkPhysicalDeviceWptrArr1 = std::vector<WvkPhysicalDeviceWptr>;
				

				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				/*!	\brief
				*/
				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

				class WvkQueueFamily;
				using WvkQueueFamilyArr1 = std::vector<WvkQueueFamily>;
				using WvkQueueFamilyPtr = WvkQueueFamily * ;
				using WvkQueueFamilyPtrArr1 = std::vector<WvkQueueFamilyPtr>;
				using WvkQueueFamilySptr = std::shared_ptr<WvkQueueFamily>;
				using WvkQueueFamilySptrArr1 = std::vector<WvkQueueFamilySptr>;
				using WvkQueueFamilyUptr = std::unique_ptr<WvkQueueFamily>;
				using WvkQueueFamilyUptrArr1 = std::vector<WvkQueueFamilyUptr>;
				using WvkQueueFamilyWptr = std::weak_ptr<WvkQueueFamily>;
				using WvkQueueFamilyWptrArr1 = std::vector<WvkQueueFamilyWptr>;
				

				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				/*!	\brief
				*/
				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

				class			VknLogicalDevice;
				using			VknLogicalDevicePtr										= VknLogicalDevice * ;
				using			VknLogicalDevicePtrArr									= std::vector<VknLogicalDevicePtr>;
				using			VknLogicalDeviceWptr									= std::weak_ptr<VknLogicalDevice>;
				using			VknLogicalDeviceWptrArr									= std::vector<VknLogicalDeviceWptr>;
				using			VknLogicalDeviceSptr									= std::shared_ptr<VknLogicalDevice>;
				using			VknLogicalDeviceSptrArr									= std::vector<VknLogicalDeviceSptr>;
				struct			VknLogicalDeviceCreateInfo;
				struct			VknLogicalDeviceQueueCreateInfo;
				using			VknLogicalDeviceQueueCreateInfoArr						= std::vector<VknLogicalDeviceQueueCreateInfo>;

				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				/*!	\brief
				*/
				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

				class			VknQueue;
				using			VknQueuePtr												= VknQueue * ;
				using			VknQueuePtrArr1											= std::vector<VknQueuePtr>;
				using			VknQueueWptr											= std::weak_ptr<VknQueue>;
				using			VknQueueWptrArr1										= std::vector<VknQueueWptr>;
				using			VknQueueSptr											= std::shared_ptr<VknQueue>;
				using			VknQueueSptrArr1										= std::vector<VknQueueSptr>;
				struct			VknQueueCreateInfo;
				struct			VknQueueQueueCreateInfo;
				using			VknQueueQueueCreateInfoArr1								= std::vector<VknQueueQueueCreateInfo>;

				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				/*!	\brief
				*/
				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

				class			VknCommandPool;
				using			VknCommandPoolPtr										= VknCommandPool * ;
				using			VknCommandPoolPtrArr									= std::vector<VknCommandPoolPtr>;
				using			VknCommandPoolWptr										= std::weak_ptr<VknCommandPool>;
				using			VknCommandPoolWptrArr									= std::vector<VknCommandPoolWptr>;
				using			VknCommandPoolSptr										= std::shared_ptr<VknCommandPool>;
				using			VknCommandPoolSptrArr									= std::vector<VknCommandPoolSptr>;
				struct			VknCommandPoolCreateInfo;

				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				/*!	\brief
				*/
				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

				class			VknCommandBuffer;
				using			VknCommandBufferPtr										= VknCommandBuffer * ;
				using			VknCommandBufferPtrArr									= std::vector<VknCommandBufferPtr>;
				using			VknCommandBufferWptr									= std::weak_ptr<VknCommandBuffer>;
				using			VknCommandBufferWptrArr									= std::vector<VknCommandBufferWptr>;
				using			VknCommandBufferSptr									= std::shared_ptr<VknCommandBuffer>;
				using			VknCommandBufferSptrArr									= std::vector<VknCommandBufferSptr>;





				// ~~~~~~~~~~~~~~~~
				// отладочные средства
				// ~~~~~~~~~~~~~~~~
					
				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				/*!	\brief
				*/
				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~	

				#define VKN_CHECK(vk_command, log)\
				if(const auto& _vk_result = vk_command; _vk_result != VK_SUCCESS) {\
					\
					std::string _cause = "";\
					\
					if (_vk_result == VK_ERROR_OUT_OF_HOST_MEMORY) _cause = "VK_ERROR_OUT_OF_HOST_MEMORY";\
					if (_vk_result == VK_ERROR_OUT_OF_DEVICE_MEMORY) _cause = "VK_ERROR_OUT_OF_DEVICE_MEMORY";\
					if (_vk_result == VK_ERROR_INITIALIZATION_FAILED) _cause = "VK_ERROR_INITIALIZATION_FAILED";\
					\
					Tools::addEntry(log, Tools::LogEntryError(std::move(_cause)));\
					\
					return false;\
					\
				} // if(vk_result != VK_SUCCESS)

			} // namespace wvk

		//} // namespace Private

	//} // namespace GPU

} // namespace CGDev

#endif // CGDEV_SOURCE_GPU_PRIVATE_VULKAN___VULKAN_H