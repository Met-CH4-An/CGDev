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

	//namespace GPU {

		//namespace Private {

			namespace wvk {

				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

				WvkStatus::WvkStatus(void) noexcept {
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

				const char* WvkStatus::getMessage(void) const noexcept {
					return m_message;
				}

				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

				size_t WvkStatus::getLength(void) const noexcept {
					return m_length;
				}

				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

				WvkStatus& WvkStatus::set(VknStatusCode code, const char* fmt, ...) noexcept {
					if (code != VknStatusCode::SUCCESSFUL) {
						m_code = code;
					}

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

				inline WvkStatus::operator bool(void) const noexcept {
					return m_code == VknStatusCode::SUCCESSFUL;
				}

				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

				WvkStatus& WvkStatus::append(const char* fmt, ...) noexcept {
                    va_list args;
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

                    va_end(args);

					return *this;
				}

				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

			} // namespace wvk

		//} // namespace Private

	//} // namespace GPU

} // namespace CGDev

#endif // CGDEV_SOURCE_GPU_PRIVATE_VULKAN__VKN_STATUS_HPP
