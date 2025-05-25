////////////////////////////////////////////////////////////////
// ������ �������-����������
////////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////
// ������ ������������� �����
////////////////////////////////////////////////////////////////
#include "vkn_queue_family_test.h"
////////////////////////////////////////////////////////////////
// ������ �������������
////////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////
// ������ ��� ����������
////////////////////////////////////////////////////////////////

namespace CGDev {

	namespace Tests {

		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		// ��������:
		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

		TEST_F(VknQueueFamilyTest, Create_WithValidInfo_ReturnsSuccess) {
			CGDev::GPU::Private::Vulkan::VknStatus _status = m_vkn_queue_family.create(m_vkn_queue_family_create_info);
			EXPECT_EQ(_status.m_code, CGDev::GPU::Private::Vulkan::VknStatusCode::SUCCESSFUL);
		}

		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		// ��������:
		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

		TEST_F(VknQueueFamilyTest, Create_MissingIndex_ReturnsError) {
			m_vkn_queue_family_create_info.index.reset(); // ������� ��������
			CGDev::GPU::Private::Vulkan::VknStatus _status = m_vkn_queue_family.create(m_vkn_queue_family_create_info);
			EXPECT_EQ(_status.m_code, CGDev::GPU::Private::Vulkan::VknStatusCode::CREATE_INFO_NO_VALID);
		}

		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		// ��������:
		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

		TEST_F(VknQueueFamilyTest, Create_NullVknCommands_ReturnsError) {
			m_vkn_queue_family_create_info.vkn_commands = nullptr;
			CGDev::GPU::Private::Vulkan::VknStatus _status = m_vkn_queue_family.create(m_vkn_queue_family_create_info);
			EXPECT_EQ(_status.m_code, CGDev::GPU::Private::Vulkan::VknStatusCode::CREATE_INFO_NO_VALID);
		}

		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		// ��������:
		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

		TEST_F(VknQueueFamilyTest, Create_NullPhysicalDevice_ReturnsError) {
			m_vkn_queue_family_create_info.vkn_physical_device = nullptr;
			CGDev::GPU::Private::Vulkan::VknStatus _status = m_vkn_queue_family.create(m_vkn_queue_family_create_info);
			EXPECT_EQ(_status.m_code, CGDev::GPU::Private::Vulkan::VknStatusCode::CREATE_INFO_NO_VALID);
		}

		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		// ��������:
		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

		TEST_F(VknQueueFamilyTest, RequestQueueFamilyProperties_ReturnsCorrectFamily) {

			m_vkn_queue_family.create(m_vkn_queue_family_create_info);

			// ����� ������������ ������
			VkQueueFamilyProperties _props = {};
			m_vkn_queue_family.requestQueueFamilyProperties(_props);

			// �������, ��� props ������ �������� �������� ����������
			EXPECT_GT(_props.queueCount, 0u);
			EXPECT_TRUE(_props.queueFlags & VK_QUEUE_GRAPHICS_BIT); // ��� ����� ������ ��������� ����
		}

		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		// ��������:
		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

		TEST_F(VknQueueFamilyTest, RequestQueueFamilyProperties2_ReturnsValidData) {
			
			m_vkn_queue_family.create(m_vkn_queue_family_create_info);

			// ����� ������������ ������
			VkQueueFamilyProperties2 _properties2 = {};
			m_vkn_queue_family.requestQueueFamilyProperties(_properties2);

			// �������� �� ���������� ����������
			EXPECT_GT(_properties2.queueFamilyProperties.queueCount, 0u);
			EXPECT_TRUE(_properties2.queueFamilyProperties.queueFlags & VK_QUEUE_GRAPHICS_BIT); // ��� ������ ��������� ����
		}

		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		// ��������:
		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

		TEST_F(VknQueueFamilyTest, RequestQueueFamilyProperties_ShouldFillExtensionProperties) {
			
			m_vkn_queue_family.create(m_vkn_queue_family_create_info);

			// ������������� ��������� ��� �����
			VkQueueFamilyGlobalPriorityPropertiesEXT testPriorityProperties = { VK_STRUCTURE_TYPE_QUEUE_FAMILY_GLOBAL_PRIORITY_PROPERTIES };

			// ����� ������
			m_vkn_queue_family.requestQueueFamilyProperties(testPriorityProperties);

			// �������� �������� (�������)
			EXPECT_EQ(testPriorityProperties.sType, VK_STRUCTURE_TYPE_QUEUE_FAMILY_GLOBAL_PRIORITY_PROPERTIES_EXT);
			// ��������, ��� �������� ���� ���������
			EXPECT_GT(testPriorityProperties.priorityCount, 0u);  // ��������, ��� ���������� ����������� ������ 0



			VkQueueFamilyGlobalPriorityPropertiesEXT _vk_queue_family_global_priority_properties = { VK_STRUCTURE_TYPE_QUEUE_FAMILY_GLOBAL_PRIORITY_PROPERTIES };
			VkQueueFamilyVideoPropertiesKHR _vk_queue_family_video_props = { VK_STRUCTURE_TYPE_QUEUE_FAMILY_VIDEO_PROPERTIES_KHR };
			VkQueueFamilyQueryResultStatusPropertiesKHR _vk_queue_family_Query_result_status_props = { VK_STRUCTURE_TYPE_QUEUE_FAMILY_QUERY_RESULT_STATUS_PROPERTIES_KHR };

			m_vkn_queue_family.requestQueueFamilyProperties(_vk_queue_family_global_priority_properties);
			m_vkn_queue_family.requestQueueFamilyProperties(_vk_queue_family_video_props);
			m_vkn_queue_family.requestQueueFamilyProperties(_vk_queue_family_Query_result_status_props);

			_vk_queue_family_global_priority_properties = { VK_STRUCTURE_TYPE_QUEUE_FAMILY_GLOBAL_PRIORITY_PROPERTIES };
			_vk_queue_family_video_props = { VK_STRUCTURE_TYPE_QUEUE_FAMILY_VIDEO_PROPERTIES_KHR };
			_vk_queue_family_Query_result_status_props = { VK_STRUCTURE_TYPE_QUEUE_FAMILY_QUERY_RESULT_STATUS_PROPERTIES_KHR };

			m_vkn_queue_family.requestQueueFamilyProperties(_vk_queue_family_global_priority_properties, _vk_queue_family_video_props, _vk_queue_family_Query_result_status_props);

		}

		TEST_F(VknQueueFamilyTest, RequestQueueFamilyProperties_FillsStructs) {

			m_vkn_queue_family.create(m_vkn_queue_family_create_info);

			VkQueueFamilyGlobalPriorityPropertiesEXT priorityProps{};
			priorityProps.sType = VK_STRUCTURE_TYPE_QUEUE_FAMILY_GLOBAL_PRIORITY_PROPERTIES_EXT;

			VkQueueFamilyVideoPropertiesKHR videoProps{};
			videoProps.sType = VK_STRUCTURE_TYPE_QUEUE_FAMILY_VIDEO_PROPERTIES_KHR;

			// �� ������ ����� ���������� "�������" �������� ��� ��������-��������
			m_vkn_queue_family.requestQueueFamilyProperties(priorityProps, videoProps);

			// ��������, ��� sType ������� ���������� (���������� ��������)
			EXPECT_EQ(priorityProps.sType, VK_STRUCTURE_TYPE_QUEUE_FAMILY_GLOBAL_PRIORITY_PROPERTIES_EXT);
			EXPECT_EQ(videoProps.sType, VK_STRUCTURE_TYPE_QUEUE_FAMILY_VIDEO_PROPERTIES_KHR);

			// � ����� ����������� ����� ����� ��������� ����, ������� ������� �����������
		}

		TEST_F(VknQueueFamilyTest, getIndexFamily) {
			m_vkn_queue_family.create(m_vkn_queue_family_create_info);

			EXPECT_EQ(m_vkn_queue_family.getIndexFamily(), m_vkn_queue_family_create_info.index);
		}

	} // namespace Tests

} // namespace CGDev