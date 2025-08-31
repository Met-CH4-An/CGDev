// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025 Metelev Met-CH4-An Andrey
// Project Lead: Metelev Andrey
// Main Technical Assistant: StarDev aka ChatGPT
#ifndef CGDEV_WVK_SOURCE__WVK_COMMAND_BUFFER_H
#define CGDEV_WVK_SOURCE__WVK_COMMAND_BUFFER_H
////////////////////////////////////////////////////////////////
// секция форвард-декларации
////////////////////////////////////////////////////////////////
#include "_wvk.h"
////////////////////////////////////////////////////////////////
// секция имплементации
////////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////
// секция родительского класса
////////////////////////////////////////////////////////////////
#include "wvk_base_object.h"
////////////////////////////////////////////////////////////////
// секция для остального
////////////////////////////////////////////////////////////////
#include "wvk_command_buffer_helpers.h"
#include "wvk_logical_device.h"
#include "wvk_dispatch_table.h"
#include "Extensions/wvk_swapchain.h"

namespace CGDev {

	namespace wvk {

		// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		/*!	\brief
		*/
		// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		struct WvkImageLocation {
			WvkImagePtr wvk_image = nullptr;
			std::optional<uint32_t> location;
		}; // struct WvkImageLocation
		using WvkImageLocationVec1 = std::vector<WvkImageLocation>;

		// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		/*!	\brief
		*/
		// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		struct WvkCommandBufferCreateInfo {
			WvkLogicalDevicePtr wvk_logical_device_ptr = nullptr;
			VkCommandBuffer vk_cmd_buffer = VK_NULL_HANDLE;
		}; // struct WvkCommandBufferCreateInfo

		// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		/*!	\brief
		*/
		// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		class WvkCommandBuffer : public GpuObject {

		public:

			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			/*!	\brief
			*/
			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			WvkCommandBuffer(void) noexcept;

			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			/*!	\brief
			*/
			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			~WvkCommandBuffer(void) noexcept;

			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			/*!	\brief
			*/
			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			WvkStatus create(const WvkCommandBufferCreateInfo& create_info) noexcept;

			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			/*!	\brief
			*/
			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			void destroy(void) noexcept;

		// cpp
		public:

		// hpp
		public:

			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			/*!	\brief
			*/
			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			inline WvkStatus reset(const VkCommandBufferResetFlags& flags = 0) const noexcept;

			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			/*!	\brief
			*/
			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			inline WvkStatus begin(const VkCommandBufferBeginInfo& info) const noexcept;

			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			/*!	\brief
			*/
			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			inline WvkStatus begin(const WvkCommandBufferBeginHelper& helper) const noexcept;
			
			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			/*!	\brief
			*/
			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			inline void end(Extensions::WvkSwapchainPtr swap, uint32_t index) const noexcept {
				m_create_info.wvk_logical_device_ptr->getWvkDispatchTable()->wvkEndCommandBuffer(m_vk_command_buffer);
				/*if (_vk_result != VK_SUCCESS) {
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
				}*/

				VkQueue vk_queue;
				m_create_info.wvk_logical_device_ptr->getWvkDispatchTable()->wvkGetDeviceQueue(nullptr, 0, 0, &vk_queue);

				VkSubmitInfo submitInfo{
					.sType = VK_STRUCTURE_TYPE_SUBMIT_INFO,
					.commandBufferCount = 1,
					.pCommandBuffers = &m_vk_command_buffer,
				};
				m_create_info.wvk_logical_device_ptr->getWvkDispatchTable()->wvkQueueSubmit(vk_queue, 1, &submitInfo, VK_NULL_HANDLE);

				VkSwapchainKHR _swapchain = swap->getSwap();

				VkPresentInfoKHR presentInfo{};
				presentInfo.sType = VK_STRUCTURE_TYPE_PRESENT_INFO_KHR;
				presentInfo.swapchainCount = 1;
				presentInfo.pSwapchains = &_swapchain;
				presentInfo.pImageIndices = &index;
				m_create_info.wvk_logical_device_ptr->getWvkDispatchTable()->wvkQueuePresentKHR(vk_queue, &presentInfo);
			}

			inline WvkStatus cmdBeginRendering(VkImageView vk_image_view) {

				VkRenderingAttachmentInfo colorAttachment = {
					.sType = VK_STRUCTURE_TYPE_RENDERING_ATTACHMENT_INFO,
					.pNext = nullptr,
					.imageView = vk_image_view,  // imageView текущего swapchain image
					.imageLayout = VK_IMAGE_LAYOUT_ATTACHMENT_OPTIMAL_KHR,
					.resolveMode = VK_RESOLVE_MODE_NONE,
					.resolveImageView = VK_NULL_HANDLE,
					.resolveImageLayout = VK_IMAGE_LAYOUT_UNDEFINED,
					.loadOp = VK_ATTACHMENT_LOAD_OP_CLEAR,
					.storeOp = VK_ATTACHMENT_STORE_OP_STORE,
					.clearValue = {.color = { {0.0f, 1.0f, 0.0f, 1.0f} } } // очистка в чёрный
				};
				VkRenderingInfo renderingInfo = {
					.sType = VK_STRUCTURE_TYPE_RENDERING_INFO,
					.pNext = nullptr,
					.flags = 0,
					.renderArea = {
						.offset = {0, 0},
						.extent = {800, 600}     // ширина/высота из swapchain
					},
					.layerCount = 1,
					.viewMask = 0,
					.colorAttachmentCount = 1,
					.pColorAttachments = &colorAttachment,
					.pDepthAttachment = nullptr,
					.pStencilAttachment = nullptr
				};
				m_create_info.wvk_logical_device_ptr->getWvkDispatchTable()->wvkCmdBeginRendering(m_vk_command_buffer, &renderingInfo);

				return WvkStatus(VknStatusCode::SUCCESSFUL);
			}

			inline WvkStatus cmdEndRendering() {
				m_create_info.wvk_logical_device_ptr->getWvkDispatchTable()->wvkCmdEndRendering(m_vk_command_buffer);
				return WvkStatus(VknStatusCode::SUCCESSFUL);
			}

			inline void Bari(VkImage vk_image, VkImageLayout old, VkImageLayout neww) {
				VkImageMemoryBarrier barrierToColor = {
					.sType = VK_STRUCTURE_TYPE_IMAGE_MEMORY_BARRIER,
					.pNext = nullptr,
					.srcAccessMask = 0,
					.dstAccessMask = VK_ACCESS_COLOR_ATTACHMENT_WRITE_BIT,
					.oldLayout = old,              // первый кадр
					//.oldLayout = VK_IMAGE_LAYOUT_PRESENT_SRC_KHR,      // для следующих кадров
					.newLayout = neww,
					.srcQueueFamilyIndex = VK_QUEUE_FAMILY_IGNORED,
					.dstQueueFamilyIndex = VK_QUEUE_FAMILY_IGNORED,
					.image = vk_image,
					.subresourceRange = {
						.aspectMask = VK_IMAGE_ASPECT_COLOR_BIT,
						.baseMipLevel = 0,
						.levelCount = 1,
						.baseArrayLayer = 0,
						.layerCount = 1
					}
				};
				m_create_info.wvk_logical_device_ptr->getWvkDispatchTable()->wvkCmdPipelineBarrier(m_vk_command_buffer, VK_PIPELINE_STAGE_TOP_OF_PIPE_BIT, VK_PIPELINE_STAGE_COLOR_ATTACHMENT_OUTPUT_BIT, 0, 0, nullptr, 0, nullptr, 1, &barrierToColor);
				
			}
		// hpp
		private:

			friend class WvkCommandPool;
			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			/*!	\brief
			*/
			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			inline VkCommandBuffer getVkCommandBuffer(void) const noexcept;

		// cpp
		private:

			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			/*!	\brief
			*/
			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			WvkStatus validationCreateInfo(const WvkCommandBufferCreateInfo& create_info) noexcept;

			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			/*!	\brief
			*/
			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			WvkStatus createVkCommandBuffer(void) noexcept;

		private:

			WvkCommandBufferCreateInfo m_create_info = {};
			VkCommandBuffer m_vk_command_buffer = VK_NULL_HANDLE;
		}; // class WvkCommandBuffer

	} // namespace wvk

} // namespace CGDev

#include "wvk_command_buffer.hpp"

#endif // CGDEV_WVK_SOURCE__WVK_COMMAND_BUFFER_H
