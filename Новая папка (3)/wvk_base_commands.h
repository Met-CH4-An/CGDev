#ifndef CGDEV_SOURCE_GPU_PRIVATE_VULKAN__VKN_BASE_COMMANDS_H
#define CGDEV_SOURCE_GPU_PRIVATE_VULKAN__VKN_BASE_COMMANDS_H
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
#include "wvk_instance.h"

namespace CGDev {

	//namespace GPU {

		//namespace Private {

			namespace wvk {

				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				/*!	\brief Структура для хранения информации о Vulkan-функции, которую необходимо загрузить вручную.
				*
				* Используется при загрузке указателей на Vulkan-процедуры через `vkGetInstanceProcAddr` или аналогичную функцию.
				* Каждая запись указывает имя Vulkan-функции и адрес переменной, куда будет сохранён загруженный указатель.
				*
				* @note Эта структура используется, например, для загрузки расширенных функций Vulkan, таких как `vkDestroySurfaceKHR`.
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
				//	const char* name;       ///< Название функции Vulkan, например "vkDestroySurfaceKHR"
				//	void** targetPtr;       ///< Указатель на переменную, куда будет сохранён загруженный адрес
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
					/*!	\brief Пытается загрузить список Vulkan - функций и сохранить их указатели.
					*
					* Метод проходит по коллекции `VknVulkanFunctionInfo`, вызывает `vknGetInstanceProcAddr`
					* и сохраняет полученные указатели по указанным адресам. Если какая - либо функция не найдена,
					* возвращается статус с кодом ошибки и сообщением о конкретной недостающей функции.
					*
					* @param[in] wvk_commands
					* Объект, предоставляющий доступ к методу `vknGetInstanceProcAddr`, через который получаются указатели.
					*
					* @param[in] wvk_vulkan_procedure_collection1
					* Список структур `VknVulkanFunctionInfo`, содержащих имена функций и места для хранения указателей.
					*
					* @return WvkStatus
					* Объект, содержащий статус операции и сообщение об ошибке(если такая произошла).
					*
					* @code
					* std::vector<VknVulkanFunctionInfo> funcs = {
					* { "vkDestroySurfaceKHR", reinterpret_cast<void**>(&m_vkDestroySurfaceKHR) },
					* { "vkCreateDebugUtilsMessengerEXT", reinterpret_cast<void**>(&m_vkCreateDebugUtilsMessengerEXT) }
					* };
					*
					* WvkStatus status = vknBaseCommands.tryLoadFunction(wvk_commands, funcs);
					* if (status.m_code != VknStatusCode::SUCCESSFUL) {
					* std::cerr << "Ошибка: " << status.getMessage() << std::endl;
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
