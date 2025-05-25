#ifndef CGDEV_SOURCE_TESTS__VKN_QUEUE_FAMILY_H
#define CGDEV_SOURCE_TESTS__VKN_QUEUE_FAMILY_H
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
#include "Private/Vulkan/vkn_queue_family.h"		// что тестируем

#include "vkn_instance_test.h"
#include "vkn_commands_instance_test.h"
#include "vkn_physical_device_test.h"

namespace CGDev {

	namespace Tests {

		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		/*!	\brief 
		*/
		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		class VknQueueFamilyTest : public ::testing::Test {

		public:

			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			/*!	\brief
			*/
			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			static void SetUpTestSuite() {

				VknPhysicalDeviceTest::SetUpTestSuite();

				s_vkn_queue_family_create_info.index = 0;
				s_vkn_queue_family_create_info.vkn_commands = VknCommandsInstanceTest::getTestObject();
				s_vkn_queue_family_create_info.vkn_physical_device = VknPhysicalDeviceTest::getTestObject();
				
				if (s_vkn_queue_family.getCreateInfo().vkn_commands == nullptr) {
					s_vkn_queue_family.create(s_vkn_queue_family_create_info);
				}
			}

			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			/*!	\brief
			*/
			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			static void TearDownTestSuite(void) {
				s_vkn_queue_family.destroy();
				VknPhysicalDeviceTest::TearDownTestSuite();
			}

		public:

			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			/*!	\brief
			*/
			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			inline static const CGDev::GPU::Private::Vulkan::VknQueueFamilyPtr getTestObject(void) noexcept {
				return &s_vkn_queue_family;
			}

		protected:

			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			/*!	\brief
			*/
			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			void SetUp() override {

				m_vkn_queue_family_create_info.index = 0;
				m_vkn_queue_family_create_info.vkn_commands = VknCommandsInstanceTest::getTestObject();
				m_vkn_queue_family_create_info.vkn_physical_device = VknPhysicalDeviceTest::getTestObject();
			}

			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			/*!	\brief
			*/
			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			void TearDown() override {

			
			}

		protected:

			inline static CGDev::GPU::Private::Vulkan::VknQueueFamily					s_vkn_queue_family;
			inline static CGDev::GPU::Private::Vulkan::VknQueueFamilyCreateInfo			s_vkn_queue_family_create_info;

			CGDev::GPU::Private::Vulkan::VknQueueFamily						m_vkn_queue_family;
			CGDev::GPU::Private::Vulkan::VknQueueFamilyCreateInfo			m_vkn_queue_family_create_info;
		};

	} // namespace Tests

} // namespace CGDev

#endif // CGDEV_SOURCE_TESTS__VKN_QUEUE_FAMILY_H
