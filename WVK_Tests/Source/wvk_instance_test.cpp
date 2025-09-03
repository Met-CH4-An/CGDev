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
#include "wvk_instance_test.h"
////////////////////////////////////////////////////////////////
// секция имплементации
////////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////
// секция для остального
////////////////////////////////////////////////////////////////

namespace CGDev {

	namespace tests {

		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		// Проверка:
		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		TEST_F(WvkInstanceTest, requestLayerProperties) {
			std::vector<VkLayerProperties> _props;
			auto _status = m_wvk_instance_ptr->requestLayerProperties(_props);

			EXPECT_EQ(_status, true);
		}

		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		// Проверка:
		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		TEST_F(WvkInstanceTest, requestExtensionProperties) {
			std::vector<VkExtensionProperties> _props;
			auto _status = m_wvk_instance_ptr->requestExtensionProperties(_props);

			EXPECT_EQ(_status, true);
		}
		
		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		// Проверка:
		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		TEST_F(WvkInstanceTest, presencePhysicalDevice) {
			const auto& _phys_dev = m_wvk_instance_ptr->getWvkPhysicalDevices()[0][0];

			EXPECT_NE(_phys_dev, nullptr);
		}

	} // namespace tests

} // namespace CGDev