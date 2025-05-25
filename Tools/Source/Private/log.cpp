////////////////////////////////////////////////////////////////
// секция форвард-декларации
////////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////
// секция заголовочного файла
////////////////////////////////////////////////////////////////
#include "log.h"
////////////////////////////////////////////////////////////////
// секция имплементации
////////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////
// секция для остального
////////////////////////////////////////////////////////////////

namespace CGDev {

	namespace Tools {

		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

		bool LogCreateInfo::isValid(void) const noexcept {

			return true;
		}

		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

		bool createLog(LogPtr& log, const LogCreateInfo& create_info) noexcept {

			// ~~~~~~~~~~~~~~~~
			// создание нового Log
			// ~~~~~~~~~~~~~~~~

			auto _log = new Private::Log();

			_log->create(create_info);

			if (_log->isValid() == false) {

				log = nullptr;

				return false;
			}

			log = std::move(_log);

			return true;

		}

		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

		void createLog(LogSptr& log, const LogCreateInfo& create_info) noexcept {

			// ~~~~~~~~~~~~~~~~
			// создание нового Log
			// ~~~~~~~~~~~~~~~~

			auto _log = std::make_shared<Private::Log>();

			_log->create(create_info);
			
			if (_log->isValid() == false) {

				log = nullptr;

				return;
			}

			log = std::move(_log);

		}

		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

		void addEntry(const LogPtr log, const LogEntry& log_entry) noexcept {

			log->print(log_entry);
		}

		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

		void addEntry(const LogSptr& log, const LogEntry& log_entry) noexcept {
			
			log->print(log_entry);
		}

		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

		namespace Private {

			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

			Log::Log(void) noexcept {
			}

			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

			Log::~Log(void) noexcept {

				throwOff();
			}

			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

			void Log::create(const LogCreateInfo& create_info) noexcept {

				// ~~~~~~~~~~~~~~~~
				// проверяем LogCreateInfo на валидность
				// ~~~~~~~~~~~~~~~~

				if (create_info.isValid() == false) {

					m_valid = false;

					return;
				}

				m_create_info = create_info;

				

				m_valid = true;
			}

			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

			void Log::destroy(void) noexcept {
				
				throwOff();
			}

			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

			void Log::print(const LogEntry& log_entry) noexcept {

				m_log_entry_collection.emplace_back(log_entry);

				// ~~~~~~~~~~~~~~~~
				// если количество записей в m_log_entry_collection превышает константу m_entries_to_throw_off_count
				// производим сброс данных
				// ~~~~~~~~~~~~~~~~

				//if (m_log_entry_collection.size() > m_entries_to_throw_off_count) {
				
					throwOff();
				//}
			}

			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

			void Log::throwOff(void) noexcept {

				// ~~~~~~~~~~~~~~~~
				// перебираем коллекцию LogEntry
				// ~~~~~~~~~~~~~~~~

				for (size_t ct_0 = 0; ct_0 < m_log_entry_collection.size(); ++ct_0) {
					const auto& it_log_entry = m_log_entry_collection[ct_0];
					
					std::string _typeEntryText = "";

					// ~~~~~~~~~~~~~~~~
					// преобразование из LogEntryType в текст
					// ~~~~~~~~~~~~~~~~

					switch (it_log_entry.m_type) {

					case LogEntryType::INFO:

						_typeEntryText = "INFO";

						break;

					case LogEntryType::WARNING:

						_typeEntryText = "WARNING";

						break;

					case LogEntryType::ERROR:

						_typeEntryText = "ERROR";

						break;

					default:
						break;

					} // switch (m_type)

					// ~~~~~~~~~~~~~~~~
					// формируем запись
					// ~~~~~~~~~~~~~~~~

					std::string _formattedEntry = "Type: " + std::move(_typeEntryText) + "\n" +								// тип записи
						"Comment: " + it_log_entry.m_comment + "\n" +														// комментарий
						"File name: " + std::move(it_log_entry.m_additionalInfo.file_name()) + "\n" +						// имя файла из std::source_location
						"Function name: " + std::move(it_log_entry.m_additionalInfo.function_name()) + "\n" +				// имя функции из std::source_location
						"line: " + std::move(std::to_string(it_log_entry.m_additionalInfo.line())) + "\n" +					// номер строки из std::source_location
						"column: " + std::move(std::to_string(it_log_entry.m_additionalInfo.column())) + "\n";				// номер столбца из std::source_location

					// ~~~~~~~~~~~~~~~~
					// вывод
					// ~~~~~~~~~~~~~~~~
					
					std::cout << std::move(_formattedEntry);
					
				} // for (size_t ct_0 = 0;

				m_log_entry_collection.clear();
			}

			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

		} // namespace Private

	} // namespace Tools

} // namespace CGDev