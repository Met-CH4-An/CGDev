#ifndef CGDEV_WVK_SOURCE__WVK_COMMAND_BUFFER_HPP
#define CGDEV_WVK_SOURCE__WVK_COMMAND_BUFFER_HPP
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
#include "wvk_dispatch_table.h"
#include "wvk_logical_device.h"

namespace CGDev {

	namespace wvk {

		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

		inline VkCommandBuffer WvkCommandBuffer::getVkCommandBuffer(void) const noexcept {
			return m_vk_command_buffer;
		}

		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

		inline WvkStatus WvkCommandBuffer::reset(const VkCommandBufferResetFlags& flags) const noexcept {
			WvkStatus _status;

			auto _vk_result = m_create_info.wvk_logical_device_ptr->getDispatchTable()->wvkResetCommandBuffer(m_vk_command_buffer, flags);
			if (_vk_result != VK_SUCCESS) {
				switch (_vk_result) {
				case VK_ERROR_OUT_OF_DEVICE_MEMORY:
					_status.append("\nwvkResetCommandBuffer is VK_ERROR_OUT_OF_DEVICE_MEMORY.");
					break;
				case VK_ERROR_UNKNOWN:
					_status.append("\nwvkResetCommandBuffer is VK_ERROR_UNKNOWN.");
					break;
				case VK_ERROR_VALIDATION_FAILED_EXT:
					_status.append("\nwvkResetCommandBuffer is VK_ERROR_VALIDATION_FAILED.");
					break;
				}
				return _status.setFail();
			}

			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			// Успех
			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			return _status.setOk();
		}

		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

		inline WvkStatus WvkCommandBuffer::begin(const VkCommandBufferUsageFlags& flags, const void* pNext, VkCommandBufferInheritanceInfo* inheritance) const noexcept {
			WvkStatus _status;

			VkCommandBufferBeginInfo _vk_begin_info = {
				.sType = VK_STRUCTURE_TYPE_COMMAND_BUFFER_BEGIN_INFO,
				.pNext = pNext,
				.flags = flags,
				.pInheritanceInfo = inheritance,
			};

			auto _vk_result = m_create_info.wvk_logical_device_ptr->getDispatchTable()->wvkBeginCommandBuffer(m_vk_command_buffer, &_vk_begin_info);
			if (_vk_result != VK_SUCCESS) {
				switch (_vk_result) {
				case VK_ERROR_OUT_OF_DEVICE_MEMORY:
					_status.append("\nwvkResetCommandBuffer is VK_ERROR_OUT_OF_DEVICE_MEMORY.");
					break;
				case VK_ERROR_OUT_OF_HOST_MEMORY:
					_status.append("\nwvkResetCommandBuffer is VK_ERROR_OUT_OF_HOST_MEMORY.");
					break;
				case VK_ERROR_UNKNOWN:
					_status.append("\nwvkResetCommandBuffer is VK_ERROR_UNKNOWN.");
					break;
				case VK_ERROR_VALIDATION_FAILED_EXT:
					_status.append("\nwvkResetCommandBuffer is VK_ERROR_VALIDATION_FAILED.");
					break;
				}
				return _status.setFail();
			}

			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			// Успех
			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			return _status.setOk();
		}

		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

		inline WvkStatus WvkCommandBuffer::begin(void) const noexcept {
			WvkStatus _status;

			begin(0, static_cast<void*>(nullptr), static_cast<VkCommandBufferInheritanceInfo*>(nullptr));

			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			// Успех
			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			return _status.setOk();
		}

		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

		inline WvkStatus WvkCommandBuffer::begin(const VkCommandBufferUsageFlags& flags) const noexcept {
			WvkStatus _status;

			begin(flags, static_cast<void*>(nullptr), static_cast<VkCommandBufferInheritanceInfo*>(nullptr));

			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			// Успех
			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			return _status.setOk();
		}

		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

		template<typename Type, typename ... Args, typename>
		inline WvkStatus WvkCommandBuffer::begin(const VkCommandBufferUsageFlags& flags, Type& type, Args& ... args) const noexcept {
			WvkStatus _status;

			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			// Связываем цепочки pNext
			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			void* prev = &type;
			((reinterpret_cast<decltype(&args)>(prev)->pNext = &args, prev = &args), ...);

			begin(flags, static_cast<void*>(&type), static_cast<VkCommandBufferInheritanceInfo*>(nullptr));

			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			// Успех
			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			return _status.setOk();
		}

		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

		inline WvkStatus WvkCommandBuffer::begin(const VkCommandBufferUsageFlags& flags, const uint32_t& dev_indices, const void* pNext) const noexcept {
			WvkStatus _status;

			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			// 
			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
#if WVK_VULKAN_API_VERSION >= WVK_VULKAN_API_VERSION_11 || WVK_KHR_device_group_creation == WVK_ENABLE
			VkDeviceGroupCommandBufferBeginInfo _vk_dev_group_cmd_buffer_begin_info = {
				.sType = VK_STRUCTURE_TYPE_DEVICE_GROUP_COMMAND_BUFFER_BEGIN_INFO,
				.pNext = pNext,
				.deviceMask = dev_indices,
			};

			const void* _pNext = &_vk_dev_group_cmd_buffer_begin_info;
#else
			const void* _pNext = pNext;
#endif

			begin(flags, _pNext, static_cast<VkCommandBufferInheritanceInfo*>(nullptr));

			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			// Успех
			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			return _status.setOk();
		}

		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

		inline WvkStatus WvkCommandBuffer::begin(const VkCommandBufferUsageFlags& flags, const uint32_t& dev_indices) const noexcept {
			WvkStatus _status;

			begin(flags, dev_indices, static_cast<void*>(nullptr));

			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			// Успех
			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			return _status.setOk();
		}

		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

		template<typename Type, typename ... Args>
		inline WvkStatus WvkCommandBuffer::begin(const VkCommandBufferUsageFlags& flags, const uint32_t& dev_indices, Type& type, Args& ... args) const noexcept {
			WvkStatus _status;

			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			// Связываем цепочки pNext
			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			void* prev = &type;
			((reinterpret_cast<decltype(&args)>(prev)->pNext = &args, prev = &args), ...);

			begin(flags, dev_indices, static_cast<void*>(&type));

			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			// Успех
			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			return _status.setOk();
		}

		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

		inline WvkStatus WvkCommandBuffer::begin(const VkCommandBufferUsageFlags& vk_cmd_buffer_usage_flags, const WvkRenderPassPtr wvk_render_pass, const WvkFrameBufferPtr wvk_frame_buffer, const bool& query_enable, const VkQueryControlFlags& vk_query_control_flags, const VkQueryPipelineStatisticFlags& vk_query_pipeline_stats_flags, const void* pNext) const noexcept {
			WvkStatus _status;

			VkCommandBufferInheritanceInfo _inheritance_info = {
				.sType = VK_STRUCTURE_TYPE_COMMAND_BUFFER_INHERITANCE_INFO,
				.pNext = pNext,
				.renderPass = nullptr,
				.subpass = 0,
				.framebuffer = nullptr,
				.occlusionQueryEnable = query_enable,
				.queryFlags = vk_query_control_flags,
				.pipelineStatistics = vk_query_pipeline_stats_flags,
			};

			begin(vk_cmd_buffer_usage_flags, static_cast<const void*>(nullptr), &_inheritance_info);

			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			// Успех
			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			return _status.setOk();
		}

		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

		inline WvkStatus WvkCommandBuffer::begin(const VkCommandBufferUsageFlags& vk_cmd_buffer_usage_flags, const WvkRenderPassPtr wvk_render_pass, const WvkFrameBufferPtr wvk_frame_buffer, const bool& query_enable, const VkQueryControlFlags& vk_query_control_flags, const VkQueryPipelineStatisticFlags& vk_query_pipeline_stats_flags) const noexcept {
			WvkStatus _status;

			begin(vk_cmd_buffer_usage_flags, wvk_render_pass, wvk_frame_buffer, query_enable, vk_query_control_flags, vk_query_pipeline_stats_flags, static_cast<const void*>(nullptr));

			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			// Успех
			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			return _status.setOk();
		}

		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

		inline WvkStatus WvkCommandBuffer::begin(const VkCommandBufferUsageFlags& vk_cmd_buffer_usage_flags, const bool& query_enable, const VkQueryControlFlags& vk_query_control_flags, const VkQueryPipelineStatisticFlags& vk_query_pipeline_stats_flags) const noexcept {
			WvkStatus _status;

			VkCommandBufferInheritanceRenderingInfo _inheritance_rendering_info = {
				.sType = VK_STRUCTURE_TYPE_COMMAND_BUFFER_INHERITANCE_RENDERING_INFO,
				.pNext = nullptr,
				.flags = 0,
				.viewMask = 0,
				.colorAttachmentCount = 0,
				//		.pColorAttachmentFormats = 0,
				//		//.depthAttachmentFormat = 0,
						//.stencilAttachmentFormat = 0,
						//.rasterizationSamples = 0,
			};

			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			// Успех
			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			return _status.setOk();
		}

		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

		inline WvkStatus WvkCommandBuffer::end(void) const noexcept {
			WvkStatus _status;

			auto _vk_result = m_create_info.wvk_logical_device_ptr->getDispatchTable()->wvkEndCommandBuffer(m_vk_command_buffer);
			if (_vk_result != VK_SUCCESS) {
				switch (_vk_result) {
				case VK_ERROR_OUT_OF_DEVICE_MEMORY:
					_status.append("\nwvkResetCommandBuffer is VK_ERROR_OUT_OF_DEVICE_MEMORY.");
					break;
				case VK_ERROR_OUT_OF_HOST_MEMORY:
					_status.append("\nwvkResetCommandBuffer is VK_ERROR_OUT_OF_HOST_MEMORY.");
					break;
				case VK_ERROR_UNKNOWN:
					_status.append("\nwvkResetCommandBuffer is VK_ERROR_UNKNOWN.");
					break;
				case VK_ERROR_VALIDATION_FAILED_EXT:
					_status.append("\nwvkResetCommandBuffer is VK_ERROR_VALIDATION_FAILED.");
					break;
				}
				return _status.setFail();
			}

			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			// Успех
			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			return _status.setOk();
		}

		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

	} // namespace wvk

} // namespace CGDev

#endif // CGDEV_WVK_SOURCE__WVK_COMMAND_BUFFER_HPP
