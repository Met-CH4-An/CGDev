#ifndef CGDEV_SOURCE_GPU_PRIVATE_VULKAN__VKN_VALIDATION_H
#define CGDEV_SOURCE_GPU_PRIVATE_VULKAN__VKN_VALIDATION_H
////////////////////////////////////////////////////////////////
// секция форвард-декларации
////////////////////////////////////////////////////////////////
#include "_vulkan.h"
////////////////////////////////////////////////////////////////
// секция имплементации
////////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////
// секция родительского класса
////////////////////////////////////////////////////////////////
#include "../gpu_object.h"
////////////////////////////////////////////////////////////////
// секция для остального
////////////////////////////////////////////////////////////////
#include "Extensions/vkn_ext_debug_utils.h"

namespace CGDev {

	namespace GPU {

		namespace Private {

			namespace Vulkan {

				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				/*!	\brief
				*/
				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				struct VknValidationCreateInfo {

					Tools::LogSptr							log = nullptr;
					VknLoaderSptr							loader = nullptr;
					VknInstanceSptr							instance = nullptr;
					
					//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					/*!	\brief
					*/
					//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					inline bool isValid(void) const noexcept;

				}; // VknValidationCreateInfo





				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				/*!	\brief
				*/
				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				class VknValidation : public GpuObject {

				public:

					//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					/*!	\brief
					*/
					//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					static void s_requestNextForVkInstance(void*& next_for_vk_instance, CGDev::Tools::LogSptr& log) noexcept;

					//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					/*!	\brief
					*/
					//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					static void s_requestLastValidationForVkInstance(bool& validation) noexcept;

				public:

					//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					/*!	\brief
					*/
					//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					inline static const std::vector<std::string>& s_getLayerNames(void) noexcept;

					//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					/*!	\brief
					*/
					//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					inline static const std::vector<std::string>& s_getExtensionNames(void) noexcept;

				private:

					static std::vector<std::string>															s_layer_name_collection;
					static std::vector<std::string>															s_extension_name_collection;
					
				public:

					//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					/*!	\brief
					*/
					//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					VknValidation(void) noexcept;

					//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					/*!	\brief
					*/
					//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					~VknValidation(void) noexcept;

					//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					/*!	\brief
					*/
					//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					void create(const VknValidationCreateInfo& create_info) noexcept;

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
					void requestLastValidation(bool& validation) noexcept;

					//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					/*!	\brief
					*/
					//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					void searchSupportedLayersAndExtensions(void) noexcept;

				public:

					//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					/*!	\brief
					*/
					//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					inline const VknValidationCreateInfo& getCreateInfo(void) const noexcept;

					//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					/*!	\brief
					*/
					//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					inline const std::vector<std::string>& getLayerNames(void) const noexcept;

					//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					/*!	\brief
					*/
					//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					inline const std::vector<std::string>& getExtensionNames(void) const noexcept;

				private:

					//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					/*!	\brief
					*/
					//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					bool createVknExtDebugUtils(void) noexcept;

					//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					/*!	\brief
					*/
					//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					void prepareLayerNames(void) noexcept;

					//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					/*!	\brief
					*/
					//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					void prepareExtensionNames(void) noexcept;

					//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					/*!	\brief
					*/
					//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					VkBool32 callbackForVkDebugUtilsMessengerCreateInfoEXT(VkDebugUtilsMessageSeverityFlagBitsEXT messageSeverity, VkDebugUtilsMessageTypeFlagsEXT messageTypes, const VkDebugUtilsMessengerCallbackDataEXT* pCallbackData, void* pUserData) noexcept;

				private:

					//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					/*!	\brief
					*/
					//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					static VkBool32 s_callbackForVkDebugUtilsMessengerCreateInfoEXT(VkDebugUtilsMessageSeverityFlagBitsEXT messageSeverity, VkDebugUtilsMessageTypeFlagsEXT messageTypes, const VkDebugUtilsMessengerCallbackDataEXT* pCallbackData, void* pUserData) noexcept;

				private:
					
					VknValidationCreateInfo															m_create_info = {};
					std::vector<std::string>														m_layer_name_collection;
					std::vector<std::string>														m_extension_name_collection;
					Extensions::VknExtDebugUtilsSptr												m_ext_debug_utils = nullptr;

				}; // class VknValidation

			} // namespace Vulkan

		} // namespace Private

	} // namespace GPU

} // namespace CGDev

#include "vkn_validation.hpp"

#endif // CGDEV_SOURCE_GPU_PRIVATE_VULKAN__VKN_VALIDATION_H
