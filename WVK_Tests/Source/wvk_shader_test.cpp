// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025 Metelev Met-CH4-An Andrey
// Project Lead: Metelev Andrey
// Main Technical Assistant: StarDev aka ChatGPT
////////////////////////////////////////////////////////////////
// секция форвард-декларации
////////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////
// секция заголовочного файла
////////////////////////////////////////////////////////////////
#include "wvk_shader_test.h"
////////////////////////////////////////////////////////////////
// секция имплементации
////////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////
// секция для остального
////////////////////////////////////////////////////////////////
#include "Extensions/wvk_debug_utils_messenger.h"

namespace CGDev {

	namespace tests {

		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		// Проверка:
		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		TEST_F(WvkShaderTest, create) {
			CGDev::wvk::WvkShaderStageCreateInfo _vertex_stage_create_info = {
				.stage = CGDev::wvk::WvkShaderStageFlags::WVK_SHADER_STAGE_VERTEX_BIT,
				.code = CGDev::utils::loadFileAsChar("D:\\Laboratory\\CGDev\\Extern\\triangle_vs.spv"),
			};

			CGDev::wvk::WvkShaderStageCreateInfo _fragment_stage_create_info = {
				.stage = CGDev::wvk::WvkShaderStageFlags::WVK_SHADER_STAGE_FRAGMENT_BIT,
				.code = CGDev::utils::loadFileAsChar("D:\\Laboratory\\CGDev\\Extern\\triangle_fs.spv"),
			};

			CGDev::wvk::WvkShaderCreateInfo _create_info = {
				.wvk_logical_device_ptr = m_wvk_logical_device_ptr.get(),
				.wvk_shader_stage_create_infos = { _vertex_stage_create_info, _fragment_stage_create_info },
			};

			m_wvk_debug_utils_messenger_ptr->hasIssues(CGDev::wvk::Extensions::VknDebugUtilsMode::ERRORS_ONLY | CGDev::wvk::Extensions::VknDebugUtilsMode::WARNINGS_ONLY);
			CGDev::wvk::WvkShaderPtr _wvk_shader = new CGDev::wvk::WvkShader();
			auto _wvk_status = _wvk_shader->create(_create_info);
			EXPECT_EQ(_wvk_status, true);

			_wvk_status = m_wvk_debug_utils_messenger_ptr->hasIssues(CGDev::wvk::Extensions::VknDebugUtilsMode::ERRORS_ONLY | CGDev::wvk::Extensions::VknDebugUtilsMode::WARNINGS_ONLY);
			EXPECT_EQ(_wvk_status, true);
		}

		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		// Проверка:
		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		TEST_F(WvkShaderTest, getBinary) {
			std::vector<CGDev::wvk::WvkShaderStageCreateInfo> _data;
			auto _wvk_result = m_wvk_shader_ptr->getBinary(_data);
			EXPECT_EQ(_wvk_result, true);
		}

	} // namespace tests

} // namespace CGDev