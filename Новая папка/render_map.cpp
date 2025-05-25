////////////////////////////////////////////////////////////////
// секция форвард-декларации
////////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////
// секция заголовочного файла
////////////////////////////////////////////////////////////////
#include "render_map.h"
////////////////////////////////////////////////////////////////
// секция имплементации
////////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////
// секция для остального
////////////////////////////////////////////////////////////////

namespace CGDev {

	namespace RPI {

		namespace Private {

			namespace Vulkan {

				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

				RenderMap::RenderMap(void) noexcept {
				}

				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

				RenderMap::~RenderMap(void) noexcept {
				}

				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

				void RenderMap::create(const RenderMapCreateInfo& create_info) noexcept {

					// ~~~~~~~~~~~~~~~~
					// проверяем на валидность LOGICAL_DEVICE_CREATE_INFO
					// ~~~~~~~~~~~~~~~~

					if (create_info.isValid() == false) {

						Tools::addEntry(create_info.log, Tools::LogEntryError("RenderMapCreateInfo не валиден"));

						m_valid = false;

						return;
					}

					m_create_info = create_info;

					m_valid = true;

				}

				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

				void RenderMap::requestCommandBuffer(CommandBufferSptr& command_buffer) noexcept {

					//TreeNode<void*, float> _tree_node0(nullptr);

					//TreeNode<float, int> _tree_node00(0.0f);
					//TreeNode<float, int> _tree_node01(0.1f);
					
					//_tree_node0.m_children_collection.push_back(_tree_node00);

					//_tree_node.m_children_collection.push_back(TreeNode(1.0));
					//_tree_node.m_children_collection.push_back(TreeNode(2.0));
					//

				}

				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

			} // namespace Vulkan

		} // namespace Private

	} // namespace RPI

} // namespace CGDev