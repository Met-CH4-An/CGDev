#ifndef CGDEV_SOURCE_GPU_PRIVATE___PRIVATE_H
#define CGDEV_SOURCE_GPU_PRIVATE___PRIVATE_H
////////////////////////////////////////////////////////////////
// секция форвард-декларации
////////////////////////////////////////////////////////////////
#include "../gpu.h"
////////////////////////////////////////////////////////////////
// секция имплементации
////////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////
// секция родительского класса
////////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////
// секция для остального
////////////////////////////////////////////////////////////////
#include <unordered_set>
#include <map>
#include <set>
#include <functional>
#include <type_traits>
#include <bitset>
#include <algorithm>
#include <numeric>
#include <stdarg.h>

// Шаблон для включения битовых операций только для enum'ов, помеченных как поддерживающих флаги
template<typename E>
struct EnableEnumFlags {
	static constexpr bool value = false;
};

// Макрос для удобства активации флагов для нужного enum
#define ENABLE_ENUM_FLAGS(EnumType)                            \
	namespace CGDev::GPU::Private::Vulkan {                    \
		template<>                                              \
		struct EnableEnumFlags<EnumType> {                     \
			static constexpr bool value = true;                \
		};                                                      \
	}

// Основные битовые операторы
template<typename E>
constexpr typename std::enable_if<EnableEnumFlags<E>::value, E>::type
operator|(E lhs, E rhs) {
	using underlying = typename std::underlying_type<E>::type;
	return static_cast<E>(
		static_cast<underlying>(lhs) | static_cast<underlying>(rhs)
		);
}

template<typename E>
constexpr typename std::enable_if<EnableEnumFlags<E>::value, E>::type
operator&(E lhs, E rhs) {
	using underlying = typename std::underlying_type<E>::type;
	return static_cast<E>(
		static_cast<underlying>(lhs) & static_cast<underlying>(rhs)
		);
}

template<typename E>
constexpr typename std::enable_if<EnableEnumFlags<E>::value, E>::type
operator~(E value) {
	using underlying = typename std::underlying_type<E>::type;
	return static_cast<E>(
		~static_cast<underlying>(value)
		);
}

template<typename E>
constexpr typename std::enable_if<EnableEnumFlags<E>::value, E&>::type
operator|=(E& lhs, E rhs) {
	lhs = lhs | rhs;
	return lhs;
}

template<typename E>
constexpr typename std::enable_if<EnableEnumFlags<E>::value, E&>::type
operator&=(E& lhs, E rhs) {
	lhs = lhs & rhs;
	return lhs;
}

template<typename E>
constexpr typename std::enable_if<EnableEnumFlags<E>::value, bool>::type
has_flag(E value, E flag) {
	return (value & flag) == flag;
}

namespace CGDev {

	namespace GPU {

		namespace Private {

			namespace Build {

				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				/*!	\brief
				*/
				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				enum class TypeBuild {

					UNKNOWN					= 0,
					RELEASE					= 1,
					DEBUG					= 2

				}; // enum class TypeBuild

				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				/*!	\brief
				*/
				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				enum class TypeHardware {

					UNKNOWN					= 0,
					VULKAN					= 1,

				}; // enum class TypeHardware

				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				/*!	\brief
				*/
				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				enum class TypePlatform {

					UNKNOWN					= 0,
					MSWINDOWS				= 1,

				}; // enum class TypePlatform

				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				/*!	\brief
				*/
				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				static const TypeBuild type_build = TypeBuild::DEBUG;

				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				/*!	\brief
				*/
				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				static const TypeHardware s_type_hardware = TypeHardware::VULKAN;

				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				/*!	\brief
				*/
				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				static const TypePlatform s_type_platform = TypePlatform::MSWINDOWS;

				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				/*!	\brief
				*/
				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				static const std::string engine_name = "GPU";

			} // namespace Build




#define ENABLE_ENUM_FLAGS1(Enum) \
inline Enum operator|(Enum a, Enum b) { return static_cast<Enum>(static_cast<std::underlying_type_t<Enum>>(a) | static_cast<std::underlying_type_t<Enum>>(b)); } \
inline Enum operator&(Enum a, Enum b) { return static_cast<Enum>(static_cast<std::underlying_type_t<Enum>>(a) & static_cast<std::underlying_type_t<Enum>>(b)); } \
inline Enum operator~(Enum a)         { return static_cast<Enum>(~static_cast<std::underlying_type_t<Enum>>(a)); } \
inline Enum& operator|=(Enum& a, Enum b) { a = a | b; return a; } \
inline Enum& operator&=(Enum& a, Enum b) { a = a & b; return a; }


				



			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			/*!	\brief
			*/
			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			
			using voidSptr = std::shared_ptr<void>;





			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			/*!	\brief
			*/
			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

			class GpuBuild;
			using GpuBuildPtr								= GpuBuild * ;
			using GpuBuildPtrArr							= std::vector<GpuBuildPtr>;
			using GpuBuildWptr								= std::weak_ptr<GpuBuild>;
			using GpuBuildWptrArr							= std::vector<GpuBuildWptr>;
			using GpuBuildSptr								= std::shared_ptr<GpuBuild>;
			using GpuBuildSptrArr							= std::vector<GpuBuildSptr>;





			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			/*!	\brief
			*/
			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

			enum class StatusCode {

				SUCCESSFUL					= 1,
				FAIL						= 2,
				CREATE_INFO_NO_VALID		= 3,


				UNKNOWN						= 4

			}; // enum class StatusCode

			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			/*!	\brief
			*/
			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

			struct Status {

				StatusCode				code = StatusCode::UNKNOWN;
				std::string				message_old = "";
				
			}; // enum class StatusCode

		} // namespace Private

	} // namespace GPU

} // namespace CGDev

#endif // CGDEV_SOURCE_GPU_PRIVATE___PRIVATE_H