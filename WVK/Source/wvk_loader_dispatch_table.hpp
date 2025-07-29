#ifndef CGDEV_WVK_SOURCE__WVK_LOADER_DISPATCH_TABLE_HPP
#define CGDEV_WVK_SOURCE__WVK_LOADER_DISPATCH_TABLE_HPP
////////////////////////////////////////////////////////////////
// ������ �������-����������
////////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////
// ������ �������������
////////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////
// ������ ������������� ������
////////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////
// ������ ��� ����������
////////////////////////////////////////////////////////////////

namespace CGDev {

    namespace wvk {

        // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
        // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

        inline VkResult WvkLoaderDispatchTable::wvkCreateInstance(const VkInstanceCreateInfo* pCreateInfo, const VkAllocationCallbacks* pAllocator, VkInstance* pInstance) const noexcept {
            return m_vkCreateInstance(pCreateInfo, pAllocator, pInstance); }

        // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
        // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

        inline VkResult WvkLoaderDispatchTable::wvkEnumerateInstanceExtensionProperties(const char* pLayerName, uint32_t* pPropertyCount, VkExtensionProperties* pProperties) const noexcept {
            return m_vkEnumerateInstanceExtensionProperties(pLayerName, pPropertyCount, pProperties); }

        // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
        // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

        inline VkResult WvkLoaderDispatchTable::wvkEnumerateInstanceLayerProperties(uint32_t* pPropertyCount, VkLayerProperties* pProperties) const noexcept {
            return m_vkEnumerateInstanceLayerProperties(pPropertyCount, pProperties); }

        // 1.1
        
        // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
        // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

        inline VkResult WvkLoaderDispatchTable::wvkEnumerateInstanceVersion(uint32_t* pApiVersion) const noexcept {
            return m_vkEnumerateInstanceVersion(pApiVersion); }

        // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
        // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

        inline const WvkLoaderDispatchTableCreateInfo& WvkLoaderDispatchTable::getCreateInfo(void) const noexcept {
            return m_create_info;
        }

    } // namespace wvk

} // namespace CGDev

#endif // CGDEV_WVK_SOURCE__WVK_LOADER_DISPATCH_TABLE_HPP