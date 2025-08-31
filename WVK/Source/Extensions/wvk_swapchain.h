// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025 Metelev Met-CH4-An Andrey
// Project Lead: Metelev Andrey
// Main Technical Assistant: StarDev aka ChatGPT
#ifndef CGDEV_WVK_SOURCE_EXTENSION__WVK_SWAPCHAIN_H
#define CGDEV_WVK_SOURCE_EXTENSION__WVK_SWAPCHAIN_H
////////////////////////////////////////////////////////////////
// секция форвард-декларации
////////////////////////////////////////////////////////////////
#include "_extensions.h"
////////////////////////////////////////////////////////////////
// секция имплементации
////////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////
// секция родительского класса
////////////////////////////////////////////////////////////////
#include "../wvk_base_object.h"
////////////////////////////////////////////////////////////////
// секция для остального
////////////////////////////////////////////////////////////////
#include "../wvk_logical_device.h"
#include "../wvk_dispatch_table.h"
#include "../wvk_fence.h"

namespace CGDev {

	namespace wvk {

		namespace Extensions {

			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			/*!	\brief
			*/
			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			struct WvkSwapchainCreateInfo {
				WvkLogicalDevicePtr wvk_logical_device_ptr = nullptr;
				WvkSurfacePtr wvk_surface_ptr = nullptr;
			}; // struct WvkSwapchainCreateInfo



			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			/*!	\brief
			*/
			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			class WvkSwapchain : public GpuObject {

			public:

				// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				/*!	\brief
				*/
				// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				WvkSwapchain(void) noexcept;

				// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				/*!	\brief
				*/
				// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				~WvkSwapchain(void) noexcept;

				// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				/*!	\brief
				*/
				// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				WvkStatus create(const  WvkSwapchainCreateInfo& create_info) noexcept;

				// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				/*!	\brief
				*/
				// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				void destroy(void) noexcept;

			// hpp
			public:

				inline void Aqueue(WvkFencePtr wvk_fence_ptr, uint32_t& index) {
					m_create_info.wvk_logical_device_ptr->getWvkDispatchTable()->wvkAcquireNextImageKHR(m_vk_swapchain, UINT64_MAX, VK_NULL_HANDLE, wvk_fence_ptr->getVkFence(), &index);
				}

				inline std::vector<VkImageView> getImages() {
					uint32_t _count = 0;
					m_create_info.wvk_logical_device_ptr->getWvkDispatchTable()->wvkGetSwapchainImagesKHR(m_vk_swapchain, &_count, nullptr);
					std::vector<VkImage> _images(_count, VK_NULL_HANDLE);
					m_create_info.wvk_logical_device_ptr->getWvkDispatchTable()->wvkGetSwapchainImagesKHR(m_vk_swapchain, &_count, _images.data());
					
					std::vector<VkImageView> _views;
					for (size_t ct_0 = 0; ct_0 < _count; ++ct_0) {
						VkImageViewCreateInfo viewInfo = {
							.sType = VK_STRUCTURE_TYPE_IMAGE_VIEW_CREATE_INFO,
							.pNext = nullptr,
							.flags = 0,
							.image = _images[0],                // VkImage, полученный из swapchain
							.viewType = VK_IMAGE_VIEW_TYPE_2D,
							.format = VK_FORMAT_B8G8R8A8_SRGB,         // тот же формат, что у swapchain
							.components = {                         // стандартный swizzle
								VK_COMPONENT_SWIZZLE_IDENTITY,
								VK_COMPONENT_SWIZZLE_IDENTITY,
								VK_COMPONENT_SWIZZLE_IDENTITY,
								VK_COMPONENT_SWIZZLE_IDENTITY
							},
							.subresourceRange = {
								.aspectMask = VK_IMAGE_ASPECT_COLOR_BIT, // цвет
								.baseMipLevel = 0,
								.levelCount = 1,
								.baseArrayLayer = 0,
								.layerCount = 1
							}
						};
						VkImageView _view = VK_NULL_HANDLE;
						m_create_info.wvk_logical_device_ptr->getWvkDispatchTable()->wvkCreateImageView(nullptr, &viewInfo, nullptr, &_view);
						_views.push_back(_view);
					}

					return _views;
				}

				inline std::vector<VkImage> getImages1() {
					uint32_t _count = 0;
					m_create_info.wvk_logical_device_ptr->getWvkDispatchTable()->wvkGetSwapchainImagesKHR(m_vk_swapchain, &_count, nullptr);
					std::vector<VkImage> _images(_count, VK_NULL_HANDLE);
					m_create_info.wvk_logical_device_ptr->getWvkDispatchTable()->wvkGetSwapchainImagesKHR(m_vk_swapchain, &_count, _images.data());
	

					return _images;
				}

				VkSwapchainKHR getSwap() {
					return m_vk_swapchain;
				}
			// cpp
			private:

				// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				/*!	\brief
				*/
				// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				WvkStatus validationCreateInfo(const  WvkSwapchainCreateInfo& create_info) noexcept;

				// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				/*!	\brief
				*/
				// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				WvkStatus create(void) noexcept;

			// data
			private:

				WvkSwapchainCreateInfo m_create_info = {};
				VkSwapchainKHR m_vk_swapchain = VK_NULL_HANDLE;
			}; // class WvkSwapchain

		} // namespace Extensions

	} // namespace wvk

} // namespace CGDev

#include "wvk_swapchain.hpp"

#endif // CGDEV_WVK_SOURCE_EXTENSION__WVK_SWAPCHAIN_H
