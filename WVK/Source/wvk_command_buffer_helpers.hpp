#ifndef CGDEV_WVK_SOURCE__WVK_COMMAND_BUFFER_HELPERS_HPP
#define CGDEV_WVK_SOURCE__WVK_COMMAND_BUFFER_HELPERS_HPP
////////////////////////////////////////////////////////////////
// секция форвард-декларации
////////////////////////////////////////////////////////////////
#include "_wvk.h"
////////////////////////////////////////////////////////////////
// секция имплементации
////////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////
// секция родительского класса
////////////////////////////////////////////////////////////////
#include "wvk_base_object.h"
////////////////////////////////////////////////////////////////
// секция для остального
////////////////////////////////////////////////////////////////

namespace CGDev {

	namespace wvk {

        // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
        // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

        inline WvkCommandBufferBeginHelper::WvkCommandBufferBeginHelper() noexcept {
            m_last_begin_next = reinterpret_cast<VkBaseOutStructure*>(&m_vk_begin_info);
            m_last_inheritance_next = reinterpret_cast<VkBaseOutStructure*>(&m_inheritance_info);
        }

        // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
        // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

        inline WvkCommandBufferBeginHelper& WvkCommandBufferBeginHelper::withUsage(VkCommandBufferUsageFlags usage_flags) noexcept {
            m_vk_begin_info.flags = usage_flags;
            return *this;
        }

        // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
        // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

        inline WvkCommandBufferBeginHelper& WvkCommandBufferBeginHelper::withGroupMask(uint32_t mask) noexcept {
            ensureGroupAttached();
            m_vk_group_begin_info.deviceMask = mask;
            return *this;
        }

        // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
        // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

        inline WvkCommandBufferBeginHelper& WvkCommandBufferBeginHelper::withRenderPass(const WvkRenderPassPtr& rp, const WvkFrameBufferPtr& fb) noexcept {
            ensureInheritanceAttached();
            m_inheritance_info.subpass = 0;            
            return *this;
        }

        // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
        // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

        inline WvkCommandBufferBeginHelper& WvkCommandBufferBeginHelper::withOcclusionQuery(VkBool32 enable) noexcept {
            ensureInheritanceAttached();
            m_inheritance_info.occlusionQueryEnable = enable;
            return *this;
        }

        // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
        // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

        inline WvkCommandBufferBeginHelper& WvkCommandBufferBeginHelper::withQueryFlags(VkQueryControlFlags flags) noexcept {
            ensureInheritanceAttached();
            m_inheritance_info.occlusionQueryEnable = VK_TRUE;
            m_inheritance_info.queryFlags = flags;
            return *this;
        }

        // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
        // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

        inline WvkCommandBufferBeginHelper& WvkCommandBufferBeginHelper::withPipelineStatistics(VkQueryPipelineStatisticFlags stats) noexcept {
            ensureInheritanceAttached();
            m_inheritance_info.pipelineStatistics = stats;
            return *this;
        }

        // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
        // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

        inline WvkCommandBufferBeginHelper& WvkCommandBufferBeginHelper::withRendering(const WvkImagePtrVec1& color_images, const WvkImagePtr& depth_image, const WvkImagePtr& stencil_image, uint32_t view_mask) noexcept {
            ensureInheritanceAttached();
            ensureRenderingAttached();

            m_inheritance_rendering_info.viewMask = view_mask;

            // пример автозаполнения
            //m_inheritance_rendering_info.colorAttachmentCount = (uint32_t)color_images.size();
            //m_color_formats_storage.resize(color_images.size());
            //for (size_t i = 0; i < color_images.size(); ++i)
            //    m_color_formats_storage[i] = color_images[i]->getFormat();
            //m_inheritance_rendering_info.pColorAttachmentFormats = m_color_formats_storage.data();

            //m_inheritance_rendering_info.depthAttachmentFormat =
            //    depth_image ? depth_image->getFormat() : VK_FORMAT_UNDEFINED;
            //m_inheritance_rendering_info.stencilAttachmentFormat =
            //    stencil_image ? stencil_image->getFormat() : VK_FORMAT_UNDEFINED;

            return *this;
        }

        // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
        // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

        inline WvkCommandBufferBeginHelper& WvkCommandBufferBeginHelper::withRenderingColor(const WvkImagePtrVec1& color_images) noexcept {
            ensureRenderingAttached();

            std::vector<VkFormat> _formats;
            //std::transform(
            //    color_images.cbegin(),
            //    color_images.cend(),
            //    std::back_inserter(_formats),
            //    [](const auto& wvk_image) {
            //        return wvk_image->getFormat();
            //    }
            //);      
            m_inheritance_rendering_info.colorAttachmentCount = static_cast<uint32_t>(_formats.size());
            m_inheritance_rendering_info.pColorAttachmentFormats = _formats.data();            

            return *this;
        }

        // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
        // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

        inline WvkCommandBufferBeginHelper& WvkCommandBufferBeginHelper::withRenderingDepth(const WvkImagePtr& depth) noexcept {
            ensureRenderingAttached();

            //m_inheritance_rendering_info.depthAttachmentFormat = depth->getFormat();
            
            return *this;
        }

        // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
        // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

        inline WvkCommandBufferBeginHelper& WvkCommandBufferBeginHelper::withRenderingStencil(const WvkImagePtr& stencil) noexcept {
            ensureRenderingAttached();

            //m_inheritance_rendering_info.stencilAttachmentFormat = stencil->getFormat();

            return *this;
        }

        // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
        // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

        inline WvkCommandBufferBeginHelper& WvkCommandBufferBeginHelper::withRenderingViewMask(const uint32_t& view_mask) noexcept {
            ensureRenderingAttached();
            m_inheritance_rendering_info.viewMask = view_mask;
            return *this;
        }

        // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
        // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

        inline WvkCommandBufferBeginHelper& WvkCommandBufferBeginHelper::withLocations(const std::vector<uint32_t>& locations) noexcept {
            ensureLocationAttached();

            m_locations_storage = locations;
            m_rendering_attachments_location_info.colorAttachmentCount =
                static_cast<uint32_t>(m_locations_storage.size());
            m_rendering_attachments_location_info.pColorAttachmentLocations =
                m_locations_storage.data();

            return *this;
        }

        // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
        // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

        template<typename T>
        inline WvkCommandBufferBeginHelper& WvkCommandBufferBeginHelper::withBeginNext(T& pNext) noexcept {
            appendToBegin(&pNext);
            return *this;
        }

        // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
        // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

        template<typename T>
        inline WvkCommandBufferBeginHelper& WvkCommandBufferBeginHelper::withInheritanceNext(T& pNext) noexcept {
            ensureInheritanceAttached();
            appendToInheritance(&pNext);
            return *this;
        }

        // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
        // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

        inline WvkCommandBufferBeginHelper& WvkCommandBufferBeginHelper::withBeginNext(VkDeviceGroupCommandBufferBeginInfo& pNext) noexcept {
            ensureGroupAttached();
            m_vk_group_begin_info = pNext;
            return *this;
        }

        // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
        // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

        inline WvkCommandBufferBeginHelper& WvkCommandBufferBeginHelper::withInheritanceNext(VkCommandBufferInheritanceRenderingInfo& pNext) noexcept {
            ensureRenderingAttached();
            m_inheritance_rendering_info = pNext;
            return *this;
        }

        // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
        // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

        inline WvkCommandBufferBeginHelper& WvkCommandBufferBeginHelper::withInheritanceNext(VkRenderingAttachmentLocationInfo& pNext) noexcept {
            ensureLocationAttached();
            m_rendering_attachments_location_info = pNext;
            return *this;
        }

        // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
        // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

        inline const VkCommandBufferBeginInfo& WvkCommandBufferBeginHelper::getBeginInfo() const noexcept { 
            return m_vk_begin_info; 
        }

        // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
        // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

        template<typename T>
        inline void WvkCommandBufferBeginHelper::appendToBegin(T* next) noexcept {
            auto* base = reinterpret_cast<VkBaseOutStructure*>(next);
            m_last_begin_next->pNext = base;
            m_last_begin_next = base;
            base->pNext = nullptr;
        }

        // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
        // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

        template<typename T>
        inline void WvkCommandBufferBeginHelper::appendToInheritance(T* next) noexcept {
            auto* base = reinterpret_cast<VkBaseOutStructure*>(next);
            m_last_inheritance_next->pNext = base;
            m_last_inheritance_next = base;
            base->pNext = nullptr;
        }

        inline void WvkCommandBufferBeginHelper::ensureGroupAttached(void) noexcept {
            if (!m_group_attached) {
                appendToBegin(&m_vk_group_begin_info);
                m_group_attached = true;
            }
        }

        // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
        // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

        inline void WvkCommandBufferBeginHelper::ensureInheritanceAttached(void) noexcept {
            if (!m_inheritance_attached) {
                m_vk_begin_info.pInheritanceInfo = &m_inheritance_info;
                m_inheritance_attached = true;
            }
        }

        // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
        // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

        inline void WvkCommandBufferBeginHelper::ensureRenderingAttached(void) noexcept {
            if (!m_rendering_attached) {
                ensureInheritanceAttached();
                appendToInheritance(&m_inheritance_rendering_info);
                m_rendering_attached = true;
            }
        }

        // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
        // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

        inline void WvkCommandBufferBeginHelper::ensureLocationAttached(void) noexcept {
            if (!m_location_attached) {
                ensureRenderingAttached();
                appendToInheritance(&m_rendering_attachments_location_info);
                m_location_attached = true;
            }
        }

        // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
        // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

	} // namespace wvk

} // namespace CGDev

#endif // CGDEV_WVK_SOURCE__WVK_COMMAND_BUFFER_HELPERS_HPP
