#ifndef CGDEV_SOURCE_TESTS__GPU_VKN_EXT_DEBUG_UTILS_COMMANDS_H
#define CGDEV_SOURCE_TESTS__GPU_VKN_EXT_DEBUG_UTILS_COMMANDS_H
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
#include "Private/Vulkan/Extensions/vkn_ext_debug_utils_commands.h"		// что тестируем

#include "vkn_commands_instance_test.h"

namespace CGDev {

	namespace Tests {

		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		/*!	\brief
		*/
		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		class VknExtDebugUtilsCommandsTest : public ::testing::Test {

		public:

			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			/*!	\brief
			*/
			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			static void SetUpTestSuite(void) {

				VknCommandsInstanceTest::SetUpTestSuite();

				s_vkn_ext_debug_utils_commands_create_info.vkn_commands = VknCommandsInstanceTest::getTestObject();

				if (s_vkn_ext_debug_utils_commands.getCreateInfo().vkn_commands == nullptr) {
					s_vkn_ext_debug_utils_commands.create(s_vkn_ext_debug_utils_commands_create_info);
				}
			}

			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			/*!	\brief
			*/
			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			static void TearDownTestSuite(void) {
				s_vkn_ext_debug_utils_commands.destroy();
				VknCommandsInstanceTest::TearDownTestSuite();
			}

		public:

			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			/*!	\brief
			*/
			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			inline static const CGDev::GPU::Private::Vulkan::Extensions::VknExtDebugUtilsCommandsPtr getTestObject(void) noexcept {

				return &s_vkn_ext_debug_utils_commands;
			}

		protected:

			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			/*!	\brief
			*/
			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			void SetUp() override {
				m_vkn_ext_debug_utils_commands_create_info.vkn_commands = VknCommandsInstanceTest::getTestObject();
			}

			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			/*!	\brief
			*/
			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			void TearDown() override {
			}

		protected:

			inline static CGDev::GPU::Private::Vulkan::Extensions::VknExtDebugUtilsCommands					s_vkn_ext_debug_utils_commands;
			inline static CGDev::GPU::Private::Vulkan::Extensions::VknExtDebugUtilsCommandsCreateInfo		s_vkn_ext_debug_utils_commands_create_info;

			CGDev::GPU::Private::Vulkan::Extensions::VknExtDebugUtilsCommands				m_vkn_ext_debug_utils_commands;
			CGDev::GPU::Private::Vulkan::Extensions::VknExtDebugUtilsCommandsCreateInfo		m_vkn_ext_debug_utils_commands_create_info;
		};

	} // namespace Tests

} // namespace CGDev

#endif // CGDEV_SOURCE_TESTS__GPU_VKN_EXT_DEBUG_UTILS_COMMANDS_H
