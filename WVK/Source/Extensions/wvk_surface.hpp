#ifndef CGDEV_WVK_SOURCE_EXTENSIONS__WVK_SURFACE_HPP
#define CGDEV_WVK_SOURCE_EXTENSIONS__WVK_SURFACE_HPP
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
#include "wvk_khr_surface_dispatch_table.h"
#include "../wvk_instance_dispatch_table.h"
#include "../wvk_physical_device.h"

namespace CGDev {

	namespace wvk {

		namespace Extensions {

			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

			inline WvkStatus WvkSurface::requestCapabilities(const WvkPhysicalDevicePtr wvk_physical_device, VkSurfaceCapabilitiesKHR& vk_surface_capabilities_khr) const noexcept {
				WvkStatus _status;

				// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				// Шаг 1. Вызов метода получения surface capabilities через invoke
				// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				auto _vk_res = wvk_physical_device->invokeWithVkPhysicalDeviceMethod(
					&WvkKhrSurfaceDispatchTable::wvkGetPhysicalDeviceSurfaceCapabilitiesKHR,
					m_create_info.wvk_khr_surface_dispatch_table,
					m_vk_surface,
					&vk_surface_capabilities_khr
				);

				// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				// Шаг 2. Обработка ошибки, если результат вызова не VK_SUCCESS
				// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				if (_vk_res != VK_SUCCESS) {
					switch (_vk_res) {
					case VK_ERROR_OUT_OF_HOST_MEMORY:
						_status.append("\n\tWvkKhrSurfaceDispatchTable::wvkGetPhysicalDeviceSurfaceCapabilitiesKHR - VK_ERROR_OUT_OF_HOST_MEMORY.");
						break;
					case VK_ERROR_OUT_OF_DEVICE_MEMORY:
						_status.append("\n\tWvkKhrSurfaceDispatchTable::wvkGetPhysicalDeviceSurfaceCapabilitiesKHR - VK_ERROR_OUT_OF_DEVICE_MEMORY.");
						break;
					case VK_ERROR_SURFACE_LOST_KHR:
						_status.append("\n\tWvkKhrSurfaceDispatchTable::wvkGetPhysicalDeviceSurfaceCapabilitiesKHR - VK_ERROR_SURFACE_LOST_KHR.");
						break;
					default:
						_status.append("\n\tWvkKhrSurfaceDispatchTable::wvkGetPhysicalDeviceSurfaceCapabilitiesKHR - unknown error.");
						break;
					}

					// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					// Шаг 3. Установка кода ошибки и возврат статуса
					// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					return _status.set(VknStatusCode::FAIL);
				}

				// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				// Шаг 4. Возврат успешного статуса
				// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				return _status.setOk();
			}

			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

			inline WvkStatus WvkSurface::requestPresentModes(const WvkPhysicalDevicePtr wvk_physical_device, std::vector<VkPresentModeKHR>& vk_present_modes) const noexcept {
				// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				// Шаг 1. Инициализация объекта статуса
				// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				WvkStatus _status;

				// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				// Шаг 2. Запрашиваем количество доступных режимов презентации
				// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				uint32_t _count = 0;
				auto _vk_res = wvk_physical_device->invokeWithVkPhysicalDeviceMethod(
					&WvkKhrSurfaceDispatchTable::wvkGetPhysicalDeviceSurfacePresentModesKHR,
					m_create_info.wvk_khr_surface_dispatch_table,
					m_vk_surface,
					&_count,
					nullptr
				);

				if (_vk_res != VK_SUCCESS) {
					// Обработка возможных ошибок
					switch (_vk_res) {
					case VK_ERROR_OUT_OF_HOST_MEMORY:
						_status.append("\n\tWvkKhrSurfaceDispatchTable::wvkGetPhysicalDeviceSurfacePresentModesKHR - VK_ERROR_OUT_OF_HOST_MEMORY.");
						break;
					case VK_ERROR_OUT_OF_DEVICE_MEMORY:
						_status.append("\n\tWvkKhrSurfaceDispatchTable::wvkGetPhysicalDeviceSurfacePresentModesKHR - VK_ERROR_OUT_OF_DEVICE_MEMORY.");
						break;
					case VK_ERROR_SURFACE_LOST_KHR:
						_status.append("\n\tWvkKhrSurfaceDispatchTable::wvkGetPhysicalDeviceSurfacePresentModesKHR - VK_ERROR_SURFACE_LOST_KHR.");
						break;
					default:
						_status.append("\n\tWvkKhrSurfaceDispatchTable::wvkGetPhysicalDeviceSurfacePresentModesKHR - неизвестная ошибка.");
						break;
					}
					return _status.set(VknStatusCode::FAIL);
				}

				// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				// Шаг 3. Выделяем память под вектор режимов
				// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				vk_present_modes.resize(_count);

				// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				// Шаг 4. Запрашиваем список поддерживаемых режимов
				// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				_vk_res = wvk_physical_device->invokeWithVkPhysicalDeviceMethod(
					&WvkKhrSurfaceDispatchTable::wvkGetPhysicalDeviceSurfacePresentModesKHR,
					m_create_info.wvk_khr_surface_dispatch_table,
					m_vk_surface,
					&_count,
					vk_present_modes.data()
				);

				if (_vk_res != VK_SUCCESS) {
					// Обработка возможных ошибок
					switch (_vk_res) {
					case VK_ERROR_OUT_OF_HOST_MEMORY:
						_status.append("\n\tWvkKhrSurfaceDispatchTable::wvkGetPhysicalDeviceSurfacePresentModesKHR - VK_ERROR_OUT_OF_HOST_MEMORY.");
						break;
					case VK_ERROR_OUT_OF_DEVICE_MEMORY:
						_status.append("\n\tWvkKhrSurfaceDispatchTable::wvkGetPhysicalDeviceSurfacePresentModesKHR - VK_ERROR_OUT_OF_DEVICE_MEMORY.");
						break;
					case VK_ERROR_SURFACE_LOST_KHR:
						_status.append("\n\tWvkKhrSurfaceDispatchTable::wvkGetPhysicalDeviceSurfacePresentModesKHR - VK_ERROR_SURFACE_LOST_KHR.");
						break;
					case VK_INCOMPLETE:
						// VK_INCOMPLETE может означать, что список был частично заполнен
						_status.append("\n\tWvkKhrSurfaceDispatchTable::wvkGetPhysicalDeviceSurfacePresentModesKHR - VK_INCOMPLETE.");
						break;
					default:
						_status.append("\n\tWvkKhrSurfaceDispatchTable::wvkGetPhysicalDeviceSurfacePresentModesKHR - неизвестная ошибка.");
						break;
					}
					return _status.set(VknStatusCode::FAIL);
				}

				// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				// Шаг 5. Возвращаем успешный статус
				// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				return _status.setOk();
			}

			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

			inline WvkStatus WvkSurface::requestPresentModeCompatibility(const WvkPhysicalDevicePtr wvk_physical_device, const VkPresentModeKHR& vk_present_mode, std::vector<VkPresentModeKHR>& out) const noexcept {
				WvkStatus _status;

				// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				// Шаг 1. Подготовка входной структуры VkSurfacePresentModeEXT
				// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				VkSurfacePresentModeEXT _in = {};
				_in.sType = VK_STRUCTURE_TYPE_SURFACE_PRESENT_MODE_EXT;
				_in.presentMode = vk_present_mode;
				_in.pNext = nullptr;

				// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				// Шаг 2. Подготовка выходной структуры VkSurfacePresentModeCompatibilityEXT (первая фаза)
				// для запроса количества совместимых режимов
				// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				VkSurfacePresentModeCompatibilityEXT _out = {};
				_out.sType = VK_STRUCTURE_TYPE_SURFACE_PRESENT_MODE_COMPATIBILITY_EXT;
				_out.pNext = nullptr;
				_out.presentModeCount = 0;
				_out.pPresentModes = nullptr;

				// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				// Шаг 3. Первый вызов запроса — получение количества режимов
				// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				_status = requestCapabilities(wvk_physical_device, _in, _out);
				if (!_status) {
					return _status.set(VknStatusCode::FAIL, "\n\tWvkSurface::requestCapabilities - fail.");
				}

				// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				// Шаг 4. Выделение памяти под совместимые режимы
				// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				out.resize(_out.presentModeCount);
				_out.pPresentModes = out.data();

				// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				// Шаг 5. Второй вызов — получение самих совместимых режимов
				// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				_status = requestCapabilities(wvk_physical_device, _in, _out);
				if (!_status) {
					return _status.set(VknStatusCode::FAIL, "\n\tWvkSurface::requestCapabilities - fail.");
				}

				// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				// Шаг 6. Возврат успешного результата
				// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				return _status.setOk();
			}

			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

			inline WvkStatus WvkSurface::requestScalingCompatibility(const WvkPhysicalDevicePtr wvk_physical_device, const VkPresentModeKHR& vk_present_mode, VkSurfacePresentScalingCapabilitiesEXT& out) const noexcept {
				WvkStatus _status;

				// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				// Шаг 1. Подготовка структуры VkSurfacePresentModeEXT,
				// которая указывает запрашиваемый present mode
				// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				VkSurfacePresentModeEXT _in = {};
				_in.sType = VK_STRUCTURE_TYPE_SURFACE_PRESENT_MODE_EXT;
				_in.presentMode = vk_present_mode;
				_in.pNext = nullptr;

				// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				// Шаг 2. Подготовка выходной структуры VkSurfacePresentScalingCapabilitiesEXT,
				// которая получит результат
				// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				out.sType = VK_STRUCTURE_TYPE_SURFACE_PRESENT_SCALING_CAPABILITIES_EXT;
				out.pNext = nullptr;

				// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				// Шаг 3. Вызов обобщённого метода запроса возможностей через цепочку pNext
				// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				return requestCapabilities(wvk_physical_device, _in, out);
			}

			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

			inline WvkStatus WvkSurface::requestCapabilities(const WvkPhysicalDevicePtr wvk_physical_device, const VkBaseInStructure* in, VkBaseOutStructure* out) const noexcept {
				WvkStatus _status;

				// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				// Шаг 1. Подготовка структуры входных параметров запроса
				// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				VkPhysicalDeviceSurfaceInfo2KHR _info2 = {};
				_info2.sType = VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SURFACE_INFO_2_KHR;
				_info2.pNext = in;
				_info2.surface = m_vk_surface;

				// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				// Шаг 2. Подготовка структуры для получения результатов
				// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				VkSurfaceCapabilities2KHR _caps2 = {};
				_caps2.sType = VK_STRUCTURE_TYPE_SURFACE_CAPABILITIES_2_KHR;
				_caps2.pNext = out;

				// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				// Шаг 3. Вызов Vulkan-функции через метод-обёртку
				// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				auto _vk_res = wvk_physical_device->invokeWithVkPhysicalDeviceMethod(
					&WvkInstanceDispatchTable::wvkGetPhysicalDeviceSurfaceCapabilities2KHR,
					m_create_info.wvk_instance_dispatch_table,
					&_info2,
					&_caps2
				);

				// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				// Шаг 4. Обработка возможных ошибок
				// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				if (_vk_res != VK_SUCCESS) {
					switch (_vk_res) {
					case VK_ERROR_OUT_OF_HOST_MEMORY:
						_status.append("\n\tWvkSurface::requestCapability - VK_ERROR_OUT_OF_HOST_MEMORY.");
						break;
					case VK_ERROR_OUT_OF_DEVICE_MEMORY:
						_status.append("\n\tWvkSurface::requestCapability - VK_ERROR_OUT_OF_DEVICE_MEMORY.");
						break;
					case VK_ERROR_SURFACE_LOST_KHR:
						_status.append("\n\tWvkSurface::requestCapability - VK_ERROR_SURFACE_LOST_KHR.");
						break;
					default:
						_status.append("\n\tWvkSurface::requestCapability - Неизвестная ошибка Vulkan.");
						break;
					}

					return _status.set(VknStatusCode::FAIL);
				}

				// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				// Шаг 5. Успешное завершение
				// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				return _status.setOk();
			}

			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

			inline WvkStatus WvkSurface::requestCapabilities(const WvkPhysicalDevicePtr wvk_physical_device, VkSurfaceProtectedCapabilitiesKHR& out) const noexcept {
				out.sType = VK_STRUCTURE_TYPE_SURFACE_PROTECTED_CAPABILITIES_KHR;
				out.pNext = nullptr;
				return requestCapabilities(wvk_physical_device, nullptr, reinterpret_cast<VkBaseOutStructure*>(&out));
			}

			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			
			template<typename In, typename Out>
			inline WvkStatus WvkSurface::requestCapabilities(const WvkPhysicalDevicePtr wvk_physical_device, const In& in, Out& out) const noexcept {
				return requestCapabilities(wvk_physical_device, reinterpret_cast<const VkBaseInStructure*>(&in), reinterpret_cast<VkBaseOutStructure*>(&out));
			}

			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

		} // namespace Extensions

	} // namespace wvk

} // namespace CGDev

#endif // CGDEV_WVK_SOURCE_EXTENSIONS__WVK_SURFACE_HPP