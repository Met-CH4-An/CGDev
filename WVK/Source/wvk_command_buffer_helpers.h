// SPDX-License-Identifier: AGPL-3.0-or-later
#ifndef CGDEV_WVK_SOURCE__WVK_COMMAND_BUFFER_HELPERS_H
#define CGDEV_WVK_SOURCE__WVK_COMMAND_BUFFER_HELPERS_H
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
        /*!	\brief
        */
        // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
        class WvkCommandBufferBeginHelper {
        public:

            // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
            /*!	\brief
            */
            // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
            inline WvkCommandBufferBeginHelper() noexcept;

            // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
            /*!	\brief
            */
            // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
            inline WvkCommandBufferBeginHelper& withUsage(VkCommandBufferUsageFlags usage_flags) noexcept;

            // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
            /*!	\brief
            */
            // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
            inline WvkCommandBufferBeginHelper& withGroupMask(uint32_t mask) noexcept;

            // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
            /*!	\brief
            */
            // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
            inline WvkCommandBufferBeginHelper& withRenderPass(const WvkRenderPassPtr& rp, const WvkFrameBufferPtr& fb) noexcept;

            // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
            /*!	\brief
            */
            // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
            inline WvkCommandBufferBeginHelper& withOcclusionQuery(VkBool32 enable) noexcept;

            // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
            /*!	\brief
            */
            // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
            inline WvkCommandBufferBeginHelper& withQueryFlags(VkQueryControlFlags flags) noexcept;

            // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
            /*!	\brief
            */
            // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
            inline WvkCommandBufferBeginHelper& withPipelineStatistics(VkQueryPipelineStatisticFlags stats) noexcept;

            // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
            /*!	\brief
            */
            // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
            inline WvkCommandBufferBeginHelper& withRendering(const WvkImagePtrVec1& color_images, const WvkImagePtr& depth_image, const WvkImagePtr& stencil_image, uint32_t view_mask) noexcept;

            // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
            /*!	\brief
            */
            // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
            inline WvkCommandBufferBeginHelper& withRenderingColor(const WvkImagePtrVec1& color_images) noexcept;

            // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
            /*!	\brief
            */
            // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
            inline WvkCommandBufferBeginHelper& withRenderingDepth(const WvkImagePtr& depth) noexcept;

            // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
            /*!	\brief
            */
            // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
            inline WvkCommandBufferBeginHelper& withRenderingStencil(const WvkImagePtr& stencil) noexcept;

            // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
            /*!	\brief
            */
            // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
            inline WvkCommandBufferBeginHelper& withRenderingViewMask(const uint32_t& view_mask) noexcept;

            // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
            /*!	\brief
            */
            // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
            inline WvkCommandBufferBeginHelper& withLocations(const std::vector<uint32_t>& locations) noexcept;

            // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
            /*!	\brief
            */
            // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
            template<typename T>
            inline WvkCommandBufferBeginHelper& withBeginNext(T& pNext) noexcept;

            // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
            /*!	\brief
            */
            // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
            template<typename T>
            inline WvkCommandBufferBeginHelper& withInheritanceNext(T& pNext) noexcept;

            // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
            /*!	\brief
            */
            // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
            inline WvkCommandBufferBeginHelper& withBeginNext(VkDeviceGroupCommandBufferBeginInfo& pNext) noexcept;

            // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
            /*!	\brief
            */
            // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
            inline WvkCommandBufferBeginHelper& withInheritanceNext(VkCommandBufferInheritanceRenderingInfo& pNext) noexcept;

            // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
            /*!	\brief
            */
            // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
            inline WvkCommandBufferBeginHelper& withInheritanceNext(VkRenderingAttachmentLocationInfo& pNext) noexcept;

        // hpp
        private:

            friend class WvkCommandBuffer;
            // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
            /*!	\brief
            */
            // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
            inline const VkCommandBufferBeginInfo& getBeginInfo(void) const noexcept;

            // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
            /*!	\brief
            */
            // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
            template<typename T>
            inline void appendToBegin(T* next) noexcept;

            // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
            /*!	\brief
            */
            // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
            template<typename T>
            inline void appendToInheritance(T* next) noexcept;

            // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
            /*!	\brief
            */
            // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
            inline void ensureGroupAttached(void) noexcept;

            // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
            /*!	\brief
            */
            // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
            inline void ensureInheritanceAttached(void) noexcept;

            // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
            /*!	\brief
            */
            // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
            inline void ensureRenderingAttached(void) noexcept;

            // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
            /*!	\brief
            */
            // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
            inline void ensureLocationAttached(void) noexcept;

        private:
            
            VkCommandBufferBeginInfo m_vk_begin_info{
                .sType = VK_STRUCTURE_TYPE_COMMAND_BUFFER_BEGIN_INFO,
                .pNext = nullptr,
                .flags = 0,
                .pInheritanceInfo = nullptr,
            };

            VkDeviceGroupCommandBufferBeginInfo m_vk_group_begin_info{
                .sType = VK_STRUCTURE_TYPE_DEVICE_GROUP_COMMAND_BUFFER_BEGIN_INFO,
                .pNext = nullptr,
                .deviceMask = 0,
            };
            bool m_group_attached = false;

            VkCommandBufferInheritanceInfo m_inheritance_info{
                .sType = VK_STRUCTURE_TYPE_COMMAND_BUFFER_INHERITANCE_INFO,
                .pNext = nullptr,
                .renderPass = VK_NULL_HANDLE,
                .subpass = 0,
                .framebuffer = VK_NULL_HANDLE,
                .occlusionQueryEnable = VK_FALSE,
                .queryFlags = 0,
                .pipelineStatistics = 0,
            };
            bool m_inheritance_attached = false;

            VkCommandBufferInheritanceRenderingInfo m_inheritance_rendering_info{
                .sType = VK_STRUCTURE_TYPE_COMMAND_BUFFER_INHERITANCE_RENDERING_INFO,
                .pNext = nullptr,
                .flags = 0,
                .viewMask = 0,
                .colorAttachmentCount = 0,
                .pColorAttachmentFormats = nullptr,
                .depthAttachmentFormat = VK_FORMAT_UNDEFINED,
                .stencilAttachmentFormat = VK_FORMAT_UNDEFINED,
                .rasterizationSamples = VK_SAMPLE_COUNT_1_BIT,
            };
            bool m_rendering_attached = false;

            VkRenderingAttachmentLocationInfo m_rendering_attachments_location_info{
                .sType = VK_STRUCTURE_TYPE_RENDERING_ATTACHMENT_LOCATION_INFO,
                .pNext = nullptr,
                .colorAttachmentCount = 0,
                .pColorAttachmentLocations = nullptr,
            };
            bool m_location_attached = false;

            // --- для хранения данных ---
            std::vector<uint32_t> m_locations_storage;
            std::vector<VkFormat> m_color_formats_storage;

            // хвосты цепочек
            VkBaseOutStructure* m_last_begin_next = nullptr;
            VkBaseOutStructure* m_last_inheritance_next = nullptr;
        }; // class WvkCommandBufferBeginHelper

	} // namespace wvk

} // namespace CGDev

#include "wvk_command_buffer_helpers.hpp"

#endif // CGDEV_WVK_SOURCE__WVK_COMMAND_BUFFER_HELPERS_H
