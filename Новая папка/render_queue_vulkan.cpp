////////////////////////////////////////////////////////////////
// ������ �������-����������
////////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////
// ������ ������������� �����
////////////////////////////////////////////////////////////////
#include "render_queue_vulkan.h"
////////////////////////////////////////////////////////////////
// ������ �������������
////////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////
// ������ ��� ����������
////////////////////////////////////////////////////////////////
#include "hcpi_library_vulkan.h"
#include "device_vulkan.h"
#include "command_pool.h"

namespace CGDev {

	namespace RPI {

		namespace Private {

			namespace Vulkan {

				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

				RenderQueueVulkan::RenderQueueVulkan(void) noexcept {
				}

				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

				RenderQueueVulkan::~RenderQueueVulkan(void) noexcept {
				}

				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

				void RenderQueueVulkan::create(void) noexcept {

					const auto& _device_vulkan = std::static_pointer_cast<DeviceVulkan>(m_create_info.device);

					// ~~~~~~~~~~~~~~~~
					// ������ CommandPool
					// ~~~~~~~~~~~~~~~~

					createCommandPool();

					if (this->isValid() == false) {

						Tools::addEntry(_device_vulkan->getCreateInfo().library->getLog(), Tools::LogEntryError("RenderQueueVulkan �� ���� ������� CommandPool"));

						m_valid = false;

						return;
					}

					m_valid = true;
				}

				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

				void RenderQueueVulkan::prepare(void) noexcept {

					// ~~~~~~~~~~~~~~~~
					// ���������� ������ m_render_buffer_collection
					// ���������� � ������ _render_buffer_collection ��� �������� ����� ��������
					// �������� ������ m_render_buffer_collection �������� _render_buffer_collection
					// !!! �� ���� �� ������� ��� ����������!!!
					// ~~~~~~~~~~~~~~~~

					//RenderBufferSptrArr _render_buffer_collection;
					//_render_buffer_collection.reserve(m_render_buffer_collection.size());

					for (size_t ct_0 = 0; ct_0 < m_render_buffer_updated_collection.size(); ++ct_0) {
						const auto& it_render_buffer = m_render_buffer_updated_collection[ct_0];

						//if (it_render_buffer != render_bufer) {

						//	_render_buffer_collection.emplace_back(it_render_buffer);
						//}

					} // for (size_t ct_0 = 0;

					//m_render_buffer_collection = std::move(_render_buffer_collection);

					m_valid = true;
				}

				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

				void RenderQueueVulkan::createCommandPool(void) noexcept {

					const auto& _device_vulkan = std::static_pointer_cast<DeviceVulkan>(m_create_info.device);

					// ~~~~~~~~~~~~~~~~
					// ��������� CommandPoolCreateInfo
					// ~~~~~~~~~~~~~~~~
				
					CommandPoolCreateInfo _command_pool_create_info		= {};
					_command_pool_create_info.log						= _device_vulkan->getCreateInfo().library->getLog();
					_command_pool_create_info.logical_device			= _device_vulkan->getLogicalDevice();

					// ~~~~~~~~~~~~~~~~
					// ������ CommandPool
					// ~~~~~~~~~~~~~~~~

					m_command_pool = std::make_shared<CommandPool>();

					m_command_pool->create(_command_pool_create_info);

					if (m_command_pool->isValid() == false) {

						Tools::addEntry(_device_vulkan->getCreateInfo().library->getLog(), Tools::LogEntryError("RenderQueueVulkan �� ���� ������� CommandPool"));

						m_valid = false;

						return;
					}

					m_valid = true;
				}

				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

			} // namespace Vulkan

		} // namespace Private

	} // namespace RPI

} // namespace CGDev