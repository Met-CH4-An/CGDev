// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025 Metelev Met-CH4-An Andrey
// Project Lead: Metelev Andrey
// Main Technical Assistant: StarDev aka ChatGPT
////////////////////////////////////////////////////////////////
// ������ �������-����������
////////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////
// ������ ������������� �����
////////////////////////////////////////////////////////////////
#include "wvk_queue.h"
////////////////////////////////////////////////////////////////
// ������ �������������
////////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////
// ������ ��� ����������
////////////////////////////////////////////////////////////////

namespace CGDev {

	//namespace GPU {

		//namespace Private {

			namespace wvk {

				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

				VknQueue::VknQueue(void) noexcept {
				}

				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

				VknQueue::~VknQueue(void) noexcept {
				}

				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

				bool VknQueue::create(const VknQueueCreateInfo& create_info) noexcept {

					// ~~~~~~~~~~~~~~~~
					// ������
					// Build::s_wvk_debug == true
					// ������
					// ~~~~~~~~~~~~~~~~
					// ��������� ���������� VknQueueCreateInfo
					// ~~~~~~~~~~~~~~~~

					//if constexpr (Build::s_wvk_debug == true) {

					//	if (validationCreateInfo(create_info) == false) return false;

					//}

					// ~~~~~~~~~~~~~~~~
					// ������
					// Build::s_wvk_debug == true
					// �����
					// ~~~~~~~~~~~~~~~~

					// ~~~~~~~~~~~~~~~~
					// ��������� VknLogicalDeviceCreateInfo
					// ~~~~~~~~~~~~~~~~

					m_create_info = create_info;

					// ~~~~~~~~~~~~~~~~
					// �������������� VkDevice
					// ~~~~~~~~~~~~~~~~

					if (prepareVkQueue() == false) {

						//Tools::addEntry(m_create_info.log, Tools::LogEntryError("prepareVkQueue() = false"));

						return false;
					}

					return true;

				}

				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

				void VknQueue::destroy(void) noexcept {

					
				}

				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

				bool VknQueue::validationCreateInfo(const VknQueueCreateInfo& create_info) const noexcept {

					//if (create_info.log == nullptr) {

					//	return false;
					//}

					//if (create_info.wvk_dispatch_table_ptr == nullptr) {

						//Tools::addEntry(create_info.log, Tools::LogEntryError("VknQueueCreateInfo::wvk_dispatch_table_ptr = nullptr"));

					//	return false;
					//}

					if (create_info.wvk_instance == nullptr) {

						//Tools::addEntry(create_info.log, Tools::LogEntryError("VknQueueCreateInfo::wvk_instance = nullptr"));

						return false;
					}

					return true;
				}

				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

				bool VknQueue::prepareVkQueue(void) noexcept {

					// ~~~~~~~~~~~~~~~~
					// �������������� VkDeviceQueueCreateInfoArr1
					// ~~~~~~~~~~~~~~~~

					

					return true;
				}

				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

			} // namespace wvk

		//} // namespace Private

	//} // namespace GPU

} // namespace CGDev