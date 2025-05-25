////////////////////////////////////////////////////////////////
// ������ �������-����������
////////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////
// ������ ������������� �����
////////////////////////////////////////////////////////////////
#include "gpu_device.h"
////////////////////////////////////////////////////////////////
// ������ �������������
////////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////
// ������ ��� ����������
////////////////////////////////////////////////////////////////
#include "gpu_library.h"

#include "Vulkan/gpu_vkn_device.h"

namespace CGDev {

	namespace GPU {

		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

		bool createGpuDevice(GpuDevicePtr& gpu_device, const GpuDeviceCreateInfo& create_info) noexcept {

			// ~~~~~~~~~~~~~~~~
			// ����� �������� ������ ��� ����� ������ Private::GpuDevice
			// � �������� ��� ������� ����� Private::GpuDevice::create
			// ~~~~~~~~~~~~~~~~

			// ~~~~~~~~~~~~~~~~
			// ������ Private::GpuDevice
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
				// ������
				// Build::type_build = Build::TypeBuild::DEBUG
				// ������
				// ~~~~~~~~~~~~~~~~
				// ��������� ���������� GpuDeviceCreateInfo
				// ~~~~~~~~~~~~~~~~
				
				if constexpr (Build::type_build == Build::TypeBuild::DEBUG) {

					if (validationCreateInfo(*create_info) == false) {

						return StatusCode::FAIL;
					}
				}

				// ~~~~~~~~~~~~~~~~
				// ������
				// Build::type_build = Build::TypeBuild::DEBUG
				// �����
				// ~~~~~~~~~~~~~~~~

				// ~~~~~~~~~~~~~~~~
				// ��������� GpuDeviceCreateInfo
				// ~~~~~~~~~~~~~~~~

				m_create_info = *create_info;

				// ~~~~~~~~~~~~~~~~
				// ������
				// Build::s_type_hardware = Build::TypeHardware::VULKAN
				// ������
				// ~~~~~~~~~~~~~~~~
				// ������ ����� ���� �������� �� ���������� �������
				// ~~~~~~~~~~~~~~~~
				
				if constexpr (Build::s_type_hardware == Build::TypeHardware::VULKAN) {

					// ~~~~~~~~~~~~~~~~
					// �������� ������� ������������� �������
					// ~~~~~~~~~~~~~~~~

					if (createImpl() == false) {

						Tools::addEntry(m_create_info.gpu_library->getLog(), Tools::LogEntryError("createImpl() = false"));

						return StatusCode::FAIL;
					}
				}

				// ~~~~~~~~~~~~~~~~
				// ������
				// Build::s_type_hardware = Build::TypeHardware::VULKAN
				// �����
				// ~~~~~~~~~~~~~~~~

				return StatusCode::SUCCESSFUL;
			}

			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

			void GpuDevice::destroy(void) noexcept {

				// ~~~~~~~~~~~~~~~~
				// ������
				// Build::s_type_hardware = Build::TypeHardware::VULKAN
				// ������
				// ~~~~~~~~~~~~~~~~
				// ������� ������������� �������
				// ~~~~~~~~~~~~~~~~

				if constexpr (Build::s_type_hardware == Build::TypeHardware::VULKAN) {

					destroyVulkanImpl();
				}

				// ~~~~~~~~~~~~~~~~
				// ������
				// Build::s_type_hardware = Build::TypeHardware::VULKAN
				// �����
				// ~~~~~~~~~~~~~~~~

				// ~~~~~~~~~~~~~~~~
				// ������� ������
				// ~~~~~~~~~~~~~~~~

				m_create_info		= {};
				m_impl				= nullptr;
			}

			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

			bool GpuDevice::execute(void) noexcept {

				// ~~~~~~~~~~~~~~~~
				// ������
				// Build::s_type_hardware = Build::TypeHardware::VULKAN
				// ������
				// ~~~~~~~~~~~~~~~~
				// ��������� �������� GpuDevice
				// ~~~~~~~~~~~~~~~~

				if constexpr (Build::s_type_hardware == Build::TypeHardware::VULKAN) {

					return executeVulkanImpl();
				}

				// ~~~~~~~~~~~~~~~~
				// ������
				// Build::s_type_hardware = Build::TypeHardware::VULKAN
				// �����
				// ~~~~~~~~~~~~~~~~

				return true;
			}

			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

			bool GpuDevice::validationCreateInfo(const GpuDeviceCreateInfo& create_info) const noexcept {

				// ~~~~~~~~~~~~~~~~
				// ��������� ���������� GpuDeviceCreateInfo
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
				// ������
				// Build::s_type_hardware = Build::TypeHardware::VULKAN
				// ������
				// ~~~~~~~~~~~~~~~~
				// GpuDevice �������� ��������� ����������� ��������,
				// � �� ���������� ������� �������� GpuVknDevice
				// ~~~~~~~~~~~~~~~~

				if constexpr (Build::s_type_hardware == Build::TypeHardware::VULKAN) {

					// ~~~~~~~~~~~~~~~~
					// ��������� GpuVknDeviceCreateInfo
					// ~~~~~~~~~~~~~~~~

					Vulkan::GpuVknDeviceCreateInfo _gpu_vkn_device_create_info = {};
					_gpu_vkn_device_create_info.log						= m_create_info.gpu_library->getLog();
					_gpu_vkn_device_create_info.gpu_device				= this;

					// ~~~~~~~~~~~~~~~~
					// ������ GpuVknDevice
					// ~~~~~~~~~~~~~~~~

					Vulkan::GpuVknDevicePtr _gpu_vkn_device = new Vulkan::GpuVknDevice();

					// ~~~~~~~~~~~~~~~~
					// ���� �� ���������� �������, �������� � ��������� �����
					// ~~~~~~~~~~~~~~~~

					if (_gpu_vkn_device->create(_gpu_vkn_device_create_info) == false) {

						Tools::addEntry(m_create_info.gpu_library->getLog(), Tools::LogEntryError("Vulkan::GpuVknDevice::create() = false"));

						return false;

					} // if

					// ~~~~~~~~~~~~~~~~
					// ���������
					// ~~~~~~~~~~~~~~~~

					m_impl = std::move(_gpu_vkn_device);
				}

				// ~~~~~~~~~~~~~~~~
				// ������
				// Build::s_type_hardware = Build::TypeHardware::VULKAN
				// �����
				// ~~~~~~~~~~~~~~~~

				return true;
			}

			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

			void GpuDevice::destroyVulkanImpl(void) noexcept {

				// ~~~~~~~~~~~~~~~~
				// ������
				// Build::s_type_hardware = Build::TypeHardware::VULKAN
				// ������
				// ~~~~~~~~~~~~~~~~
				// GpuDevice �������� ��������� ����������� ��������,
				// � �� ���������� ������� �������� GpuVknDevice
				// ~~~~~~~~~~~~~~~~

				if constexpr (Build::s_type_hardware == Build::TypeHardware::VULKAN) {

					// ~~~~~~~~~~~~~~~~
					// ��������� Vulkan::GpuVknDevice
					// ~~~~~~~~~~~~~~~~

					static_cast<Vulkan::GpuVknDevicePtr>(m_impl)->destroy();

					delete static_cast<Vulkan::GpuVknDevicePtr>(m_impl);
				}

				// ~~~~~~~~~~~~~~~~
				// ������
				// Build::s_type_hardware = Build::TypeHardware::VULKAN
				// �����
				// ~~~~~~~~~~~~~~~~
			}

			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

			bool GpuDevice::executeVulkanImpl(void) noexcept {

				// ~~~~~~~~~~~~~~~~
				// ������
				// Build::s_type_hardware = Build::TypeHardware::VULKAN
				// ������
				// ~~~~~~~~~~~~~~~~
				// ��� ���������� ���������� �������� GpuDevice, ������� ����������
				// � Vulkan::GpuVknDevice
				// ~~~~~~~~~~~~~~~~

				if constexpr (Build::s_type_hardware == Build::TypeHardware::VULKAN) {

					// ~~~~~~~~~~~~~~~~
					// ��������� �������� ����� Vulkan::GpuVknDevice
					// ~~~~~~~~~~~~~~~~

					return static_cast<Vulkan::GpuVknDevicePtr>(m_impl)->execute();					
				}

				// ~~~~~~~~~~~~~~~~
				// ������
				// Build::s_type_hardware = Build::TypeHardware::VULKAN
				// �����
				// ~~~~~~~~~~~~~~~~
			
				return true;
			}

			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

		} // namespace Private

	} // namespace GPU

} // namespace CGDev