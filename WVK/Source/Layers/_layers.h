#ifndef CGDEV_SOURCE_GPU_PRIVATE_VULKAN_LAYERS___LAYERS_H
#define CGDEV_SOURCE_GPU_PRIVATE_VULKAN_LAYERS___LAYERS_H
////////////////////////////////////////////////////////////////
// секция форвард-декларации
////////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////
// секция имплементации
////////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////
// секция родительского класса
////////////////////////////////////////////////////////////////
#include "../_wvk.h"
////////////////////////////////////////////////////////////////
// секция для остального
////////////////////////////////////////////////////////////////

namespace CGDev {

	//namespace GPU {

		//namespace Private {

			namespace wvk {

				namespace Layers {

					//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					/*!	\brief
					*/
					//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

					class			VknLayerKhronosValidation;
					using			VknLayerKhronosValidationPtr							= VknLayerKhronosValidation * ;
					using			VknLayerKhronosValidationPtrArr							= std::vector<VknLayerKhronosValidationPtr>;
					using			VknLayerKhronosValidationWptr							= std::weak_ptr<VknLayerKhronosValidation>;
					using			VknLayerKhronosValidationWptrArr						= std::vector<VknLayerKhronosValidationWptr>;
					using			VknLayerKhronosValidationSptr							= std::shared_ptr<VknLayerKhronosValidation>;
					using			VknLayerKhronosValidationSptrArr						= std::vector<VknLayerKhronosValidationSptr>;

				} // namespace Layers

			} // namespace wvk

		//} // namespace Private

	//} // namespace GPU

} // namespace CGDev

#endif // CGDEV_SOURCE_GPU_PRIVATE_VULKAN_LAYERS___LAYERS_H