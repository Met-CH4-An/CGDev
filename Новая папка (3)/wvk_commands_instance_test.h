#ifndef CGDEV_SOURCE_TESTS__GPU_VKN_COMMANDS_INSTANCE_H
#define CGDEV_SOURCE_TESTS__GPU_VKN_COMMANDS_INSTANCE_H
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
#include "wvk_instance_test.h"

namespace CGDev {

	namespace tests {

		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		/*!	\brief Группа тестов для VknGlobalCommands
		*/
		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		class WvkCommandsInstanceTest : public ::testing::Test {

		public:

			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			/*!	\brief
			*/
			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			static void SetUpTestSuite(void) {
				//WvkLoaderTest::SetUpTestSuite();
				//WvkInstanceTest::SetUpTestSuite();

				//s_vkn_commands_create_info.wvk_loader = WvkLoaderTest::getTestObject();
				//s_vkn_commands_create_info.wvk_instance = WvkInstanceTest::getTestObject();
				//s_vkn_commands_create_info.wvk_logical_device = nullptr;

				//if (s_vkn_commands.getCreateInfo().wvk_instance == nullptr) {
				//	s_vkn_commands.create(&s_vkn_commands_create_info);
				//}
			}

			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			/*!	\brief
			*/
			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			static void TearDownTestSuite(void) {
				//s_vkn_commands.destroy();
				//WvkInstanceTest::TearDownTestSuite();
				//WvkLoaderTest::TearDownTestSuite();
			}

		public:

			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			/*!	\brief
			*/
			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			inline static const CGDev::wvk::WvkCommandsPtr getTestObject(void) noexcept {

				return &s_vkn_commands;
			}

		protected:

			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			/*!	\brief
			*/
			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			void SetUp() override {
				//m_vkn_commands_create_info.wvk_loader = WvkLoaderTest::getTestObject();
				//m_vkn_commands_create_info.wvk_instance = WvkInstanceTest::getTestObject();;
				//m_vkn_commands_create_info.wvk_logical_device = nullptr;

			}

			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			/*!	\brief
			*/
			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			void TearDown() override {

				//s_vkn_commands.destroy();
			}

		protected:

			inline static CGDev::wvk::WvkGlobalCommands s_vkn_commands;
			inline static CGDev::wvk::WvkCommandsCreateInfo s_vkn_commands_create_info;

			CGDev::wvk::WvkGlobalCommands m_vkn_commands;
			CGDev::wvk::WvkCommandsCreateInfo m_vkn_commands_create_info;
		};

	} // namespace tests

} // namespace CGDev

#endif // CGDEV_SOURCE_TESTS__GPU_VKN_COMMANDS_INSTANCE_H
