// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025 Metelev Met-CH4-An Andrey
// Project Lead: Metelev Andrey
// Main Technical Assistant: StarDev aka ChatGPT
#ifndef CGDEV_WVK_SOURCE___WVK_H
#define CGDEV_WVK_SOURCE___WVK_H
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
#include <iostream>
#include <fstream>
#include <sstream>
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

		namespace build {

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
			static const VulkanVersion vulkan_version = VulkanVersion::VERSION_10;
			constexpr std::string_view application_name = "CGDev Project";
			constexpr uint32_t application_version = VK_MAKE_API_VERSION(0, 0, 0, 2);
			constexpr std::string_view engine_name = "Wvk Project: vulkan wrapper";
			constexpr uint32_t engine_version = VK_MAKE_API_VERSION(0, 0, 0, 2);


#define WVK_PLATFORM_MSWINDOWS 1
#define WVK_PLATFORM_LINUX 2
#define WVK_PLATFORM_ANDROID 2

#define WVK_PLATFORM WVK_PLATFORM_MSWINDOWS

			struct Support {
				std::string_view name = {};
				bool enable = false;
			};

			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			/*!	\brief
			*/
			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			inline constexpr auto instanceLayers(void) {
				std::array<Support, 1> _layers{};
				_layers[0] = Support{ .name = "VK_LAYER_KHRONOS_validation", .enable = true, };
				return _layers;
			};

			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			/*!	\brief
			*/
			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			inline constexpr std::array<Support, 4> instanceExtensions(void) {
				std::array<Support, 4> _exts{};
				_exts[0] = Support{ .name = "VK_EXT_debug_utils", .enable = true, };
				_exts[1] = Support{ .name = "VK_KHR_get_physical_device_properties2", .enable = true, };
				_exts[2] = Support{ .name = "VK_KHR_surface", .enable = true, };
				_exts[3] = Support{ .name = "VK_KHR_win32_surface", .enable = true, };
				return _exts;
			};

			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			/*!	\brief
			*/
			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			inline constexpr std::array<Support, 10> deviceExtensions(void) {
				std::array<Support, 10> _exts{};
				_exts[0] = Support{ .name = "VK_KHR_dynamic_rendering_local_read", .enable = true, };
				_exts[1] = Support{ .name = "VK_EXT_shader_object", .enable = true, };
				_exts[2] = Support{ .name = "VK_KHR_dynamic_rendering", .enable = true, };
				_exts[3] = Support{ .name = "VK_KHR_depth_stencil_resolve", .enable = true, };
				_exts[4] = Support{ .name = "VK_KHR_create_renderpass2", .enable = true, };
				_exts[5] = Support{ .name = "VK_KHR_maintenance1", .enable = true, };
				_exts[6] = Support{ .name = "VK_KHR_maintenance2", .enable = true, };
				_exts[7] = Support{ .name = "VK_KHR_multiview", .enable = true, };
				_exts[8] = Support{ .name = "VK_KHR_swapchain", .enable = true, };
				_exts[9] = Support{ .name = "VK_KHR_synchronization2", .enable = true, };
				return _exts;
			};

			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			/*!	\brief
			*/
			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			inline static constexpr bool isLayerEnabled(std::string_view name) {
				for (size_t ct_0 = 0; ct_0 < instanceLayers().size(); ++ct_0) {
					const auto& _name = instanceLayers()[ct_0].name;
					if (_name == name) {
						return instanceLayers()[ct_0].enable;
					}
				}
				return false;
			}

			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			/*!	\brief
			*/
			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			inline static constexpr bool isExtensionEnabled(std::string_view name) {		
				for (size_t ct_0 = 0; ct_0 < instanceExtensions().size(); ++ct_0) {
					if (instanceExtensions()[ct_0].name == name) {
						return instanceExtensions()[ct_0].enable;
					}
				}
				
				for (size_t ct_0 = 0; ct_0 < deviceExtensions().size(); ++ct_0) {
					if (deviceExtensions()[ct_0].name == name) {
						return deviceExtensions()[ct_0].enable;
					}
				}
				return false;
			}

			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			/*!	\brief
			*/
			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			inline static constexpr auto getInstanceLayerNames(void) {
				std::array<const char*, instanceLayers().size()> _names{};

				for (size_t ct_0 = 0; ct_0 < instanceLayers().size(); ++ct_0) {
					_names[ct_0] = instanceLayers()[ct_0].name.data();
				}

				return _names;
			}

			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			/*!	\brief
			*/
			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			inline static constexpr auto getInstanceExtensionNames(void) {
				std::array<const char*, instanceExtensions().size()> _names{};

				for (size_t ct_0 = 0; ct_0 < instanceExtensions().size(); ++ct_0) {
					_names[ct_0] = instanceExtensions()[ct_0].name.data();
				}

				return _names;
			}

			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			/*!	\brief
			*/
			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			inline static constexpr auto getDeviceExtensionNames(void) {
				std::array<const char*, deviceExtensions().size()> _names{};

				for(size_t ct_0 = 0; ct_0 < deviceExtensions().size(); ++ct_0) {
					_names[ct_0] = deviceExtensions()[ct_0].name.data();
				}
				
				return _names;
			}	

		} // namespace build





				
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

		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		/*!	\brief
		*/
		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		class WvkShader;
		using WvkShaderPtr = WvkShader * ;
		using WvkShaderPtrVec1 = std::vector<WvkShaderPtr>;
		using WvkShaderSptr = std::shared_ptr<WvkShader>;
		using WvkShaderSptrVec1 = std::vector<WvkShaderSptr>;
		using WvkShaderUptr = std::unique_ptr<WvkShader>;
		using WvkShaderUptrVec1 = std::vector<WvkShaderSptr>;
		using WvkShaderWptr = std::weak_ptr<WvkShader>;
		using WvkShaderWptrVec1 = std::vector<WvkShaderWptr>;

		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		/*!	\brief
		*/
		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		class WvkImage;
		using WvkImagePtr = WvkImage * ;
		using WvkImagePtrVec1 = std::vector<WvkImagePtr>;
		using WvkImageSptr = std::shared_ptr<WvkImage>;
		using WvkImageSptrVec1 = std::vector<WvkImageSptr>;
		using WvkImageUptr = std::unique_ptr<WvkImage>;
		using WvkImageUptrVec1 = std::vector<WvkImageSptr>;
		using WvkImageWptr = std::weak_ptr<WvkImage>;
		using WvkImageWptrVec1 = std::vector<WvkImageWptr>;
		
		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		/*!	\brief
		*/
		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		class WvkFence;
		using WvkFencePtr = WvkFence * ;
		using WvkFencePtrVec1 = std::vector<WvkFencePtr>;
		using WvkFenceSptr = std::shared_ptr<WvkFence>;
		using WvkFenceSptrVec1 = std::vector<WvkFenceSptr>;
		using WvkFenceUptr = std::unique_ptr<WvkFence>;
		using WvkFenceUptrVec1 = std::vector<WvkFenceUptr>;
		using WvkFenceWptr = std::weak_ptr<WvkFence>;
		using WvkFenceWptrVec1 = std::vector<WvkFenceWptr>;

		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		/*!	\brief
		*/
		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		namespace mswindows {
			class WvkDispatchTableMSWindows;
		}

		namespace Extensions {
			class WvkDebugUtilsMessenger;
			class WvkSwapchain;
			class WvkSurface;
		}	

	} // namespace wvk

} // namespace CGDev

#endif // CGDEV_WVK_SOURCE___WVK_H