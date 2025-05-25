#ifndef CGDEV_SOURCE_TESTS__VKN_LOGICAL_DEVICE_H
#define CGDEV_SOURCE_TESTS__VKN_LOGICAL_DEVICE_H
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
#include "Private/Vulkan/vkn_logical_device.h"		// что тестируем

#include "vkn_commands_instance_test.h"
#include "vkn_physical_device_test.h"
#include "vkn_queue_family_test.h"
#include "vkn_ext_debug_utils_test.h"

namespace CGDev {

	namespace Tests {

		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		/*!	\brief
		*/
		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		class VknLogicalDeviceTest : public ::testing::Test {

		public:

			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			/*!	\brief
			*/
			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			static void SetUpTestSuite() {

				VknCommandsInstanceTest::SetUpTestSuite();
				VknExtDebugUtilsTest::SetUpTestSuite();
				VknPhysicalDeviceTest::SetUpTestSuite();
				VknQueueFamilyTest::SetUpTestSuite();

				s_vkn_logical_device_queue_create_info.vkn_queue_family = VknQueueFamilyTest::getTestObject();
				s_vkn_logical_device_queue_create_info.queue_count = 1;
				s_vkn_logical_device_queue_create_info.priority_collection = { 2.0f };

				s_vkn_logical_device_create_info.vkn_logical_device_queue_create_info_collection = { s_vkn_logical_device_queue_create_info };
				s_vkn_logical_device_create_info.vkn_commands = VknCommandsInstanceTest::getTestObject();
				s_vkn_logical_device_create_info.vkn_physical_device_collection = { VknPhysicalDeviceTest::getTestObject() };

				s_vkn_logical_device.create(s_vkn_logical_device_create_info);
			}

			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			/*!	\brief
			*/
			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			static void TearDownTestSuite(void) {
				s_vkn_logical_device.destroy();
			
				VknPhysicalDeviceTest::TearDownTestSuite();
				VknExtDebugUtilsTest::TearDownTestSuite();
				VknCommandsInstanceTest::TearDownTestSuite();
			}

		protected:

			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			/*!	\brief
			*/
			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			void SetUp() override {	
				m_vkn_logical_device_queue_create_info.vkn_queue_family = VknQueueFamilyTest::getTestObject();
				m_vkn_logical_device_queue_create_info.queue_count = 1;
				m_vkn_logical_device_queue_create_info.priority_collection = { 1.0f };

				m_vkn_logical_device_create_info.vkn_logical_device_queue_create_info_collection = { s_vkn_logical_device_queue_create_info };
				m_vkn_logical_device_create_info.vkn_commands = VknCommandsInstanceTest::getTestObject();
				m_vkn_logical_device_create_info.vkn_physical_device_collection = { VknPhysicalDeviceTest::getTestObject() };
			}

			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			/*!	\brief
			*/
			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			void TearDown() override {
			}

		protected:

			inline static CGDev::GPU::Private::Vulkan::VknLogicalDevice						s_vkn_logical_device;
			inline static CGDev::GPU::Private::Vulkan::VknLogicalDeviceCreateInfo			s_vkn_logical_device_create_info;
			inline static CGDev::GPU::Private::Vulkan::VknLogicalDeviceQueueCreateInfo		s_vkn_logical_device_queue_create_info;

			CGDev::GPU::Private::Vulkan::VknLogicalDevice					m_vkn_logical_device;
			CGDev::GPU::Private::Vulkan::VknLogicalDeviceCreateInfo			m_vkn_logical_device_create_info;
			CGDev::GPU::Private::Vulkan::VknLogicalDeviceQueueCreateInfo	m_vkn_logical_device_queue_create_info;
		};

	} // namespace Tests

} // namespace CGDev

#endif // CGDEV_SOURCE_TESTS__VKN_LOGICAL_DEVICE_H
