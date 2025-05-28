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

			void requestPhysicalDeviceProperties(VkPhysicalDeviceProperties& vk_physical_device_properties) const noexcept;

			void requestPhysicalDeviceProperties(VkPhysicalDeviceProperties2& vk_physical_device_properties) const noexcept;

		public:			

			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			/*!	\brief
			*  ������������� ����� ������� ����������� ������� ����������� ���������� Vulkan.
			*  ���������� VkPhysicalDeviceProperties2 � ������� pNext ��� ��������� ����������
			*  � ���������, ����������� ��� ���������� ��������� VkPhysicalDeviceXProperties.
			*
			* @tparam VkPhysicalDeviceXProperties ��� ����������� ��������� ������� Vulkan.
			*
			* @param[out] vk_physical_device_x_properties
			*  ������ �� ��������� ����������� ������� ����������� ����������, ������� ����� ���������.
			*
			* @note
			*  ����� �������� ������ � Vulkan API ������ 1.1 � ����.
			*  ��� ����� ������ ������ ����� �� ���������� ������� ��������.
			*
			* @code
			* VkPhysicalDeviceVulkan11Properties props11{};
			* wvkPhysicalDevice.requestPhysicalDeviceProperties(props11);
			* // props11 �������� ������������ ���������� Vulkan 1.1
			* @endcode
			*/
			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			template<typename VkPhysicalDeviceXProperties>
			inline void requestPhysicalDeviceProperties(VkPhysicalDeviceXProperties& vk_physical_device_x_properties) const noexcept;

		public:
			
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
			* wvk_physical_device->invokeWithVkPhysicalDevice(&DeviceHandler::handle, handler, 5);
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
