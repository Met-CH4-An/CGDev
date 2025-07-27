#ifndef CGDEV_SOURCE_GPU_PRIVATE_VULKAN__VKN_BASE_COMMANDS_H
#define CGDEV_SOURCE_GPU_PRIVATE_VULKAN__VKN_BASE_COMMANDS_H
////////////////////////////////////////////////////////////////
// ������ �������-����������
////////////////////////////////////////////////////////////////
#include "_wvk.h"
////////////////////////////////////////////////////////////////
// ������ �������������
////////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////
// ������ ������������� ������
////////////////////////////////////////////////////////////////
#include "wvk_base_object.h"
////////////////////////////////////////////////////////////////
// ������ ��� ����������
////////////////////////////////////////////////////////////////
#include "wvk_instance.h"

namespace CGDev {

	//namespace GPU {

		//namespace Private {

			namespace wvk {

				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				/*!	\brief ��������� ��� �������� ���������� � Vulkan-�������, ������� ���������� ��������� �������.
				*
				* ������������ ��� �������� ���������� �� Vulkan-��������� ����� `vkGetInstanceProcAddr` ��� ����������� �������.
				* ������ ������ ��������� ��� Vulkan-������� � ����� ����������, ���� ����� ������� ����������� ���������.
				*
				* @note ��� ��������� ������������, ��������, ��� �������� ����������� ������� Vulkan, ����� ��� `vkDestroySurfaceKHR`.
				*
				* @code
				* VknVulkanFunctionInfo destroy_surface_info {
				*     "vkDestroySurfaceKHR",
				*     reinterpret_cast<void**>(&m_vkDestroySurfaceKHR)
				* };
				* @endcode
				*/
				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				//struct VknVulkanFunctionInfo {
				//	const char* name;       ///< �������� ������� Vulkan, �������� "vkDestroySurfaceKHR"
				//	void** targetPtr;       ///< ��������� �� ����������, ���� ����� ������� ����������� �����
				//	VknVulkanFunctionInfo(const char* n, void** ptr)
				//		: name(n), targetPtr(ptr) {
				//	}
				//};				
		




				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				/*!	\brief
				*/
				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				class VknBaseCommands : public GpuObject {

				protected:

					//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					/*!	\brief �������� ��������� ������ Vulkan - ������� � ��������� �� ���������.
					*
					* ����� �������� �� ��������� `VknVulkanFunctionInfo`, �������� `vknGetInstanceProcAddr`
					* � ��������� ���������� ��������� �� ��������� �������. ���� ����� - ���� ������� �� �������,
					* ������������ ������ � ����� ������ � ���������� � ���������� ����������� �������.
					*
					* @param[in] wvk_commands
					* ������, ��������������� ������ � ������ `vknGetInstanceProcAddr`, ����� ������� ���������� ���������.
					*
					* @param[in] wvk_vulkan_procedure_collection1
					* ������ �������� `VknVulkanFunctionInfo`, ���������� ����� ������� � ����� ��� �������� ����������.
					*
					* @return WvkStatus
					* ������, ���������� ������ �������� � ��������� �� ������(���� ����� ���������).
					*
					* @code
					* std::vector<VknVulkanFunctionInfo> funcs = {
					* { "vkDestroySurfaceKHR", reinterpret_cast<void**>(&m_vkDestroySurfaceKHR) },
					* { "vkCreateDebugUtilsMessengerEXT", reinterpret_cast<void**>(&m_vkCreateDebugUtilsMessengerEXT) }
					* };
					*
					* WvkStatus status = vknBaseCommands.tryLoadFunction(wvk_commands, funcs);
					* if (status.m_code != VknStatusCode::SUCCESSFUL) {
					* std::cerr << "������: " << status.getMessage() << std::endl;
					* }
					* @endcode
					*/
					//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					//WvkStatus tryLoadFunction(WvkCommandsPtr wvk_commands, std::vector<VknVulkanFunctionInfo> wvk_vulkan_procedure_collection1) const noexcept;
				};

			} // namespace wvk

		//} // namespace Private

	//} // namespace GPU

} // namespace CGDev

#include "wvk_base_commands.hpp"

#endif // CGDEV_SOURCE_GPU_PRIVATE_VULKAN__VKN_BASE_COMMANDS_H
