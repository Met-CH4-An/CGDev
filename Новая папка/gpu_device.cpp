////////////////////////////////////////////////////////////////
// секция форвард-декларации
////////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////
// секция заголовочного файла
////////////////////////////////////////////////////////////////
#include "gpu_device.h"
////////////////////////////////////////////////////////////////
// секция имплементации
////////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////
// секция для остального
////////////////////////////////////////////////////////////////
#include "gpu_library.h"

#include "Vulkan/gpu_vkn_device.h"

namespace CGDev {

	namespace GPU {

		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

		bool createGpuDevice(GpuDevicePtr& gpu_device, const GpuDeviceCreateInfo& create_info) noexcept {

			// ~~~~~~~~~~~~~~~~
			// метод выделяет память под новый объект Private::GpuDevice
			// и пытается его создать через Private::GpuDevice::create
			// ~~~~~~~~~~~~~~~~

			// ~~~~~~~~~~~~~~~~
			// создаём Private::GpuDevice
			// ~~~~~~~~~~~~~~~~

			auto _gpu_device = new Private::GpuDevice();

			//if (_gpu_device->create(create_info) == false) {

			//	_gpu_device = nullptr;

			//	return false;
			//}

			//gpu_device = std::move(_gpu_device);

			return true;
		}

		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

		bool executeGpuDevice(GpuDevicePtr& gpu_device) noexcept {

			// ~~~~~~~~~~~~~~~~
			// 
			// ~~~~~~~~~~~~~~~~

			return gpu_device->execute();

		}

		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

		namespace Private {

			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

			GpuDevice::GpuDevice(void) noexcept {
			}

			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

			GpuDevice::~GpuDevice(void) noexcept {
			}

			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

			StatusCode GpuDevice::create(const GpuDeviceCreateInfo* create_info) noexcept {

				// ~~~~~~~~~~~~~~~~
				// СБОРКА
				// Build::type_build = Build::TypeBuild::DEBUG
				// НАЧАЛО
				// ~~~~~~~~~~~~~~~~
				// проверяем валидность GpuDeviceCreateInfo
				// ~~~~~~~~~~~~~~~~
				
				if constexpr (Build::type_build == Build::TypeBuild::DEBUG) {

					if (validationCreateInfo(*create_info) == false) {

						return StatusCode::FAIL;
					}
				}

				// ~~~~~~~~~~~~~~~~
				// СБОРКА
				// Build::type_build = Build::TypeBuild::DEBUG
				// КОНЕЦ
				// ~~~~~~~~~~~~~~~~

				// ~~~~~~~~~~~~~~~~
				// сохраняем GpuDeviceCreateInfo
				// ~~~~~~~~~~~~~~~~

				m_create_info = *create_info;

				// ~~~~~~~~~~~~~~~~
				// СБОРКА
				// Build::s_type_hardware = Build::TypeHardware::VULKAN
				// НАЧАЛО
				// ~~~~~~~~~~~~~~~~
				// данная часть кода отвечает за реализацию вулкана
				// ~~~~~~~~~~~~~~~~
				
				if constexpr (Build::s_type_hardware == Build::TypeHardware::VULKAN) {

					// ~~~~~~~~~~~~~~~~
					// пытаемся создать имплементацию вулкана
					// ~~~~~~~~~~~~~~~~

					if (createImpl() == false) {

						Tools::addEntry(m_create_info.gpu_library->getLog(), Tools::LogEntryError("createImpl() = false"));

						return StatusCode::FAIL;
					}
				}

				// ~~~~~~~~~~~~~~~~
				// СБОРКА
				// Build::s_type_hardware = Build::TypeHardware::VULKAN
				// КОНЕЦ
				// ~~~~~~~~~~~~~~~~

				return StatusCode::SUCCESSFUL;
			}

			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

			void GpuDevice::destroy(void) noexcept {

				// ~~~~~~~~~~~~~~~~
				// СБОРКА
				// Build::s_type_hardware = Build::TypeHardware::VULKAN
				// НАЧАЛО
				// ~~~~~~~~~~~~~~~~
				// удаляем имплементацию вулкана
				// ~~~~~~~~~~~~~~~~

				if constexpr (Build::s_type_hardware == Build::TypeHardware::VULKAN) {

					destroyVulkanImpl();
				}

				// ~~~~~~~~~~~~~~~~
				// СБОРКА
				// Build::s_type_hardware = Build::TypeHardware::VULKAN
				// КОНЕЦ
				// ~~~~~~~~~~~~~~~~

				// ~~~~~~~~~~~~~~~~
				// очистка данных
				// ~~~~~~~~~~~~~~~~

				m_create_info		= {};
				m_impl				= nullptr;
			}

			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

			bool GpuDevice::execute(void) noexcept {

				// ~~~~~~~~~~~~~~~~
				// СБОРКА
				// Build::s_type_hardware = Build::TypeHardware::VULKAN
				// НАЧАЛО
				// ~~~~~~~~~~~~~~~~
				// выполняем операции GpuDevice
				// ~~~~~~~~~~~~~~~~

				if constexpr (Build::s_type_hardware == Build::TypeHardware::VULKAN) {

					return executeVulkanImpl();
				}

				// ~~~~~~~~~~~~~~~~
				// СБОРКА
				// Build::s_type_hardware = Build::TypeHardware::VULKAN
				// КОНЕЦ
				// ~~~~~~~~~~~~~~~~

				return true;
			}

			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

			bool GpuDevice::validationCreateInfo(const GpuDeviceCreateInfo& create_info) const noexcept {

				// ~~~~~~~~~~~~~~~~
				// проверяем валидность GpuDeviceCreateInfo
				// ~~~~~~~~~~~~~~~~

				if (create_info.gpu_library == nullptr) {

					return false;
				}
				
				if (create_info.gpu_type == GpuDeviceType::UNKNOWN) {

					Tools::addEntry(create_info.gpu_library->getLog(), Tools::LogEntryError("GpuDeviceCreateInfo::gpu_type == GpuDeviceType::UNKNOWN"));

					return false;
				}

				return true;
			}

			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

			bool GpuDevice::createImpl(void) noexcept {

				// ~~~~~~~~~~~~~~~~
				// СБОРКА
				// Build::s_type_hardware = Build::TypeHardware::VULKAN
				// НАЧАЛО
				// ~~~~~~~~~~~~~~~~
				// GpuDevice является хардварно независимым объектом,
				// а за реализацию вулкана отвечает GpuVknDevice
				// ~~~~~~~~~~~~~~~~

				if constexpr (Build::s_type_hardware == Build::TypeHardware::VULKAN) {

					// ~~~~~~~~~~~~~~~~
					// заполняем GpuVknDeviceCreateInfo
					// ~~~~~~~~~~~~~~~~

					Vulkan::GpuVknDeviceCreateInfo _gpu_vkn_device_create_info = {};
					_gpu_vkn_device_create_info.log						= m_create_info.gpu_library->getLog();
					_gpu_vkn_device_create_info.gpu_device				= this;

					// ~~~~~~~~~~~~~~~~
					// создаём GpuVknDevice
					// ~~~~~~~~~~~~~~~~

					Vulkan::GpuVknDevicePtr _gpu_vkn_device = new Vulkan::GpuVknDevice();

					// ~~~~~~~~~~~~~~~~
					// если не получилось создать, логируем и завершаем метод
					// ~~~~~~~~~~~~~~~~

					if (_gpu_vkn_device->create(_gpu_vkn_device_create_info) == false) {

						Tools::addEntry(m_create_info.gpu_library->getLog(), Tools::LogEntryError("Vulkan::GpuVknDevice::create() = false"));

						return false;

					} // if

					// ~~~~~~~~~~~~~~~~
					// сохраняем
					// ~~~~~~~~~~~~~~~~

					m_impl = std::move(_gpu_vkn_device);
				}

				// ~~~~~~~~~~~~~~~~
				// СБОРКА
				// Build::s_type_hardware = Build::TypeHardware::VULKAN
				// КОНЕЦ
				// ~~~~~~~~~~~~~~~~

				return true;
			}

			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

			void GpuDevice::destroyVulkanImpl(void) noexcept {

				// ~~~~~~~~~~~~~~~~
				// СБОРКА
				// Build::s_type_hardware = Build::TypeHardware::VULKAN
				// НАЧАЛО
				// ~~~~~~~~~~~~~~~~
				// GpuDevice является хардварно независимым объектом,
				// а за реализацию вулкана отвечает GpuVknDevice
				// ~~~~~~~~~~~~~~~~

				if constexpr (Build::s_type_hardware == Build::TypeHardware::VULKAN) {

					// ~~~~~~~~~~~~~~~~
					// завершаем Vulkan::GpuVknDevice
					// ~~~~~~~~~~~~~~~~

					static_cast<Vulkan::GpuVknDevicePtr>(m_impl)->destroy();

					delete static_cast<Vulkan::GpuVknDevicePtr>(m_impl);
				}

				// ~~~~~~~~~~~~~~~~
				// СБОРКА
				// Build::s_type_hardware = Build::TypeHardware::VULKAN
				// КОНЕЦ
				// ~~~~~~~~~~~~~~~~
			}

			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

			bool GpuDevice::executeVulkanImpl(void) noexcept {

				// ~~~~~~~~~~~~~~~~
				// СБОРКА
				// Build::s_type_hardware = Build::TypeHardware::VULKAN
				// НАЧАЛО
				// ~~~~~~~~~~~~~~~~
				// для выполнения заложенных операций GpuDevice, передаём управление
				// в Vulkan::GpuVknDevice
				// ~~~~~~~~~~~~~~~~

				if constexpr (Build::s_type_hardware == Build::TypeHardware::VULKAN) {

					// ~~~~~~~~~~~~~~~~
					// выполняем операции через Vulkan::GpuVknDevice
					// ~~~~~~~~~~~~~~~~

					return static_cast<Vulkan::GpuVknDevicePtr>(m_impl)->execute();					
				}

				// ~~~~~~~~~~~~~~~~
				// СБОРКА
				// Build::s_type_hardware = Build::TypeHardware::VULKAN
				// КОНЕЦ
				// ~~~~~~~~~~~~~~~~
			
				return true;
			}

			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

		} // namespace Private

	} // namespace GPU

} // namespace CGDev