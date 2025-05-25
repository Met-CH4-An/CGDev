////////////////////////////////////////////////////////////////
// ������ �������-����������
////////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////
// ������ ������������� �����
////////////////////////////////////////////////////////////////
#include "vkn_ext_debug_utils_test.h"
////////////////////////////////////////////////////////////////
// ������ �������������
////////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////
// ������ ��� ����������
////////////////////////////////////////////////////////////////

namespace CGDev {

	namespace Tests {

		// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		// ��������:
		// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

        TEST_F(VknExtDebugUtilsTest, ValidationFailsIfCommandsIsNullptr) {
            m_vkn_ext_debug_utils_create_info.vkn_debug_utils_commands = nullptr;

            CGDev::GPU::Private::Vulkan::VknStatus _status = m_vkn_ext_debug_utils.create(m_vkn_ext_debug_utils_create_info); // ��������� ���������

            EXPECT_EQ(_status.m_code, CGDev::GPU::Private::Vulkan::VknStatusCode::FAIL);
        }

        // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
        // ��������:
        // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

        TEST_F(VknExtDebugUtilsTest, ValidationFailsIfModeIsUnknown) {
            m_vkn_ext_debug_utils_create_info.mode = CGDev::GPU::Private::Vulkan::Extensions::VknDebugUtilsMode::UNKNOWN;

            CGDev::GPU::Private::Vulkan::VknStatus _status = m_vkn_ext_debug_utils.create(m_vkn_ext_debug_utils_create_info); // ��������� ���������

            EXPECT_EQ(_status.m_code, CGDev::GPU::Private::Vulkan::VknStatusCode::FAIL);
        }

        // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
        // ��������:
        // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

        TEST_F(VknExtDebugUtilsTest, ValidationSucceedsWithValidInput) {
            CGDev::GPU::Private::Vulkan::VknStatus _status = m_vkn_ext_debug_utils.create(m_vkn_ext_debug_utils_create_info); // ��������� ���������

            EXPECT_EQ(_status.m_code, CGDev::GPU::Private::Vulkan::VknStatusCode::SUCCESSFUL);
        }

        // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
        // ��������:
        // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

        TEST_F(VknExtDebugUtilsTest, HasStatusReturnsFalseWhenEmpty) {
            m_vkn_ext_debug_utils.create(m_vkn_ext_debug_utils_create_info); // ��������� ���������

            EXPECT_FALSE(m_vkn_ext_debug_utils.hasStatus(CGDev::GPU::Private::Vulkan::Extensions::VknDebugUtilsMode::ALL_TYPES | CGDev::GPU::Private::Vulkan::Extensions::VknDebugUtilsMode::ALL_SEVERITIES));
        }

		
	}

} // namespace CGDev