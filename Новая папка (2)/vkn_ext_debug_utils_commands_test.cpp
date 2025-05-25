////////////////////////////////////////////////////////////////
// секция форвард-декларации
////////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////
// секция заголовочного файла
////////////////////////////////////////////////////////////////
#include "vkn_ext_debug_utils_commands_test.h"
////////////////////////////////////////////////////////////////
// секция имплементации
////////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////
// секция для остального
////////////////////////////////////////////////////////////////

namespace CGDev {

	namespace Tests {

		// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		// Проверка:
		// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

		TEST_F(VknExtDebugUtilsCommandsTest, CreateReturnsSuccessWithValidInput) {
			auto _status = m_vkn_ext_debug_utils_commands.create(m_vkn_ext_debug_utils_commands_create_info);

			EXPECT_EQ(_status.m_code, CGDev::GPU::Private::Vulkan::VknStatusCode::SUCCESSFUL);
		}

		// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		// Проверка:
		// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

		TEST_F(VknExtDebugUtilsCommandsTest, CreateReturnsFailWhenCreateInfoIsInvalid) {
			m_vkn_ext_debug_utils_commands_create_info.vkn_commands = nullptr;

			auto _status = m_vkn_ext_debug_utils_commands.create(m_vkn_ext_debug_utils_commands_create_info);

			// Проверка: должен быть возвращён статус ошибки
			EXPECT_EQ(_status.m_code, CGDev::GPU::Private::Vulkan::VknStatusCode::FAIL);
		}

		// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		// Проверка:
		// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

		TEST_F(VknExtDebugUtilsCommandsTest, GetCreateInfoReturnsReferenceToStoredInfo) {
			auto _status = m_vkn_ext_debug_utils_commands.create(m_vkn_ext_debug_utils_commands_create_info);

			// Проверка, что возвращается та же структура
			EXPECT_EQ(m_vkn_ext_debug_utils_commands.getCreateInfo().vkn_commands, m_vkn_ext_debug_utils_commands_create_info.vkn_commands);
		}
	}

} // namespace CGDev