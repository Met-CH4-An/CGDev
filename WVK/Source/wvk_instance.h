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
			std::vector<std::string> vk_layer_names;
			std::vector<std::string> vk_extension_names;
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
			//template<typename Method, typename Object, typename... Args>
			//inline std::enable_if_t<
			//	std::is_invocable_v<Method, Object, VkInstance, Args...>&&
			//	std::is_void_v<std::invoke_result_t<Method, Object, VkInstance, Args...>>,
			//	void
			//>
			//	invokeWithVkInstanceMethod(Method&& method, Object&& object, Args&&... args);

			//template<typename Method, typename Object, typename... Args>
			//inline std::enable_if_t<
			//	std::is_invocable_v<Method, Object, VkInstance, Args...> &&
			//	!std::is_void_v<std::invoke_result_t<Method, Object, VkInstance, Args...>>,
			//	std::invoke_result_t<Method, Object, VkInstance, Args...>
			//> invokeWithVkInstanceMethod(Method&& method, Object&& object, Args&&... args);

			//template<typename Method, typename Object, typename... Args>
			//inline std::enable_if_t<
			//	!std::is_invocable_v<Method, Object, VkInstance, Args...>,
			//	void
			//>
			//	invokeWithVkInstanceMethod(Method&&, Object&&, Args&&...);

			//template<typename Method, typename... Args>
			//inline std::enable_if_t<
			//	std::is_invocable_v<Method, VkInstance, Args...>&&
			//	std::is_void_v<std::invoke_result_t<Method, VkInstance, Args...>>,
			//	void
			//>
			//	invokeWithVkInstanceFunction(Method&& method, Args&&... args);

			//template<typename Method, typename... Args>
			//inline std::enable_if_t<
			//	std::is_invocable_v<Method, VkInstance, Args...> &&
			//	!std::is_void_v<std::invoke_result_t<Method, VkInstance, Args...>>,
			//	std::invoke_result_t<Method, VkInstance, Args...>
			//>
			//	invokeWithVkInstanceFunction(Method&& method, Args&&... args);

		// cpp
		public:

			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			/*!	\@brief
			*/
			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			WvkStatus requestLayerProperties(std::vector<VkLayerProperties>& vk_layer_properties) const noexcept;

			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			/*!	\@brief
			*/
			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			WvkStatus requestExtensionProperties(std::vector<VkExtensionProperties>& vk_extension_properties) const noexcept;

			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			/*!	\@brief
			*/
			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			WvkStatus requestPhysicalDevices(std::vector<WvkPhysicalDevicePtr>& vk_physical_devices) const noexcept;
			
			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			/*!	\@brief
			*/
			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			inline const WvkInstanceCreateInfo& getCreateInfo(void) const noexcept;

		// hpp
		public:

			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			/*!	\@brief
			*/
			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			inline const WvkPhysicalDeviceUptrVec2& getWvkPhysicalDevices(void) const noexcept;

		// hpp
		private:
		
			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			/*!	\@brief
			*/
			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			friend class WvkPhysicalDevice;
			friend class WvkLogicalDevice;
			inline const WvkDispatchTableUptr& getWvkDispatchTable(void) const noexcept;
			
			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			/*!	\@brief
			*/
			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			friend class WvkInstanceDispatchTable;
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
			WvkStatus validationCreateInfo(const WvkInstanceCreateInfo& create_info) noexcept;

			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			/*!	\brief
			*/
			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			WvkStatus createGlobalDispatchTable(void) noexcept;

			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			/*!	\brief
			*/
			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			WvkStatus createInstanceDispatchTable(void) noexcept;
			
			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			/*!	\brief
			*/
			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			WvkStatus createPhysicalDevices(void) noexcept;

			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			/*!	\brief
			*/
			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			WvkStatus prepareLayers(std::vector<const char*>& layer_names) const noexcept;

			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			/*!	\brief
			*/
			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			WvkStatus prepareExtensions(std::vector<const char*>& extension_names) const noexcept;

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

			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			/*!	\brief
			*/
			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			static VkBool32 s_debugCallback(VkDebugUtilsMessageSeverityFlagBitsEXT messageSeverity, VkDebugUtilsMessageTypeFlagsEXT messageTypes, const VkDebugUtilsMessengerCallbackDataEXT* pCallbackData, void* pUserData) noexcept;

		private:
			
			WvkInstanceCreateInfo m_create_info = {};
			WvkDispatchTableUptr m_wvk_global_dispatch_table_ptr = nullptr;
			WvkDispatchTableUptr m_wvk_instance_dispatch_table_ptr = nullptr;
			WvkPhysicalDeviceUptrVec2 m_wvk_physical_devices;
			VkInstance m_vk_instance = VK_NULL_HANDLE;
			VkLayerPropertiesArr m_layer_properties_collection;
			VkExtensionPropertiesArr m_extension_properties_collection;
			std::vector<const char*> m_layer_name_collection;
			std::vector<const char*> m_extension_name_collection;
		}; // class WvkInstance

	} // namespace wvk

} // namespace CGDev

#include "wvk_instance.hpp"

#endif // CGDEV_WVK_SOURCE__WVK_INSTANCE_H
