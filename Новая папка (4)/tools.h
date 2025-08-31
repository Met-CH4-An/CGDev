#ifndef CGDEV_SOURCE_TOOLS__TOOLS_H
#define CGDEV_SOURCE_TOOLS__TOOLS_H
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
#include <source_location>
#include <memory>
#include <iostream>
#include <string>
#include <vector>
#include <map>
#include <functional>
#include <type_traits>
#include <tuple>

namespace CGDev {

	namespace Tools {

		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		/*!	\brief
		*/
		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		enum class LogEntryType : uint16_t {

			UNKNOWN = 0,

			INFO = 1,
			WARNING = 2,
			ERROR = 3

		}; //enum class LogEntryType



		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		/*!	\brief
		*/
		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		struct LogEntry {

			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			/*!	\brief
			*/
			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			constexpr inline LogEntry(const LogEntryType& type, const std::string& comment, const std::source_location& callLocation = std::source_location::current()) noexcept :
				m_type(type),
				m_comment(comment),
				m_additionalInfo(callLocation) {}

			LogEntryType				m_type = LogEntryType::UNKNOWN;
			std::string					m_comment = "";
			std::source_location		m_additionalInfo = {};

		}; // struct LogEntry



		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		/*!	\brief
		*/
		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		struct LogEntryInfo : public LogEntry {

			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			/*!	\brief
			*/
			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			constexpr inline LogEntryInfo(const std::string& comment, const std::source_location& callLocation = std::source_location::current()) noexcept :
				LogEntry(LogEntryType::INFO, comment, callLocation) {}

		}; // struct LogEntryInfo



		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		/*!	\brief
		*/
		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		struct LogEntryWarning : public LogEntry {

			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			/*!	\brief
			*/
			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			constexpr inline LogEntryWarning(const std::string& comment, const std::source_location& callLocation = std::source_location::current()) noexcept :
				LogEntry(LogEntryType::WARNING, comment, callLocation) {}

		}; // struct LogEntryWarning



		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		/*!	\brief
		*/
		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		struct LogEntryError : public LogEntry {

			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			/*!	\brief
			*/
			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			constexpr inline LogEntryError(const std::string& comment, const std::source_location& callLocation = std::source_location::current()) noexcept :
				LogEntry(LogEntryType::ERROR, comment, callLocation) {}

		}; // struct LogEntryError



		namespace Private {

			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			/*!	\brief
			*/
			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

			class Log;

		} // namespace Private

		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		/*!	\brief
		*/
		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		
		using			LogPtr								= Private::Log * ;
		using			LogPtrArr							= std::vector<LogPtr>;
		using			LogWptr								= std::weak_ptr<Private::Log>;
		using			LogWptrArr							= std::vector<LogWptr>;
		using			LogSptr								= std::shared_ptr<Private::Log>;
		using			LogSptrArr							= std::vector<LogSptr>;

		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		/*!	\brief
		*/
		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

		struct			LogEntry;
		using			LogEntryArr							= std::vector<LogEntry>;
		using			LogEntryPtr							= LogEntry * ;
		using			LogEntryPtrArr						= std::vector<LogEntryPtr>;
		using			LogEntryWptr						= std::weak_ptr<LogEntry>;
		using			LogEntryWptrArr						= std::vector<LogEntryWptr>;
		using			LogEntrySptr						= std::shared_ptr<LogEntry>;
		using			LogEntrySptrArr						= std::vector<LogEntrySptr>;



		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		/*!	\brief
		*/
		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		struct LogCreateInfo {

			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			/*!	\brief
			*/
			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			bool isValid(void) const noexcept;

		}; // struct LogCreateInfo



		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		/*!	\brief Краткое описание.
		*
		* Полное описание.
		*
		* @param log Параметр 0
		* @param log_create_info Параметр 1
		*/
		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		bool createLog(LogPtr& log, const LogCreateInfo& create_info) noexcept;



		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		/*!	\brief Краткое описание.
		*
		* Полное описание.
		*
		* @param log Параметр 0
		* @param log_create_info Параметр 1
		*/
		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		void createLog(LogSptr& log, const LogCreateInfo& create_info) noexcept;



		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		/*!	\brief Краткое описание.
		*
		* Полное описание.
		*
		* @param log Параметр 0
		* @param log_create_info Параметр 1
		*/
		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		void addEntry(const LogPtr log, const LogEntry& log_entry) noexcept;



		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		/*!	\brief Краткое описание.
		*
		* Полное описание.
		*
		* @param log Параметр 0
		* @param log_create_info Параметр 1
		*/
		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		void addEntry(const LogSptr& log, const LogEntry& log_entry) noexcept;

	} // namespace Tools

} // namespace CGDev

#endif // CGDEV_SOURCE_TOOLS__TOOLS_H