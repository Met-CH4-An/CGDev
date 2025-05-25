#ifndef CGDEV_SOURCE_GPU__GPU_H
#define CGDEV_SOURCE_GPU__GPU_H
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
#include <vector>
#include <memory>
#include <string>
#include <array>
#include <optional>

namespace CGDev {

	namespace GPU {

		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		/*!	\brief
		*/
		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

		template<typename enum_bits>
		inline constexpr enum_bits operator| (enum_bits left, enum_bits right) noexcept {

			return static_cast<enum_bits>(static_cast<int>(left) | static_cast<int>(right));
		}

		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		/*!	\brief
		*/
		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

		template<typename enum_bits>
		inline constexpr enum_bits operator& (enum_bits left, enum_bits right) noexcept {

			return static_cast<enum_bits>(static_cast<int>(left) & static_cast<int>(right));
		}
		
		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		/*!	\brief
		*/
		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

		namespace Private {

			class GpuLibrary;
		}

		using			GpuLibrary								= Private::GpuLibrary;
		using			GpuLibraryPtr							= GpuLibrary* ;
		using			GpuLibraryPtrArr						= std::vector<GpuLibraryPtr>;
		using			GpuLibraryWptr							= std::weak_ptr<GpuLibrary>;
		using			GpuLibrarytWptrArr						= std::vector<GpuLibraryWptr>;
		using			GpuLibrarySptr							= std::shared_ptr<GpuLibrary>;
		using			GpuLibrarySptrArr						= std::vector<GpuLibrarySptr>;

		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		/*!	\brief
		*/
		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

		namespace Private {

			class GpuDevice;
		}

		using			GpuDevice								= Private::GpuDevice;
		using			GpuDevicePtr							= GpuDevice * ;
		using			GpuDevicePtrArr							= std::vector<GpuDevicePtr>;
		using			GpuDeviceWptr							= std::weak_ptr<GpuDevice>;
		using			GpuDeviceWptrArr						= std::vector<GpuDeviceWptr>;
		using			GpuDeviceSptr							= std::shared_ptr<GpuDevice>;
		using			GpuDeviceSptrArr						= std::vector<GpuDeviceSptr>;
	
		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		/*!	\brief
		*/
		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		struct GpuLibraryCreateInfo {

			std::string						application_name = "";
			std::vector<std::string>		feature_name_collection;

		}; // struct GpuLibraryCreateInfo

		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		/*!	\brief
		*/
		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		enum class GpuDeviceType {

			UNKNOWN					= 0x00000000,

			GRAPHICS				= 0x00000001,
			COMPUTE					= 0x00000002,
			VIDEO_DECODE			= 0x00000004	

		}; // enum class GpuDeviceType

		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		/*!	\brief
		*/
		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		struct GpuDeviceCreateInfo {

			GpuLibraryPtr		gpu_library = nullptr;
			GpuDeviceType		gpu_type = GpuDeviceType::UNKNOWN;

		}; // struct GpuDeviceCreateInfo

		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		/*!	\brief Краткое описание.
		*
		* Полное описание.
		*
		* @param library Параметр 0
		* @param library_create_info Параметр 1
		*/
		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		bool createGpuLibrary(GpuLibraryPtr& gpu_library, const GpuLibraryCreateInfo& create_info) noexcept;

		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		/*!	\brief Краткое описание.
		*
		* Полное описание.
		*
		* @param library Параметр 0
		* @param library_create_info Параметр 1
		*/
		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		bool destroyGpuLibrary(GpuLibraryPtr& gpu_library) noexcept;

		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		/*!	\brief Краткое описание.
		*
		* Полное описание.
		*
		* @param library Параметр 0
		* @param library_create_info Параметр 1
		*/
		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		bool createGpuDevice(GpuDevicePtr& gpu_device, const GpuDeviceCreateInfo& create_info) noexcept;

		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		/*!	\brief Краткое описание.
		*
		* Полное описание.
		*
		* @param library Параметр 0
		* @param library_create_info Параметр 1
		*/
		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		bool executeGpuDevice(GpuDevicePtr& gpu_device) noexcept;

		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		/*!	\brief Краткое описание.
		*
		* Полное описание.
		*
		* @param library Параметр 0
		* @param library_create_info Параметр 1
		*/
		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		//void createRenderQueue(RenderQueueSptr& render_queue, const RenderQueueCreateInfo& create_info) noexcept;

		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		/*!	\brief Краткое описание.
		*
		* Полное описание.
		*
		* @param library Параметр 0
		* @param library_create_info Параметр 1
		*/
		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		//void pushRenderBuffer(RenderQueueSptr& render_queue, const RenderBufferSptr& render_buffer) noexcept;

		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		/*!	\brief Краткое описание.
		*
		* Полное описание.
		*
		* @param library Параметр 0
		* @param library_create_info Параметр 1
		*/
		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		//void removeRenderBuffer(RenderQueueSptr& render_queue, const RenderBufferSptr& render_buffer) noexcept;
		 
		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		/*!	\brief Краткое описание.
		*
		* Полное описание.
		*
		* @param library Параметр 0
		* @param library_create_info Параметр 1
		*/
		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		//void createRenderBuffer(RenderBufferSptr& render_buffer, const RenderBufferCreateInfo& render_buffer_create_info) noexcept;
		
	} // namespace GPU

} // namespace CGDev

#endif // CGDEV_SOURCE_GPU__GPU_H