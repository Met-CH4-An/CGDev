// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025 Metelev Met-CH4-An Andrey
// Project Lead: Metelev Andrey
// Main Technical Assistant: StarDev aka ChatGPT
#ifndef CGDEV_WVK_SOURCE__WVK_SHADER_H
#define CGDEV_WVK_SOURCE__WVK_SHADER_H
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
#include "wvk_base_object.h"
////////////////////////////////////////////////////////////////
// секция для остального
////////////////////////////////////////////////////////////////

namespace CGDev {

	namespace wvk {

		// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		/*!	\brief
		*/
		// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		enum class WvkShaderStageFlags {
			WVK_SHADER_STAGE_UNKNOWN = 0,
			WVK_SHADER_STAGE_VERTEX_BIT = VK_SHADER_STAGE_VERTEX_BIT,
			WVK_SHADER_STAGE_TESSELLATION_CONTROL_BIT = VK_SHADER_STAGE_TESSELLATION_CONTROL_BIT,
			WVK_SHADER_STAGE_TESSELLATION_EVALUATION_BIT = VK_SHADER_STAGE_TESSELLATION_EVALUATION_BIT,
			WVK_SHADER_STAGE_GEOMETRY_BIT = VK_SHADER_STAGE_GEOMETRY_BIT,
			WVK_SHADER_STAGE_FRAGMENT_BIT = VK_SHADER_STAGE_FRAGMENT_BIT,
			WVK_SHADER_STAGE_COMPUTE_BIT = VK_SHADER_STAGE_COMPUTE_BIT,
			WVK_SHADER_STAGE_ALL_GRAPHICS = VK_SHADER_STAGE_ALL_GRAPHICS,
			WVK_SHADER_STAGE_RAY_GEN = VK_SHADER_STAGE_RAYGEN_BIT_KHR,
			WVK_SHADER_STAGE_MISS = VK_SHADER_STAGE_MISS_BIT_KHR,
			WVK_SHADER_STAGE_CLOSEST_HIT = VK_SHADER_STAGE_CLOSEST_HIT_BIT_KHR,
			WVK_SHADER_STAGE_ANY_HIT = VK_SHADER_STAGE_ANY_HIT_BIT_KHR,
		}; // enum class WvkShaderStageFlags

		// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		/*!	\brief
		*/
		// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		struct WvkShaderStageCreateInfo {
			WvkShaderStageFlags stage = WvkShaderStageFlags::WVK_SHADER_STAGE_UNKNOWN;
			std::vector<std::byte> code;
			std::string entry_point_name = "";
		}; // struct WvkShaderStageCreateInfo

		// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		/*!	\brief
		*/
		// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		struct WvkShaderCreateInfo {
			WvkLogicalDevicePtr wvk_logical_device_ptr = nullptr;
			std::vector<WvkShaderStageCreateInfo> wvk_shader_stage_create_infos;
		}; // struct WvkShaderCreateInfo

		// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		/*!	\brief
		*/
		// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		class WvkShader : public GpuObject {

		public:

			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			/*!	\brief
			*/
			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			WvkShader(void) noexcept;

			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			/*!	\brief
			*/
			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			~WvkShader(void) noexcept;

			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			/*!	\brief
			*/
			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			WvkStatus create(const WvkShaderCreateInfo& create_info) noexcept;

			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			/*!	\brief
			*/
			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			void destroy(void) noexcept;

		// cpp
		public:

		// hpp
		public:

			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			/*!	\brief
			*/
			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			inline WvkStatus getBinary(std::vector<WvkShaderStageCreateInfo>& wvk_shader_stages) const noexcept;

		// hpp
		private:

			friend class WvkCommandPool;
			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			/*!	\brief
			*/
			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			//inline VkCommandBuffer getVkCommandBuffer(void) const noexcept;

		// cpp
		private:

			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			/*!	\brief
			*/
			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			WvkStatus validationCreateInfo(const WvkShaderCreateInfo& create_info) noexcept;

			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			/*!	\brief
			*/
			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			WvkStatus create(void) noexcept;

		private:

			WvkShaderCreateInfo m_create_info = {};

			VkShaderModule m_vk_shader_module = VK_NULL_HANDLE;
#if WVK_EXT_shader_object == WVK_ENABLE
			std::vector<VkShaderEXT> m_vk_shaders;
#endif
		}; // class WvkShader

	} // namespace wvk

} // namespace CGDev

#include "wvk_shader.hpp"
#include "wvk_shader_helper.h"

#endif // CGDEV_WVK_SOURCE__WVK_SHADER_H
