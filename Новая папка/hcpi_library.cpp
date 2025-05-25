////////////////////////////////////////////////////////////////
// секция форвард-декларации
////////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////
// секция заголовочного файла
////////////////////////////////////////////////////////////////
#include "hcpi_library.h"
#include "../rpi.h"
////////////////////////////////////////////////////////////////
// секция имплементации
////////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////
// секция для остального
////////////////////////////////////////////////////////////////
#include "Vulkan/hcpi_library_vulkan.h"

using HCPI_LIBRARY_PLATFORM_CREATE_INFO			= CGDev::RPI::Private::Vulkan::HCPI_LIBRARY_VULKAN_CREATE_INFO;
using HCPILibraryPlatform						= CGDev::RPI::Private::Vulkan::HCPILibraryVulkan;

namespace CGDev {

	namespace RPI {

		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

		void createLibrary(HCPILibrarySptr& hcpi_library, const HCPI_LIBRARY_CREATE_INFO& create_info) noexcept {

			// ~~~~~~~~~~~~~~~~
			// новый объект HCPILibrarySptr
			// ~~~~~~~~~~~~~~~~
				
			auto _hcpi_library = std::make_shared<Private::HCPILibrary>();

			_hcpi_library->create(create_info);

			if (_hcpi_library->isValid() == false) {

				return;
			}

			hcpi_library = std::move(_hcpi_library);
		}

		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

		bool HCPI_LIBRARY_CREATE_INFO::isValid(void) const noexcept {

			return true;
		}

		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

		namespace Private {

			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

			HCPILibrary::HCPILibrary(void) noexcept {
			}

			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

			HCPILibrary::~HCPILibrary(void) noexcept {
			}

			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

			void HCPILibrary::create(const HCPI_LIBRARY_CREATE_INFO& create_info) noexcept {

				// ~~~~~~~~~~~~~~~~
				// проверяем HCPI_LIBRARY_CREATE_INFO на валидность
				// ~~~~~~~~~~~~~~~~

				if (create_info.isValid() == false) {

					m_valid = false;

					return;
				}

				m_create_info = create_info;

				// ~~~~~~~~~~~~~~~~
				// создаём лог
				// ~~~~~~~~~~~~~~~~

				Tools::LogCreateInfo _log_create_info = {};

				Tools::createLog(m_log, _log_create_info);

				if (m_log == nullptr) {

					m_valid = false;
					
					return;
				}

				// ~~~~~~~~~~~~~~~~
				// создаём HCPILibraryPlatform
				// ~~~~~~~~~~~~~~~~

				createHCPILibraryPlatform();

				if (isValid() == false) {

					Tools::addEntry(getLog(), Tools::LogEntryError("Не удалось создать HCPILibraryPlatform"));

					return;
				}

				m_valid = true;
			}

			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

			void HCPILibrary::createHCPILibraryPlatform(void) noexcept {

				// ~~~~~~~~~~~~~~~~
				// описываем HCPI_LIBRARY_PLATFORM_CREATE_INFO
				// ~~~~~~~~~~~~~~~~

				HCPI_LIBRARY_PLATFORM_CREATE_INFO _hcpi_library_platform_create_info		= {};
				_hcpi_library_platform_create_info.library									= this;

				// ~~~~~~~~~~~~~~~~
				// создаём HCPILibraryPlatform
				// ~~~~~~~~~~~~~~~~

				auto _hcpi_library_platform = std::make_shared<HCPILibraryPlatform>();

				_hcpi_library_platform->create(_hcpi_library_platform_create_info);

				if (_hcpi_library_platform->isValid() == false) {

					Tools::addEntry(getLog(), Tools::LogEntryError("HCPILibrary не смог создать платформозависимую часть"));

					m_valid = false;

					return;
				}

				m_platform = std::move(_hcpi_library_platform);

				m_valid = true;
			}

			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

		} // namespace Private

	} // namespace RPI

} // namespace CGDev