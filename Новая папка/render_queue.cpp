////////////////////////////////////////////////////////////////
// секция форвард-декларации
////////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////
// секция заголовочного файла
////////////////////////////////////////////////////////////////
#include "render_queue.h"
#include "../rpi.h"
////////////////////////////////////////////////////////////////
// секция имплементации
////////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////
// секция для остального
////////////////////////////////////////////////////////////////
#include "hcpi_library.h"
#include "hcpi_device.h"
#include "Vulkan/render_queue_vulkan.h"

namespace CGDev {

	namespace RPI {

		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

		void createRenderQueue(RenderQueueSptr& render_queue, const RenderQueueCreateInfo& create_info) noexcept {

			RenderQueueSptr _render_queue = std::make_shared<Private::Vulkan::RenderQueueVulkan>();

			_render_queue->create(create_info);

			if (_render_queue->isValid() == false) {

				_render_queue = nullptr;

				return;
			}

			render_queue = std::move(_render_queue);
		}

		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

		void pushRenderBuffer(RenderQueueSptr& render_queue, const RenderBufferSptr& render_buffer) noexcept {

			render_queue->pushRenderBuffer(render_buffer);
		}

		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

		void removeRenderBuffer(RenderQueueSptr& render_queue, const RenderBufferSptr& render_buffer) noexcept {

			render_queue->removeRenderBuffer(render_buffer);
		}

		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

		bool RenderQueueCreateInfo::isValid(void) const noexcept {

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

			RenderQueue::RenderQueue(void) noexcept {
			}

			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

			RenderQueue::~RenderQueue(void) noexcept {
			}

			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

			void RenderQueue::create(const RenderQueueCreateInfo& create_info) noexcept {

				// ~~~~~~~~~~~~~~~~
				// проверяем RenderQueueCreateInfo на валидность
				// ~~~~~~~~~~~~~~~~

				if (create_info.isValid() == false) {

					m_valid = false;

					return;
				}

				m_create_info = create_info;

				// ~~~~~~~~~~~~~~~~
				// создаём RenderQueuePlatform
				// ~~~~~~~~~~~~~~~~

				static_cast<Vulkan::RenderQueueVulkan*>(this)->create();

				if (this->isValid() == false) {

					Tools::addEntry(m_create_info.device->getCreateInfo().library->getLog(), Tools::LogEntryError("Не удалось создать RenderQueuePlatform"));

					return;
				}

				m_valid = true;
			}

			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

			void RenderQueue::pushRenderBuffer(const RenderBufferSptr& render_bufer) noexcept {

				// ~~~~~~~~~~~~~~~~
				// добавляем render_bufer в вектор m_render_buffer_collection
				// ~~~~~~~~~~~~~~~~

				m_render_buffer_collection.push_back(render_bufer);

				// ~~~~~~~~~~~~~~~~
				// добавляем render_bufer в вектор m_render_buffer_updated_collection
				// ~~~~~~~~~~~~~~~~

				m_render_buffer_updated_collection.push_back(render_bufer);
			}

			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

			void RenderQueue::removeRenderBuffer(const RenderBufferSptr& render_bufer) noexcept {

				// ~~~~~~~~~~~~~~~~
				// перебираем вектор m_render_buffer_collection
				// записываем в вектор _render_buffer_collection все элементы кроме искомого
				// заменяем вектор m_render_buffer_collection вектором _render_buffer_collection
				// !!! не знаю на сколько это эффективно!!!
				// ~~~~~~~~~~~~~~~~

				RenderBufferSptrArr _render_buffer_collection;
				_render_buffer_collection.reserve(m_render_buffer_collection.size());

				for (size_t ct_0 = 0; ct_0 < m_render_buffer_collection.size(); ++ct_0) {
					const auto& it_render_buffer = m_render_buffer_collection[ct_0];

					if (it_render_buffer != render_bufer) {

						_render_buffer_collection.emplace_back(it_render_buffer);
					}

				} // for (size_t ct_0 = 0;
				
				m_render_buffer_collection = std::move(_render_buffer_collection);

				// ~~~~~~~~~~~~~~~~
				// перебираем вектор m_render_buffer_updated_collection
				// записываем в вектор _render_buffer_collection все элементы кроме искомого
				// заменяем вектор m_render_buffer_updated_collection вектором _render_buffer_collection
				// !!! не знаю на сколько это эффективно!!!
				// ~~~~~~~~~~~~~~~~

				RenderBufferSptrArr _render_buffer_updated_collection;
				_render_buffer_updated_collection.reserve(m_render_buffer_updated_collection.size());

				for (size_t ct_0 = 0; ct_0 < m_render_buffer_updated_collection.size(); ++ct_0) {
					const auto& it_render_buffer = m_render_buffer_updated_collection[ct_0];

					if (it_render_buffer != render_bufer) {

						_render_buffer_updated_collection.emplace_back(it_render_buffer);
					}

				} // for (size_t ct_0 = 0;

				m_render_buffer_updated_collection = std::move(_render_buffer_updated_collection);

			}

			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

		} // namespace Private

	} // namespace RPI

} // namespace CGDev