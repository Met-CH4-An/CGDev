// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025 Metelev Met-CH4-An Andrey
// Project Lead: Metelev Andrey
// Main Technical Assistant: StarDev aka ChatGPT
#ifndef CGDEV_WVK_SOURCE__WVK_FENCE_H
#define CGDEV_WVK_SOURCE__WVK_FENCE_H
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
#include "wvk_logical_device.h"
#include "wvk_dispatch_table.h"

namespace CGDev {

	namespace wvk {

		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		/*!	\brief
		*/
		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		struct WvkFenceCreateInfo {
			WvkLogicalDevicePtr wvk_logical_device_ptr = nullptr;
		}; // struct WvkFenceCreateInfo

		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		/*!	\brief
		*/
		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		class WvkFence : public GpuObject {

		public:

			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			/*!	\brief
			*/
			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			inline WvkFence(void) noexcept {}

			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			/*!	\brief
			*/
			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			inline ~WvkFence(void) noexcept {}

			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			/*!	\brief
			*/
			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			inline WvkStatus create(const WvkFenceCreateInfo& create_info) noexcept {
				WvkStatus _status;

				VkFenceCreateInfo _create_info = {
					.sType = VK_STRUCTURE_TYPE_FENCE_CREATE_INFO,
					.pNext = nullptr,
					.flags = 0,
				};
				m_create_info = create_info;
				m_create_info.wvk_logical_device_ptr->getWvkDispatchTable()->wvkCreateFence(nullptr, &_create_info, nullptr, &m_vk_fence);

				return _status.setOk();
			}

			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			/*!	\brief
			*/
			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			inline void destroy(void) noexcept {}

			inline void wait(void) {
				auto _vk_result = m_create_info.wvk_logical_device_ptr->getWvkDispatchTable()->wvkWaitForFences(nullptr, 1, &m_vk_fence, VK_TRUE, UINT64_MAX);
			}

			inline void reset(void) {
				auto _vk_result = m_create_info.wvk_logical_device_ptr->getWvkDispatchTable()->wvkResetFences(nullptr, 1, &m_vk_fence);
			}

			inline const VkFence getVkFence(void) { return m_vk_fence; }
		// hpp
		public:


		// hpp
		private:

		// cpp
		private:

		private:
			WvkFenceCreateInfo m_create_info = {};
			VkFence m_vk_fence = VK_NULL_HANDLE;
		}; // class WvkCommandPool

	} // namespace wvk

} // namespace CGDev

#endif // CGDEV_WVK_SOURCE__WVK_FENCE_H
