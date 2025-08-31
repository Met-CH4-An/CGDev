// SPDX-License-Identifier: AGPL-3.0-or-later
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
		TEST_F(WvkInstanceTest, createFakeLayer) {
			CGDev::wvk::WvkInstanceCreateInfo _create_info = {
				.vk_layer_names = { "fake_layer" },
			};
			CGDev::wvk::WvkInstance _wvk_instance;

			auto _wvk_status = _wvk_instance.create(_create_info);

			EXPECT_EQ(_wvk_status.isOk(), false);
		}

		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		// Проверка:
		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		TEST_F(WvkInstanceTest, createFakeExtension) {
			CGDev::wvk::WvkInstanceCreateInfo _create_info = {
				.vk_extension_names = { "fake_extension" },
			};
			CGDev::wvk::WvkInstance _wvk_instance;

			auto _wvk_status = _wvk_instance.create(_create_info);

			EXPECT_EQ(_wvk_status.isOk(), false);
		}

		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		// Проверка:
		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		TEST_F(WvkInstanceTest, presencePhysicalDevice) {
			const auto& _phys_dev = wvk_instance_ptr->getWvkPhysicalDevices()[0][0];

			EXPECT_NE(_phys_dev, nullptr);
		}

	} // namespace tests

} // namespace CGDev