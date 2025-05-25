////////////////////////////////////////////////////////////////
// секция форвард-декларации
////////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////
// секция заголовочного файла
////////////////////////////////////////////////////////////////
#include "vkn_queue_family_test.h"
////////////////////////////////////////////////////////////////
// секция имплементации
////////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////
// секция для остального
////////////////////////////////////////////////////////////////

namespace CGDev {

	namespace Tests {

		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		// Проверка:
		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

		TEST_F(VknQueueFamilyTest, Create_WithValidInfo_ReturnsSuccess) {
			CGDev::GPU::Private::Vulkan::VknStatus _status = m_vkn_queue_family.create(m_vkn_queue_family_create_info);
			EXPECT_EQ(_status.m_code, CGDev::GPU::Private::Vulkan::VknStatusCode::SUCCESSFUL);
		}

		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		// Проверка:
		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

		TEST_F(VknQueueFamilyTest, Create_MissingIndex_ReturnsError) {
			m_vkn_queue_family_create_info.index.reset(); // Убираем значение
			CGDev::GPU::Private::Vulkan::VknStatus _status = m_vkn_queue_family.create(m_vkn_queue_family_create_info);
			EXPECT_EQ(_status.m_code, CGDev::GPU::Private::Vulkan::VknStatusCode::CREATE_INFO_NO_VALID);
		}

		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		// Проверка:
		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

		TEST_F(VknQueueFamilyTest, Create_NullVknCommands_ReturnsError) {
			m_vkn_queue_family_create_info.vkn_commands = nullptr;
			CGDev::GPU::Private::Vulkan::VknStatus _status = m_vkn_queue_family.create(m_vkn_queue_family_create_info);
			EXPECT_EQ(_status.m_code, CGDev::GPU::Private::Vulkan::VknStatusCode::CREATE_INFO_NO_VALID);
		}

		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		// Проверка:
		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

		TEST_F(VknQueueFamilyTest, Create_NullPhysicalDevice_ReturnsError) {
			m_vkn_queue_family_create_info.vkn_physical_device = nullptr;
			CGDev::GPU::Private::Vulkan::VknStatus _status = m_vkn_queue_family.create(m_vkn_queue_family_create_info);
			EXPECT_EQ(_status.m_code, CGDev::GPU::Private::Vulkan::VknStatusCode::CREATE_INFO_NO_VALID);
		}

		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		// Проверка:
		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

		TEST_F(VknQueueFamilyTest, RequestQueueFamilyProperties_ReturnsCorrectFamily) {

			m_vkn_queue_family.create(m_vkn_queue_family_create_info);

			// Вызов тестируемого метода
			VkQueueFamilyProperties _props = {};
			m_vkn_queue_family.requestQueueFamilyProperties(_props);

			// Ожидаем, что props теперь содержит валидную информацию
			EXPECT_GT(_props.queueCount, 0u);
			EXPECT_TRUE(_props.queueFlags & VK_QUEUE_GRAPHICS_BIT); // Или любой другой ожидаемый флаг
		}

		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		// Проверка:
		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

		TEST_F(VknQueueFamilyTest, RequestQueueFamilyProperties2_ReturnsValidData) {
			
			m_vkn_queue_family.create(m_vkn_queue_family_create_info);

			// Вызов тестируемого метода
			VkQueueFamilyProperties2 _properties2 = {};
			m_vkn_queue_family.requestQueueFamilyProperties(_properties2);

			// Проверки на валидность результата
			EXPECT_GT(_properties2.queueFamilyProperties.queueCount, 0u);
			EXPECT_TRUE(_properties2.queueFamilyProperties.queueFlags & VK_QUEUE_GRAPHICS_BIT); // или другой ожидаемый флаг
		}

		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		// Проверка:
		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

		TEST_F(VknQueueFamilyTest, RequestQueueFamilyProperties_ShouldFillExtensionProperties) {
			
			m_vkn_queue_family.create(m_vkn_queue_family_create_info);

			// Инициализация структуры для теста
			VkQueueFamilyGlobalPriorityPropertiesEXT testPriorityProperties = { VK_STRUCTURE_TYPE_QUEUE_FAMILY_GLOBAL_PRIORITY_PROPERTIES };

			// Вызов метода
			m_vkn_queue_family.requestQueueFamilyProperties(testPriorityProperties);

			// Проверка значений (условно)
			EXPECT_EQ(testPriorityProperties.sType, VK_STRUCTURE_TYPE_QUEUE_FAMILY_GLOBAL_PRIORITY_PROPERTIES_EXT);
			// Проверка, что значения были заполнены
			EXPECT_GT(testPriorityProperties.priorityCount, 0u);  // Проверка, что количество приоритетов больше 0



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

			// До вызова можно установить "нулевые" значения или значения-заглушки
			m_vkn_queue_family.requestQueueFamilyProperties(priorityProps, videoProps);

			// Проверим, что sType остался правильным (простейшая проверка)
			EXPECT_EQ(priorityProps.sType, VK_STRUCTURE_TYPE_QUEUE_FAMILY_GLOBAL_PRIORITY_PROPERTIES_EXT);
			EXPECT_EQ(videoProps.sType, VK_STRUCTURE_TYPE_QUEUE_FAMILY_VIDEO_PROPERTIES_KHR);

			// В более продвинутом тесте можно проверять поля, которые реально заполняются
		}

		TEST_F(VknQueueFamilyTest, getIndexFamily) {
			m_vkn_queue_family.create(m_vkn_queue_family_create_info);

			EXPECT_EQ(m_vkn_queue_family.getIndexFamily(), m_vkn_queue_family_create_info.index);
		}

	} // namespace Tests

} // namespace CGDev