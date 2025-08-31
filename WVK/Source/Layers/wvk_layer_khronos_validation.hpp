// SPDX-License-Identifier: AGPL-3.0-or-later
#ifndef CGDEV_SOURCE_GPU_PRIVATE_VULKAN_LAYERS__VKN_LAYER_KHRONOS_VALIDATION_HPP
#define CGDEV_SOURCE_GPU_PRIVATE_VULKAN_LAYERS__VKN_LAYER_KHRONOS_VALIDATION_HPP
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

				namespace Layers {

					//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

					inline const std::string& VknLayerKhronosValidation::s_getName(void) noexcept {

						return s_name;
					}

					//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

				} // namespace Layers

			} // namespace wvk

		//} // namespace Private

	//} // namespace GPU

} // namespace CGDev

#endif // CGDEV_SOURCE_GPU_PRIVATE_VULKAN_LAYERS__VKN_LAYER_KHRONOS_VALIDATION_HPP
