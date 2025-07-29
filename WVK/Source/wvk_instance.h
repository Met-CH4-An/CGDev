#ifndef CGDEV_WVK_SOURCE__WVK_INSTANCE_H
#define CGDEV_WVK_SOURCE__WVK_INSTANCE_H
////////////////////////////////////////////////////////////////
// ������ �������-����������
////////////////////////////////////////////////////////////////
#include "_wvk.h"
////////////////////////////////////////////////////////////////
// ������ �������������
////////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////
// ������ ������������� ������
////////////////////////////////////////////////////////////////
#include "wvk_base_object.h"
////////////////////////////////////////////////////////////////
// ������ ��� ����������
////////////////////////////////////////////////////////////////

namespace CGDev {

	namespace wvk {

		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		/*!	\brief
		*/
		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		struct WvkInstanceCreateInfo {
			WvkLoaderDispatchTablePtr wvk_loader_dispatch_table = nullptr;
		}; // struct WvkInstanceCreateInfo





		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		/*!	\brief
		*/
		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		class WvkInstance : public GpuObject {

		public:

			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			/*!	\brief
			*/
			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			WvkInstance(void) noexcept;

			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			/*!	\brief
			*/
			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			~WvkInstance(void) noexcept;

			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			/*!	\brief
			* ������ ������ WvkInstance � �������������� VkInstance.
			*
			* @details
			* ����� ��������� ��������� ������������� Vulkan-��������:
			* ��������� ������� ������, �������������� ���� � ����������,
			* ����� ������ VkInstance � ��������� ���� `m_valid`.
			* � ������ ������ ���������� ��������� ������������.
			*
			* @param[in] create_info
			* ���������, ���������� ��������� ��� �������� ��������.
			*
			* @return
			* ���������� ������ ���������� ��������:
			* - `SUCCESSFUL` � �������� ��������.
			* - `ALREADY_INITIALIZED` � ������� ��� ������.
			* - `FAIL` � ��� ����� ������ ������.
			*
			* @code
			* WvkInstance instance;
			* WvkInstanceCreateInfo info = { ... };
			* WvkStatus status = instance.create(info);
			* if (!status) {
			*     std::cerr << "������: " << status.message() << std::endl;
			* }
			* @endcode
			*/
			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			WvkStatus create(const WvkInstanceCreateInfo& create_info) noexcept;

			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			/*!	\brief ����������� Vulkan-����������.
				* 
				* ���� ����� �������� �� ���������� ������������ ���� ��������, ��������� � Vulkan-�����������.
				* �������� � ����:
				* - ����� ������� vkDestroyInstance ��� ����������� Vulkan-����������,
				* - ������� ���� ������, ��������� � ��������� ����������, ����� ���:
				*   - ������� ��������� m_create_info,
				*   - ��������� ��������� �� Vulkan-��������� (m_vk_instance),
				*   - ������� ��������� ���� � ���������� (m_layer_properties_collection, m_extension_properties_collection, m_layer_name_collection, m_extension_name_collection).
				* 
				* ����� ���������� ���� ������� ��������� Vulkan ������ �� ����� ��������, � ��� ��������� � ��� ������� ����� �����������.
				*/
			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			void destroy(void) noexcept;

		public:

			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			/*!	\brief
			*/
			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			template<typename Method, typename Object, typename... Args>
			inline std::enable_if_t<
				std::is_invocable_v<Method, Object, VkInstance, Args...>&&
				std::is_void_v<std::invoke_result_t<Method, Object, VkInstance, Args...>>,
				void
			>
				invokeWithVkInstanceMethod(Method&& method, Object&& object, Args&&... args);

			template<typename Method, typename Object, typename... Args>
			inline std::enable_if_t<
				std::is_invocable_v<Method, Object, VkInstance, Args...> &&
				!std::is_void_v<std::invoke_result_t<Method, Object, VkInstance, Args...>>,
				std::invoke_result_t<Method, Object, VkInstance, Args...>
			> invokeWithVkInstanceMethod(Method&& method, Object&& object, Args&&... args);

			template<typename Method, typename Object, typename... Args>
			inline std::enable_if_t<
				!std::is_invocable_v<Method, Object, VkInstance, Args...>,
				void
			>
				invokeWithVkInstanceMethod(Method&&, Object&&, Args&&...);

			template<typename Method, typename... Args>
			inline std::enable_if_t<
				std::is_invocable_v<Method, VkInstance, Args...>&&
				std::is_void_v<std::invoke_result_t<Method, VkInstance, Args...>>,
				void
			>
				invokeWithVkInstanceFunction(Method&& method, Args&&... args);

			template<typename Method, typename... Args>
			inline std::enable_if_t<
				std::is_invocable_v<Method, VkInstance, Args...> &&
				!std::is_void_v<std::invoke_result_t<Method, VkInstance, Args...>>,
				std::invoke_result_t<Method, VkInstance, Args...>
			>
				invokeWithVkInstanceFunction(Method&& method, Args&&... args);

		// hpp
		public:

			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			/*!	\@brief
			*/
			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			inline const WvkInstanceCreateInfo& getCreateInfo(void) const noexcept;

		// hpp
		private:

			friend class WvkInstanceDispatchTable;
			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			/*!	\@brief ���������� �������� ������ VkInstance.
			* 
			* ���� ����� ������������� ������ � ����������� Vulkan-�������� (`VkInstance`),
			* ���������� � �������� ������������� ������� `VknInstance`. 
			* ����� ��������� ��� ������ � �� �������� ��������� �������.
			*
			* @note ������������ �������� �������� ������� �� ���������� ������.
			* ������������ �� ������ ������� ���������� ��� �������� `VkInstance`.
			* ���������� �������� ����� ������� ������� �� `VknInstance`.
			*
			* @return ����������� ������ �� ������ `VkInstance`.
			*/
			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			inline const VkInstance& getVkInstance(void) const noexcept;

		// cpp
		private:

			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			/*!	\brief ��������� ���������� ��������� `WvkInstanceCreateInfo`.
			*
			* ����� ������������ ��� �������� ������������ ������� ������, ���������� ��� ��������
			* ���������� `WvkInstance`. � ���������, �� ���������, ��� ������������ ��������� ��
			* `wvk_loader_dispatch_table` �� �������� `nullptr`.
			*
			* ��� ����� ��� �������������� ������ ��� ���������� ������������� ��������������������
			* ���������� � Vulkan API.
			*
			* @note ����� �� ����������� ���������� � �������� `noexcept`.
			*
			* @return ���������� `WvkStatus::OK()`, ���� ��� �������� �������� �������.
			*         � ��������� ������ � ��� ������ � ��������� ��������� �������.
			*
			* @code
			* WvkInstance instance;
			* WvkStatus status = instance.validationCreateInfo();
			* if (!status) {
			*     log->error("������ ���������: {}", status.message());
			* }
			* @endcode
			*/
			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			WvkStatus validationCreateInfo(void) const noexcept;

			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			/*!	\brief �������������� � ���������� ���� Vulkan ��� ����������.
			*
			* ����� ��������� ������������ ���� ��������� ���� Vulkan � ���������� �� �
			* ������������������ ������, ���������� � ������ (���� �������� ��������� ���������).
			* � ������, ���� ���� �� ������, ����� ���������� ������. ����� ����� ������������ ������,
			* ����������� ��� ������������ ����, � ������������� ��������� ���������� � ��������� ���������.
			*
			* @note ����� ���������� `vkEnumerateInstanceLayerProperties` ��� ��������� ������ ��������� ����.
			*
			* @note ���� �������� ��������� ���������, ����� ��������� ������� ��������� ����
			*       � ��������� �� � ��������� �������������� ����.
			*
			* @note ����� �� ����������� ���������� (`noexcept`).
			*
			* @return ���������� ������ ����������. ���� ���� ������� ������������, ����������
			*         ������ "OK". � ������ ������ � ��������������� ��� ������ � ��������� ��������.
			*
			* @code
			* WvkInstance instance;
			* WvkStatus status = instance.prepareLayer();
			* if (!status) {
			*     log->error("������ ���������� ���� Vulkan: {}", status.message());
			* }
			* @endcode
			*/
			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			WvkStatus prepareLayer(void) noexcept;

			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			/*!	\brief �������������� ������ ���������� Vulkan, ����������� ��� �������� ����������.
			*
			* ����� ��������� ������ ��������� ���������� �� ���������, ���������� �� � ����������
			* (���� �������� ��������� ����� `wvk::Build::ValidationBuildInfo`) � ���������
			* ���������� ��������� ��� ����������� �������������.
			*
			* @return WvkStatus
			* - [out] ��� ��������� � ��������� �� ������ ��� �������������.
			*
			* @code
			* WvkStatus status = instance.prepareExtension();
			* if (status.failed()) {
			*     std::cerr << status.message_old;
			* }
			* @endcode
			*/
			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			WvkStatus prepareExtension(void) noexcept;

			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			/*!	\brief ������ ��������� Vulkan.
			*
			* ����� �������������� � ������ ��������� Vulkan � �������������� ����������
			* �������� � ������������ ���� � ����������. �� ��������� ��������� `VkApplicationInfo`
			* � ��������� `VkInstanceCreateInfo`, ������� ��������� � ������� `vknCreateInstance()`.
			* ���� �������� ���������� Vulkan ����������� ��������, ����� ���������� ��������� ���������
			* �� ������.
			*
			* @note ����� ���������� `VkResult` ��� �������� ���������� �������� ����������.
			*       ������ ���������������� � ����������� �� ���� ������ Vulkan.
			*
			* @note ����� �� ����������� ���������� (`noexcept`).
			*
			* @return ���������� ������ ����������. ���� �������� ���������� ������ �������,
			*         ���������� ������ "OK". � ������ ������ � ��������������� ��� ������
			*         � ��������� ��������.
			*
			* @code
			* WvkInstance instance;
			* WvkStatus status = instance.createVkInstance();
			* if (!status) {
			*     log->error("������ �������� ���������� Vulkan: {}", status.message());
			* }
			* @endcode
			*/
			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			WvkStatus createVkInstance(void) noexcept;

			void reset(void) noexcept;

		private:

			WvkInstanceCreateInfo m_create_info = {};
			VkInstance m_vk_instance = nullptr;
			VkLayerPropertiesArr m_layer_properties_collection;
			VkExtensionPropertiesArr m_extension_properties_collection;
			std::vector<const char*> m_layer_name_collection;
			std::vector<const char*> m_extension_name_collection;
		}; // class WvkInstance

	} // namespace wvk

} // namespace CGDev

#include "wvk_instance.hpp"

#endif // CGDEV_WVK_SOURCE__WVK_INSTANCE_H
