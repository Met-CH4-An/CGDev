// SPDX-License-Identifier: AGPL-3.0-or-later
#ifndef CGDEV_WVK_SOURCE__WVK_SHADER_HELPER_HPP
#define CGDEV_WVK_SOURCE__WVK_SHADER_HELPER_HPP
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
////////////////////////////////////////////////////////////////
// секция для остального
////////////////////////////////////////////////////////////////
#include "file.h"

namespace CGDev {

	namespace wvk {

		// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

		inline WvkShaderHelperGraphics::WvkShaderHelperGraphics(const WvkLogicalDevicePtr& wvk_logical_device) noexcept {
			m_create_info.wvk_logical_device_ptr = wvk_logical_device;
		}

		// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

		inline WvkShaderHelperGraphics& WvkShaderHelperGraphics::vertex(void) noexcept {
			m_create_info.wvk_shader_stage_create_infos.emplace_back(WvkShaderStageCreateInfo{
				.stage = WvkShaderStageFlags::WVK_SHADER_STAGE_VERTEX_BIT,
				.code = {},
				});

			return *this;
		}

		// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

		inline WvkShaderHelperGraphics& WvkShaderHelperGraphics::tessControl(void) noexcept {
			m_create_info.wvk_shader_stage_create_infos.emplace_back(WvkShaderStageCreateInfo{
				.stage = WvkShaderStageFlags::WVK_SHADER_STAGE_TESSELLATION_CONTROL_BIT,
				.code = {},
				});

			return *this;
		}

		// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

		inline WvkShaderHelperGraphics& WvkShaderHelperGraphics::tessEval(void) noexcept {
			m_create_info.wvk_shader_stage_create_infos.emplace_back(WvkShaderStageCreateInfo{
				.stage = WvkShaderStageFlags::WVK_SHADER_STAGE_TESSELLATION_EVALUATION_BIT,
				.code = {},
				});

			return *this;
		}

		// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

		inline WvkShaderHelperGraphics& WvkShaderHelperGraphics::fragment(void) noexcept {
			m_create_info.wvk_shader_stage_create_infos.emplace_back(WvkShaderStageCreateInfo{
				.stage = WvkShaderStageFlags::WVK_SHADER_STAGE_FRAGMENT_BIT,
				.code = {},
				});

			return *this;
		}

		// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

		inline WvkShaderHelperGraphics& WvkShaderHelperGraphics::vertex(const std::string& name) noexcept {
			m_create_info.wvk_shader_stage_create_infos.emplace_back(WvkShaderStageCreateInfo{
				.stage = WvkShaderStageFlags::WVK_SHADER_STAGE_VERTEX_BIT,
				.code = utils::loadFileAsChar(name),
				});

			return *this;
		}

		// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

		inline WvkShaderHelperGraphics& WvkShaderHelperGraphics::tessControl(const std::string& name) noexcept {
			m_create_info.wvk_shader_stage_create_infos.emplace_back(WvkShaderStageCreateInfo{
				.stage = WvkShaderStageFlags::WVK_SHADER_STAGE_TESSELLATION_CONTROL_BIT,
				.code = utils::loadFileAsChar(name),
				});

			return *this;
		}

		// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

		inline WvkShaderHelperGraphics& WvkShaderHelperGraphics::tessEval(const std::string& name) noexcept {
			m_create_info.wvk_shader_stage_create_infos.emplace_back(WvkShaderStageCreateInfo{
				.stage = WvkShaderStageFlags::WVK_SHADER_STAGE_TESSELLATION_EVALUATION_BIT,
				.code = utils::loadFileAsChar(name),
				});

			return *this;
		}

		// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

		inline WvkShaderHelperGraphics& WvkShaderHelperGraphics::fragment(const std::string& name) noexcept {
			m_create_info.wvk_shader_stage_create_infos.emplace_back(WvkShaderStageCreateInfo{
				.stage = WvkShaderStageFlags::WVK_SHADER_STAGE_FRAGMENT_BIT,
				.code = utils::loadFileAsChar(name),
				});

			return *this;
		}
		
		// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

		inline WvkShaderCreateInfo WvkShaderHelperGraphics::build(void) noexcept {
			return m_create_info;
		}

		// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

		inline WvkShaderHelperRayTracing::WvkShaderHelperRayTracing(const WvkLogicalDevicePtr& wvk_logical_device) noexcept {
			m_create_info.wvk_logical_device_ptr = wvk_logical_device;
		}

		// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

		inline WvkShaderHelperRayTracing& WvkShaderHelperRayTracing::rayGen(void) noexcept {
			m_create_info.wvk_shader_stage_create_infos.emplace_back(WvkShaderStageCreateInfo{
				.stage = WvkShaderStageFlags::WVK_SHADER_STAGE_RAY_GEN,
				.code = {},
				});

			return *this;
		}

		// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

		inline WvkShaderHelperRayTracing& WvkShaderHelperRayTracing::miss(void) noexcept {
			m_create_info.wvk_shader_stage_create_infos.emplace_back(WvkShaderStageCreateInfo{
				.stage = WvkShaderStageFlags::WVK_SHADER_STAGE_MISS,
				.code = {},
				});

			return *this;
		}

		// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

		inline WvkShaderHelperRayTracing& WvkShaderHelperRayTracing::closestHit(void) noexcept {
			m_create_info.wvk_shader_stage_create_infos.emplace_back(WvkShaderStageCreateInfo{
				.stage = WvkShaderStageFlags::WVK_SHADER_STAGE_CLOSEST_HIT,
				.code = {},
				});

			return *this;
		}

		// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

		inline WvkShaderHelperRayTracing& WvkShaderHelperRayTracing::anyHit(void) noexcept {
			m_create_info.wvk_shader_stage_create_infos.emplace_back(WvkShaderStageCreateInfo{
				.stage = WvkShaderStageFlags::WVK_SHADER_STAGE_ANY_HIT,
				.code = {},
				});

			return *this;
		}

		// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

		inline WvkShaderHelperRayTracing& WvkShaderHelperRayTracing::rayGen(const std::string& name) noexcept {
			m_create_info.wvk_shader_stage_create_infos.emplace_back(WvkShaderStageCreateInfo{
				.stage = WvkShaderStageFlags::WVK_SHADER_STAGE_RAY_GEN,
				.code = utils::loadFileAsChar(name),
				});

			return *this;
		}

		// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

		inline WvkShaderHelperRayTracing& WvkShaderHelperRayTracing::miss(const std::string& name) noexcept {
			m_create_info.wvk_shader_stage_create_infos.emplace_back(WvkShaderStageCreateInfo{
				.stage = WvkShaderStageFlags::WVK_SHADER_STAGE_MISS,
				.code = utils::loadFileAsChar(name),
				});

			return *this;
		}

		// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

		inline WvkShaderHelperRayTracing& WvkShaderHelperRayTracing::closestHit(const std::string& name) noexcept {
			m_create_info.wvk_shader_stage_create_infos.emplace_back(WvkShaderStageCreateInfo{
				.stage = WvkShaderStageFlags::WVK_SHADER_STAGE_CLOSEST_HIT,
				.code = utils::loadFileAsChar(name),
				});

			return *this;
		}

		// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

		inline WvkShaderHelperRayTracing& WvkShaderHelperRayTracing::anyHit(const std::string& name) noexcept {
			m_create_info.wvk_shader_stage_create_infos.emplace_back(WvkShaderStageCreateInfo{
				.stage = WvkShaderStageFlags::WVK_SHADER_STAGE_ANY_HIT,
				.code = utils::loadFileAsChar(name),
				});

			return *this;
		}
	
		// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

		inline WvkShaderCreateInfo WvkShaderHelperRayTracing::build(void) noexcept {
			return m_create_info;
		}

		// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

		inline WvkShaderHelperGraphics WvkShaderHelper::graphics(const WvkLogicalDevicePtr& wvk_logical_device) noexcept {
			return WvkShaderHelperGraphics(wvk_logical_device);
		}

		// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

		inline WvkShaderHelperRayTracing WvkShaderHelper::rayTracing(const WvkLogicalDevicePtr& wvk_logical_device) noexcept {
			return WvkShaderHelperRayTracing(wvk_logical_device);
		}
	
		// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

	} // namespace wvk

} // namespace CGDev

#endif // CGDEV_WVK_SOURCE__WVK_SHADER_HELPER_HPP
