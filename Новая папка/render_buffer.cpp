////////////////////////////////////////////////////////////////
// секция форвард-декларации
////////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////
// секция заголовочного файла
////////////////////////////////////////////////////////////////
#include "render_buffer.h"
#include "../rpi.h"
////////////////////////////////////////////////////////////////
// секция имплементации
////////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////
// секция для остального
////////////////////////////////////////////////////////////////
#include "hcpi_library.h"
#include "hcpi_device.h"
#include "render_queue.h"
#include "Vulkan/render_buffer_vulkan.h"

namespace CGDev {

	namespace RPI {

		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

		void createRenderBuffer(RenderBufferSptr& render_buffer, const RenderBufferCreateInfo& render_buffer_create_info) noexcept {

			RenderBufferSptr _render_buffer = std::make_shared<Private::Vulkan::RenderBufferVulkan>();

			_render_buffer->create(render_buffer_create_info);

			if (_render_buffer->isValid() == false) {

				render_buffer = nullptr;

				return;
			}

			render_buffer = std::move(_render_buffer);
		}

		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

		bool RenderBufferCreateInfo::isValid(void) const noexcept {

			if (device == nullptr) {

				return false;
			}

			return true;
		}

		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

		namespace Private {

			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

			RenderBuffer::RenderBuffer(void) noexcept {
			}

			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

			RenderBuffer::~RenderBuffer(void) noexcept {
			}

			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

			void RenderBuffer::create(const RenderBufferCreateInfo& render_buffer_create_info) noexcept {

				// ~~~~~~~~~~~~~~~~
				// проверяем RenderBufferCreateInfo на валидность
				// ~~~~~~~~~~~~~~~~

				if (render_buffer_create_info.isValid() == false) {

					m_valid = false;

					return;
				}

				m_create_info = render_buffer_create_info;

				// ~~~~~~~~~~~~~~~~
				// создаём RenderBufferPlatform
				// ~~~~~~~~~~~~~~~~

				static_cast<Vulkan::RenderBufferVulkan*>(this)->create();

				if (this->isValid() == false) {

					Tools::addEntry(m_create_info.device->getCreateInfo().library->getLog(), Tools::LogEntryError("Не удалось создать RenderBufferPlatform"));

					return;
				}
				
				m_valid = true;
			}

			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

		} // namespace Private

	} // namespace RPI

} // namespace CGDev