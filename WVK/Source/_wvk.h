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
#include <span>
#include <iterator>
#include <unordered_set>
#include <unordered_map>
#include <typeindex>
#include <any>
#include <stdexcept>
#include "wvk_status.h"
#include "vulkan/vulkan.h"
#include <ranges>


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

	namespace wvk {

		namespace Build {

			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			/*!	\brief
			*/
			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			enum class VulkanVersion {

				UNKNOWN = 0,

				VERSION_10 = VK_MAKE_API_VERSION(0, 1, 0, 0),
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

#define WVK_PLATFORM_MSWINDOWS 1
#define WVK_PLATFORM_LINUX 2
#define WVK_PLATFORM_ANDROID 2

#define WVK_PLATFORM WVK_PLATFORM_MSWINDOWS

#define WVK_VULKAN_API_VERSION_10 10
#define WVK_VULKAN_API_VERSION_11 11
#define WVK_VULKAN_API_VERSION_12 12
#define WVK_VULKAN_API_VERSION_13 13
#define WVK_VULKAN_API_VERSION_14 14

#define WVK_VULKAN_API_VERSION WVK_VULKAN_API_VERSION_10

#define VK_API_VERSION_FROM_WVK(ver) VK_API_VERSION_FROM_WVK_IMPL(ver)
#define VK_API_VERSION_FROM_WVK_IMPL(ver) VK_API_VERSION_##ver

#define VK_API_VERSION_10 VK_API_VERSION_1_0
#define VK_API_VERSION_11 VK_API_VERSION_1_1
#define VK_API_VERSION_12 VK_API_VERSION_1_2
#define VK_API_VERSION_13 VK_API_VERSION_1_3

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
//
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

#define WVK_DISABLE 0
#define WVK_ENABLE 1

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
//
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
#define WVK_LAYER_KHRONOS_validation WVK_ENABLE
	#if WVK_LAYER_KHRONOS_validation == WVK_ENABLE
	#define WVK_LAYER_KHRONOS_validation_name \
			X("VK_LAYER_KHRONOS_validation")
	#else
	#define WVK_LAYER_KHRONOS_validation_name
	#endif

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
//
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

#define WVK_KHR_get_physical_device_properties2 WVK_ENABLE
#if WVK_KHR_get_physical_device_properties2 == WVK_ENABLE
#define WVK_KHR_get_physical_device_properties2_name \
		X("VK_KHR_get_physical_device_properties2")
#else
#define WVK_KHR_get_physical_device_properties2_name
#endif

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
//
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
#define WVK_KHR_get_surface_capabilities2 WVK_ENABLE
#if WVK_KHR_get_surface_capabilities2 == WVK_ENABLE
#define WVK_KHR_get_surface_capabilities2_name \
		X("VK_KHR_get_surface_capabilities2")
#else
#define WVK_KHR_get_surface_capabilities2_name
#endif

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
//
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
#define WVK_KHR_device_group_creation WVK_DISABLE
#if WVK_KHR_device_group_creation == WVK_ENABLE
#define WVK_KHR_device_group_creation_name \
		X("VK_KHR_device_group_creation")
#else
#define WVK_KHR_device_group_creation_name
#endif

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
//
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
#define WVK_KHR_maintenance1 WVK_ENABLE
#if WVK_KHR_maintenance1 == WVK_ENABLE
#define WVK_KHR_maintenance1_name \
		X("VK_KHR_maintenance1")
#else
#define WVK_KHR_maintenance1_name
#endif
			

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
//
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
#define WVK_EXT_debug_utils WVK_ENABLE
	#if WVK_EXT_debug_utils == WVK_ENABLE
		#define WVK_EXT_debug_utils_name \
		X("VK_EXT_debug_utils")
	#else
		#define WVK_EXT_debug_utils_name
	#endif

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
//
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
#define WVK_COMPILE_TIME_LAYERS \
	WVK_LAYER_KHRONOS_validation_name

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
//
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
#define WVK_COMPILE_TIME_INSTANCE_EXTENSIONS \
	WVK_EXT_debug_utils_name \
	WVK_KHR_get_physical_device_properties2_name \
	X("VK_KHR_surface") \
	WVK_KHR_get_surface_capabilities2_name \
	/*X("VK_KHR_surface_protected_capabilities")*/\
	X("VK_KHR_win32_surface") \
	WVK_KHR_device_group_creation_name

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
//
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
#define WVK_COMPILE_TIME_LOGICAL_DEVICE_EXTENSIONS \
	WVK_KHR_maintenance1_name



			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			/*!	\brief
			*/
			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			//static const bool s_wvk_debug = []() { if constexpr (Build::type_build == Private::Build::TypeBuild::DEBUG) return true; else return false; }();
			static const bool s_wvk_debug = true;

			static constexpr bool enable = true;
			static constexpr uint32_t vulkan_sdk_version = VK_HEADER_VERSION;
			static constexpr VulkanVersion vulkan_api_version = VulkanVersion::VERSION_10;
			static constexpr PlatformType platform_type = PlatformType::MSWindows;
			static constexpr std::array<const char*, 1> vulkan_compile_time_layers = {

				"VK_LAYER_KHRONOS_validation"						
			};
			static constexpr std::array<const char*, 7> vulkan_compile_time_extensions = {
				"VK_EXT_debug_utils",
				"VK_KHR_get_physical_device_properties2",
				"VK_KHR_surface",
				"VK_KHR_get_surface_capabilities2",
				"VK_KHR_surface_protected_capabilities",
				"VK_KHR_win32_surface",
				"VK_KHR_device_group_creation"
			};
			static constexpr std::span<const char* const> getInstanceExtensions(void) {
				return vulkan_compile_time_extensions;
			}
			static constexpr bool find(std::string_view value) {
				for (std::string_view item : vulkan_compile_time_extensions) {
					if (item == value) {
						return true;
					}
				}
				return false;
			}

					

			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			/*!	\brief
			*/
			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			struct SurfaceBuildInfo {

				static constexpr const bool enable = true;
				static constexpr std::array<std::string_view, 0> layer_name_collection = {};
				static constexpr std::array<std::string_view, 2> extension_name_collection = { "VK_KHR_surface", "VK_KHR_win32_surface" };
				static constexpr PlatformType platform_type = platform_type;
			};

			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			/*!	\brief
			*/
			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			static const bool s_vk_ext_debug_utils = []() { if constexpr (s_wvk_debug == true) return true; else return false; }();

		} //namespace Build





				
		using VkLayerPropertiesArr = std::vector<VkLayerProperties>;
		using VkExtensionPropertiesArr = std::vector<VkExtensionProperties>;

		using VkBaseInStructureArr1 = std::vector< VkBaseInStructure>;
		using VkBaseOutStructureArr1 = std::vector<VkBaseOutStructure>;
						
				
		using VkQueueFamilyGlobalPriorityPropertiesKHRVec1 = std::vector<VkQueueFamilyGlobalPriorityPropertiesKHR>;
		using VkQueueFamilyQueryResultStatusPropertiesKHRVec1 = std::vector<VkQueueFamilyQueryResultStatusPropertiesKHR>;
		using VkQueueFamilyVideoPropertiesKHRVec1 = std::vector<VkQueueFamilyVideoPropertiesKHR>;
				
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

		class WvkDispatchTable;
		using WvkDispatchTablePtr = WvkDispatchTable*;
		using WvkDispatchTablePtrVec1 = std::vector<WvkDispatchTablePtr>;
		using WvkDispatchTableSptr = std::shared_ptr<WvkDispatchTable>;
		using WvkDispatchTableSptrVec1 = std::vector<WvkDispatchTableSptr>;
		using WvkDispatchTableUptr = std::unique_ptr<WvkDispatchTable>;
		using WvkDispatchTableUptrVec1 = std::vector<WvkDispatchTableUptr>;
		using WvkDispatchTableWptr = std::weak_ptr<WvkDispatchTable>;
		using WvkDispatchTableWptrVec1 = std::vector<WvkDispatchTableWptr>;

		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		/*!	\brief
		*/
		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

		class WvkPhysicalDevice;
		using WvkPhysicalDevicePtr = WvkPhysicalDevice * ;
		using WvkPhysicalDevicePtrVec1 = std::vector<WvkPhysicalDevicePtr>;
		using WvkPhysicalDevicePtrVec2 = std::vector<WvkPhysicalDevicePtrVec1>;
		using WvkPhysicalDeviceSptr = std::shared_ptr<WvkPhysicalDevice>;
		using WvkPhysicalDeviceSptrVec1 = std::vector<WvkPhysicalDeviceSptr>;
		using WvkPhysicalDeviceSptrVec2 = std::vector<WvkPhysicalDeviceSptrVec1>;
		using WvkPhysicalDeviceUptr = std::unique_ptr<WvkPhysicalDevice>;
		using WvkPhysicalDeviceUptrVec1 = std::vector<WvkPhysicalDeviceUptr>;
		using WvkPhysicalDeviceUptrVec2 = std::vector<WvkPhysicalDeviceUptrVec1>;
		using WvkPhysicalDeviceWptr = std::weak_ptr<WvkPhysicalDevice>;
		using WvkPhysicalDeviceWptrVec1 = std::vector<WvkPhysicalDeviceWptr>;
				

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

		class WvkLogicalDevice;
		using WvkLogicalDevicePtr = WvkLogicalDevice * ;
		using WvkLogicalDevicePtrVec1 = std::vector<WvkLogicalDevicePtr>;
		using WvkLogicalDeviceSptr = std::shared_ptr<WvkLogicalDevice>;
		using WvkLogicalDeviceSptrVec1 = std::vector<WvkLogicalDeviceSptr>;
		using WvkLogicalDeviceUptr = std::unique_ptr<WvkLogicalDevice>;
		using WvkLogicalDeviceUptrVec1 = std::vector<WvkLogicalDeviceUptr>;
		using WvkLogicalDeviceWptr = std::weak_ptr<WvkLogicalDevice>;
		using WvkLogicalDeviceWptrVec1 = std::vector<WvkLogicalDeviceWptr>;		
		struct WvkLogicalDeviceCreateInfo;
		struct WvkLogicalDeviceQueueCreateInfo;
		using WvkLogicalDeviceQueueCreateInfoVec1 = std::vector<WvkLogicalDeviceQueueCreateInfo>;

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

		class WvkCommandPool;
		using WvkCommandPoolPtr = WvkCommandPool * ;
		using WvkCommandPoolPtrVec1 = std::vector<WvkCommandPoolPtr>;
		using WvkCommandPoolSptr = std::shared_ptr<WvkCommandPool>;
		using WvkCommandPoolSptrVec1 = std::vector<WvkCommandPoolSptr>;
		using WvkCommandPoolUptr = std::unique_ptr<WvkCommandPool>;
		using WvkCommandPoolUptrVec1 = std::vector<WvkCommandPoolUptr>;
		using WvkCommandPoolWptr = std::weak_ptr<WvkCommandPool>;
		using WvkCommandPoolWptrVec1 = std::vector<WvkCommandPoolWptr>;
		

		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		/*!	\brief
		*/
		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

		class WvkCommandBuffer;
		using WvkCommandBufferPtr = WvkCommandBuffer * ;
		using WvkCommandBufferPtrVec1 = std::vector<WvkCommandBufferPtr>;
		using WvkCommandBufferSptr = std::shared_ptr<WvkCommandBuffer>;
		using WvkCommandBufferSptrVec1 = std::vector<WvkCommandBufferSptr>;
		using WvkCommandBufferUptr = std::unique_ptr<WvkCommandBuffer>;
		using WvkCommandBufferUptrVec1 = std::vector<WvkCommandBufferSptr>;
		using WvkCommandBufferWptr = std::weak_ptr<WvkCommandBuffer>;
		using WvkCommandBufferWptrVec1 = std::vector<WvkCommandBufferWptr>;

		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		/*!	\brief
		*/
		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

		class WvkFrameBuffer;
		using WvkFrameBufferPtr = WvkFrameBuffer*;
		using WvkFrameBufferPtrVec1 = std::vector<WvkFrameBufferPtr>;
		using WvkFrameBufferSptr = std::shared_ptr<WvkFrameBuffer>;
		using WvkFrameBufferSptrVec1 = std::vector<WvkFrameBufferSptr>;
		using WvkFrameBufferUptr = std::unique_ptr<WvkFrameBuffer>;
		using WvkFrameBufferUptrVec1 = std::vector<WvkFrameBufferSptr>;
		using WvkFrameBufferWptr = std::weak_ptr<WvkFrameBuffer>;
		using WvkFrameBufferWptrVec1 = std::vector<WvkFrameBufferWptr>;

		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		/*!	\brief
		*/
		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

		class WvkRenderPass;
		using WvkRenderPassPtr = WvkRenderPass * ;
		using WvkRenderPassPtrVec1 = std::vector<WvkRenderPassPtr>;
		using WvkRenderPassSptr = std::shared_ptr<WvkRenderPass>;
		using WvkRenderPassSptrVec1 = std::vector<WvkRenderPassSptr>;
		using WvkRenderPassUptr = std::unique_ptr<WvkRenderPass>;
		using WvkRenderPassUptrVec1 = std::vector<WvkRenderPassSptr>;
		using WvkRenderPassWptr = std::weak_ptr<WvkRenderPass>;
		using WvkRenderPassWptrVec1 = std::vector<WvkRenderPassWptr>;
		





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

} // namespace CGDev

#endif // CGDEV_SOURCE_GPU_PRIVATE_VULKAN___VULKAN_H