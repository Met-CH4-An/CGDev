////////////////////////////////////////////////////////////////
// секция форвард-декларации
////////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////
// секция заголовочного файла
////////////////////////////////////////////////////////////////
#include "wvk_commands_instance_test.h"
////////////////////////////////////////////////////////////////
// секция имплементации
////////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////
// секция для остального
////////////////////////////////////////////////////////////////

namespace CGDev {

	namespace tests {

		// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		// Тест:
		// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		TEST_F(WvkCommandsInstanceTest, CreateWithLogAndLoaderAndInstanceSucceeds) {

			//auto status = s_vkn_commands.create(&s_vkn_commands_create_info);

			//EXPECT_EQ(status.isOk(), true);
		}

		// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		// Тест:
		// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		TEST_F(WvkCommandsInstanceTest, CreateFailsWhenVknLoaderIsNull) {

			//s_vkn_commands_create_info.wvk_loader = nullptr;

			//auto status = s_vkn_commands.create(&s_vkn_commands_create_info);

			//EXPECT_NE(status.isOk(), true);
		}

		// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		// Тест:
		// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		TEST_F(WvkCommandsInstanceTest, CreateFailsWhenVknInstanceIsNull) {

			//s_vkn_commands_create_info.wvk_instance = nullptr;

			//auto status = s_vkn_commands.create(&s_vkn_commands_create_info);

			//EXPECT_NE(status.isOk(), true);
		}
	}

} // namespace CGDev