// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025 Metelev Met-CH4-An Andrey
// Project Lead: Metelev Andrey
// Main Technical Assistant: StarDev aka ChatGPT
////////////////////////////////////////////////////////////////
// секция форвард-декларации
////////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////
// секция заголовочного файла
////////////////////////////////////////////////////////////////
#include "shader_object.h"
////////////////////////////////////////////////////////////////
// секция имплементации
////////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////
// секция для остального
////////////////////////////////////////////////////////////////

namespace CGDev {

	namespace samples {

		// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

		void ShaderObject::onInit(void) noexcept {
			
		}

		// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

		void ShaderObject::onUpdate(void) noexcept {

		}

		// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

		void ShaderObject::onRender(void) noexcept {
			static uint32_t _cur_frame = 0;
			static uint32_t _index = 0;
			wvk_swapchain_ptr->Aqueue(wvk_fence_ptrs[2].get(), _index);
			wvk_fence_ptrs[2]->wait();
			wvk_fence_ptrs[2]->reset();
			CGDev::wvk::WvkCommandBufferBeginHelper _begin_helper = {};
			
			wvk_command_buffers[_index]->reset();
			wvk_command_buffers[_index]->begin(_begin_helper);
			wvk_command_buffers[_index]->Bari(m_vk_images[_index], VK_IMAGE_LAYOUT_UNDEFINED, VK_IMAGE_LAYOUT_COLOR_ATTACHMENT_OPTIMAL);
			wvk_command_buffers[_index]->cmdBeginRendering(m_vk_image_views[_index]);
			wvk_command_buffers[_index]->cmdEndRendering();
			wvk_command_buffers[_index]->Bari(m_vk_images[_index], VK_IMAGE_LAYOUT_COLOR_ATTACHMENT_OPTIMAL, VK_IMAGE_LAYOUT_PRESENT_SRC_KHR);
			wvk_command_buffers[_index]->end(wvk_swapchain_ptr.get(), _index, wvk_fence_ptrs[_cur_frame]->getVkFence());

			_cur_frame = (_cur_frame + 1) % 2;
		}

		// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

		void ShaderObject::onDestroy(void) noexcept {

		}

		// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

	} // namespace samples

} // namespace CGDev