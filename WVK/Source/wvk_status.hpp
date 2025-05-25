#ifndef CGDEV_SOURCE_GPU_PRIVATE_VULKAN__VKN_STATUS_HPP
#define CGDEV_SOURCE_GPU_PRIVATE_VULKAN__VKN_STATUS_HPP
////////////////////////////////////////////////////////////////
// секция форвард-декларации
////////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////
// секция имплементации
////////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////
// секция родительского класса
////////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////
// секция для остального
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

                    // Вычисляем доступное место в буфере
                    size_t available = sizeof(m_message) - m_length;

                    // Проверяем, что места достаточно
                    if (available > 0) {
                        // Используем vsnprintf_s для безопасной записи
                        int written = vsnprintf_s(
                            m_message + m_length,   // Где писать
                            available,              // Сколько места осталось
                            _TRUNCATE,              // Обрезка при переполнении
                            fmt,                    // Формат
                            args                    // Аргументы
                        );

                        // Если записано что-то, обновляем m_length
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
