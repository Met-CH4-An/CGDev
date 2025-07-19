#ifndef CGDEV_WVK_SOURCE__WVK_PHYSICAL_DEVICE_H
#define CGDEV_WVK_SOURCE__WVK_PHYSICAL_DEVICE_H
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
		struct WvkPhysicalDeviceCreateInfo {
			VkPhysicalDevice vk_physical_device = VK_NULL_HANDLE;
			WvkInstanceDtPtr wvk_instance_dispatch_table = nullptr;
		}; // struct WvkPhysicalDeviceCreateInfo
				
		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		/*!	\brief
		*/
		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		class WvkPhysicalDevice : public GpuObject {

		public:

			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			/*!	\brief
			*/
			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			WvkPhysicalDevice(void) noexcept;

			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			/*!	\brief
			*/
			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			~WvkPhysicalDevice(void) noexcept;

			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			/*!	\brief ������ ���������� ���������� Vulkan.
			* 
			* ����� ��������� ������ ������������� ����������� ���������� Vulkan, ������� ��������
			* ������ ����� ����� `validationCreateInfo()`, � ����� ������������� ����������� ����������
			* ����� ����� `createVkPhysicalDevice()`. ��� ���� ������������ ���������� �������� ����������,
			* �������� � ����������� �������������.
			*
			* @note ���� �������� �������� �� ����� ������ (����� `ValidationBuildInfo`), �����
			*      ������� ��������� ��������� � �������������� `validationCreateInfo()`.
			*      ���� ��������� �� ��������, ����� ���������� ������.
			*
			* @code
			* WvkPhysicalDevice physical_device;
			* WvkPhysicalDeviceCreateInfo create_info;
			* // ���������� create_info ������������ �������...
			* physical_device.create(create_info); // �������� � ������������� ����������� ����������
			* @endcode
			*/
			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			WvkStatus create(const WvkPhysicalDeviceCreateInfo& create_info) noexcept;

			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			/*!	\brief
			*/
			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			void destroy(void) noexcept;

		// hpp
		public:			

			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			/*!	\brief ����������� ������� �������� ����������� ���������� Vulkan.
			*
			* ������ ����� �������� �������� �������� ����������� ���������� Vulkan � ���������
			* ���������� ��������� VkPhysicalDeviceProperties � ������� �������
			* wvkGetPhysicalDeviceProperties �� ������� ��������������� ��������.
			*
			* @param[out] vk_physical_device_properties
			*   ��������� VkPhysicalDeviceProperties, ������� ����� ��������� ���������� ����������� ����������.
			*
			* @note
			*   ����� �� ��������� �������������� �������� � ������ ���������� � Vulkan ����� ������� ���������������.
			*
			* @code
			* VkPhysicalDeviceProperties props{};
			* wvk_physical_device.requestPhysicalDeviceProperties(props);
			* // props ������ �������� �������� �������� ����������� ����������
			* @endcode
			*/
			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			inline void requestProperties(VkPhysicalDeviceProperties& vk_physical_device_properties) const noexcept;
			
			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			/*!	\brief ����������� �������� ����������� ���������� ����� ������� `pNext`, ��������� `vkGetPhysicalDeviceProperties2`.
			*
			* ����� ������� ������ ���� �������� Vulkan 1.1 ��� �������� ���������� `VK_KHR_get_physical_device_properties2`.
			* ���� ���������������� ���������� � ������������ ������ `FEATURE_NOT_ENABLED`.
			*
			* @param[out] out
			* ��������� �� ������ ��������� � ������� `VkBaseOutStructure`, ���������� ����������� ��������.
			* ��������, ��� ����� ���� :
			* -`VkPhysicalDeviceIDProperties`
			* -`VkPhysicalDeviceSubgroupProperties`
			* -`VkPhysicalDevicePointClippingProperties`
			* � �.�.
			*
			* @return
			* ���������� `WvkStatus::ok()`, ���� �������� ������� ��������.
			* ����� � ���������� `FEATURE_NOT_ENABLED` � ����������.
			*
			* @code
			* VkPhysicalDeviceIDProperties id_props = {};
			* id_props.sType = VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_ID_PROPERTIES;
			*
			* WvkStatus status = physical_device->requestProperties(reinterpret_cast<VkBaseOutStructure*>(&id_props));
			* if (status.isOk()) {
			*   // �������� ������� ���������
			*	processUUID(id_props.deviceUUID);
			* } else {
			*	log->error("�������� ���������� �� ���� ���������: {}", status.message());
			* }
			*@endcode
			*/
			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			inline WvkStatus requestProperties(VkBaseOutStructure* out) const noexcept;

			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			/*!	\brief
			*/
			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			template<typename Out>
			inline WvkStatus requestProperties(Out& out) const noexcept;

			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			/*!	\brief ����������� �������� ���� �������� ��������** � ����������� ���������� � ��������� �� � ���������� ���������.
			*
			* @param[out] queue_family_properties_collection
			* ���������, � ������� ����� �������� ��������� `VkQueueFamilyProperties` � ���� ��� ������� ��������� ��������.
			*
			* @return
			* ������ `WvkStatus`, ����������� �� ���������� ��������.
			*
			* @code
			* WvkPhysicalDevice device = ...;
			* VkQueueFamilyPropertiesVec1 props;
			* WvkStatus status = device.requestQueueFamilyProperties(props);
			* if (status.isOk()) {
			*     // ������������ props
			* }
			* @endcode
			*/
			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			inline WvkStatus requestQueueFamilyProperties(VkQueueFamilyPropertiesVec1& queue_family_properties_collection) const noexcept;

			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			/*!	\briefbrief
			* Requests extended properties of all available queue families using the `vkGetPhysicalDeviceQueueFamilyProperties2` mechanism.
			*
			* @tparam Out
			* Type of the extended structure to retrieve per queue family (must begin with `VkStructureType` and be chainable via `pNext`).
			*
			* @param[out] out
			* Vector of `Out` structures to be filled with extended queue family properties.
			*
			* @param[in] vk_structure_type
			* The `VkStructureType` corresponding to the `Out` type (e.g., `VK_STRUCTURE_TYPE_QUEUE_FAMILY_PROPERTIES_2` or an extension type).
			*
			* @return
			* `WvkStatus::ok()` if the query was successful and the feature is available.
			* Otherwise returns `FEATURE_NOT_ENABLED` if Vulkan 1.1 or the extension `VK_KHR_get_physical_device_properties2` is not present.
			*
			* @code
			* std::vector<VkQueueFamilyVideoPropertiesKHR> video_queue_props;
			* status = device.requestQueueFamilyProperties(video_queue_props, VK_STRUCTURE_TYPE_QUEUE_FAMILY_VIDEO_PROPERTIES_KHR);
			* if (!status) {
			*     log->error("Query failed: {}", status.message());
			* }
			* @endcode
			*/
			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			template<typename Out>
			inline WvkStatus requestQueueFamilyProperties(std::vector<Out>& out, const VkStructureType& vk_structure_type) const noexcept;

			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			/*!	\brief ���������, ������ �� ��� �������� ���������� ���������� Vulkan � ���� ������ ���������.
			*
			* ����� ���������� Vulkan API `vkEnumeratePhysicalDeviceGroups` (��� `KHR`-�������),
			* ����� ����������, ������ �� `this` � ��� ���������� �� `wvk_physical_device_collection`
			* � ���� � �� �� ���������� ������.
			*
			* @param[in]  wvk_physical_device_collection  ��������� ���������� �� ���������� ����������.
			* @param[out] compatibility                   ���������� `true`, ���� ��� ���������� ���������� (� ����� ������).
			*
			* @return
			* ���������� ������ ����������. � ������ ������ � �������� �������� �������.
			*
			* @code
			* bool is_compatible = false;
			* WvkStatus status = wvk_physical_device->checkCompatibility(devices, is_compatible);
			* if (status.isOk() && is_compatible) {
			*     // ��� ���������� � ����� ������
			* }
			* @endcode
			*/
			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			inline WvkStatus checkCompatibility(const WvkPhysicalDevicePtrArr1& wvk_physical_device_collection, bool& compatibility) const noexcept;

			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			/*!	\brief
			*/
			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			//template<typename Out>
			//inline WvkStatus requestQueueFamilyProperties(std::vector<Out>& out, const VkStructureType& vk_structure_type) const noexcept;
			
			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			/*!	\brief ������������� ����� ������ � �������������� ������������ VkPhysicalDevice.
			*
			* ���� ������ ������������ ��� ������ ������ (��� ������ ����������� �������),
			* �������� ��������� `VkPhysicalDevice` � �������� ���������. ����� ����� ������
			* � ����� `method(object, m_vk_physical_device, args...)`, ��� `args` �
			* �������������� ���������������� ���������.
			*
			* @tparam Method ��� ����������� ������� (��������� �� �����, ������ � �.�.).
			* @tparam Object ��� �������, �� ������� ���������� �����.
			* @tparam Args ���� �������������� ����������, ������������ ������.
			*
			* @param[in] method �����, ������� ���������� �������.
			* @param[in] object ������, �� ������� ���������� �����.
			* @param[in] args �������������� ���������, ������������ ������.
			*
			* @note ��� ���������� �������� ������ ��� �������, ������������ void � ����������
			*       � ���� `method(object, VkPhysicalDevice, args...)`.
			*
			* @code
			* struct DeviceHandler {
			*     void handle(VkPhysicalDevice device, int index) {
			*         std::cout << "Handling device " << device << " at index " << index << std::endl;
			*     }
			* };
			*
			* DeviceHandler handler;
			* wvk_physical_device_ptr->invokeWithVkPhysicalDevice(&DeviceHandler::handle, handler, 5);
			* @endcode
			*/
			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			template<typename Method, typename Object, typename... Args>
			inline std::enable_if_t<
				std::is_invocable_v<Method, Object, VkPhysicalDevice, Args...>&&
				std::is_void_v<std::invoke_result_t<Method, Object, VkPhysicalDevice, Args...>>,
				void
			> 
				invokeWithVkPhysicalDeviceMethod(Method&& method, Object&& object, Args&&... args) const noexcept;

			template<typename Method, typename Object, typename... Args>
			inline std::enable_if_t<
				std::is_invocable_v<Method, Object, VkPhysicalDevice, Args...> &&
				!std::is_void_v<std::invoke_result_t<Method, Object, VkPhysicalDevice, Args...>>,
				std::invoke_result_t<Method, Object, VkPhysicalDevice, Args...>
			> invokeWithVkPhysicalDeviceMethod(Method&& method, Object&& object, Args&&... args) const noexcept;

			template<typename Method, typename Object, typename... Args>
			inline std::enable_if_t<
				!std::is_invocable_v<Method, Object, VkPhysicalDevice, Args...>,
				void
			>
				invokeWithVkPhysicalDeviceMethod(Method&&, Object&&, Args&&...) const noexcept;

			template<typename Method, typename... Args>
			inline std::enable_if_t<
				std::is_invocable_v<Method, VkPhysicalDevice, Args...>&&
				std::is_void_v<std::invoke_result_t<Method, VkPhysicalDevice, Args...>>,
				void
			>
				invokeWithVkPhysicalDeviceFunction(Method&& method, Args&&... args) const noexcept;

			template<typename Method, typename... Args>
			inline std::enable_if_t<
				std::is_invocable_v<Method, VkPhysicalDevice, Args...> &&
				!std::is_void_v<std::invoke_result_t<Method, VkPhysicalDevice, Args...>>,
				std::invoke_result_t<Method, VkPhysicalDevice, Args...>
			> invokeWithVkPhysicalDeviceFunction(Method&& method, Args&&... args) const noexcept;
		
		private:

			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			/*!	\brief
			* *��������� ��������� ����� ��������� `WvkPhysicalDeviceCreateInfo`.*
			*
			* ����� ���������, ��� �������� ���� ��������� `m_create_info` �� �������� ������������ ��������:
			* - `vk_physical_device` �� ������ ���� `VK_NULL_HANDLE`
			* - `wvk_instance_dispatch_table` �� ������ ���� `nullptr`
			*
			* � ������ ��������� ������� ������������ ������ ������ � ���������.
			*
			* @return
			* ���������� `WvkStatus::setOk()` � ������ ������, ���� ������ � ��������������� ����� `VknStatusCode::FAIL`.
			*
			* @code
			* WvkPhysicalDevice device;
			* WvkStatus status = device.validationCreateInfo();
			* if (!status) {
			*     std::cerr << "������ ���������: " << status.message() << std::endl;
			* }
			* @endcode
			*/
			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			WvkStatus validationCreateInfo(void) const noexcept;

			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			/*!	\brief ������ ���������� ���������� Vulkan.
			* 
			* ����� �������������� ������ ����������� ���������� Vulkan, ��������� ���������� �
			* ��������� `WvkPhysicalDeviceCreateInfo` �������� `vk_physical_device`. 
			* ��� ���������� ���������� ����� ������������ ��� ����������� �������� � Vulkan.
			*
			* @note ����� �� ��������� �������������� �������� ��� �������������,
			*      ��� ��� ��������������, ��� `vk_physical_device` ��� ����� ��������� ��������.
			*
			* @code
			* WvkPhysicalDevice physical_device;
			* WvkPhysicalDeviceCreateInfo create_info;
			* create_info.vk_physical_device = some_physical_device_handle; // ��������� ����������� ����������
			* physical_device.createVkPhysicalDevice(); // ������������� ����������� ����������
			* @endcode
			*/
			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			WvkStatus createVkPhysicalDevice(void) noexcept;

			void reset(void) noexcept;

		private:

			WvkPhysicalDeviceCreateInfo						m_create_info = {};
			VkPhysicalDevice								m_vk_physical_device = VK_NULL_HANDLE;
		}; // class WvkPhysicalDevice

	} // namespace wvk

} // namespace CGDev

#include "wvk_physical_device.hpp"

#endif // CGDEV_WVK_SOURCE__WVK_PHYSICAL_DEVICE_H
