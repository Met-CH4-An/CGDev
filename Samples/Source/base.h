// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025 Metelev Met-CH4-An Andrey
// Project Lead: Metelev Andrey
// Main Technical Assistant: StarDev aka ChatGPT
#ifndef CGDEV_SAMPLES_SOURCE__BASE_H
#define CGDEV_SAMPLES_SOURCE__BASE_H
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
#include "wvk_instance.h"
#include "wvk_logical_device.h"
#include "wvk_command_pool.h"
#include "wvk_command_buffer.h"
#include "wvk_fence.h"
#include "Extensions/wvk_surface.h"
#include "Extensions/wvk_swapchain.h"

namespace CGDev {

	namespace samples {

		// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		/*!	\brief
		*/
		// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		class Base {

		public:

			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			/*!	\brief
			*/
			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			Base(void) noexcept;

			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			/*!	\brief
			*/
			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			~Base(void) noexcept;

			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			/*!	\brief
			*/
			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			virtual void onInit(void) = 0;

			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			/*!	\brief
			*/
			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			virtual void onUpdate(void) = 0;

			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			/*!	\brief
			*/
			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			virtual void onRender(void) = 0;

			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			/*!	\brief
			*/
			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			virtual void onDestroy(void) = 0;

		// cpp
		public:

		// hpp
		public:

		// hpp
		private:

		// cpp
		private:

		// data
		protected:
			HWND m_hwnd = NULL;
			HINSTANCE m_hinstance = NULL;
			CGDev::wvk::WvkInstanceUptr wvk_instance_ptr;
			CGDev::wvk::WvkLogicalDeviceUptr wvk_logical_device_ptr;
			CGDev::wvk::WvkCommandPoolUptr wvk_command_pool_ptr;
			CGDev::wvk::WvkCommandBufferPtrVec1 wvk_command_buffers;
			CGDev::wvk::WvkFenceUptr wvk_fence_ptr;
			CGDev::wvk::Extensions::WvkSurfaceUptr wvk_surface_ptr;
			CGDev::wvk::Extensions::WvkSwapchainUptr wvk_swapchain_ptr;
		}; // class Base

	} // namespace samples

} // namespace CGDev

#endif // CGDEV_SAMPLES_SOURCE__BASE_H
