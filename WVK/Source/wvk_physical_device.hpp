// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025 Metelev Met-CH4-An Andrey
// Project Lead: Metelev Andrey
// Main Technical Assistant: StarDev aka ChatGPT
#ifndef CGDEV_WVK_SOURCE__WVK_PHYSICAL_DEVICE_HPP
#define CGDEV_WVK_SOURCE__WVK_PHYSICAL_DEVICE_HPP
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
#include "wvk_dispatch_table.h"

namespace CGDev {

	namespace wvk {

		// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

		inline WvkStatus WvkPhysicalDevice::requestProperties(VkPhysicalDeviceProperties& vk_physical_device_properties) const noexcept {
			WvkStatus _status;

			m_wvk_dispatch_table_ptr->wvkGetPhysicalDeviceProperties(m_vk_physical_device, &vk_physical_device_properties);

			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			// Успех
			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			return _status.setOk();
		}

		// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

		template<typename Type, typename ... Args>
		inline WvkStatus WvkPhysicalDevice::requestProperties(Type& out, Args& ... args) const noexcept {
			WvkStatus _status;

#if WVK_VULKAN_API_VERSION >= WVK_VULKAN_API_VERSION_11 || WVK_KHR_get_physical_device_properties2 == WVK_ENABLE
			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			// Подготавливаем VkPhysicalDeviceProperties2
			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			VkPhysicalDeviceProperties2 _props2 = {
				.sType = VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_PROPERTIES_2,
				.pNext = &out,
			};

			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			// Связываем цепочки pNext
			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			void* prev = &out;
			((reinterpret_cast<decltype(&args)>(prev)->pNext = &args, prev = &args), ...);

			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			// Запрашиваем VkPhysicalDeviceProperties2
			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			m_wvk_dispatch_table_ptr->wvkGetPhysicalDeviceProperties2(m_vk_physical_device, &_props2);

			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			// Успех
			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			return _status.setOk();
#else
			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			// Функционал не доступен
			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			return _status.set(VknStatusCode::FEATURE_NOT_ENABLED, "Vulkan 1.1 or VK_KHR_get_physical_device_properties2 is not available.");
#endif
		}

		// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

		inline WvkStatus WvkPhysicalDevice::requestQueueFamilyProperties(std::vector<VkQueueFamilyProperties>& out) const noexcept {
			WvkStatus _status;

			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			// Количество VkQueueFamilyProperties
			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			uint32_t _count = 0;
			m_wvk_dispatch_table_ptr->wvkGetPhysicalDeviceQueueFamilyProperties(m_vk_physical_device, &_count, nullptr);

			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			// Выделяем память в векторах
			out.resize(_count);

			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			// Запрашиваем VkQueueFamilyProperties
			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			m_wvk_dispatch_table_ptr->wvkGetPhysicalDeviceQueueFamilyProperties(m_vk_physical_device, &_count, out.data());

			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			// Успех
			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			return _status.setOk();
		}

		// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

		template<typename Type, typename ... Args>
		inline WvkStatus WvkPhysicalDevice::requestQueueFamilyProperties(std::vector<Type>& out, Args ... args) const noexcept {
			WvkStatus _status;

#if WVK_VULKAN_API_VERSION >= WVK_VULKAN_API_VERSION_11 || WVK_KHR_get_physical_device_properties2 == WVK_ENABLE
			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			// Количество VkQueueFamilyProperties2
			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			uint32_t _count = 0;
			m_wvk_dispatch_table_ptr->wvkGetPhysicalDeviceQueueFamilyProperties2(m_vk_physical_device, &_count, nullptr);

			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			// Выделяем память в векторах
			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			out.resize(_count);
			(args.resize(_count), ...);
			std::vector<VkQueueFamilyProperties2> _props2(_count, { VK_STRUCTURE_TYPE_QUEUE_FAMILY_PROPERTIES_2 });
			
			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			// Связываем цепочки pNext
			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			auto link = [](auto& a, auto& b) noexcept {
				VkStructureType _type_a = a.front().sType;
				VkStructureType _type_b = b.front().sType;
				for (size_t i = 0; i < a.size(); ++i) {
					a[i].sType = _type_a;
					b[i].sType = _type_b;
					a[i].pNext = &b[i];
				}
				};

			link(_props2, out);
			auto* prev = &out;
			((link(*prev, args), prev = &args), ...);

			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			// Запрашиваем VkQueueFamilyProperties2
			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			m_wvk_dispatch_table_ptr->wvkGetPhysicalDeviceQueueFamilyProperties2(m_vk_physical_device, &_count, _props2.data());

			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			// Успех
			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			return _status.setOk();
#else
			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			// Функционал не доступен
			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			return _status.set(VknStatusCode::FEATURE_NOT_ENABLED, "Vulkan 1.1 or VK_KHR_get_physical_device_properties2 is not available.");
#endif
		}

		// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

		inline const WvkInstancePtr& WvkPhysicalDevice::getWvkInstance(void) const noexcept {
			return m_create_info.wvk_instance_ptr;
		}

		// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

		inline const VkPhysicalDevice& WvkPhysicalDevice::getVkPhysicalDevice(void) const noexcept {
			return m_create_info.vk_physical_device;
		}

		// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

	} // namespace wvk

} // namespace CGDev

#endif // CGDEV_WVK_SOURCE__WVK_PHYSICAL_DEVICE_HPP
