#ifndef CGDEV_WVK_SOURCE_EXTENSIONS__WVK_SURFACE_H
#define CGDEV_WVK_SOURCE_EXTENSIONS__WVK_SURFACE_H
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

namespace CGDev {

	namespace wvk {

		namespace Extensions {

			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			/*!	\brief
			*/
			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			struct WvkSurfacePlatformCreateInfo {
			}; // struct WvkSurfacePlatformCreateInfo

			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			/*!	\brief 
			*/
			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			struct WvkSurfaceCreateInfo {
				WvkInstanceDispatchTablePtr wvk_instance_dispatch_table = nullptr;
				WvkKhrSurfaceDispatchTablePtr wvk_khr_surface_dispatch_table = nullptr;
				WvkSurfacePlatformCreateInfo* wvk_surface_platform_create_info = nullptr;
			}; // struct WvkSurfaceCreateInfo

			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			/*!	\
			*/
			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			class WvkSurface : public GpuObject {

			public:

				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				/*!	\brief
				*/
				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				WvkSurface(void) noexcept;

				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				/*!	\brief
				*/
				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				~WvkSurface(void) noexcept;

				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				/*!	\
				*/
				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				WvkStatus create(const WvkSurfaceCreateInfo& create_info) noexcept;

				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				/*!	\brief
				*/
				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				void destroy(void) noexcept;

			public:

				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				/*!	\brief
				* *Запрашивает возможности поверхности для указанного физического устройства.*
				*
				* @details
				* *Метод вызывает функцию `vkGetPhysicalDeviceSurfaceCapabilitiesKHR` через соответствующую таблицу вызовов,
				* чтобы получить данные о возможностях поверхности (например, поддерживаемые размеры, количество изображений и т.п.).*
				*
				* @param[in]  wvk_physical_device
				* *Указатель на объект физического устройства, от имени которого происходит вызов.*
				*
				* @param[out] vk_surface_capabilities_khr
				* *Структура, в которую будут записаны возможности поверхности. Должна быть валидной.*
				*
				* @retval WvkStatus
				* *Статус операции. В случае неудачи возвращается подробное описание ошибки.*
				*
				* @code
				* WvkSurface surface;
				* VkSurfaceCapabilitiesKHR caps{};
				* WvkStatus status = surface.requestCapabilities(myPhysicalDevice, caps);
				* if (!status) {
				*     std::cerr << status.toString();
				* }
				* @endcode
				*/
				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				inline WvkStatus requestCapabilities(const WvkPhysicalDevicePtr wvk_physical_device, VkSurfaceCapabilitiesKHR& vk_surface_capabilities_khr) const noexcept;

				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				/*!	\brief
				* Получает список поддерживаемых режимов презентации (VkPresentModeKHR) для текущей поверхности.
				*
				* @param[in]  wvk_physical_device   Указатель на физическое устройство Vulkan (обёртка WvkPhysicalDevicePtr).
				* @param[out] vk_present_modes      Вектор, в который будут записаны поддерживаемые режимы презентации.
				*
				* @return Возвращает статус выполнения операции.
				*
				* Возможные ошибки:
				* - VK_ERROR_OUT_OF_HOST_MEMORY
				* - VK_ERROR_OUT_OF_DEVICE_MEMORY
				* - VK_ERROR_SURFACE_LOST_KHR
				* - VK_INCOMPLETE (при втором вызове)
				*
				* @code
				* WvkSurface surface = ...;
				* WvkPhysicalDevicePtr device = ...;
				* std::vector<VkPresentModeKHR> modes;
				* WvkStatus status = surface.requestPresentModes(device, modes);
				* if (status) {
				*     // Успешно
				*     printPresentModes(modes);
				* } else {
				*     std::cerr << status.message() << std::endl;
				* }
				* @endcode
				*/
				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				inline WvkStatus requestPresentModes(const WvkPhysicalDevicePtr wvk_physical_device, std::vector<VkPresentModeKHR>& vk_present_modes) const noexcept;

				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				/*!	\brief
				*     Запрашивает список режимов презентации, совместимых с заданным `VkPresentModeKHR`.
				*
				* @param [in]  wvk_physical_device
				*     Указатель на обёртку физического устройства, на котором выполняется запрос.
				*
				* @param [in]  vk_present_mode
				*     Режим презентации, для которого требуется получить список совместимых режимов.
				*
				* @param [out] out
				*     Вектор, в который будет записан список совместимых режимов презентации (`VkPresentModeKHR`).
				*
				* @return
				*     Объект `WvkStatus`, отражающий успешность операции.
				*
				* @code
				* std::vector<VkPresentModeKHR> compatible_modes;
				* WvkStatus status = wvk_surface->requestPresentModeCompatibility(
				*     wvk_physical_device,
				*     VK_PRESENT_MODE_MAILBOX_KHR,
				*     compatible_modes
				* );
				* if (!status) {
				*     // Обработка ошибки
				* }
				* @endcode
				*/
				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				inline WvkStatus requestPresentModeCompatibility(const WvkPhysicalDevicePtr wvk_physical_device, const VkPresentModeKHR& vk_present_mode, std::vector<VkPresentModeKHR>& out) const noexcept;

				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				/*!	\brief
				*     Запрашивает возможности масштабирования изображения (scaling capabilities)
				*     для заданного режима презентации (`VkPresentModeKHR`) через цепочку `pNext`.
				*
				* @param [in]  wvk_physical_device
				*     Указатель на объект логического представления физического устройства.
				*
				* @param [in]  vk_present_mode
				*     Режим презентации, для которого требуется получить информацию о масштабировании.
				*
				* @param [out] out
				*     Структура `VkSurfacePresentScalingCapabilitiesEXT`, куда будет записана
				*     информация о поддерживаемых режимах масштабирования.
				*
				* @return
				*     Объект `WvkStatus`, содержащий результат операции.
				*
				* @code
				* VkSurfacePresentScalingCapabilitiesEXT scaling_caps = {};
				* WvkStatus status = wvk_surface->requestScalingCompatibility(
				*     wvk_physical_device,
				*     VK_PRESENT_MODE_FIFO_KHR,
				*     scaling_caps
				* );
				* if (!status) {
				*     // Обработка ошибки
				* }
				* @endcode
				*/
				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				WvkStatus requestScalingCompatibility(const WvkPhysicalDevicePtr wvk_physical_device, const VkPresentModeKHR& vk_present_mode, VkSurfacePresentScalingCapabilitiesEXT& out) const noexcept;

				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				/*!	\brief
				*     Выполняет запрос расширенной информации о surface capabilities с использованием структуры `VkBaseInStructure` и `VkBaseOutStructure`.
				*
				* @param [in]  wvk_physical_device
				*     Указатель на логический объект физического устройства Vulkan.
				*
				* @param [in]  in
				*     Указатель на цепочку входных структур, унаследованных от `VkBaseInStructure`.
				*     Может быть `nullptr`, если дополнительные параметры не требуются.
				*
				* @param [out] out
				*     Указатель на структуру или цепочку структур, унаследованных от `VkBaseOutStructure`,
				*     куда будут записаны результаты запроса.
				*
				* @return
				*     Объект `WvkStatus`, содержащий результат операции.
				*     При ошибке статус содержит сообщение и код `FAIL`.
				*
				* @code
				* VkSurfaceProtectedCapabilitiesKHR _caps = { VK_STRUCTURE_TYPE_SURFACE_PROTECTED_CAPABILITIES_KHR };
				* WvkStatus status = wvk_surface->requestCapability(wvk_physical_device, nullptr, &_caps);
				* if (!status) {
				*     // Обработка ошибки
				* }
				* @endcode
				*/
				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				inline WvkStatus requestCapabilities(const WvkPhysicalDevicePtr wvk_physical_device, const VkBaseInStructure* in, VkBaseOutStructure* out) const noexcept;

				inline WvkStatus requestCapabilities(const WvkPhysicalDevicePtr wvk_physical_device, VkSurfaceProtectedCapabilitiesKHR& out) const noexcept;

				template<typename In, typename Out>
				inline WvkStatus requestCapabilities(const WvkPhysicalDevicePtr wvk_physical_device, const In& in, Out& out) const noexcept;

			private:

				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				/*!	\
				*/
				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				WvkStatus validationCreateInfo(void) const noexcept;

				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				/*!	\brief 
				* Создаёт VkSurfaceKHR в зависимости от текущей платформы.
				* 
				* @details
				* На данный момент реализована поддержка только платформы MSWindows. Используется
				* структура WvkSurfaceMSWindowsCreateInfo, которая содержит дескрипторы окна и инстанса,
				* а также указатель на расширение WvkKhrWin32Surface.
				* 
				* @return WvkStatus 
				* Возвращает статус выполнения. Если операция прошла успешно — `OK`. В противном случае — код ошибки.
				* 
				* @code
				* WvkSurface surface;
				* surface.setCreateInfo(...); // Установить платформенно-зависимую информацию
				* WvkStatus status = surface.createVkSurface();
				* if (!status.isOk()) {
				*     log->error(status.message());
				* }
				* @endcode
				*/
				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				WvkStatus createVkSurface(void) noexcept;

			private:

				WvkSurfaceCreateInfo m_create_info = {};
				VkSurfaceKHR m_vk_surface = VK_NULL_HANDLE;
				struct WvkSurfaceImpl;
				std::unique_ptr<WvkSurfaceImpl> m_surface_impl = nullptr;
			}; // class WvkSurface

		} // namespace Extensions

	} // namespace wvk

} // namespace CGDev

#include "wvk_surface.hpp"

#endif // CGDEV_WVK_SOURCE_EXTENSIONS__WVK_SURFACE_H
