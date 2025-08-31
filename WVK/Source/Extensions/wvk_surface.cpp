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
#include "wvk_surface.h"
////////////////////////////////////////////////////////////////
// секция имплементации
////////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////
// секция для остального
////////////////////////////////////////////////////////////////
#include "MSWindows/wvk_surface_mswindows.h"
using WvkSurfacePlatform = CGDev::wvk::Extensions::mswindows::WvkSurfaceMSWindows;

#include "../wvk_physical_device.h"
#include "../wvk_instance.h" 

namespace CGDev {

	namespace wvk {

		namespace Extensions {
			
			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			/*!	\brief
			*/
			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			struct WvkSurface::WvkSurfaceImpl {
			public:
				WvkSurfaceImpl() = default;
				~WvkSurfaceImpl() = default;

				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				/*!	\brief
				*/
				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				WvkStatus create(mswindows::WvkSurfaceMSWindowsCreateInfo& create_info) {
					if (Build::SurfaceBuildInfo::platform_type == Build::PlatformType::MSWindows) {
						WvkStatus _status;
															
						m_surface_platform.create(create_info);

						return _status.setOk();
					}
				}

				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				/*!	\brief
				*/
				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				void requestVkSurface(VkSurfaceKHR& vk_surface_khr) {
					m_surface_platform.requestVkSurface(vk_surface_khr);
				}

				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				/*!	\brief
				*/
				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				void destroy() {

				}

			private:
				WvkSurfacePlatform m_surface_platform;
			};

			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

			WvkSurface::WvkSurface(void) noexcept {
			}

			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

			WvkSurface::~WvkSurface(void) noexcept {
			}

			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

			WvkStatus WvkSurface::create(const WvkSurfaceCreateInfo& create_info) noexcept {
				WvkStatus _status;

				m_create_info = create_info;
							
				_status = validationCreateInfo();
						
				if (!_status) {
					reset();
					return _status.set(VknStatusCode::FAIL, "\n\tVknExtDebugUtils::validationCreateInfo() - fail.");
				}

				m_surface_impl = std::make_unique<WvkSurfaceImpl>();

				_status = createVkSurface();

				if (!_status) {
					reset();
					return _status.set(VknStatusCode::FAIL, "\n\tWvkSurface::createVkSurface - fail.");
				}
							
				m_valid = true;
				return _status.setOk();
			}

			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

			void WvkSurface::destroy(void) noexcept {

				//m_create_info.wvk_khr_surface_commands->vkDestroyDebugUtilsMessengerEXT();

				// ~~~~~~~~~~~~~~~~
				// очистка данных
				// ~~~~~~~~~~~~~~~~

				m_create_info = {};
			}

			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

			WvkStatus WvkSurface::requestFormats(const WvkPhysicalDevicePtr wvk_physical_device_ptr, std::vector<VkSurfaceFormatKHR>& out) const noexcept {
				WvkStatus _status;

				// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				// Шаг 1. Проверяем, активировано ли расширение VK_KHR_surface
				// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				/*if constexpr (Build::find(WvkKhrSurfaceDT::s_getName())) {

					// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					// Шаг 2. Очистить вектор и подготовить переменную для подсчёта форматов
					// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					uint32_t _count = 0;
					out.clear();

					// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					// Шаг 3. Первый вызов: получить количество поддерживаемых форматов
					// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					//m_create_info.wvk_instance_dt_ptr->
					//auto _vk_res = wvk_physical_device_ptr->invokeWithVkPhysicalDeviceMethod(
					//	&WvkKhrSurfaceDT::wvkGetPhysicalDeviceSurfaceFormatsKHR,
					//	m_create_info.wvk_khr_surface_dt,
					//	m_vk_surface,
					//	&_count,
					//	nullptr
					//);

					/*if (_vk_res != VK_SUCCESS) {
						switch (_vk_res) {
						case VK_ERROR_OUT_OF_HOST_MEMORY:
							_status.append("\n\tWvkKhrSurfaceDispatchTable::wvkGetPhysicalDeviceSurfaceFormatsKHR - VK_ERROR_OUT_OF_HOST_MEMORY.");
							break;
						case VK_ERROR_OUT_OF_DEVICE_MEMORY:
							_status.append("\n\tWvkKhrSurfaceDispatchTable::wvkGetPhysicalDeviceSurfaceFormatsKHR - VK_ERROR_OUT_OF_DEVICE_MEMORY.");
							break;
						case VK_ERROR_SURFACE_LOST_KHR:
							_status.append("\n\tWvkKhrSurfaceDispatchTable::wvkGetPhysicalDeviceSurfaceFormatsKHR - VK_ERROR_SURFACE_LOST_KHR.");
							break;
						}
						return _status.set(VknStatusCode::FAIL);
					}*/

					// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					// Шаг 4. Выделить место в векторе под нужное количество элементов
					// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					//out.resize(_count);

					// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					// Шаг 5. Второй вызов: получить сами форматы
					// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					//_vk_res = wvk_physical_device_ptr->invokeWithVkPhysicalDeviceMethod(
					//	&WvkKhrSurfaceDT::wvkGetPhysicalDeviceSurfaceFormatsKHR,
					//	m_create_info.wvk_khr_surface_dt,
					//	m_vk_surface,
					//	&_count,
					//	out.data()
					//);

					/*if (_vk_res != VK_SUCCESS) {
						switch (_vk_res) {
						case VK_INCOMPLETE:
							_status.append("\n\tWvkKhrSurfaceDispatchTable::wvkGetPhysicalDeviceSurfaceFormatsKHR - VK_INCOMPLETE.");
							break;
						case VK_ERROR_OUT_OF_HOST_MEMORY:
							_status.append("\n\tWvkKhrSurfaceDispatchTable::wvkGetPhysicalDeviceSurfaceFormatsKHR - VK_ERROR_OUT_OF_HOST_MEMORY.");
							break;
						case VK_ERROR_OUT_OF_DEVICE_MEMORY:
							_status.append("\n\tWvkKhrSurfaceDispatchTable::wvkGetPhysicalDeviceSurfaceFormatsKHR - VK_ERROR_OUT_OF_DEVICE_MEMORY.");
							break;
						case VK_ERROR_SURFACE_LOST_KHR:
							_status.append("\n\tWvkKhrSurfaceDispatchTable::wvkGetPhysicalDeviceSurfaceFormatsKHR - VK_ERROR_SURFACE_LOST_KHR.");
							break;
						}
						return _status.set(VknStatusCode::FAIL);
					}*/

					// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					// Шаг 6. Успешное завершение
					// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					return _status.setOk();
				//}*/

				// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				// Шаг 7. Если VK_KHR_surface не активирован — возвращаем соответствующий статус
				// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				return _status.set(VknStatusCode::FEATURE_NOT_ENABLED, "\n\tVK_KHR_surface - not enabled.");
			}

			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

			WvkStatus WvkSurface::requestSupport(const WvkPhysicalDevicePtr wvk_physical_device_ptr, const WvkQueueFamilyPtr wvk_queue_family_ptr, bool& out) const noexcept {
				WvkStatus _status;

				// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				// Шаг 1. Проверяем, активировано ли расширение VK_KHR_surface
				// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				/*if constexpr (Build::find(WvkKhrSurfaceDT::s_getName())) {
					/*VkBool32 _support = VK_FALSE;
					out = false;

					// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					// Шаг 2. Вызываем vkGetPhysicalDeviceSurfaceSupportKHR для проверки поддержки present
					// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					auto _vk_res = wvk_physical_device_ptr->invokeWithVkPhysicalDeviceMethod(
						&WvkKhrSurfaceDT::wvkGetPhysicalDeviceSurfaceSupportKHR,
						m_create_info.wvk_khr_surface_dt,
						wvk_queue_family_ptr->getIndexFamily(),
						m_vk_surface,
						&_support
					);

					// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					// Шаг 3. Обрабатываем возможные ошибки вызова
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
						return _status.set(VknStatusCode::FAIL);
					}

					// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					// Шаг 4. Записываем результат проверки в out
					// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					out = (_support == VK_TRUE);

					// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					// Шаг 5. Возвращаем успешный статус
					// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					return _status.setOk();*/
				//}*/

				// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				// Шаг 6. Если VK_KHR_surface не активирован — возвращаем соответствующий статус
				// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				return _status.set(VknStatusCode::FEATURE_NOT_ENABLED, "\n\tWvkSurface::requestSupport - not enabled.");
			}

			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

			WvkStatus WvkSurface::requestPresentModes(const WvkPhysicalDevicePtr wvk_physical_device_ptr, std::vector<VkPresentModeKHR>& vk_present_modes) const noexcept {
				WvkStatus _status;

				// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				// Шаг 1. Проверяем, активировано ли расширение VK_KHR_surface
				// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				//if constexpr (Build::find(WvkKhrSurfaceDT::s_getName())) {

					// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					// Шаг 1. Инициализация объекта статуса
					// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					/*WvkStatus _status;

					// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					// Шаг 2. Запрашиваем количество доступных режимов презентации
					// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					uint32_t _count = 0;
					auto _vk_res = wvk_physical_device_ptr->invokeWithVkPhysicalDeviceMethod(
						&WvkKhrSurfaceDT::wvkGetPhysicalDeviceSurfacePresentModesKHR,
						m_create_info.wvk_khr_surface_dt,
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
					_vk_res = wvk_physical_device_ptr->invokeWithVkPhysicalDeviceMethod(
						&WvkKhrSurfaceDT::wvkGetPhysicalDeviceSurfacePresentModesKHR,
						m_create_info.wvk_khr_surface_dt,
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
					return _status.setOk();*/
				//}

				// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				// Шаг 5. Если VK_KHR_surface не активирован — возвращаем соответствующий статус
				// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				return _status.set(VknStatusCode::FEATURE_NOT_ENABLED, "\n\tWvkSurface::requestSupport - not enabled.");
			}

			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

			WvkStatus WvkSurface::requestCapabilities(const WvkPhysicalDevicePtr wvk_physical_device_ptr, VkSurfaceCapabilitiesKHR& vk_surface_capabilities_khr) const noexcept {
				WvkStatus _status;

				//if constexpr (Build::find(WvkKhrSurfaceDT::s_getName())) {

					// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					// Шаг 1. Вызов метода получения surface capabilities через invoke
					// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					/*auto _vk_res = wvk_physical_device_ptr->invokeWithVkPhysicalDeviceMethod(
						&WvkKhrSurfaceDT::wvkGetPhysicalDeviceSurfaceCapabilitiesKHR,
						m_create_info.wvk_khr_surface_dt,
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
					return _status.setOk();*/
				//}

				// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				// Шаг 5. Если VK_KHR_surface не активирован — возвращаем соответствующий статус
				// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				return _status.set(VknStatusCode::FEATURE_NOT_ENABLED, "\n\tWvkSurface::requestSupport - not enabled.");
			}

			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

			WvkStatus WvkSurface::requestCapabilities(const WvkPhysicalDevicePtr wvk_physical_device_ptr, const VkBaseInStructure* in, VkBaseOutStructure* out) const noexcept {
				WvkStatus _status;

				//if constexpr (Build::vulkan_api_version >= Build::VulkanVersion::VERSION_11 ||
					/*Build::find(WvkKhrGetSurfaceCapabilities2DT::s_getName())) {

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
					auto _vk_res = wvk_physical_device_ptr->invokeWithVkPhysicalDeviceMethod(
						&WvkInstanceDispatchTable::wvkGetPhysicalDeviceSurfaceCapabilities2KHR,
						m_create_info.wvk_instance_dt_ptr,
						&_info2,
						&_caps2
					);

					// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					// Шаг 4. Обработка возможных ошибок
					// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					if (_vk_res != VK_SUCCESS) {
						switch (_vk_res) {
						case VK_ERROR_OUT_OF_HOST_MEMORY:
							_status.append("\n\tWvkInstanceDispatchTable::wvkGetPhysicalDeviceSurfaceCapabilities2KHR - VK_ERROR_OUT_OF_HOST_MEMORY.");
							break;
						case VK_ERROR_OUT_OF_DEVICE_MEMORY:
							_status.append("\n\tWvkInstanceDispatchTable::wvkGetPhysicalDeviceSurfaceCapabilities2KHR - VK_ERROR_OUT_OF_DEVICE_MEMORY.");
							break;
						case VK_ERROR_SURFACE_LOST_KHR:
							_status.append("\n\tWvkInstanceDispatchTable::wvkGetPhysicalDeviceSurfaceCapabilities2KHR - VK_ERROR_SURFACE_LOST_KHR.");
							break;
						}
						return _status.set(VknStatusCode::FAIL);
					}*/

					// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					// Шаг 5. Успешное завершение
					// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					//return _status.setOk();
				//}

				// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				// Шаг 6. Если VK_KHR_surface не активирован — возвращаем соответствующий статус
				// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				return _status.set(VknStatusCode::FEATURE_NOT_ENABLED, "\n\tWvkSurface::requestSupport - not enabled.");
			}

			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

			WvkStatus WvkSurface::requestCapabilities(const WvkPhysicalDevicePtr wvk_physical_device_ptr, VkSurfaceProtectedCapabilitiesKHR& out) const noexcept {
				out.sType = VK_STRUCTURE_TYPE_SURFACE_PROTECTED_CAPABILITIES_KHR;
				out.pNext = nullptr;
				return requestCapabilities(wvk_physical_device_ptr, nullptr, reinterpret_cast<VkBaseOutStructure*>(&out));
			}

			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

			WvkStatus WvkSurface::requestPresentModeCompatibility(const WvkPhysicalDevicePtr wvk_physical_device_ptr, const VkPresentModeKHR& vk_present_mode, std::vector<VkPresentModeKHR>& out) const noexcept {
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
				_status = requestCapabilities(wvk_physical_device_ptr, _in, _out);
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
				_status = requestCapabilities(wvk_physical_device_ptr, _in, _out);
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

			WvkStatus WvkSurface::requestScalingCompatibility(const WvkPhysicalDevicePtr wvk_physical_device_ptr, const VkPresentModeKHR& vk_present_mode, VkSurfacePresentScalingCapabilitiesEXT& out) const noexcept {
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
				return requestCapabilities(wvk_physical_device_ptr, _in, out);
			}

			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

			WvkStatus WvkSurface::validationCreateInfo(void) const noexcept {
				WvkStatus _status;

				//if (m_create_info.wvk_instance_dt_ptr == nullptr) {
				//	return _status.set(VknStatusCode::FAIL, "\n\tWvkSurfaceCreateInfo::wvk_instance_dt_ptr - nullptr.");
				//}
				//else if (m_create_info.wvk_khr_surface_dt == nullptr) {
				//	return _status.set(VknStatusCode::FAIL, "\n\tWvkSurfaceCreateInfo::wvk_khr_surface_dt - nullptr.");
				//}
				//else if (m_create_info.wvk_khr_get_surface_capabilities2_dispatch_table == nullptr) {
				//	return _status.set(VknStatusCode::FAIL, "\n\tWvkSurfaceCreateInfo::wvk_khr_get_surface_capabilities2_dispatch_table - nullptr.");
				//}
				//else if (m_create_info.wvk_surface_platform_create_info == nullptr) {
				//	return _status.set(VknStatusCode::FAIL, "\n\tWvkSurfaceCreateInfo::wvk_surface_platform_create_info - nullptr.");
				//}				
			
				return _status.setOk();
			}

			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					
			WvkStatus WvkSurface::createVkSurface() noexcept {
				WvkStatus _status;

				VkWin32SurfaceCreateInfoKHR _create_info = {
					.sType = VK_STRUCTURE_TYPE_WIN32_SURFACE_CREATE_INFO_KHR,
					.pNext = nullptr,
					.flags = 0,
					.hinstance = static_cast<HINSTANCE>(m_create_info.win32.hinstance),
					.hwnd = static_cast<HWND>(m_create_info.win32.hwnd),
				};
				auto _vk_result = m_create_info.wvk_instance_ptr->getWvkDispatchTable()->wvkCreateWin32SurfaceKHR(&_create_info, nullptr, &m_vk_surface);
				//m_surface_impl->create(*static_cast<mswindows::WvkSurfaceMSWindowsCreateInfo*>(m_create_info.wvk_surface_platform_create_info));
						
				//m_surface_impl->requestVkSurface(m_vk_surface);

				return _status.setOk();
			}

			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

			void WvkSurface::reset(void) noexcept {
				m_create_info = {};
				m_vk_surface = VK_NULL_HANDLE;

				m_valid = false;
			}

			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

		} // namespace Extensions

	} // namespace wvk

} // namespace CGDev