#ifndef CGDEV_SOURCE_TESTS__GPU_VKN_EXT_DEBUG_UTILS_H
#define CGDEV_SOURCE_TESTS__GPU_VKN_EXT_DEBUG_UTILS_H
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
#include "Private/Vulkan/Extensions/vkn_ext_debug_utils.h"		// что тестируем

#include "vkn_ext_debug_utils_commands_test.h"

namespace CGDev {

	namespace Tests {

		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		/*!	\brief
		*/
		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		class VknExtDebugUtilsTest : public ::testing::Test {

		public:

			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			/*!	\brief
			*/
			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			static void SetUpTestSuite(void) {
				VknExtDebugUtilsCommandsTest::SetUpTestSuite();
				
				s_vkn_ext_debug_utils_create_info.vkn_debug_utils_commands = VknExtDebugUtilsCommandsTest::getTestObject();
				s_vkn_ext_debug_utils_create_info.mode = 
					CGDev::GPU::Private::Vulkan::Extensions::VknDebugUtilsMode::ERRORS_ONLY |
					CGDev::GPU::Private::Vulkan::Extensions::VknDebugUtilsMode::WARNINGS_ONLY |
					CGDev::GPU::Private::Vulkan::Extensions::VknDebugUtilsMode::VALIDATION |
					CGDev::GPU::Private::Vulkan::Extensions::VknDebugUtilsMode::PERFORMANCE;
				
				if (s_vkn_ext_debug_utils.getCreateInfo().vkn_debug_utils_commands == nullptr) {
					s_vkn_ext_debug_utils.create(s_vkn_ext_debug_utils_create_info);
				}
			}

			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			/*!	\brief
			*/
			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			static void TearDownTestSuite(void) {			
				s_vkn_ext_debug_utils.destroy();
				VknExtDebugUtilsCommandsTest::TearDownTestSuite();
			}

		public:

			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			/*!	\brief
			*/
			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			inline static const CGDev::GPU::Private::Vulkan::Extensions::VknExtDebugUtilsPtr getTestObject(void) noexcept {

				return &s_vkn_ext_debug_utils;
			}

		protected:

			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			/*!	\brief
			*/
			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			void SetUp() override {
				m_vkn_ext_debug_utils_create_info.vkn_debug_utils_commands = VknExtDebugUtilsCommandsTest::getTestObject();
				m_vkn_ext_debug_utils_create_info.mode =
					CGDev::GPU::Private::Vulkan::Extensions::VknDebugUtilsMode::ERRORS_ONLY |
					CGDev::GPU::Private::Vulkan::Extensions::VknDebugUtilsMode::WARNINGS_ONLY |
					CGDev::GPU::Private::Vulkan::Extensions::VknDebugUtilsMode::VALIDATION |
					CGDev::GPU::Private::Vulkan::Extensions::VknDebugUtilsMode::PERFORMANCE;
			}

			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			/*!	\brief
			*/
			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			void TearDown() override {
			}

		protected:

			inline static CGDev::GPU::Private::Vulkan::Extensions::VknExtDebugUtils					s_vkn_ext_debug_utils;
			inline static CGDev::GPU::Private::Vulkan::Extensions::VknExtDebugUtilsCreateInfo		s_vkn_ext_debug_utils_create_info;

			CGDev::GPU::Private::Vulkan::Extensions::VknExtDebugUtils				m_vkn_ext_debug_utils;
			CGDev::GPU::Private::Vulkan::Extensions::VknExtDebugUtilsCreateInfo		m_vkn_ext_debug_utils_create_info;
		};

	} // namespace Tests

} // namespace CGDev

#endif // CGDEV_SOURCE_TESTS__GPU_VKN_EXT_DEBUG_UTILS_H
