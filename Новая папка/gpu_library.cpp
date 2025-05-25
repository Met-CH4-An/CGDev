////////////////////////////////////////////////////////////////
// ������ �������-����������
////////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////
// ������ ������������� �����
////////////////////////////////////////////////////////////////
#include "gpu_library.h"
////////////////////////////////////////////////////////////////
// ������ �������������
////////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////
// ������ ��� ����������
////////////////////////////////////////////////////////////////
#include "gpu_device.h"
#include "Vulkan/gpu_vkn_library.h"

namespace CGDev {

	namespace GPU {

		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

		bool createGpuLibrary(GpuLibraryPtr& gpu_library, const GpuLibraryCreateInfo& create_info) noexcept {

			// ~~~~~~~~~~~~~~~~
			// ����� �������� ������ ��� ����� ������ Private::GpuLibrary
			// � �������� ��� ������� ����� Private::GpuLibrary::create
			// ~~~~~~~~~~~~~~~~
			
			// ~~~~~~~~~~~~~~~~
			// ������ Private::GpuLibrary
			// ~~~~~~~~~~~~~~~~

			auto _gpu_library = new Private::GpuLibrary();

			if (_gpu_library->create(&create_info) == Private::StatusCode::FAIL) {
			
				gpu_library = nullptr;

				return false;
			}

			gpu_library = std::move(_gpu_library);

			return true;
		}

		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

		bool destroyGpuLibrary(GpuLibraryPtr& gpu_library) noexcept {

			// ~~~~~~~~~~~~~~~~
			// ���������� Private::GpuLibrary
			// ~~~~~~~~~~~~~~~~

			gpu_library->destroy();

			delete gpu_library;

			return true;
		}

		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

		namespace Private {

			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

			GpuLibrary::GpuLibrary(void) noexcept {
			}

			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

			GpuLibrary::~GpuLibrary(void) noexcept {
			}

			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

			StatusCode GpuLibrary::create(const GpuLibraryCreateInfo* create_info) noexcept {

				// ~~~~~~~~~~~~~~~~
				// �������� ������������� GpuObject
				// ~~~~~~~~~~~~~~~~

				GpuObject::create();
				
				// ~~~~~~~~~~~~~~~~
				// 
				// ~~~~~~~~~~~~~~~~

				m_create_info = *create_info;

				// ~~~~~~~~~~~~~~~~
				// ������ Tools::Log
				// ~~~~~~~~~~~~~~~~

				if (createLog() == StatusCode::FAIL) {

					return StatusCode::FAIL;
				}

				// ~~~~~~~~~~~~~~~~
				// ������ ���������� GAPI (������ ���� ������)
				// ~~~~~~~~~~~~~~~~

				if (createImpl() == StatusCode::FAIL) {

					Tools::addEntry(s_log, Tools::LogEntryError("createImpl() = StatusCode::FAIL"));

					return StatusCode::FAIL;
				}

				return StatusCode::SUCCESSFUL;
			}

			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

			void GpuLibrary::destroy(void) noexcept {

				// ~~~~~~~~~~~~~~~~
				// ��������� GpuDevice
				// ~~~~~~~~~~~~~~~~
				
				for (size_t ct_gpu_device_collection = 0; ct_gpu_device_collection < m_gpu_device_collection.size(); ++ct_gpu_device_collection) {
					auto it_gpu_device_collection = m_gpu_device_collection[ct_gpu_device_collection];

					it_gpu_device_collection->destroy();
				}

				// ~~~~~~~~~~~~~~~~
				// ��������� ���������� GAPI (������ ���� ������)
				// ~~~~~~~~~~~~~~~~

				destroyImpl();

				// ~~~~~~~~~~~~~~~~
				// ����������� ������������� GpuObject
				// ~~~~~~~~~~~~~~~~

				this->GpuObject::destroy();

				// ~~~~~~~~~~~~~~~~
				// ������� ������
				// ~~~~~~~~~~~~~~~~

				m_create_info = {};
				s_log = nullptr;
				m_impl = nullptr;
				m_gpu_device_collection = {};				
			}

			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

			StatusCode GpuLibrary::createGPUDevice(const GpuDeviceCreateInfo& create_info, GpuDevicePtr* gpu_device) noexcept {

				// ~~~~~~~~~~~~~~~~
				// ��������� ������ ��� GpuDevice
				// ����� GpuDevice::create()
				// ~~~~~~~~~~~~~~~~

				// ~~~~~~~~~~~~~~~~
				// ���������� GpuDevice
				// ~~~~~~~~~~~~~~~~

				GpuDevicePtr _gpu_device = nullptr;
				
				if (allocate(&_gpu_device) != StatusCode::SUCCESSFUL) {

					return StatusCode::FAIL;
				}

				// ~~~~~~~~~~~~~~~~
				// ������ GpuDevice
				// ~~~~~~~~~~~~~~~~

				if (_gpu_device->create(&create_info) != StatusCode::SUCCESSFUL) {

					return StatusCode::FAIL;
				}

				// ~~~~~~~~~~~~~~~~
				// 
				// ~~~~~~~~~~~~~~~~
				
				*gpu_device = m_gpu_device_collection.emplace_back(std::move(_gpu_device));

				// ~~~~~~~~~~~~~~~~
				// 
				// ~~~~~~~~~~~~~~~~

				return StatusCode::SUCCESSFUL;
			}

			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

			void GpuLibrary::destroyGPUDevice(GpuDevicePtr* gpu_device) noexcept {


			}

			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

			StatusCode GpuLibrary::createLog(void) noexcept {

				// ~~~~~~~~~~~~~~~~
				// ��������� Tools::LogCreateInfo
				// ~~~~~~~~~~~~~~~~

				Tools::LogCreateInfo _log_create_info = {};

				// ~~~~~~~~~~~~~~~~
				// ������ Tools::Log
				// ~~~~~~~~~~~~~~~~

				if (Tools::createLog(s_log, _log_create_info) == false) {
				
					return StatusCode::FAIL;

				} // if (Tools::createLog

				return StatusCode::SUCCESSFUL;
			}

			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

			StatusCode GpuLibrary::createImpl(void) noexcept {

				// ~~~~~~~~~~~~~~~~
				// ������
				// Build::s_type_hardware == Build::TypeHardware::VULKAN
				// ������
				// ~~~~~~~~~~~~~~~~
				
				if constexpr (Build::s_type_hardware == Build::TypeHardware::VULKAN) {

					// ~~~~~~~~~~~~~~~~
					// ��������� GpuVknLibraryCreateInfo
					// ~~~~~~~~~~~~~~~~

					Vulkan::GpuVknLibraryCreateInfo _gpu_vkn_library_create_info	= {};
					_gpu_vkn_library_create_info.log								= s_log;
					_gpu_vkn_library_create_info.gpu_library						= this;

					// ~~~~~~~~~~~~~~~~
					// ������ GpuVknLibrary
					// ~~~~~~~~~~~~~~~~

					Vulkan::GpuVknLibraryPtr _gpu_vkn_library = nullptr;

					allocate(&_gpu_vkn_library);

					if (_gpu_vkn_library->create(_gpu_vkn_library_create_info) == false) {

						Tools::addEntry(s_log, Tools::LogEntryError("Vulkan::GpuVknLibraryPtr::create() = StatusCode::FAIL"));

						return StatusCode::FAIL;

					} // if (_gpu_vkn_library->create(_gpu_vkn_library_create_info) == false)

					m_impl = std::move(_gpu_vkn_library);

					return StatusCode::SUCCESSFUL;

				}

				// ~~~~~~~~~~~~~~~~
				// ������
				// Build::s_type_hardware == Build::TypeHardware::VULKAN
				// �����
				// ~~~~~~~~~~~~~~~~
				
				Tools::addEntry(s_log, Tools::LogEntryError("Unknown::GpuUnknownLibraryPtr::create() = StatusCode::FAIL"));

				return StatusCode::FAIL;
			}

			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

			void GpuLibrary::destroyImpl(void) noexcept {

				// ~~~~~~~~~~~~~~~~
				// ������
				// Build::s_type_hardware == Build::TypeHardware::VULKAN
				// ������
				// ~~~~~~~~~~~~~~~~

				if constexpr (Build::s_type_hardware == Build::TypeHardware::VULKAN) {

					// ~~~~~~~~~~~~~~~~
					// 
					// ~~~~~~~~~~~~~~~~

					static_cast<Vulkan::GpuVknLibraryPtr>(m_impl)->destroy();
					
				}

				// ~~~~~~~~~~~~~~~~
				// ������
				// Build::s_type_hardware == Build::TypeHardware::VULKAN
				// �����
				// ~~~~~~~~~~~~~~~~
			}

			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

		} // namespace Private

	} // namespace GPU

} // namespace CGDev
