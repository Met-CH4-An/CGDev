#ifndef CGDEV_SOURCE_TESTS__GPU_VKN_COMMANDS_GLOBAL_H
#define CGDEV_SOURCE_TESTS__GPU_VKN_COMMANDS_GLOBAL_H
////////////////////////////////////////////////////////////////
// секция форвард-декларации
////////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////
// секция имплементации
////////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////
// секция родительского класса
////////////////////////////////////////////////////////////////
#include "tests.h"
////////////////////////////////////////////////////////////////
// секция для остального
////////////////////////////////////////////////////////////////
#include "wvk_commands.h"	// что тестируем

#include "wvk_loader_test.h"

namespace CGDev {

	namespace tests {

		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		/*!	\brief Группа тестов для VknGlobalCommands
		*/
		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		class WvkCommandsGlobalTest : public ::testing::Test {
		
		public:

			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			/*!	\brief
			*/
			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			static void SetUpTestSuite(void) {
				//WvkLoaderTest::SetUpTestSuite();

				//s_vkn_commands_create_info.wvk_loader = WvkLoaderTest::getTestObject();
				//s_vkn_commands_create_info.wvk_instance = nullptr;
				//s_vkn_commands_create_info.wvk_logical_device = nullptr;

				//if (s_vkn_commands_global_level.getCreateInfo().wvk_loader == nullptr) {
				//	s_vkn_commands_global_level.create(&s_vkn_commands_create_info);
				//}
			}

			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			/*!	\brief
			*/
			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			static void TearDownTestSuite(void) {

				//s_vkn_commands_global_level.destroy();

				//WvkLoaderTest::TearDownTestSuite();
			}

		public:

			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			/*!	\brief
			*/
			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			inline static const CGDev::wvk::WvkCommandsPtr getTestObject(void) noexcept {

				return &s_vkn_commands_global_level;
			}

		protected:

			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			/*!	\brief
			*/
			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			void SetUp() override {
				//m_vkn_commands_create_info.wvk_loader = WvkLoaderTest::getTestObject();
				//m_vkn_commands_create_info.wvk_instance = nullptr;
				//m_vkn_commands_create_info.wvk_logical_device = nullptr;

			}

			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			/*!	\brief
			*/
			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			void TearDown() override {

				//m_vkn_commands_global_level.destroy();
			}

		protected:

			inline static CGDev::wvk::WvkGlobalCommands s_vkn_commands_global_level;
			inline static CGDev::wvk::WvkCommandsCreateInfo s_vkn_commands_create_info;

			CGDev::wvk::WvkGlobalCommands m_vkn_commands_global_level;
			CGDev::wvk::WvkCommandsCreateInfo m_vkn_commands_create_info;
		};

	} // namespace tests

} // namespace CGDev

#endif // CGDEV_SOURCE_TESTS__GPU_VKN_COMMANDS_GLOBAL_H
