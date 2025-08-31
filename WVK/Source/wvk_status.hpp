// SPDX-License-Identifier: AGPL-3.0-or-later
#ifndef CGDEV_SOURCE_GPU_PRIVATE_VULKAN__VKN_STATUS_HPP
#define CGDEV_SOURCE_GPU_PRIVATE_VULKAN__VKN_STATUS_HPP
////////////////////////////////////////////////////////////////
// ������ �������-����������
////////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////
// ������ �������������
////////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////
// ������ ������������� ������
////////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////
// ������ ��� ����������
////////////////////////////////////////////////////////////////

namespace CGDev {

	namespace wvk {

		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

		WvkStatus::WvkStatus(void) noexcept {
		}

		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

		inline WvkStatus::WvkStatus(const VknStatusCode& code) noexcept
			:m_code(code) {
		}

		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

		WvkStatus::~WvkStatus(void) noexcept {
		}

		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

		inline bool WvkStatus::isOk(void) const noexcept {
			if (m_code == VknStatusCode::SUCCESSFUL) return true;

			return false;
		}

		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

		inline WvkStatus::operator bool(void) const noexcept {
			return isOk();
		}

		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

		inline const VknStatusCode& WvkStatus::getCode(void) const noexcept {
			return m_code;
		}

		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

		const char* WvkStatus::getMessage(void) const noexcept {
			return m_message.c_str();
		}

		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

		size_t WvkStatus::getLength(void) const noexcept {
			return m_length;
		}

		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

		WvkStatus& WvkStatus::set(VknStatusCode code, const char* fmt, ...) noexcept {
			//if (code != VknStatusCode::SUCCESSFUL) {
				m_code = code;
			//}

			if (fmt) {
				va_list args;
				va_start(args, fmt);
				append(fmt, args);
				va_end(args);
			}

			return *this;
		}

		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

		WvkStatus& WvkStatus::set(VknStatusCode code, const char* fmt, va_list args) noexcept {
			m_code = code;
			if (fmt) {
				append(fmt, args);
			}
			return *this;
		}

		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

		WvkStatus& WvkStatus::append(const char* fmt, ...) noexcept {
            /*va_list args;
            va_start(args, fmt);

            // ��������� ��������� ����� � ������
            size_t available = sizeof(m_message) - m_length;

            // ���������, ��� ����� ����������
            if (available > 0) {
                // ���������� vsnprintf_s ��� ���������� ������
                int written = vsnprintf_s(
                    m_message + m_length,   // ��� ������
                    available,              // ������� ����� ��������
                    _TRUNCATE,              // ������� ��� ������������
                    fmt,                    // ������
                    args                    // ���������
                );

                // ���� �������� ���-��, ��������� m_length
                if (written >= 0) {
                    m_length += written;
                }
            }

            va_end(args);*/

			constexpr size_t TEMP_BUFFER_SIZE = 1024;
			char temp[TEMP_BUFFER_SIZE];

			va_list args;
			va_start(args, fmt);

			int written = vsnprintf(temp, TEMP_BUFFER_SIZE, fmt, args);

			va_end(args);

			if (written > 0) {
				m_message.append(temp, written);
			}


			return *this;
		}

		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

		inline WvkStatus& WvkStatus::setOk(void) noexcept {
			return set(VknStatusCode::SUCCESSFUL);
		}

		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

		inline WvkStatus& WvkStatus::setFail(const char* fmt, ...) noexcept {
			va_list args;
			va_start(args, fmt);
			auto& _result = set(VknStatusCode::FAIL, fmt, args);
			va_end(args);
			return _result;
		}

		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

		inline WvkStatus& WvkStatus::operator << (const char* fmt) noexcept {
			return append(fmt);
		}

		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

	} // namespace wvk

} // namespace CGDev

#endif // CGDEV_SOURCE_GPU_PRIVATE_VULKAN__VKN_STATUS_HPP
