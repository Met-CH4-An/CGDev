////////////////////////////////////////////////////////////////
// секция форвард-декларации
////////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////
// секция заголовочного файла
////////////////////////////////////////////////////////////////
#include "wvk_loader_dispatch_table.h"
////////////////////////////////////////////////////////////////
// секция имплементации
////////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////
// секция для остального
////////////////////////////////////////////////////////////////
#include "wvk_loader.h"

namespace CGDev {

	namespace wvk {

		// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

		WvkLoaderDispatchTable::WvkLoaderDispatchTable(void) noexcept {
		}

		// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

		WvkLoaderDispatchTable::~WvkLoaderDispatchTable(void) noexcept {
		}

		// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

		WvkStatus WvkLoaderDispatchTable::create(const WvkLoaderDispatchTableCreateInfo& create_info) noexcept {
			WvkStatus _status;

			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			// Шаг 1. Проверка: была ли уже инициализирована таблица
			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			if (m_valid) {
				return _status.set(VknStatusCode::ALREADY_INITIALIZED);
			}

			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			// Шаг 2. Сохраняем входную структуру
			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			m_create_info = create_info;

			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			// Шаг 3. Проверка валидности входных данных (если валидация включена)
			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			if constexpr (Build::ValidationBuildInfo::enable) {
				_status = validationCreateInfo();
				if (!_status) {
					reset();  // Очистка ресурсов перед выходом
					return _status.set(VknStatusCode::FAIL, "\n\tWvkLoaderDispatchTable::validationCreateInfo - fail.");
				}
			}

			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			// Шаг 4. Загружаем процедуры и формируем таблицу
			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			_status = loadProcedure();
			if (!_status) {
				reset();  // Очистка при неудаче
				return _status.set(VknStatusCode::FAIL, "\n\tWvkLoaderDispatchTable::loadProcedure - fail.");
			}

			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			// Шаг 5. Помечаем таблицу как валидную
			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			m_valid = true;

			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			// Шаг 6. Возвращаем успех
			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			return _status.setOk();
		}

		// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

		void WvkLoaderDispatchTable::destroy(void) noexcept {
			reset();
		}

		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

		WvkStatus WvkLoaderDispatchTable::validationCreateInfo(void) const noexcept {
			WvkStatus _status;

			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			// Шаг 1. Проверка обязательного указателя wvk_loader
			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			if (m_create_info.wvk_loader == nullptr) {
				return _status.set(VknStatusCode::FAIL, "\n\tWvkLoaderDispatchTableCreateInfo::wvk_loader - nullptr.");
			}

			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			// Шаг 2. Все проверки пройдены успешно
			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			return _status.setOk();
		}

		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

		WvkStatus WvkLoaderDispatchTable::loadProcedure(void) noexcept {
			WvkStatus _status;

			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			// Шаг 1. Запрос загрузки глобальных процедур Vulkan через WvkLoader
			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			std::vector<WvkVulkanProcedureInfo> _procedures = {
				{ "vkEnumerateInstanceVersion", reinterpret_cast<void**>(&m_vkEnumerateInstanceVersion) },
				{ "vkEnumerateInstanceLayerProperties", reinterpret_cast<void**>(&m_vkEnumerateInstanceLayerProperties) },
				{ "vkEnumerateInstanceExtensionProperties", reinterpret_cast<void**>(&m_vkEnumerateInstanceExtensionProperties) },
				{ "vkCreateInstance", reinterpret_cast<void**>(&m_vkCreateInstance) }
			};
			_status = m_create_info.wvk_loader->loadProcedure(VK_NULL_HANDLE, _procedures);
				
			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			// Шаг 2. Проверка результата
			// Если хотя бы одна функция не загружена — фиксируем ошибку
			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			if (!_status) {
				return _status.set(VknStatusCode::FAIL, "\n\tWvkLoader::loadProcedure - fail.");
			}

			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			// Шаг 3. Успешная инициализация
			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			return _status.setOk();
		}

		// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

		void WvkLoaderDispatchTable::reset(void) noexcept {
			m_create_info = {};

			// ~~~~~~~~~~~~~~~~
			// Vulkan 1.0
			// ~~~~~~~~~~~~~~~~
			m_vkCreateInstance = VK_NULL_HANDLE;
			m_vkEnumerateInstanceExtensionProperties = VK_NULL_HANDLE;
			m_vkEnumerateInstanceLayerProperties = VK_NULL_HANDLE;

			// ~~~~~~~~~~~~~~~~
			// Vulkan 1.1
			// ~~~~~~~~~~~~~~~~
			m_vkEnumerateInstanceVersion = VK_NULL_HANDLE;

			m_valid = false;
		}

		// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

	} // namespace wvk

} // namespace CGDev
