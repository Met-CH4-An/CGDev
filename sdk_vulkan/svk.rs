pub type VkSampleMask = u32; //
pub type VkBool32 = u32; //
pub type VkFlags = u32; //
pub type VkFlags64 = u64; //
pub type VkDeviceSize = u64; //
pub type VkDeviceAddress = u64; //
pub struct VkFramebufferCreateFlags(VkFlags); //

pub struct VkQueryPoolCreateFlags(VkFlags); //

pub struct VkRenderPassCreateFlags(VkFlags); //

pub struct VkSamplerCreateFlags(VkFlags); //

pub struct VkPipelineLayoutCreateFlags(VkFlags); //

pub struct VkPipelineCacheCreateFlags(VkFlags); //

pub struct VkPipelineDepthStencilStateCreateFlags(VkFlags); //

pub struct VkPipelineDepthStencilStateCreateFlags(VkFlags); //

pub struct VkPipelineDynamicStateCreateFlags(VkFlags); //

pub struct VkPipelineColorBlendStateCreateFlags(VkFlags); //

pub struct VkPipelineColorBlendStateCreateFlags(VkFlags); //

pub struct VkPipelineMultisampleStateCreateFlags(VkFlags); //

pub struct VkPipelineRasterizationStateCreateFlags(VkFlags); //

pub struct VkPipelineViewportStateCreateFlags(VkFlags); //

pub struct VkPipelineTessellationStateCreateFlags(VkFlags); //

pub struct VkPipelineInputAssemblyStateCreateFlags(VkFlags); //

pub struct VkPipelineVertexInputStateCreateFlags(VkFlags); //

pub struct VkPipelineShaderStageCreateFlags(VkFlags); //

pub struct VkDescriptorSetLayoutCreateFlags(VkFlags); //

pub struct VkBufferViewCreateFlags(VkFlags); //

pub struct VkInstanceCreateFlags(VkFlags); //

pub struct VkDeviceCreateFlags(VkFlags); //

pub struct VkDeviceQueueCreateFlags(VkFlags); //

pub struct VkQueueFlags(VkFlags); //

pub struct VkMemoryPropertyFlags(VkFlags); //

pub struct VkMemoryHeapFlags(VkFlags); //

pub struct VkAccessFlags(VkFlags); //

pub struct VkBufferUsageFlags(VkFlags); //

pub struct VkBufferCreateFlags(VkFlags); //

pub struct VkShaderStageFlags(VkFlags); //

pub struct VkImageUsageFlags(VkFlags); //

pub struct VkImageCreateFlags(VkFlags); //

pub struct VkImageViewCreateFlags(VkFlags); //

pub struct VkPipelineCreateFlags(VkFlags); //

pub struct VkColorComponentFlags(VkFlags); //

pub struct VkFenceCreateFlags(VkFlags); //

pub struct VkSemaphoreCreateFlags(VkFlags); //

pub struct VkFormatFeatureFlags(VkFlags); //

pub struct VkQueryControlFlags(VkFlags); //

pub struct VkQueryResultFlags(VkFlags); //

pub struct VkShaderModuleCreateFlags(VkFlags); //

pub struct VkEventCreateFlags(VkFlags); //

pub struct VkCommandPoolCreateFlags(VkFlags); //

pub struct VkCommandPoolResetFlags(VkFlags); //

pub struct VkCommandBufferResetFlags(VkFlags); //

pub struct VkCommandBufferUsageFlags(VkFlags); //

pub struct VkQueryPipelineStatisticFlags(VkFlags); //

pub struct VkMemoryMapFlags(VkFlags); //

pub struct VkMemoryUnmapFlags(VkFlags); //

pub type VkMemoryUnmapFlagsKHR = VkMemoryUnmapFlags; //

pub struct VkImageAspectFlags(VkFlags); //

pub struct VkSparseMemoryBindFlags(VkFlags); //

pub struct VkSparseImageFormatFlags(VkFlags); //

pub struct VkSubpassDescriptionFlags(VkFlags); //

pub struct VkPipelineStageFlags(VkFlags); //

pub struct VkSampleCountFlags(VkFlags); //

pub struct VkAttachmentDescriptionFlags(VkFlags); //

pub struct VkStencilFaceFlags(VkFlags); //

pub struct VkCullModeFlags(VkFlags); //

pub struct VkDescriptorPoolCreateFlags(VkFlags); //

pub struct VkDescriptorPoolResetFlags(VkFlags); //

pub struct VkDependencyFlags(VkFlags); //

pub struct VkSubgroupFeatureFlags(VkFlags); //

pub struct VkIndirectCommandsLayoutUsageFlagsNV(VkFlags); //

pub struct VkIndirectStateFlagsNV(VkFlags); //

pub struct VkGeometryFlagsKHR(VkFlags); //

pub type VkGeometryFlagsNV = VkGeometryFlagsKHR; //

pub struct VkGeometryInstanceFlagsKHR(VkFlags); //

pub type VkGeometryInstanceFlagsNV = VkGeometryInstanceFlagsKHR; //

pub struct VkClusterAccelerationStructureGeometryFlagsNV(VkFlags); //

pub struct VkClusterAccelerationStructureClusterFlagsNV(VkFlags); //

pub struct VkClusterAccelerationStructureAddressResolutionFlagsNV(VkFlags); //

pub struct VkBuildAccelerationStructureFlagsKHR(VkFlags); //

pub type VkBuildAccelerationStructureFlagsNV = VkBuildAccelerationStructureFlagsKHR; //

pub struct VkPrivateDataSlotCreateFlags(VkFlags); //

pub type VkPrivateDataSlotCreateFlagsEXT = VkPrivateDataSlotCreateFlags; //

pub struct VkAccelerationStructureCreateFlagsKHR(VkFlags); //

pub struct VkDescriptorUpdateTemplateCreateFlags(VkFlags); //

pub type VkDescriptorUpdateTemplateCreateFlagsKHR = VkDescriptorUpdateTemplateCreateFlags; //

pub struct VkPipelineCreationFeedbackFlags(VkFlags); //

pub type VkPipelineCreationFeedbackFlagsEXT = VkPipelineCreationFeedbackFlags; //

pub struct VkPerformanceCounterDescriptionFlagsKHR(VkFlags); //

pub struct VkAcquireProfilingLockFlagsKHR(VkFlags); //

pub struct VkSemaphoreWaitFlags(VkFlags); //

pub type VkSemaphoreWaitFlagsKHR = VkSemaphoreWaitFlags; //

pub struct VkPipelineCompilerControlFlagsAMD(VkFlags); //

pub struct VkShaderCorePropertiesFlagsAMD(VkFlags); //

pub struct VkDeviceDiagnosticsConfigFlagsNV(VkFlags); //

pub struct VkRefreshObjectFlagsKHR(VkFlags); //

pub struct VkAccessFlags2(VkFlags64); //

pub type VkAccessFlags2KHR = VkAccessFlags2; //

pub struct VkPipelineStageFlags2(VkFlags64); //

pub type VkPipelineStageFlags2KHR = VkPipelineStageFlags2; //

pub struct VkAccelerationStructureMotionInfoFlagsNV(VkFlags); //

pub struct VkAccelerationStructureMotionInstanceFlagsNV(VkFlags); //

pub struct VkFormatFeatureFlags2(VkFlags64); //

pub type VkFormatFeatureFlags2KHR = VkFormatFeatureFlags2; //

pub struct VkFormatFeatureFlags4KHR(VkFlags64); //

pub struct VkRenderingFlags(VkFlags); //

pub struct VkMemoryDecompressionMethodFlagsEXT(VkFlags64); //

pub type VkMemoryDecompressionMethodFlagsNV = VkMemoryDecompressionMethodFlagsEXT; //

pub type VkRenderingFlagsKHR = VkRenderingFlags; //

pub struct VkDeviceFaultFlagsKHR(VkFlags); //

pub struct VkBuildMicromapFlagsEXT(VkFlags); //

pub struct VkMicromapCreateFlagsEXT(VkFlags); //

pub struct VkIndirectCommandsLayoutUsageFlagsEXT(VkFlags); //

pub struct VkIndirectCommandsInputModeFlagsEXT(VkFlags); //

pub struct VkDirectDriverLoadingFlagsLUNARG(VkFlags); //

pub struct VkPipelineCreateFlags2(VkFlags64); //

pub type VkPipelineCreateFlags2KHR = VkPipelineCreateFlags2; //

pub struct VkBufferUsageFlags2(VkFlags64); //

pub type VkBufferUsageFlags2KHR = VkBufferUsageFlags2; //

pub struct VkImageUsageFlags2KHR(VkFlags64); //

pub struct VkImageCreateFlags2KHR(VkFlags64); //

pub struct VkAddressCopyFlagsKHR(VkFlags); //

pub struct VkTensorCreateFlagsARM(VkFlags64); //

pub struct VkTensorUsageFlagsARM(VkFlags64); //

pub struct VkTensorViewCreateFlagsARM(VkFlags64); //

pub struct VkDataGraphPipelineSessionCreateFlagsARM(VkFlags64); //

pub struct VkDataGraphPipelineDispatchFlagsARM(VkFlags64); //

pub struct VkVideoEncodeRgbModelConversionFlagsVALVE(VkFlags); //

pub struct VkVideoEncodeRgbRangeCompressionFlagsVALVE(VkFlags); //

pub struct VkVideoEncodeRgbChromaOffsetFlagsVALVE(VkFlags); //

pub struct VkSpirvResourceTypeFlagsEXT(VkFlags); //

pub struct VkGpaSqShaderStageFlagsAMD(VkFlags); //

pub struct VkGpaPerfBlockPropertiesFlagsAMD(VkFlags); //

pub struct VkPhysicalDeviceGpaPropertiesFlagsAMD(VkFlags); //

pub struct VkAddressCommandFlagsKHR(VkFlags); //

pub struct VkCompositeAlphaFlagsKHR(VkFlags); //

pub struct VkDisplayPlaneAlphaFlagsKHR(VkFlags); //

pub struct VkSurfaceTransformFlagsKHR(VkFlags); //

pub struct VkSwapchainCreateFlagsKHR(VkFlags); //

pub struct VkDisplayModeCreateFlagsKHR(VkFlags); //

pub struct VkDisplaySurfaceCreateFlagsKHR(VkFlags); //

pub struct VkAndroidSurfaceCreateFlagsKHR(VkFlags); //

pub struct VkViSurfaceCreateFlagsNN(VkFlags); //

pub struct VkWaylandSurfaceCreateFlagsKHR(VkFlags); //

pub struct VkUbmSurfaceCreateFlagsSEC(VkFlags); //

pub struct VkWin32SurfaceCreateFlagsKHR(VkFlags); //

pub struct VkXlibSurfaceCreateFlagsKHR(VkFlags); //

pub struct VkXcbSurfaceCreateFlagsKHR(VkFlags); //

pub struct VkDirectFBSurfaceCreateFlagsEXT(VkFlags); //

pub struct VkIOSSurfaceCreateFlagsMVK(VkFlags); //

pub struct VkMacOSSurfaceCreateFlagsMVK(VkFlags); //

pub struct VkMetalSurfaceCreateFlagsEXT(VkFlags); //

pub struct VkImagePipeSurfaceCreateFlagsFUCHSIA(VkFlags); //

pub struct VkStreamDescriptorSurfaceCreateFlagsGGP(VkFlags); //

pub struct VkHeadlessSurfaceCreateFlagsEXT(VkFlags); //

pub struct VkScreenSurfaceCreateFlagsQNX(VkFlags); //

pub struct VkPeerMemoryFeatureFlags(VkFlags); //

pub type VkPeerMemoryFeatureFlagsKHR = VkPeerMemoryFeatureFlags; //

pub struct VkMemoryAllocateFlags(VkFlags); //

pub type VkMemoryAllocateFlagsKHR = VkMemoryAllocateFlags; //

pub struct VkDeviceGroupPresentModeFlagsKHR(VkFlags); //

pub struct VkDebugReportFlagsEXT(VkFlags); //

pub struct VkCommandPoolTrimFlags(VkFlags); //

pub type VkCommandPoolTrimFlagsKHR = VkCommandPoolTrimFlags; //

pub struct VkExternalMemoryHandleTypeFlagsNV(VkFlags); //

pub struct VkClusterAccelerationStructureIndexFormatFlagsNV(VkFlags); //

pub struct VkExternalMemoryFeatureFlagsNV(VkFlags); //

pub struct VkExternalMemoryHandleTypeFlags(VkFlags); //

pub type VkExternalMemoryHandleTypeFlagsKHR = VkExternalMemoryHandleTypeFlags; //

pub struct VkExternalMemoryFeatureFlags(VkFlags); //

pub type VkExternalMemoryFeatureFlagsKHR = VkExternalMemoryFeatureFlags; //

pub struct VkExternalSemaphoreHandleTypeFlags(VkFlags); //

pub type VkExternalSemaphoreHandleTypeFlagsKHR = VkExternalSemaphoreHandleTypeFlags; //

pub struct VkExternalSemaphoreFeatureFlags(VkFlags); //

pub type VkExternalSemaphoreFeatureFlagsKHR = VkExternalSemaphoreFeatureFlags; //

pub struct VkSemaphoreImportFlags(VkFlags); //

pub type VkSemaphoreImportFlagsKHR = VkSemaphoreImportFlags; //

pub struct VkExternalFenceHandleTypeFlags(VkFlags); //

pub type VkExternalFenceHandleTypeFlagsKHR = VkExternalFenceHandleTypeFlags; //

pub struct VkExternalFenceFeatureFlags(VkFlags); //

pub type VkExternalFenceFeatureFlagsKHR = VkExternalFenceFeatureFlags; //

pub struct VkFenceImportFlags(VkFlags); //

pub type VkFenceImportFlagsKHR = VkFenceImportFlags; //

pub struct VkSurfaceCounterFlagsEXT(VkFlags); //

pub struct VkPipelineViewportSwizzleStateCreateFlagsNV(VkFlags); //

pub struct VkPipelineDiscardRectangleStateCreateFlagsEXT(VkFlags); //

pub struct VkPipelineCoverageToColorStateCreateFlagsNV(VkFlags); //

pub struct VkPipelineCoverageModulationStateCreateFlagsNV(VkFlags); //

pub struct VkPipelineCoverageReductionStateCreateFlagsNV(VkFlags); //

pub struct VkValidationCacheCreateFlagsEXT(VkFlags); //

pub struct VkDebugUtilsMessageSeverityFlagsEXT(VkFlags); //

pub struct VkDebugUtilsMessageTypeFlagsEXT(VkFlags); //

pub struct VkDebugUtilsMessengerCreateFlagsEXT(VkFlags); //

pub struct VkDebugUtilsMessengerCallbackDataFlagsEXT(VkFlags); //

pub struct VkDeviceMemoryReportFlagsEXT(VkFlags); //

pub struct VkPipelineRasterizationConservativeStateCreateFlagsEXT(VkFlags); //

pub struct VkDescriptorBindingFlags(VkFlags); //

pub type VkDescriptorBindingFlagsEXT = VkDescriptorBindingFlags; //

pub struct VkConditionalRenderingFlagsEXT(VkFlags); //

pub struct VkResolveModeFlags(VkFlags); //

pub type VkResolveModeFlagsKHR = VkResolveModeFlags; //

pub struct VkPipelineRasterizationStateStreamCreateFlagsEXT(VkFlags); //

pub struct VkPipelineRasterizationDepthClipStateCreateFlagsEXT(VkFlags); //

pub struct VkSwapchainImageUsageFlagsANDROID(VkFlags); //

pub struct VkToolPurposeFlags(VkFlags); //

pub type VkToolPurposeFlagsEXT = VkToolPurposeFlags; //

pub struct VkSubmitFlags(VkFlags); //

pub type VkSubmitFlagsKHR = VkSubmitFlags; //

pub struct VkImageFormatConstraintsFlagsFUCHSIA(VkFlags); //

pub struct VkHostImageCopyFlags(VkFlags); //

pub type VkHostImageCopyFlagsEXT = VkHostImageCopyFlags; //

pub struct VkPartitionedAccelerationStructureInstanceFlagsNV(VkFlags); //

pub struct VkImageConstraintsInfoFlagsFUCHSIA(VkFlags); //

pub struct VkGraphicsPipelineLibraryFlagsEXT(VkFlags); //

pub struct VkImageCompressionFlagsEXT(VkFlags); //

pub struct VkImageCompressionFixedRateFlagsEXT(VkFlags); //

pub struct VkExportMetalObjectTypeFlagsEXT(VkFlags); //

pub struct VkRenderingAttachmentFlagsKHR(VkFlags); //

pub struct VkResolveImageFlagsKHR(VkFlags); //

pub struct VkDeviceAddressBindingFlagsEXT(VkFlags); //

pub struct VkOpticalFlowGridSizeFlagsNV(VkFlags); //

pub struct VkOpticalFlowUsageFlagsNV(VkFlags); //

pub struct VkOpticalFlowSessionCreateFlagsNV(VkFlags); //

pub struct VkOpticalFlowExecuteFlagsNV(VkFlags); //

pub struct VkFrameBoundaryFlagsEXT(VkFlags); //

pub struct VkPresentScalingFlagsKHR(VkFlags); //

pub type VkPresentScalingFlagsEXT = VkPresentScalingFlagsKHR; //

pub struct VkPresentGravityFlagsKHR(VkFlags); //

pub type VkPresentGravityFlagsEXT = VkPresentGravityFlagsKHR; //

pub struct VkShaderCreateFlagsEXT(VkFlags); //

pub struct VkTileShadingRenderPassFlagsQCOM(VkFlags); //

pub struct VkPhysicalDeviceSchedulingControlsFlagsARM(VkFlags64); //

pub struct VkSurfaceCreateFlagsOHOS(VkFlags); //

pub struct VkPresentStageFlagsEXT(VkFlags); //

pub struct VkPastPresentationTimingFlagsEXT(VkFlags); //

pub struct VkPresentTimingInfoFlagsEXT(VkFlags); //

pub struct VkSwapchainImageUsageFlagsOHOS(VkFlags); //

pub struct VkPerformanceCounterDescriptionFlagsARM(VkFlags); //

pub struct VkShaderInstrumentationValuesFlagsARM(VkFlags); //

pub struct VkDataGraphTOSAQualityFlagsARM(VkFlags); //

pub struct VkDataGraphOpticalFlowGridSizeFlagsARM(VkFlags); //

pub struct VkDataGraphOpticalFlowImageUsageFlagsARM(VkFlags); //

pub struct VkDataGraphOpticalFlowCreateFlagsARM(VkFlags); //

pub struct VkDataGraphOpticalFlowExecuteFlagsARM(VkFlags); //

pub struct VkVideoCodecOperationFlagsKHR(VkFlags); //

pub struct VkVideoCapabilityFlagsKHR(VkFlags); //

pub struct VkVideoSessionCreateFlagsKHR(VkFlags); //

pub struct VkVideoSessionParametersCreateFlagsKHR(VkFlags); //

pub struct VkVideoBeginCodingFlagsKHR(VkFlags); //

pub struct VkVideoEndCodingFlagsKHR(VkFlags); //

pub struct VkVideoCodingControlFlagsKHR(VkFlags); //

pub struct VkVideoDecodeUsageFlagsKHR(VkFlags); //

pub struct VkVideoDecodeCapabilityFlagsKHR(VkFlags); //

pub struct VkVideoDecodeFlagsKHR(VkFlags); //

pub struct VkVideoDecodeH264PictureLayoutFlagsKHR(VkFlags); //

pub struct VkVideoEncodeFlagsKHR(VkFlags); //

pub struct VkVideoEncodeUsageFlagsKHR(VkFlags); //

pub struct VkVideoEncodeContentFlagsKHR(VkFlags); //

pub struct VkVideoEncodeCapabilityFlagsKHR(VkFlags); //

pub struct VkVideoEncodeFeedbackFlagsKHR(VkFlags); //

pub struct VkVideoEncodePerPartitionFeedbackFlagsKHR(VkFlags); //

pub struct VkVideoEncodeRateControlFlagsKHR(VkFlags); //

pub struct VkVideoEncodeRateControlModeFlagsKHR(VkFlags); //

pub struct VkVideoEncodeIntraRefreshModeFlagsKHR(VkFlags); //

pub struct VkVideoChromaSubsamplingFlagsKHR(VkFlags); //

pub struct VkVideoComponentBitDepthFlagsKHR(VkFlags); //

pub struct VkVideoEncodeH264CapabilityFlagsKHR(VkFlags); //

pub struct VkVideoEncodeH264StdFlagsKHR(VkFlags); //

pub struct VkVideoEncodeH264RateControlFlagsKHR(VkFlags); //

pub struct VkVideoEncodeH265CapabilityFlagsKHR(VkFlags); //

pub struct VkVideoEncodeH265StdFlagsKHR(VkFlags); //

pub struct VkVideoEncodeH265RateControlFlagsKHR(VkFlags); //

pub struct VkVideoEncodeH265CtbSizeFlagsKHR(VkFlags); //

pub struct VkVideoEncodeH265TransformBlockSizeFlagsKHR(VkFlags); //

pub struct VkVideoEncodeAV1CapabilityFlagsKHR(VkFlags); //

pub struct VkVideoEncodeAV1StdFlagsKHR(VkFlags); //

pub struct VkVideoEncodeAV1RateControlFlagsKHR(VkFlags); //

pub struct VkVideoEncodeAV1SuperblockSizeFlagsKHR(VkFlags); //

pub struct VkAccessFlags3KHR(VkFlags64); //

pub struct VkCooperativeMatrixFlagsEXT(VkFlags); //

pub struct VkImageLayout(i32); //
impl VkImageLayout {
    pub const VK_IMAGE_LAYOUT_UNDEFINED: i32 = 0;
    pub const VK_IMAGE_LAYOUT_GENERAL: i32 = 1;
    pub const VK_IMAGE_LAYOUT_COLOR_ATTACHMENT_OPTIMAL: i32 = 2;
    pub const VK_IMAGE_LAYOUT_DEPTH_STENCIL_ATTACHMENT_OPTIMAL: i32 = 3;
    pub const VK_IMAGE_LAYOUT_DEPTH_STENCIL_READ_ONLY_OPTIMAL: i32 = 4;
    pub const VK_IMAGE_LAYOUT_SHADER_READ_ONLY_OPTIMAL: i32 = 5;
    pub const VK_IMAGE_LAYOUT_TRANSFER_SRC_OPTIMAL: i32 = 6;
    pub const VK_IMAGE_LAYOUT_TRANSFER_DST_OPTIMAL: i32 = 7;
    pub const VK_IMAGE_LAYOUT_PREINITIALIZED: i32 = 8;
}

pub struct VkAttachmentLoadOp(i32); //
impl VkAttachmentLoadOp {
    pub const VK_ATTACHMENT_LOAD_OP_LOAD: i32 = 0;
    pub const VK_ATTACHMENT_LOAD_OP_CLEAR: i32 = 1;
    pub const VK_ATTACHMENT_LOAD_OP_DONT_CARE: i32 = 2;
}

pub struct VkAttachmentStoreOp(i32); //
impl VkAttachmentStoreOp {
    pub const VK_ATTACHMENT_STORE_OP_STORE: i32 = 0;
    pub const VK_ATTACHMENT_STORE_OP_DONT_CARE: i32 = 1;
}

pub struct VkImageType(i32); //
impl VkImageType {
    pub const VK_IMAGE_TYPE_1D: i32 = 0;
    pub const VK_IMAGE_TYPE_2D: i32 = 1;
    pub const VK_IMAGE_TYPE_3D: i32 = 2;
}

pub struct VkImageTiling(i32); //
impl VkImageTiling {
    pub const VK_IMAGE_TILING_OPTIMAL: i32 = 0;
    pub const VK_IMAGE_TILING_LINEAR: i32 = 1;
}

pub struct VkImageViewType(i32); //
impl VkImageViewType {
    pub const VK_IMAGE_VIEW_TYPE_1D: i32 = 0;
    pub const VK_IMAGE_VIEW_TYPE_2D: i32 = 1;
    pub const VK_IMAGE_VIEW_TYPE_3D: i32 = 2;
    pub const VK_IMAGE_VIEW_TYPE_CUBE: i32 = 3;
    pub const VK_IMAGE_VIEW_TYPE_1D_ARRAY: i32 = 4;
    pub const VK_IMAGE_VIEW_TYPE_2D_ARRAY: i32 = 5;
    pub const VK_IMAGE_VIEW_TYPE_CUBE_ARRAY: i32 = 6;
}

pub struct VkCommandBufferLevel(i32); //
impl VkCommandBufferLevel {
    pub const VK_COMMAND_BUFFER_LEVEL_PRIMARY: i32 = 0;
    pub const VK_COMMAND_BUFFER_LEVEL_SECONDARY: i32 = 1;
}

pub struct VkComponentSwizzle(i32); //
impl VkComponentSwizzle {
    pub const VK_COMPONENT_SWIZZLE_IDENTITY: i32 = 0;
    pub const VK_COMPONENT_SWIZZLE_ZERO: i32 = 1;
    pub const VK_COMPONENT_SWIZZLE_ONE: i32 = 2;
    pub const VK_COMPONENT_SWIZZLE_R: i32 = 3;
    pub const VK_COMPONENT_SWIZZLE_G: i32 = 4;
    pub const VK_COMPONENT_SWIZZLE_B: i32 = 5;
    pub const VK_COMPONENT_SWIZZLE_A: i32 = 6;
}

pub struct VkDescriptorType(i32); //
impl VkDescriptorType {
    pub const VK_DESCRIPTOR_TYPE_SAMPLER: i32 = 0;
    pub const VK_DESCRIPTOR_TYPE_COMBINED_IMAGE_SAMPLER: i32 = 1;
    pub const VK_DESCRIPTOR_TYPE_SAMPLED_IMAGE: i32 = 2;
    pub const VK_DESCRIPTOR_TYPE_STORAGE_IMAGE: i32 = 3;
    pub const VK_DESCRIPTOR_TYPE_UNIFORM_TEXEL_BUFFER: i32 = 4;
    pub const VK_DESCRIPTOR_TYPE_STORAGE_TEXEL_BUFFER: i32 = 5;
    pub const VK_DESCRIPTOR_TYPE_UNIFORM_BUFFER: i32 = 6;
    pub const VK_DESCRIPTOR_TYPE_STORAGE_BUFFER: i32 = 7;
    pub const VK_DESCRIPTOR_TYPE_UNIFORM_BUFFER_DYNAMIC: i32 = 8;
    pub const VK_DESCRIPTOR_TYPE_STORAGE_BUFFER_DYNAMIC: i32 = 9;
    pub const VK_DESCRIPTOR_TYPE_INPUT_ATTACHMENT: i32 = 10;
}

pub struct VkQueryType(i32); //
impl VkQueryType {
    pub const VK_QUERY_TYPE_OCCLUSION: i32 = 0;
    pub const VK_QUERY_TYPE_PIPELINE_STATISTICS: i32 = 1;
    pub const VK_QUERY_TYPE_TIMESTAMP: i32 = 2;
}

pub struct VkBorderColor(i32); //
impl VkBorderColor {
    pub const VK_BORDER_COLOR_FLOAT_TRANSPARENT_BLACK: i32 = 0;
    pub const VK_BORDER_COLOR_INT_TRANSPARENT_BLACK: i32 = 1;
    pub const VK_BORDER_COLOR_FLOAT_OPAQUE_BLACK: i32 = 2;
    pub const VK_BORDER_COLOR_INT_OPAQUE_BLACK: i32 = 3;
    pub const VK_BORDER_COLOR_FLOAT_OPAQUE_WHITE: i32 = 4;
    pub const VK_BORDER_COLOR_INT_OPAQUE_WHITE: i32 = 5;
}

pub struct VkPipelineBindPoint(i32); //
impl VkPipelineBindPoint {
    pub const VK_PIPELINE_BIND_POINT_GRAPHICS: i32 = 0;
    pub const VK_PIPELINE_BIND_POINT_COMPUTE: i32 = 1;
}

pub struct VkPipelineCacheHeaderVersion(i32); //
impl VkPipelineCacheHeaderVersion {
    pub const VK_PIPELINE_CACHE_HEADER_VERSION_ONE: i32 = 1;
}

pub type VkPipelineCacheCreateFlagBits = VkPipelineCacheCreateFlags; //
impl VkPipelineCacheCreateFlagBits {
}

pub struct VkPrimitiveTopology(i32); //
impl VkPrimitiveTopology {
    pub const VK_PRIMITIVE_TOPOLOGY_POINT_LIST: i32 = 0;
    pub const VK_PRIMITIVE_TOPOLOGY_LINE_LIST: i32 = 1;
    pub const VK_PRIMITIVE_TOPOLOGY_LINE_STRIP: i32 = 2;
    pub const VK_PRIMITIVE_TOPOLOGY_TRIANGLE_LIST: i32 = 3;
    pub const VK_PRIMITIVE_TOPOLOGY_TRIANGLE_STRIP: i32 = 4;
    pub const VK_PRIMITIVE_TOPOLOGY_TRIANGLE_FAN: i32 = 5;
    pub const VK_PRIMITIVE_TOPOLOGY_LINE_LIST_WITH_ADJACENCY: i32 = 6;
    pub const VK_PRIMITIVE_TOPOLOGY_LINE_STRIP_WITH_ADJACENCY: i32 = 7;
    pub const VK_PRIMITIVE_TOPOLOGY_TRIANGLE_LIST_WITH_ADJACENCY: i32 = 8;
    pub const VK_PRIMITIVE_TOPOLOGY_TRIANGLE_STRIP_WITH_ADJACENCY: i32 = 9;
    pub const VK_PRIMITIVE_TOPOLOGY_PATCH_LIST: i32 = 10;
}

pub struct VkSharingMode(i32); //
impl VkSharingMode {
    pub const VK_SHARING_MODE_EXCLUSIVE: i32 = 0;
    pub const VK_SHARING_MODE_CONCURRENT: i32 = 1;
}

pub struct VkIndexType(i32); //
impl VkIndexType {
    pub const VK_INDEX_TYPE_UINT16: i32 = 0;
    pub const VK_INDEX_TYPE_UINT32: i32 = 1;
}

pub struct VkFilter(i32); //
impl VkFilter {
    pub const VK_FILTER_NEAREST: i32 = 0;
    pub const VK_FILTER_LINEAR: i32 = 1;
}

pub struct VkSamplerMipmapMode(i32); //
impl VkSamplerMipmapMode {
    pub const VK_SAMPLER_MIPMAP_MODE_NEAREST: i32 = 0;
    pub const VK_SAMPLER_MIPMAP_MODE_LINEAR: i32 = 1;
}

pub struct VkSamplerAddressMode(i32); //
impl VkSamplerAddressMode {
    pub const VK_SAMPLER_ADDRESS_MODE_REPEAT: i32 = 0;
    pub const VK_SAMPLER_ADDRESS_MODE_MIRRORED_REPEAT: i32 = 1;
    pub const VK_SAMPLER_ADDRESS_MODE_CLAMP_TO_EDGE: i32 = 2;
    pub const VK_SAMPLER_ADDRESS_MODE_CLAMP_TO_BORDER: i32 = 3;
}

pub struct VkCompareOp(i32); //
impl VkCompareOp {
    pub const VK_COMPARE_OP_NEVER: i32 = 0;
    pub const VK_COMPARE_OP_LESS: i32 = 1;
    pub const VK_COMPARE_OP_EQUAL: i32 = 2;
    pub const VK_COMPARE_OP_LESS_OR_EQUAL: i32 = 3;
    pub const VK_COMPARE_OP_GREATER: i32 = 4;
    pub const VK_COMPARE_OP_NOT_EQUAL: i32 = 5;
    pub const VK_COMPARE_OP_GREATER_OR_EQUAL: i32 = 6;
    pub const VK_COMPARE_OP_ALWAYS: i32 = 7;
}

pub struct VkPolygonMode(i32); //
impl VkPolygonMode {
    pub const VK_POLYGON_MODE_FILL: i32 = 0;
    pub const VK_POLYGON_MODE_LINE: i32 = 1;
    pub const VK_POLYGON_MODE_POINT: i32 = 2;
}

pub struct VkFrontFace(i32); //
impl VkFrontFace {
    pub const VK_FRONT_FACE_COUNTER_CLOCKWISE: i32 = 0;
    pub const VK_FRONT_FACE_CLOCKWISE: i32 = 1;
}

pub struct VkBlendFactor(i32); //
impl VkBlendFactor {
    pub const VK_BLEND_FACTOR_ZERO: i32 = 0;
    pub const VK_BLEND_FACTOR_ONE: i32 = 1;
    pub const VK_BLEND_FACTOR_SRC_COLOR: i32 = 2;
    pub const VK_BLEND_FACTOR_ONE_MINUS_SRC_COLOR: i32 = 3;
    pub const VK_BLEND_FACTOR_DST_COLOR: i32 = 4;
    pub const VK_BLEND_FACTOR_ONE_MINUS_DST_COLOR: i32 = 5;
    pub const VK_BLEND_FACTOR_SRC_ALPHA: i32 = 6;
    pub const VK_BLEND_FACTOR_ONE_MINUS_SRC_ALPHA: i32 = 7;
    pub const VK_BLEND_FACTOR_DST_ALPHA: i32 = 8;
    pub const VK_BLEND_FACTOR_ONE_MINUS_DST_ALPHA: i32 = 9;
    pub const VK_BLEND_FACTOR_CONSTANT_COLOR: i32 = 10;
    pub const VK_BLEND_FACTOR_ONE_MINUS_CONSTANT_COLOR: i32 = 11;
    pub const VK_BLEND_FACTOR_CONSTANT_ALPHA: i32 = 12;
    pub const VK_BLEND_FACTOR_ONE_MINUS_CONSTANT_ALPHA: i32 = 13;
    pub const VK_BLEND_FACTOR_SRC_ALPHA_SATURATE: i32 = 14;
    pub const VK_BLEND_FACTOR_SRC1_COLOR: i32 = 15;
    pub const VK_BLEND_FACTOR_ONE_MINUS_SRC1_COLOR: i32 = 16;
    pub const VK_BLEND_FACTOR_SRC1_ALPHA: i32 = 17;
    pub const VK_BLEND_FACTOR_ONE_MINUS_SRC1_ALPHA: i32 = 18;
}

pub struct VkBlendOp(i32); //
impl VkBlendOp {
    pub const VK_BLEND_OP_ADD: i32 = 0;
    pub const VK_BLEND_OP_SUBTRACT: i32 = 1;
    pub const VK_BLEND_OP_REVERSE_SUBTRACT: i32 = 2;
    pub const VK_BLEND_OP_MIN: i32 = 3;
    pub const VK_BLEND_OP_MAX: i32 = 4;
}

pub struct VkStencilOp(i32); //
impl VkStencilOp {
    pub const VK_STENCIL_OP_KEEP: i32 = 0;
    pub const VK_STENCIL_OP_ZERO: i32 = 1;
    pub const VK_STENCIL_OP_REPLACE: i32 = 2;
    pub const VK_STENCIL_OP_INCREMENT_AND_CLAMP: i32 = 3;
    pub const VK_STENCIL_OP_DECREMENT_AND_CLAMP: i32 = 4;
    pub const VK_STENCIL_OP_INVERT: i32 = 5;
    pub const VK_STENCIL_OP_INCREMENT_AND_WRAP: i32 = 6;
    pub const VK_STENCIL_OP_DECREMENT_AND_WRAP: i32 = 7;
}

pub struct VkLogicOp(i32); //
impl VkLogicOp {
    pub const VK_LOGIC_OP_CLEAR: i32 = 0;
    pub const VK_LOGIC_OP_AND: i32 = 1;
    pub const VK_LOGIC_OP_AND_REVERSE: i32 = 2;
    pub const VK_LOGIC_OP_COPY: i32 = 3;
    pub const VK_LOGIC_OP_AND_INVERTED: i32 = 4;
    pub const VK_LOGIC_OP_NO_OP: i32 = 5;
    pub const VK_LOGIC_OP_XOR: i32 = 6;
    pub const VK_LOGIC_OP_OR: i32 = 7;
    pub const VK_LOGIC_OP_NOR: i32 = 8;
    pub const VK_LOGIC_OP_EQUIVALENT: i32 = 9;
    pub const VK_LOGIC_OP_INVERT: i32 = 10;
    pub const VK_LOGIC_OP_OR_REVERSE: i32 = 11;
    pub const VK_LOGIC_OP_COPY_INVERTED: i32 = 12;
    pub const VK_LOGIC_OP_OR_INVERTED: i32 = 13;
    pub const VK_LOGIC_OP_NAND: i32 = 14;
    pub const VK_LOGIC_OP_SET: i32 = 15;
}

pub struct VkInternalAllocationType(i32); //
impl VkInternalAllocationType {
    pub const VK_INTERNAL_ALLOCATION_TYPE_EXECUTABLE: i32 = 0;
}

pub struct VkSystemAllocationScope(i32); //
impl VkSystemAllocationScope {
    pub const VK_SYSTEM_ALLOCATION_SCOPE_COMMAND: i32 = 0;
    pub const VK_SYSTEM_ALLOCATION_SCOPE_OBJECT: i32 = 1;
    pub const VK_SYSTEM_ALLOCATION_SCOPE_CACHE: i32 = 2;
    pub const VK_SYSTEM_ALLOCATION_SCOPE_DEVICE: i32 = 3;
    pub const VK_SYSTEM_ALLOCATION_SCOPE_INSTANCE: i32 = 4;
}

pub struct VkPhysicalDeviceType(i32); //
impl VkPhysicalDeviceType {
    pub const VK_PHYSICAL_DEVICE_TYPE_OTHER: i32 = 0;
    pub const VK_PHYSICAL_DEVICE_TYPE_INTEGRATED_GPU: i32 = 1;
    pub const VK_PHYSICAL_DEVICE_TYPE_DISCRETE_GPU: i32 = 2;
    pub const VK_PHYSICAL_DEVICE_TYPE_VIRTUAL_GPU: i32 = 3;
    pub const VK_PHYSICAL_DEVICE_TYPE_CPU: i32 = 4;
}

pub struct VkVertexInputRate(i32); //
impl VkVertexInputRate {
    pub const VK_VERTEX_INPUT_RATE_VERTEX: i32 = 0;
    pub const VK_VERTEX_INPUT_RATE_INSTANCE: i32 = 1;
}

pub struct VkFormat(i32); //Vulkan format definitions
impl VkFormat {
    pub const VK_FORMAT_UNDEFINED: i32 = 0;
    pub const VK_FORMAT_R4G4_UNORM_PACK8: i32 = 1;
    pub const VK_FORMAT_R4G4B4A4_UNORM_PACK16: i32 = 2;
    pub const VK_FORMAT_B4G4R4A4_UNORM_PACK16: i32 = 3;
    pub const VK_FORMAT_R5G6B5_UNORM_PACK16: i32 = 4;
    pub const VK_FORMAT_B5G6R5_UNORM_PACK16: i32 = 5;
    pub const VK_FORMAT_R5G5B5A1_UNORM_PACK16: i32 = 6;
    pub const VK_FORMAT_B5G5R5A1_UNORM_PACK16: i32 = 7;
    pub const VK_FORMAT_A1R5G5B5_UNORM_PACK16: i32 = 8;
    pub const VK_FORMAT_R8_UNORM: i32 = 9;
    pub const VK_FORMAT_R8_SNORM: i32 = 10;
    pub const VK_FORMAT_R8_USCALED: i32 = 11;
    pub const VK_FORMAT_R8_SSCALED: i32 = 12;
    pub const VK_FORMAT_R8_UINT: i32 = 13;
    pub const VK_FORMAT_R8_SINT: i32 = 14;
    pub const VK_FORMAT_R8_SRGB: i32 = 15;
    pub const VK_FORMAT_R8G8_UNORM: i32 = 16;
    pub const VK_FORMAT_R8G8_SNORM: i32 = 17;
    pub const VK_FORMAT_R8G8_USCALED: i32 = 18;
    pub const VK_FORMAT_R8G8_SSCALED: i32 = 19;
    pub const VK_FORMAT_R8G8_UINT: i32 = 20;
    pub const VK_FORMAT_R8G8_SINT: i32 = 21;
    pub const VK_FORMAT_R8G8_SRGB: i32 = 22;
    pub const VK_FORMAT_R8G8B8_UNORM: i32 = 23;
    pub const VK_FORMAT_R8G8B8_SNORM: i32 = 24;
    pub const VK_FORMAT_R8G8B8_USCALED: i32 = 25;
    pub const VK_FORMAT_R8G8B8_SSCALED: i32 = 26;
    pub const VK_FORMAT_R8G8B8_UINT: i32 = 27;
    pub const VK_FORMAT_R8G8B8_SINT: i32 = 28;
    pub const VK_FORMAT_R8G8B8_SRGB: i32 = 29;
    pub const VK_FORMAT_B8G8R8_UNORM: i32 = 30;
    pub const VK_FORMAT_B8G8R8_SNORM: i32 = 31;
    pub const VK_FORMAT_B8G8R8_USCALED: i32 = 32;
    pub const VK_FORMAT_B8G8R8_SSCALED: i32 = 33;
    pub const VK_FORMAT_B8G8R8_UINT: i32 = 34;
    pub const VK_FORMAT_B8G8R8_SINT: i32 = 35;
    pub const VK_FORMAT_B8G8R8_SRGB: i32 = 36;
    pub const VK_FORMAT_R8G8B8A8_UNORM: i32 = 37;
    pub const VK_FORMAT_R8G8B8A8_SNORM: i32 = 38;
    pub const VK_FORMAT_R8G8B8A8_USCALED: i32 = 39;
    pub const VK_FORMAT_R8G8B8A8_SSCALED: i32 = 40;
    pub const VK_FORMAT_R8G8B8A8_UINT: i32 = 41;
    pub const VK_FORMAT_R8G8B8A8_SINT: i32 = 42;
    pub const VK_FORMAT_R8G8B8A8_SRGB: i32 = 43;
    pub const VK_FORMAT_B8G8R8A8_UNORM: i32 = 44;
    pub const VK_FORMAT_B8G8R8A8_SNORM: i32 = 45;
    pub const VK_FORMAT_B8G8R8A8_USCALED: i32 = 46;
    pub const VK_FORMAT_B8G8R8A8_SSCALED: i32 = 47;
    pub const VK_FORMAT_B8G8R8A8_UINT: i32 = 48;
    pub const VK_FORMAT_B8G8R8A8_SINT: i32 = 49;
    pub const VK_FORMAT_B8G8R8A8_SRGB: i32 = 50;
    pub const VK_FORMAT_A8B8G8R8_UNORM_PACK32: i32 = 51;
    pub const VK_FORMAT_A8B8G8R8_SNORM_PACK32: i32 = 52;
    pub const VK_FORMAT_A8B8G8R8_USCALED_PACK32: i32 = 53;
    pub const VK_FORMAT_A8B8G8R8_SSCALED_PACK32: i32 = 54;
    pub const VK_FORMAT_A8B8G8R8_UINT_PACK32: i32 = 55;
    pub const VK_FORMAT_A8B8G8R8_SINT_PACK32: i32 = 56;
    pub const VK_FORMAT_A8B8G8R8_SRGB_PACK32: i32 = 57;
    pub const VK_FORMAT_A2R10G10B10_UNORM_PACK32: i32 = 58;
    pub const VK_FORMAT_A2R10G10B10_SNORM_PACK32: i32 = 59;
    pub const VK_FORMAT_A2R10G10B10_USCALED_PACK32: i32 = 60;
    pub const VK_FORMAT_A2R10G10B10_SSCALED_PACK32: i32 = 61;
    pub const VK_FORMAT_A2R10G10B10_UINT_PACK32: i32 = 62;
    pub const VK_FORMAT_A2R10G10B10_SINT_PACK32: i32 = 63;
    pub const VK_FORMAT_A2B10G10R10_UNORM_PACK32: i32 = 64;
    pub const VK_FORMAT_A2B10G10R10_SNORM_PACK32: i32 = 65;
    pub const VK_FORMAT_A2B10G10R10_USCALED_PACK32: i32 = 66;
    pub const VK_FORMAT_A2B10G10R10_SSCALED_PACK32: i32 = 67;
    pub const VK_FORMAT_A2B10G10R10_UINT_PACK32: i32 = 68;
    pub const VK_FORMAT_A2B10G10R10_SINT_PACK32: i32 = 69;
    pub const VK_FORMAT_R16_UNORM: i32 = 70;
    pub const VK_FORMAT_R16_SNORM: i32 = 71;
    pub const VK_FORMAT_R16_USCALED: i32 = 72;
    pub const VK_FORMAT_R16_SSCALED: i32 = 73;
    pub const VK_FORMAT_R16_UINT: i32 = 74;
    pub const VK_FORMAT_R16_SINT: i32 = 75;
    pub const VK_FORMAT_R16_SFLOAT: i32 = 76;
    pub const VK_FORMAT_R16G16_UNORM: i32 = 77;
    pub const VK_FORMAT_R16G16_SNORM: i32 = 78;
    pub const VK_FORMAT_R16G16_USCALED: i32 = 79;
    pub const VK_FORMAT_R16G16_SSCALED: i32 = 80;
    pub const VK_FORMAT_R16G16_UINT: i32 = 81;
    pub const VK_FORMAT_R16G16_SINT: i32 = 82;
    pub const VK_FORMAT_R16G16_SFLOAT: i32 = 83;
    pub const VK_FORMAT_R16G16B16_UNORM: i32 = 84;
    pub const VK_FORMAT_R16G16B16_SNORM: i32 = 85;
    pub const VK_FORMAT_R16G16B16_USCALED: i32 = 86;
    pub const VK_FORMAT_R16G16B16_SSCALED: i32 = 87;
    pub const VK_FORMAT_R16G16B16_UINT: i32 = 88;
    pub const VK_FORMAT_R16G16B16_SINT: i32 = 89;
    pub const VK_FORMAT_R16G16B16_SFLOAT: i32 = 90;
    pub const VK_FORMAT_R16G16B16A16_UNORM: i32 = 91;
    pub const VK_FORMAT_R16G16B16A16_SNORM: i32 = 92;
    pub const VK_FORMAT_R16G16B16A16_USCALED: i32 = 93;
    pub const VK_FORMAT_R16G16B16A16_SSCALED: i32 = 94;
    pub const VK_FORMAT_R16G16B16A16_UINT: i32 = 95;
    pub const VK_FORMAT_R16G16B16A16_SINT: i32 = 96;
    pub const VK_FORMAT_R16G16B16A16_SFLOAT: i32 = 97;
    pub const VK_FORMAT_R32_UINT: i32 = 98;
    pub const VK_FORMAT_R32_SINT: i32 = 99;
    pub const VK_FORMAT_R32_SFLOAT: i32 = 100;
    pub const VK_FORMAT_R32G32_UINT: i32 = 101;
    pub const VK_FORMAT_R32G32_SINT: i32 = 102;
    pub const VK_FORMAT_R32G32_SFLOAT: i32 = 103;
    pub const VK_FORMAT_R32G32B32_UINT: i32 = 104;
    pub const VK_FORMAT_R32G32B32_SINT: i32 = 105;
    pub const VK_FORMAT_R32G32B32_SFLOAT: i32 = 106;
    pub const VK_FORMAT_R32G32B32A32_UINT: i32 = 107;
    pub const VK_FORMAT_R32G32B32A32_SINT: i32 = 108;
    pub const VK_FORMAT_R32G32B32A32_SFLOAT: i32 = 109;
    pub const VK_FORMAT_R64_UINT: i32 = 110;
    pub const VK_FORMAT_R64_SINT: i32 = 111;
    pub const VK_FORMAT_R64_SFLOAT: i32 = 112;
    pub const VK_FORMAT_R64G64_UINT: i32 = 113;
    pub const VK_FORMAT_R64G64_SINT: i32 = 114;
    pub const VK_FORMAT_R64G64_SFLOAT: i32 = 115;
    pub const VK_FORMAT_R64G64B64_UINT: i32 = 116;
    pub const VK_FORMAT_R64G64B64_SINT: i32 = 117;
    pub const VK_FORMAT_R64G64B64_SFLOAT: i32 = 118;
    pub const VK_FORMAT_R64G64B64A64_UINT: i32 = 119;
    pub const VK_FORMAT_R64G64B64A64_SINT: i32 = 120;
    pub const VK_FORMAT_R64G64B64A64_SFLOAT: i32 = 121;
    pub const VK_FORMAT_B10G11R11_UFLOAT_PACK32: i32 = 122;
    pub const VK_FORMAT_E5B9G9R9_UFLOAT_PACK32: i32 = 123;
    pub const VK_FORMAT_D16_UNORM: i32 = 124;
    pub const VK_FORMAT_X8_D24_UNORM_PACK32: i32 = 125;
    pub const VK_FORMAT_D32_SFLOAT: i32 = 126;
    pub const VK_FORMAT_S8_UINT: i32 = 127;
    pub const VK_FORMAT_D16_UNORM_S8_UINT: i32 = 128;
    pub const VK_FORMAT_D24_UNORM_S8_UINT: i32 = 129;
    pub const VK_FORMAT_D32_SFLOAT_S8_UINT: i32 = 130;
    pub const VK_FORMAT_BC1_RGB_UNORM_BLOCK: i32 = 131;
    pub const VK_FORMAT_BC1_RGB_SRGB_BLOCK: i32 = 132;
    pub const VK_FORMAT_BC1_RGBA_UNORM_BLOCK: i32 = 133;
    pub const VK_FORMAT_BC1_RGBA_SRGB_BLOCK: i32 = 134;
    pub const VK_FORMAT_BC2_UNORM_BLOCK: i32 = 135;
    pub const VK_FORMAT_BC2_SRGB_BLOCK: i32 = 136;
    pub const VK_FORMAT_BC3_UNORM_BLOCK: i32 = 137;
    pub const VK_FORMAT_BC3_SRGB_BLOCK: i32 = 138;
    pub const VK_FORMAT_BC4_UNORM_BLOCK: i32 = 139;
    pub const VK_FORMAT_BC4_SNORM_BLOCK: i32 = 140;
    pub const VK_FORMAT_BC5_UNORM_BLOCK: i32 = 141;
    pub const VK_FORMAT_BC5_SNORM_BLOCK: i32 = 142;
    pub const VK_FORMAT_BC6H_UFLOAT_BLOCK: i32 = 143;
    pub const VK_FORMAT_BC6H_SFLOAT_BLOCK: i32 = 144;
    pub const VK_FORMAT_BC7_UNORM_BLOCK: i32 = 145;
    pub const VK_FORMAT_BC7_SRGB_BLOCK: i32 = 146;
    pub const VK_FORMAT_ETC2_R8G8B8_UNORM_BLOCK: i32 = 147;
    pub const VK_FORMAT_ETC2_R8G8B8_SRGB_BLOCK: i32 = 148;
    pub const VK_FORMAT_ETC2_R8G8B8A1_UNORM_BLOCK: i32 = 149;
    pub const VK_FORMAT_ETC2_R8G8B8A1_SRGB_BLOCK: i32 = 150;
    pub const VK_FORMAT_ETC2_R8G8B8A8_UNORM_BLOCK: i32 = 151;
    pub const VK_FORMAT_ETC2_R8G8B8A8_SRGB_BLOCK: i32 = 152;
    pub const VK_FORMAT_EAC_R11_UNORM_BLOCK: i32 = 153;
    pub const VK_FORMAT_EAC_R11_SNORM_BLOCK: i32 = 154;
    pub const VK_FORMAT_EAC_R11G11_UNORM_BLOCK: i32 = 155;
    pub const VK_FORMAT_EAC_R11G11_SNORM_BLOCK: i32 = 156;
    pub const VK_FORMAT_ASTC_4x4_UNORM_BLOCK: i32 = 157;
    pub const VK_FORMAT_ASTC_4x4_SRGB_BLOCK: i32 = 158;
    pub const VK_FORMAT_ASTC_5x4_UNORM_BLOCK: i32 = 159;
    pub const VK_FORMAT_ASTC_5x4_SRGB_BLOCK: i32 = 160;
    pub const VK_FORMAT_ASTC_5x5_UNORM_BLOCK: i32 = 161;
    pub const VK_FORMAT_ASTC_5x5_SRGB_BLOCK: i32 = 162;
    pub const VK_FORMAT_ASTC_6x5_UNORM_BLOCK: i32 = 163;
    pub const VK_FORMAT_ASTC_6x5_SRGB_BLOCK: i32 = 164;
    pub const VK_FORMAT_ASTC_6x6_UNORM_BLOCK: i32 = 165;
    pub const VK_FORMAT_ASTC_6x6_SRGB_BLOCK: i32 = 166;
    pub const VK_FORMAT_ASTC_8x5_UNORM_BLOCK: i32 = 167;
    pub const VK_FORMAT_ASTC_8x5_SRGB_BLOCK: i32 = 168;
    pub const VK_FORMAT_ASTC_8x6_UNORM_BLOCK: i32 = 169;
    pub const VK_FORMAT_ASTC_8x6_SRGB_BLOCK: i32 = 170;
    pub const VK_FORMAT_ASTC_8x8_UNORM_BLOCK: i32 = 171;
    pub const VK_FORMAT_ASTC_8x8_SRGB_BLOCK: i32 = 172;
    pub const VK_FORMAT_ASTC_10x5_UNORM_BLOCK: i32 = 173;
    pub const VK_FORMAT_ASTC_10x5_SRGB_BLOCK: i32 = 174;
    pub const VK_FORMAT_ASTC_10x6_UNORM_BLOCK: i32 = 175;
    pub const VK_FORMAT_ASTC_10x6_SRGB_BLOCK: i32 = 176;
    pub const VK_FORMAT_ASTC_10x8_UNORM_BLOCK: i32 = 177;
    pub const VK_FORMAT_ASTC_10x8_SRGB_BLOCK: i32 = 178;
    pub const VK_FORMAT_ASTC_10x10_UNORM_BLOCK: i32 = 179;
    pub const VK_FORMAT_ASTC_10x10_SRGB_BLOCK: i32 = 180;
    pub const VK_FORMAT_ASTC_12x10_UNORM_BLOCK: i32 = 181;
    pub const VK_FORMAT_ASTC_12x10_SRGB_BLOCK: i32 = 182;
    pub const VK_FORMAT_ASTC_12x12_UNORM_BLOCK: i32 = 183;
    pub const VK_FORMAT_ASTC_12x12_SRGB_BLOCK: i32 = 184;
}

pub struct VkStructureType(i32); //Structure type enumerant
impl VkStructureType {
    pub const VK_STRUCTURE_TYPE_APPLICATION_INFO: i32 = 0;
    pub const VK_STRUCTURE_TYPE_INSTANCE_CREATE_INFO: i32 = 1;
    pub const VK_STRUCTURE_TYPE_DEVICE_QUEUE_CREATE_INFO: i32 = 2;
    pub const VK_STRUCTURE_TYPE_DEVICE_CREATE_INFO: i32 = 3;
    pub const VK_STRUCTURE_TYPE_SUBMIT_INFO: i32 = 4;
    pub const VK_STRUCTURE_TYPE_MEMORY_ALLOCATE_INFO: i32 = 5;
    pub const VK_STRUCTURE_TYPE_MAPPED_MEMORY_RANGE: i32 = 6;
    pub const VK_STRUCTURE_TYPE_BIND_SPARSE_INFO: i32 = 7;
    pub const VK_STRUCTURE_TYPE_FENCE_CREATE_INFO: i32 = 8;
    pub const VK_STRUCTURE_TYPE_SEMAPHORE_CREATE_INFO: i32 = 9;
    pub const VK_STRUCTURE_TYPE_EVENT_CREATE_INFO: i32 = 10;
    pub const VK_STRUCTURE_TYPE_QUERY_POOL_CREATE_INFO: i32 = 11;
    pub const VK_STRUCTURE_TYPE_BUFFER_CREATE_INFO: i32 = 12;
    pub const VK_STRUCTURE_TYPE_BUFFER_VIEW_CREATE_INFO: i32 = 13;
    pub const VK_STRUCTURE_TYPE_IMAGE_CREATE_INFO: i32 = 14;
    pub const VK_STRUCTURE_TYPE_IMAGE_VIEW_CREATE_INFO: i32 = 15;
    pub const VK_STRUCTURE_TYPE_SHADER_MODULE_CREATE_INFO: i32 = 16;
    pub const VK_STRUCTURE_TYPE_PIPELINE_CACHE_CREATE_INFO: i32 = 17;
    pub const VK_STRUCTURE_TYPE_PIPELINE_SHADER_STAGE_CREATE_INFO: i32 = 18;
    pub const VK_STRUCTURE_TYPE_PIPELINE_VERTEX_INPUT_STATE_CREATE_INFO: i32 = 19;
    pub const VK_STRUCTURE_TYPE_PIPELINE_INPUT_ASSEMBLY_STATE_CREATE_INFO: i32 = 20;
    pub const VK_STRUCTURE_TYPE_PIPELINE_TESSELLATION_STATE_CREATE_INFO: i32 = 21;
    pub const VK_STRUCTURE_TYPE_PIPELINE_VIEWPORT_STATE_CREATE_INFO: i32 = 22;
    pub const VK_STRUCTURE_TYPE_PIPELINE_RASTERIZATION_STATE_CREATE_INFO: i32 = 23;
    pub const VK_STRUCTURE_TYPE_PIPELINE_MULTISAMPLE_STATE_CREATE_INFO: i32 = 24;
    pub const VK_STRUCTURE_TYPE_PIPELINE_DEPTH_STENCIL_STATE_CREATE_INFO: i32 = 25;
    pub const VK_STRUCTURE_TYPE_PIPELINE_COLOR_BLEND_STATE_CREATE_INFO: i32 = 26;
    pub const VK_STRUCTURE_TYPE_PIPELINE_DYNAMIC_STATE_CREATE_INFO: i32 = 27;
    pub const VK_STRUCTURE_TYPE_GRAPHICS_PIPELINE_CREATE_INFO: i32 = 28;
    pub const VK_STRUCTURE_TYPE_COMPUTE_PIPELINE_CREATE_INFO: i32 = 29;
    pub const VK_STRUCTURE_TYPE_PIPELINE_LAYOUT_CREATE_INFO: i32 = 30;
    pub const VK_STRUCTURE_TYPE_SAMPLER_CREATE_INFO: i32 = 31;
    pub const VK_STRUCTURE_TYPE_DESCRIPTOR_SET_LAYOUT_CREATE_INFO: i32 = 32;
    pub const VK_STRUCTURE_TYPE_DESCRIPTOR_POOL_CREATE_INFO: i32 = 33;
    pub const VK_STRUCTURE_TYPE_DESCRIPTOR_SET_ALLOCATE_INFO: i32 = 34;
    pub const VK_STRUCTURE_TYPE_WRITE_DESCRIPTOR_SET: i32 = 35;
    pub const VK_STRUCTURE_TYPE_COPY_DESCRIPTOR_SET: i32 = 36;
    pub const VK_STRUCTURE_TYPE_FRAMEBUFFER_CREATE_INFO: i32 = 37;
    pub const VK_STRUCTURE_TYPE_RENDER_PASS_CREATE_INFO: i32 = 38;
    pub const VK_STRUCTURE_TYPE_COMMAND_POOL_CREATE_INFO: i32 = 39;
    pub const VK_STRUCTURE_TYPE_COMMAND_BUFFER_ALLOCATE_INFO: i32 = 40;
    pub const VK_STRUCTURE_TYPE_COMMAND_BUFFER_INHERITANCE_INFO: i32 = 41;
    pub const VK_STRUCTURE_TYPE_COMMAND_BUFFER_BEGIN_INFO: i32 = 42;
    pub const VK_STRUCTURE_TYPE_RENDER_PASS_BEGIN_INFO: i32 = 43;
    pub const VK_STRUCTURE_TYPE_BUFFER_MEMORY_BARRIER: i32 = 44;
    pub const VK_STRUCTURE_TYPE_IMAGE_MEMORY_BARRIER: i32 = 45;
    pub const VK_STRUCTURE_TYPE_MEMORY_BARRIER: i32 = 46;
    pub const VK_STRUCTURE_TYPE_LOADER_INSTANCE_CREATE_INFO: i32 = 47;
    pub const VK_STRUCTURE_TYPE_LOADER_DEVICE_CREATE_INFO: i32 = 48;
}

pub struct VkSubpassContents(i32); //
impl VkSubpassContents {
    pub const VK_SUBPASS_CONTENTS_INLINE: i32 = 0;
    pub const VK_SUBPASS_CONTENTS_SECONDARY_COMMAND_BUFFERS: i32 = 1;
}

pub struct VkResult(i32); //API result codes
impl VkResult {
    pub const VK_SUCCESS: i32 = 0;
    pub const VK_NOT_READY: i32 = 1;
    pub const VK_TIMEOUT: i32 = 2;
    pub const VK_EVENT_SET: i32 = 3;
    pub const VK_EVENT_RESET: i32 = 4;
    pub const VK_INCOMPLETE: i32 = 5;
    pub const VK_ERROR_OUT_OF_HOST_MEMORY: i32 = -1;
    pub const VK_ERROR_OUT_OF_DEVICE_MEMORY: i32 = -2;
    pub const VK_ERROR_INITIALIZATION_FAILED: i32 = -3;
    pub const VK_ERROR_DEVICE_LOST: i32 = -4;
    pub const VK_ERROR_MEMORY_MAP_FAILED: i32 = -5;
    pub const VK_ERROR_LAYER_NOT_PRESENT: i32 = -6;
    pub const VK_ERROR_EXTENSION_NOT_PRESENT: i32 = -7;
    pub const VK_ERROR_FEATURE_NOT_PRESENT: i32 = -8;
    pub const VK_ERROR_INCOMPATIBLE_DRIVER: i32 = -9;
    pub const VK_ERROR_TOO_MANY_OBJECTS: i32 = -10;
    pub const VK_ERROR_FORMAT_NOT_SUPPORTED: i32 = -11;
    pub const VK_ERROR_FRAGMENTED_POOL: i32 = -12;
    pub const VK_ERROR_UNKNOWN: i32 = -13;
}

pub struct VkDynamicState(i32); //
impl VkDynamicState {
    pub const VK_DYNAMIC_STATE_VIEWPORT: i32 = 0;
    pub const VK_DYNAMIC_STATE_SCISSOR: i32 = 1;
    pub const VK_DYNAMIC_STATE_LINE_WIDTH: i32 = 2;
    pub const VK_DYNAMIC_STATE_DEPTH_BIAS: i32 = 3;
    pub const VK_DYNAMIC_STATE_BLEND_CONSTANTS: i32 = 4;
    pub const VK_DYNAMIC_STATE_DEPTH_BOUNDS: i32 = 5;
    pub const VK_DYNAMIC_STATE_STENCIL_COMPARE_MASK: i32 = 6;
    pub const VK_DYNAMIC_STATE_STENCIL_WRITE_MASK: i32 = 7;
    pub const VK_DYNAMIC_STATE_STENCIL_REFERENCE: i32 = 8;
}

pub struct VkDescriptorUpdateTemplateType(i32); //
impl VkDescriptorUpdateTemplateType {
    pub const VK_DESCRIPTOR_UPDATE_TEMPLATE_TYPE_DESCRIPTOR_SET: i32 = 0;
}

pub struct VkObjectType(i32); //Enums to track objects of various types - also see objtypeenum attributes on type tags
impl VkObjectType {
    pub const VK_OBJECT_TYPE_UNKNOWN: i32 = 0;
    pub const VK_OBJECT_TYPE_INSTANCE: i32 = 1;
    pub const VK_OBJECT_TYPE_PHYSICAL_DEVICE: i32 = 2;
    pub const VK_OBJECT_TYPE_DEVICE: i32 = 3;
    pub const VK_OBJECT_TYPE_QUEUE: i32 = 4;
    pub const VK_OBJECT_TYPE_SEMAPHORE: i32 = 5;
    pub const VK_OBJECT_TYPE_COMMAND_BUFFER: i32 = 6;
    pub const VK_OBJECT_TYPE_FENCE: i32 = 7;
    pub const VK_OBJECT_TYPE_DEVICE_MEMORY: i32 = 8;
    pub const VK_OBJECT_TYPE_BUFFER: i32 = 9;
    pub const VK_OBJECT_TYPE_IMAGE: i32 = 10;
    pub const VK_OBJECT_TYPE_EVENT: i32 = 11;
    pub const VK_OBJECT_TYPE_QUERY_POOL: i32 = 12;
    pub const VK_OBJECT_TYPE_BUFFER_VIEW: i32 = 13;
    pub const VK_OBJECT_TYPE_IMAGE_VIEW: i32 = 14;
    pub const VK_OBJECT_TYPE_SHADER_MODULE: i32 = 15;
    pub const VK_OBJECT_TYPE_PIPELINE_CACHE: i32 = 16;
    pub const VK_OBJECT_TYPE_PIPELINE_LAYOUT: i32 = 17;
    pub const VK_OBJECT_TYPE_RENDER_PASS: i32 = 18;
    pub const VK_OBJECT_TYPE_PIPELINE: i32 = 19;
    pub const VK_OBJECT_TYPE_DESCRIPTOR_SET_LAYOUT: i32 = 20;
    pub const VK_OBJECT_TYPE_SAMPLER: i32 = 21;
    pub const VK_OBJECT_TYPE_DESCRIPTOR_POOL: i32 = 22;
    pub const VK_OBJECT_TYPE_DESCRIPTOR_SET: i32 = 23;
    pub const VK_OBJECT_TYPE_FRAMEBUFFER: i32 = 24;
    pub const VK_OBJECT_TYPE_COMMAND_POOL: i32 = 25;
}

pub struct VkRayTracingInvocationReorderModeEXT(i32); //
impl VkRayTracingInvocationReorderModeEXT {
    pub const VK_RAY_TRACING_INVOCATION_REORDER_MODE_NONE_EXT: i32 = 0;
    pub const VK_RAY_TRACING_INVOCATION_REORDER_MODE_REORDER_EXT: i32 = 1;
}

pub struct VkRayTracingLssIndexingModeNV(i32); //
impl VkRayTracingLssIndexingModeNV {
    pub const VK_RAY_TRACING_LSS_INDEXING_MODE_LIST_NV: i32 = 0;
    pub const VK_RAY_TRACING_LSS_INDEXING_MODE_SUCCESSIVE_NV: i32 = 1;
}

pub struct VkRayTracingLssPrimitiveEndCapsModeNV(i32); //
impl VkRayTracingLssPrimitiveEndCapsModeNV {
    pub const VK_RAY_TRACING_LSS_PRIMITIVE_END_CAPS_MODE_NONE_NV: i32 = 0;
    pub const VK_RAY_TRACING_LSS_PRIMITIVE_END_CAPS_MODE_CHAINED_NV: i32 = 1;
}

pub struct VkDirectDriverLoadingModeLUNARG(i32); //
impl VkDirectDriverLoadingModeLUNARG {
    pub const VK_DIRECT_DRIVER_LOADING_MODE_EXCLUSIVE_LUNARG: i32 = 0;
    pub const VK_DIRECT_DRIVER_LOADING_MODE_INCLUSIVE_LUNARG: i32 = 1;
}

pub struct VkAntiLagModeAMD(i32); //
impl VkAntiLagModeAMD {
    pub const VK_ANTI_LAG_MODE_DRIVER_CONTROL_AMD: i32 = 0;
    pub const VK_ANTI_LAG_MODE_ON_AMD: i32 = 1;
    pub const VK_ANTI_LAG_MODE_OFF_AMD: i32 = 2;
}

pub struct VkAntiLagStageAMD(i32); //
impl VkAntiLagStageAMD {
    pub const VK_ANTI_LAG_STAGE_INPUT_AMD: i32 = 0;
    pub const VK_ANTI_LAG_STAGE_PRESENT_AMD: i32 = 1;
}

pub type VkQueueFlagBits = VkQueueFlags; //
impl VkQueueFlagBits {
    pub const VK_QUEUE_GRAPHICS_BIT: VkQueueFlags = VkQueueFlags(1);
    pub const VK_QUEUE_COMPUTE_BIT: VkQueueFlags = VkQueueFlags(2);
    pub const VK_QUEUE_TRANSFER_BIT: VkQueueFlags = VkQueueFlags(4);
    pub const VK_QUEUE_SPARSE_BINDING_BIT: VkQueueFlags = VkQueueFlags(8);
}

pub type VkCullModeFlagBits = VkCullModeFlags; //
impl VkCullModeFlagBits {
    pub const VK_CULL_MODE_NONE: VkCullModeFlags = VkCullModeFlags(0);
    pub const VK_CULL_MODE_FRONT_BIT: VkCullModeFlags = VkCullModeFlags(1);
    pub const VK_CULL_MODE_BACK_BIT: VkCullModeFlags = VkCullModeFlags(2);
    pub const VK_CULL_MODE_FRONT_AND_BACK: VkCullModeFlags = VkCullModeFlags(0);
}

pub type VkRenderPassCreateFlagBits = VkRenderPassCreateFlags; //
impl VkRenderPassCreateFlagBits {
}

pub type VkDeviceQueueCreateFlagBits = VkDeviceQueueCreateFlags; //
impl VkDeviceQueueCreateFlagBits {
}

pub type VkMemoryPropertyFlagBits = VkMemoryPropertyFlags; //
impl VkMemoryPropertyFlagBits {
    pub const VK_MEMORY_PROPERTY_DEVICE_LOCAL_BIT: VkMemoryPropertyFlags = VkMemoryPropertyFlags(1);
    pub const VK_MEMORY_PROPERTY_HOST_VISIBLE_BIT: VkMemoryPropertyFlags = VkMemoryPropertyFlags(2);
    pub const VK_MEMORY_PROPERTY_HOST_COHERENT_BIT: VkMemoryPropertyFlags = VkMemoryPropertyFlags(4);
    pub const VK_MEMORY_PROPERTY_HOST_CACHED_BIT: VkMemoryPropertyFlags = VkMemoryPropertyFlags(8);
    pub const VK_MEMORY_PROPERTY_LAZILY_ALLOCATED_BIT: VkMemoryPropertyFlags = VkMemoryPropertyFlags(16);
}

pub type VkMemoryHeapFlagBits = VkMemoryHeapFlags; //
impl VkMemoryHeapFlagBits {
    pub const VK_MEMORY_HEAP_DEVICE_LOCAL_BIT: VkMemoryHeapFlags = VkMemoryHeapFlags(1);
}

pub type VkAccessFlagBits = VkAccessFlags; //
impl VkAccessFlagBits {
    pub const VK_ACCESS_INDIRECT_COMMAND_READ_BIT: VkAccessFlags = VkAccessFlags(1);
    pub const VK_ACCESS_INDEX_READ_BIT: VkAccessFlags = VkAccessFlags(2);
    pub const VK_ACCESS_VERTEX_ATTRIBUTE_READ_BIT: VkAccessFlags = VkAccessFlags(4);
    pub const VK_ACCESS_UNIFORM_READ_BIT: VkAccessFlags = VkAccessFlags(8);
    pub const VK_ACCESS_INPUT_ATTACHMENT_READ_BIT: VkAccessFlags = VkAccessFlags(16);
    pub const VK_ACCESS_SHADER_READ_BIT: VkAccessFlags = VkAccessFlags(32);
    pub const VK_ACCESS_SHADER_WRITE_BIT: VkAccessFlags = VkAccessFlags(64);
    pub const VK_ACCESS_COLOR_ATTACHMENT_READ_BIT: VkAccessFlags = VkAccessFlags(128);
    pub const VK_ACCESS_COLOR_ATTACHMENT_WRITE_BIT: VkAccessFlags = VkAccessFlags(256);
    pub const VK_ACCESS_DEPTH_STENCIL_ATTACHMENT_READ_BIT: VkAccessFlags = VkAccessFlags(512);
    pub const VK_ACCESS_DEPTH_STENCIL_ATTACHMENT_WRITE_BIT: VkAccessFlags = VkAccessFlags(1024);
    pub const VK_ACCESS_TRANSFER_READ_BIT: VkAccessFlags = VkAccessFlags(2048);
    pub const VK_ACCESS_TRANSFER_WRITE_BIT: VkAccessFlags = VkAccessFlags(4096);
    pub const VK_ACCESS_HOST_READ_BIT: VkAccessFlags = VkAccessFlags(8192);
    pub const VK_ACCESS_HOST_WRITE_BIT: VkAccessFlags = VkAccessFlags(16384);
    pub const VK_ACCESS_MEMORY_READ_BIT: VkAccessFlags = VkAccessFlags(32768);
    pub const VK_ACCESS_MEMORY_WRITE_BIT: VkAccessFlags = VkAccessFlags(65536);
}

pub type VkBufferUsageFlagBits = VkBufferUsageFlags; //
impl VkBufferUsageFlagBits {
    pub const VK_BUFFER_USAGE_TRANSFER_SRC_BIT: VkBufferUsageFlags = VkBufferUsageFlags(1);
    pub const VK_BUFFER_USAGE_TRANSFER_DST_BIT: VkBufferUsageFlags = VkBufferUsageFlags(2);
    pub const VK_BUFFER_USAGE_UNIFORM_TEXEL_BUFFER_BIT: VkBufferUsageFlags = VkBufferUsageFlags(4);
    pub const VK_BUFFER_USAGE_STORAGE_TEXEL_BUFFER_BIT: VkBufferUsageFlags = VkBufferUsageFlags(8);
    pub const VK_BUFFER_USAGE_UNIFORM_BUFFER_BIT: VkBufferUsageFlags = VkBufferUsageFlags(16);
    pub const VK_BUFFER_USAGE_STORAGE_BUFFER_BIT: VkBufferUsageFlags = VkBufferUsageFlags(32);
    pub const VK_BUFFER_USAGE_INDEX_BUFFER_BIT: VkBufferUsageFlags = VkBufferUsageFlags(64);
    pub const VK_BUFFER_USAGE_VERTEX_BUFFER_BIT: VkBufferUsageFlags = VkBufferUsageFlags(128);
    pub const VK_BUFFER_USAGE_INDIRECT_BUFFER_BIT: VkBufferUsageFlags = VkBufferUsageFlags(256);
}

pub type VkBufferUsageFlagBits2 = VkBufferUsageFlags2; //
impl VkBufferUsageFlagBits2 {
    pub const VK_BUFFER_USAGE_2_TRANSFER_SRC_BIT: VkBufferUsageFlags2 = VkBufferUsageFlags2(1);
    pub const VK_BUFFER_USAGE_2_TRANSFER_DST_BIT: VkBufferUsageFlags2 = VkBufferUsageFlags2(2);
    pub const VK_BUFFER_USAGE_2_UNIFORM_TEXEL_BUFFER_BIT: VkBufferUsageFlags2 = VkBufferUsageFlags2(4);
    pub const VK_BUFFER_USAGE_2_STORAGE_TEXEL_BUFFER_BIT: VkBufferUsageFlags2 = VkBufferUsageFlags2(8);
    pub const VK_BUFFER_USAGE_2_UNIFORM_BUFFER_BIT: VkBufferUsageFlags2 = VkBufferUsageFlags2(16);
    pub const VK_BUFFER_USAGE_2_STORAGE_BUFFER_BIT: VkBufferUsageFlags2 = VkBufferUsageFlags2(32);
    pub const VK_BUFFER_USAGE_2_INDEX_BUFFER_BIT: VkBufferUsageFlags2 = VkBufferUsageFlags2(64);
    pub const VK_BUFFER_USAGE_2_VERTEX_BUFFER_BIT: VkBufferUsageFlags2 = VkBufferUsageFlags2(128);
    pub const VK_BUFFER_USAGE_2_INDIRECT_BUFFER_BIT: VkBufferUsageFlags2 = VkBufferUsageFlags2(256);
}

pub type VkBufferCreateFlagBits = VkBufferCreateFlags; //
impl VkBufferCreateFlagBits {
    pub const VK_BUFFER_CREATE_SPARSE_BINDING_BIT: VkBufferCreateFlags = VkBufferCreateFlags(1);
    pub const VK_BUFFER_CREATE_SPARSE_RESIDENCY_BIT: VkBufferCreateFlags = VkBufferCreateFlags(2);
    pub const VK_BUFFER_CREATE_SPARSE_ALIASED_BIT: VkBufferCreateFlags = VkBufferCreateFlags(4);
}

pub type VkShaderStageFlagBits = VkShaderStageFlags; //
impl VkShaderStageFlagBits {
    pub const VK_SHADER_STAGE_VERTEX_BIT: VkShaderStageFlags = VkShaderStageFlags(1);
    pub const VK_SHADER_STAGE_TESSELLATION_CONTROL_BIT: VkShaderStageFlags = VkShaderStageFlags(2);
    pub const VK_SHADER_STAGE_TESSELLATION_EVALUATION_BIT: VkShaderStageFlags = VkShaderStageFlags(4);
    pub const VK_SHADER_STAGE_GEOMETRY_BIT: VkShaderStageFlags = VkShaderStageFlags(8);
    pub const VK_SHADER_STAGE_FRAGMENT_BIT: VkShaderStageFlags = VkShaderStageFlags(16);
    pub const VK_SHADER_STAGE_COMPUTE_BIT: VkShaderStageFlags = VkShaderStageFlags(32);
    pub const VK_SHADER_STAGE_ALL_GRAPHICS: VkShaderStageFlags = VkShaderStageFlags(0);
    pub const VK_SHADER_STAGE_ALL: VkShaderStageFlags = VkShaderStageFlags(0);
}

pub type VkImageUsageFlagBits = VkImageUsageFlags; //
impl VkImageUsageFlagBits {
    pub const VK_IMAGE_USAGE_TRANSFER_SRC_BIT: VkImageUsageFlags = VkImageUsageFlags(1);
    pub const VK_IMAGE_USAGE_TRANSFER_DST_BIT: VkImageUsageFlags = VkImageUsageFlags(2);
    pub const VK_IMAGE_USAGE_SAMPLED_BIT: VkImageUsageFlags = VkImageUsageFlags(4);
    pub const VK_IMAGE_USAGE_STORAGE_BIT: VkImageUsageFlags = VkImageUsageFlags(8);
    pub const VK_IMAGE_USAGE_COLOR_ATTACHMENT_BIT: VkImageUsageFlags = VkImageUsageFlags(16);
    pub const VK_IMAGE_USAGE_DEPTH_STENCIL_ATTACHMENT_BIT: VkImageUsageFlags = VkImageUsageFlags(32);
    pub const VK_IMAGE_USAGE_TRANSIENT_ATTACHMENT_BIT: VkImageUsageFlags = VkImageUsageFlags(64);
    pub const VK_IMAGE_USAGE_INPUT_ATTACHMENT_BIT: VkImageUsageFlags = VkImageUsageFlags(128);
}

pub type VkImageUsageFlagBits2KHR = VkImageUsageFlags2KHR; //
impl VkImageUsageFlagBits2KHR {
    pub const VK_IMAGE_USAGE_2_TRANSFER_SRC_BIT_KHR: VkImageUsageFlags2KHR = VkImageUsageFlags2KHR(1);
    pub const VK_IMAGE_USAGE_2_TRANSFER_DST_BIT_KHR: VkImageUsageFlags2KHR = VkImageUsageFlags2KHR(2);
    pub const VK_IMAGE_USAGE_2_SAMPLED_BIT_KHR: VkImageUsageFlags2KHR = VkImageUsageFlags2KHR(4);
    pub const VK_IMAGE_USAGE_2_STORAGE_BIT_KHR: VkImageUsageFlags2KHR = VkImageUsageFlags2KHR(8);
    pub const VK_IMAGE_USAGE_2_COLOR_ATTACHMENT_BIT_KHR: VkImageUsageFlags2KHR = VkImageUsageFlags2KHR(16);
    pub const VK_IMAGE_USAGE_2_DEPTH_STENCIL_ATTACHMENT_BIT_KHR: VkImageUsageFlags2KHR = VkImageUsageFlags2KHR(32);
    pub const VK_IMAGE_USAGE_2_TRANSIENT_ATTACHMENT_BIT_KHR: VkImageUsageFlags2KHR = VkImageUsageFlags2KHR(64);
    pub const VK_IMAGE_USAGE_2_INPUT_ATTACHMENT_BIT_KHR: VkImageUsageFlags2KHR = VkImageUsageFlags2KHR(128);
}

pub type VkImageCreateFlagBits = VkImageCreateFlags; //
impl VkImageCreateFlagBits {
    pub const VK_IMAGE_CREATE_SPARSE_BINDING_BIT: VkImageCreateFlags = VkImageCreateFlags(1);
    pub const VK_IMAGE_CREATE_SPARSE_RESIDENCY_BIT: VkImageCreateFlags = VkImageCreateFlags(2);
    pub const VK_IMAGE_CREATE_SPARSE_ALIASED_BIT: VkImageCreateFlags = VkImageCreateFlags(4);
    pub const VK_IMAGE_CREATE_MUTABLE_FORMAT_BIT: VkImageCreateFlags = VkImageCreateFlags(8);
    pub const VK_IMAGE_CREATE_CUBE_COMPATIBLE_BIT: VkImageCreateFlags = VkImageCreateFlags(16);
}

pub type VkImageCreateFlagBits2KHR = VkImageCreateFlags2KHR; //
impl VkImageCreateFlagBits2KHR {
    pub const VK_IMAGE_CREATE_2_SPARSE_BINDING_BIT_KHR: VkImageCreateFlags2KHR = VkImageCreateFlags2KHR(1);
    pub const VK_IMAGE_CREATE_2_SPARSE_RESIDENCY_BIT_KHR: VkImageCreateFlags2KHR = VkImageCreateFlags2KHR(2);
    pub const VK_IMAGE_CREATE_2_SPARSE_ALIASED_BIT_KHR: VkImageCreateFlags2KHR = VkImageCreateFlags2KHR(4);
    pub const VK_IMAGE_CREATE_2_MUTABLE_FORMAT_BIT_KHR: VkImageCreateFlags2KHR = VkImageCreateFlags2KHR(8);
    pub const VK_IMAGE_CREATE_2_CUBE_COMPATIBLE_BIT_KHR: VkImageCreateFlags2KHR = VkImageCreateFlags2KHR(16);
}

pub type VkImageViewCreateFlagBits = VkImageViewCreateFlags; //
impl VkImageViewCreateFlagBits {
}

pub type VkSamplerCreateFlagBits = VkSamplerCreateFlags; //
impl VkSamplerCreateFlagBits {
}

pub type VkPipelineCreateFlagBits = VkPipelineCreateFlags; //
impl VkPipelineCreateFlagBits {
    pub const VK_PIPELINE_CREATE_DISABLE_OPTIMIZATION_BIT: VkPipelineCreateFlags = VkPipelineCreateFlags(1);
    pub const VK_PIPELINE_CREATE_ALLOW_DERIVATIVES_BIT: VkPipelineCreateFlags = VkPipelineCreateFlags(2);
    pub const VK_PIPELINE_CREATE_DERIVATIVE_BIT: VkPipelineCreateFlags = VkPipelineCreateFlags(4);
}

pub type VkPipelineCreateFlagBits2 = VkPipelineCreateFlags2; //
impl VkPipelineCreateFlagBits2 {
    pub const VK_PIPELINE_CREATE_2_DISABLE_OPTIMIZATION_BIT: VkPipelineCreateFlags2 = VkPipelineCreateFlags2(1);
    pub const VK_PIPELINE_CREATE_2_ALLOW_DERIVATIVES_BIT: VkPipelineCreateFlags2 = VkPipelineCreateFlags2(2);
    pub const VK_PIPELINE_CREATE_2_DERIVATIVE_BIT: VkPipelineCreateFlags2 = VkPipelineCreateFlags2(4);
    pub const VK_PIPELINE_CREATE_2_VIEW_INDEX_FROM_DEVICE_INDEX_BIT: VkPipelineCreateFlags2 = VkPipelineCreateFlags2(8);
    pub const VK_PIPELINE_CREATE_2_DISPATCH_BASE_BIT: VkPipelineCreateFlags2 = VkPipelineCreateFlags2(16);
    pub const VK_PIPELINE_CREATE_2_FAIL_ON_PIPELINE_COMPILE_REQUIRED_BIT: VkPipelineCreateFlags2 = VkPipelineCreateFlags2(256);
    pub const VK_PIPELINE_CREATE_2_EARLY_RETURN_ON_FAILURE_BIT: VkPipelineCreateFlags2 = VkPipelineCreateFlags2(512);
    pub const VK_PIPELINE_CREATE_2_NO_PROTECTED_ACCESS_BIT: VkPipelineCreateFlags2 = VkPipelineCreateFlags2(134217728);
    pub const VK_PIPELINE_CREATE_2_PROTECTED_ACCESS_ONLY_BIT: VkPipelineCreateFlags2 = VkPipelineCreateFlags2(1073741824);
}

pub type VkPipelineShaderStageCreateFlagBits = VkPipelineShaderStageCreateFlags; //
impl VkPipelineShaderStageCreateFlagBits {
}

pub type VkColorComponentFlagBits = VkColorComponentFlags; //
impl VkColorComponentFlagBits {
    pub const VK_COLOR_COMPONENT_R_BIT: VkColorComponentFlags = VkColorComponentFlags(1);
    pub const VK_COLOR_COMPONENT_G_BIT: VkColorComponentFlags = VkColorComponentFlags(2);
    pub const VK_COLOR_COMPONENT_B_BIT: VkColorComponentFlags = VkColorComponentFlags(4);
    pub const VK_COLOR_COMPONENT_A_BIT: VkColorComponentFlags = VkColorComponentFlags(8);
}

pub type VkFenceCreateFlagBits = VkFenceCreateFlags; //
impl VkFenceCreateFlagBits {
    pub const VK_FENCE_CREATE_SIGNALED_BIT: VkFenceCreateFlags = VkFenceCreateFlags(1);
}

pub type VkSemaphoreCreateFlagBits = VkSemaphoreCreateFlags; //
impl VkSemaphoreCreateFlagBits {
}

pub type VkFormatFeatureFlagBits = VkFormatFeatureFlags; //
impl VkFormatFeatureFlagBits {
    pub const VK_FORMAT_FEATURE_SAMPLED_IMAGE_BIT: VkFormatFeatureFlags = VkFormatFeatureFlags(1);
    pub const VK_FORMAT_FEATURE_STORAGE_IMAGE_BIT: VkFormatFeatureFlags = VkFormatFeatureFlags(2);
    pub const VK_FORMAT_FEATURE_STORAGE_IMAGE_ATOMIC_BIT: VkFormatFeatureFlags = VkFormatFeatureFlags(4);
    pub const VK_FORMAT_FEATURE_UNIFORM_TEXEL_BUFFER_BIT: VkFormatFeatureFlags = VkFormatFeatureFlags(8);
    pub const VK_FORMAT_FEATURE_STORAGE_TEXEL_BUFFER_BIT: VkFormatFeatureFlags = VkFormatFeatureFlags(16);
    pub const VK_FORMAT_FEATURE_STORAGE_TEXEL_BUFFER_ATOMIC_BIT: VkFormatFeatureFlags = VkFormatFeatureFlags(32);
    pub const VK_FORMAT_FEATURE_VERTEX_BUFFER_BIT: VkFormatFeatureFlags = VkFormatFeatureFlags(64);
    pub const VK_FORMAT_FEATURE_COLOR_ATTACHMENT_BIT: VkFormatFeatureFlags = VkFormatFeatureFlags(128);
    pub const VK_FORMAT_FEATURE_COLOR_ATTACHMENT_BLEND_BIT: VkFormatFeatureFlags = VkFormatFeatureFlags(256);
    pub const VK_FORMAT_FEATURE_DEPTH_STENCIL_ATTACHMENT_BIT: VkFormatFeatureFlags = VkFormatFeatureFlags(512);
    pub const VK_FORMAT_FEATURE_BLIT_SRC_BIT: VkFormatFeatureFlags = VkFormatFeatureFlags(1024);
    pub const VK_FORMAT_FEATURE_BLIT_DST_BIT: VkFormatFeatureFlags = VkFormatFeatureFlags(2048);
    pub const VK_FORMAT_FEATURE_SAMPLED_IMAGE_FILTER_LINEAR_BIT: VkFormatFeatureFlags = VkFormatFeatureFlags(4096);
}

pub type VkQueryControlFlagBits = VkQueryControlFlags; //
impl VkQueryControlFlagBits {
    pub const VK_QUERY_CONTROL_PRECISE_BIT: VkQueryControlFlags = VkQueryControlFlags(1);
}

pub type VkQueryResultFlagBits = VkQueryResultFlags; //
impl VkQueryResultFlagBits {
    pub const VK_QUERY_RESULT_64_BIT: VkQueryResultFlags = VkQueryResultFlags(1);
    pub const VK_QUERY_RESULT_WAIT_BIT: VkQueryResultFlags = VkQueryResultFlags(2);
    pub const VK_QUERY_RESULT_WITH_AVAILABILITY_BIT: VkQueryResultFlags = VkQueryResultFlags(4);
    pub const VK_QUERY_RESULT_PARTIAL_BIT: VkQueryResultFlags = VkQueryResultFlags(8);
}

pub type VkCommandBufferUsageFlagBits = VkCommandBufferUsageFlags; //
impl VkCommandBufferUsageFlagBits {
    pub const VK_COMMAND_BUFFER_USAGE_ONE_TIME_SUBMIT_BIT: VkCommandBufferUsageFlags = VkCommandBufferUsageFlags(1);
    pub const VK_COMMAND_BUFFER_USAGE_RENDER_PASS_CONTINUE_BIT: VkCommandBufferUsageFlags = VkCommandBufferUsageFlags(2);
    pub const VK_COMMAND_BUFFER_USAGE_SIMULTANEOUS_USE_BIT: VkCommandBufferUsageFlags = VkCommandBufferUsageFlags(4);
}

pub type VkQueryPipelineStatisticFlagBits = VkQueryPipelineStatisticFlags; //
impl VkQueryPipelineStatisticFlagBits {
    pub const VK_QUERY_PIPELINE_STATISTIC_INPUT_ASSEMBLY_VERTICES_BIT: VkQueryPipelineStatisticFlags = VkQueryPipelineStatisticFlags(1);
    pub const VK_QUERY_PIPELINE_STATISTIC_INPUT_ASSEMBLY_PRIMITIVES_BIT: VkQueryPipelineStatisticFlags = VkQueryPipelineStatisticFlags(2);
    pub const VK_QUERY_PIPELINE_STATISTIC_VERTEX_SHADER_INVOCATIONS_BIT: VkQueryPipelineStatisticFlags = VkQueryPipelineStatisticFlags(4);
    pub const VK_QUERY_PIPELINE_STATISTIC_GEOMETRY_SHADER_INVOCATIONS_BIT: VkQueryPipelineStatisticFlags = VkQueryPipelineStatisticFlags(8);
    pub const VK_QUERY_PIPELINE_STATISTIC_GEOMETRY_SHADER_PRIMITIVES_BIT: VkQueryPipelineStatisticFlags = VkQueryPipelineStatisticFlags(16);
    pub const VK_QUERY_PIPELINE_STATISTIC_CLIPPING_INVOCATIONS_BIT: VkQueryPipelineStatisticFlags = VkQueryPipelineStatisticFlags(32);
    pub const VK_QUERY_PIPELINE_STATISTIC_CLIPPING_PRIMITIVES_BIT: VkQueryPipelineStatisticFlags = VkQueryPipelineStatisticFlags(64);
    pub const VK_QUERY_PIPELINE_STATISTIC_FRAGMENT_SHADER_INVOCATIONS_BIT: VkQueryPipelineStatisticFlags = VkQueryPipelineStatisticFlags(128);
    pub const VK_QUERY_PIPELINE_STATISTIC_TESSELLATION_CONTROL_SHADER_PATCHES_BIT: VkQueryPipelineStatisticFlags = VkQueryPipelineStatisticFlags(256);
    pub const VK_QUERY_PIPELINE_STATISTIC_TESSELLATION_EVALUATION_SHADER_INVOCATIONS_BIT: VkQueryPipelineStatisticFlags = VkQueryPipelineStatisticFlags(512);
    pub const VK_QUERY_PIPELINE_STATISTIC_COMPUTE_SHADER_INVOCATIONS_BIT: VkQueryPipelineStatisticFlags = VkQueryPipelineStatisticFlags(1024);
}

pub type VkMemoryMapFlagBits = VkMemoryMapFlags; //
impl VkMemoryMapFlagBits {
}

pub type VkImageAspectFlagBits = VkImageAspectFlags; //
impl VkImageAspectFlagBits {
    pub const VK_IMAGE_ASPECT_COLOR_BIT: VkImageAspectFlags = VkImageAspectFlags(1);
    pub const VK_IMAGE_ASPECT_DEPTH_BIT: VkImageAspectFlags = VkImageAspectFlags(2);
    pub const VK_IMAGE_ASPECT_STENCIL_BIT: VkImageAspectFlags = VkImageAspectFlags(4);
    pub const VK_IMAGE_ASPECT_METADATA_BIT: VkImageAspectFlags = VkImageAspectFlags(8);
}

pub type VkSparseImageFormatFlagBits = VkSparseImageFormatFlags; //
impl VkSparseImageFormatFlagBits {
    pub const VK_SPARSE_IMAGE_FORMAT_SINGLE_MIPTAIL_BIT: VkSparseImageFormatFlags = VkSparseImageFormatFlags(1);
    pub const VK_SPARSE_IMAGE_FORMAT_ALIGNED_MIP_SIZE_BIT: VkSparseImageFormatFlags = VkSparseImageFormatFlags(2);
    pub const VK_SPARSE_IMAGE_FORMAT_NONSTANDARD_BLOCK_SIZE_BIT: VkSparseImageFormatFlags = VkSparseImageFormatFlags(4);
}

pub type VkSparseMemoryBindFlagBits = VkSparseMemoryBindFlags; //
impl VkSparseMemoryBindFlagBits {
    pub const VK_SPARSE_MEMORY_BIND_METADATA_BIT: VkSparseMemoryBindFlags = VkSparseMemoryBindFlags(1);
}

pub type VkPipelineStageFlagBits = VkPipelineStageFlags; //
impl VkPipelineStageFlagBits {
    pub const VK_PIPELINE_STAGE_TOP_OF_PIPE_BIT: VkPipelineStageFlags = VkPipelineStageFlags(1);
    pub const VK_PIPELINE_STAGE_DRAW_INDIRECT_BIT: VkPipelineStageFlags = VkPipelineStageFlags(2);
    pub const VK_PIPELINE_STAGE_VERTEX_INPUT_BIT: VkPipelineStageFlags = VkPipelineStageFlags(4);
    pub const VK_PIPELINE_STAGE_VERTEX_SHADER_BIT: VkPipelineStageFlags = VkPipelineStageFlags(8);
    pub const VK_PIPELINE_STAGE_TESSELLATION_CONTROL_SHADER_BIT: VkPipelineStageFlags = VkPipelineStageFlags(16);
    pub const VK_PIPELINE_STAGE_TESSELLATION_EVALUATION_SHADER_BIT: VkPipelineStageFlags = VkPipelineStageFlags(32);
    pub const VK_PIPELINE_STAGE_GEOMETRY_SHADER_BIT: VkPipelineStageFlags = VkPipelineStageFlags(64);
    pub const VK_PIPELINE_STAGE_FRAGMENT_SHADER_BIT: VkPipelineStageFlags = VkPipelineStageFlags(128);
    pub const VK_PIPELINE_STAGE_EARLY_FRAGMENT_TESTS_BIT: VkPipelineStageFlags = VkPipelineStageFlags(256);
    pub const VK_PIPELINE_STAGE_LATE_FRAGMENT_TESTS_BIT: VkPipelineStageFlags = VkPipelineStageFlags(512);
    pub const VK_PIPELINE_STAGE_COLOR_ATTACHMENT_OUTPUT_BIT: VkPipelineStageFlags = VkPipelineStageFlags(1024);
    pub const VK_PIPELINE_STAGE_COMPUTE_SHADER_BIT: VkPipelineStageFlags = VkPipelineStageFlags(2048);
    pub const VK_PIPELINE_STAGE_TRANSFER_BIT: VkPipelineStageFlags = VkPipelineStageFlags(4096);
    pub const VK_PIPELINE_STAGE_BOTTOM_OF_PIPE_BIT: VkPipelineStageFlags = VkPipelineStageFlags(8192);
    pub const VK_PIPELINE_STAGE_HOST_BIT: VkPipelineStageFlags = VkPipelineStageFlags(16384);
    pub const VK_PIPELINE_STAGE_ALL_GRAPHICS_BIT: VkPipelineStageFlags = VkPipelineStageFlags(32768);
    pub const VK_PIPELINE_STAGE_ALL_COMMANDS_BIT: VkPipelineStageFlags = VkPipelineStageFlags(65536);
}

pub type VkCommandPoolCreateFlagBits = VkCommandPoolCreateFlags; //
impl VkCommandPoolCreateFlagBits {
    pub const VK_COMMAND_POOL_CREATE_TRANSIENT_BIT: VkCommandPoolCreateFlags = VkCommandPoolCreateFlags(1);
    pub const VK_COMMAND_POOL_CREATE_RESET_COMMAND_BUFFER_BIT: VkCommandPoolCreateFlags = VkCommandPoolCreateFlags(2);
}

pub type VkCommandPoolResetFlagBits = VkCommandPoolResetFlags; //
impl VkCommandPoolResetFlagBits {
    pub const VK_COMMAND_POOL_RESET_RELEASE_RESOURCES_BIT: VkCommandPoolResetFlags = VkCommandPoolResetFlags(1);
}

pub type VkCommandBufferResetFlagBits = VkCommandBufferResetFlags; //
impl VkCommandBufferResetFlagBits {
    pub const VK_COMMAND_BUFFER_RESET_RELEASE_RESOURCES_BIT: VkCommandBufferResetFlags = VkCommandBufferResetFlags(1);
}

pub type VkSampleCountFlagBits = VkSampleCountFlags; //
impl VkSampleCountFlagBits {
    pub const VK_SAMPLE_COUNT_1_BIT: VkSampleCountFlags = VkSampleCountFlags(1);
    pub const VK_SAMPLE_COUNT_2_BIT: VkSampleCountFlags = VkSampleCountFlags(2);
    pub const VK_SAMPLE_COUNT_4_BIT: VkSampleCountFlags = VkSampleCountFlags(4);
    pub const VK_SAMPLE_COUNT_8_BIT: VkSampleCountFlags = VkSampleCountFlags(8);
    pub const VK_SAMPLE_COUNT_16_BIT: VkSampleCountFlags = VkSampleCountFlags(16);
    pub const VK_SAMPLE_COUNT_32_BIT: VkSampleCountFlags = VkSampleCountFlags(32);
    pub const VK_SAMPLE_COUNT_64_BIT: VkSampleCountFlags = VkSampleCountFlags(64);
}

pub type VkAttachmentDescriptionFlagBits = VkAttachmentDescriptionFlags; //
impl VkAttachmentDescriptionFlagBits {
    pub const VK_ATTACHMENT_DESCRIPTION_MAY_ALIAS_BIT: VkAttachmentDescriptionFlags = VkAttachmentDescriptionFlags(1);
}

pub type VkStencilFaceFlagBits = VkStencilFaceFlags; //
impl VkStencilFaceFlagBits {
    pub const VK_STENCIL_FACE_FRONT_BIT: VkStencilFaceFlags = VkStencilFaceFlags(1);
    pub const VK_STENCIL_FACE_BACK_BIT: VkStencilFaceFlags = VkStencilFaceFlags(2);
    pub const VK_STENCIL_FACE_FRONT_AND_BACK: VkStencilFaceFlags = VkStencilFaceFlags(0);
    pub const VK_STENCIL_FRONT_AND_BACK: VkStencilFaceFlags = Self::VK_STENCIL_FACE_FRONT_AND_BACK;
}

pub type VkDescriptorPoolCreateFlagBits = VkDescriptorPoolCreateFlags; //
impl VkDescriptorPoolCreateFlagBits {
    pub const VK_DESCRIPTOR_POOL_CREATE_FREE_DESCRIPTOR_SET_BIT: VkDescriptorPoolCreateFlags = VkDescriptorPoolCreateFlags(1);
}

pub type VkDependencyFlagBits = VkDependencyFlags; //
impl VkDependencyFlagBits {
    pub const VK_DEPENDENCY_BY_REGION_BIT: VkDependencyFlags = VkDependencyFlags(1);
}

pub struct VkSemaphoreType(i32); //
impl VkSemaphoreType {
    pub const VK_SEMAPHORE_TYPE_BINARY: i32 = 0;
    pub const VK_SEMAPHORE_TYPE_TIMELINE: i32 = 1;
}

pub type VkSemaphoreWaitFlagBits = VkSemaphoreWaitFlags; //
impl VkSemaphoreWaitFlagBits {
    pub const VK_SEMAPHORE_WAIT_ANY_BIT: VkSemaphoreWaitFlags = VkSemaphoreWaitFlags(1);
}

pub struct VkPresentModeKHR(i32); //
impl VkPresentModeKHR {
    pub const VK_PRESENT_MODE_IMMEDIATE_KHR: i32 = 0;
    pub const VK_PRESENT_MODE_MAILBOX_KHR: i32 = 1;
    pub const VK_PRESENT_MODE_FIFO_KHR: i32 = 2;
    pub const VK_PRESENT_MODE_FIFO_RELAXED_KHR: i32 = 3;
}

pub struct VkColorSpaceKHR(i32); //
impl VkColorSpaceKHR {
    pub const VK_COLOR_SPACE_SRGB_NONLINEAR_KHR: i32 = 0;
    pub const VK_COLORSPACE_SRGB_NONLINEAR_KHR: i32 = Self::VK_COLOR_SPACE_SRGB_NONLINEAR_KHR;
}

pub type VkDisplayPlaneAlphaFlagBitsKHR = VkDisplayPlaneAlphaFlagsKHR; //
impl VkDisplayPlaneAlphaFlagBitsKHR {
    pub const VK_DISPLAY_PLANE_ALPHA_OPAQUE_BIT_KHR: VkDisplayPlaneAlphaFlagsKHR = VkDisplayPlaneAlphaFlagsKHR(1);
    pub const VK_DISPLAY_PLANE_ALPHA_GLOBAL_BIT_KHR: VkDisplayPlaneAlphaFlagsKHR = VkDisplayPlaneAlphaFlagsKHR(2);
    pub const VK_DISPLAY_PLANE_ALPHA_PER_PIXEL_BIT_KHR: VkDisplayPlaneAlphaFlagsKHR = VkDisplayPlaneAlphaFlagsKHR(4);
    pub const VK_DISPLAY_PLANE_ALPHA_PER_PIXEL_PREMULTIPLIED_BIT_KHR: VkDisplayPlaneAlphaFlagsKHR = VkDisplayPlaneAlphaFlagsKHR(8);
}

pub type VkCompositeAlphaFlagBitsKHR = VkCompositeAlphaFlagsKHR; //
impl VkCompositeAlphaFlagBitsKHR {
    pub const VK_COMPOSITE_ALPHA_OPAQUE_BIT_KHR: VkCompositeAlphaFlagsKHR = VkCompositeAlphaFlagsKHR(1);
    pub const VK_COMPOSITE_ALPHA_PRE_MULTIPLIED_BIT_KHR: VkCompositeAlphaFlagsKHR = VkCompositeAlphaFlagsKHR(2);
    pub const VK_COMPOSITE_ALPHA_POST_MULTIPLIED_BIT_KHR: VkCompositeAlphaFlagsKHR = VkCompositeAlphaFlagsKHR(4);
    pub const VK_COMPOSITE_ALPHA_INHERIT_BIT_KHR: VkCompositeAlphaFlagsKHR = VkCompositeAlphaFlagsKHR(8);
}

pub type VkSurfaceTransformFlagBitsKHR = VkSurfaceTransformFlagsKHR; //
impl VkSurfaceTransformFlagBitsKHR {
    pub const VK_SURFACE_TRANSFORM_IDENTITY_BIT_KHR: VkSurfaceTransformFlagsKHR = VkSurfaceTransformFlagsKHR(1);
    pub const VK_SURFACE_TRANSFORM_ROTATE_90_BIT_KHR: VkSurfaceTransformFlagsKHR = VkSurfaceTransformFlagsKHR(2);
    pub const VK_SURFACE_TRANSFORM_ROTATE_180_BIT_KHR: VkSurfaceTransformFlagsKHR = VkSurfaceTransformFlagsKHR(4);
    pub const VK_SURFACE_TRANSFORM_ROTATE_270_BIT_KHR: VkSurfaceTransformFlagsKHR = VkSurfaceTransformFlagsKHR(8);
    pub const VK_SURFACE_TRANSFORM_HORIZONTAL_MIRROR_BIT_KHR: VkSurfaceTransformFlagsKHR = VkSurfaceTransformFlagsKHR(16);
    pub const VK_SURFACE_TRANSFORM_HORIZONTAL_MIRROR_ROTATE_90_BIT_KHR: VkSurfaceTransformFlagsKHR = VkSurfaceTransformFlagsKHR(32);
    pub const VK_SURFACE_TRANSFORM_HORIZONTAL_MIRROR_ROTATE_180_BIT_KHR: VkSurfaceTransformFlagsKHR = VkSurfaceTransformFlagsKHR(64);
    pub const VK_SURFACE_TRANSFORM_HORIZONTAL_MIRROR_ROTATE_270_BIT_KHR: VkSurfaceTransformFlagsKHR = VkSurfaceTransformFlagsKHR(128);
    pub const VK_SURFACE_TRANSFORM_INHERIT_BIT_KHR: VkSurfaceTransformFlagsKHR = VkSurfaceTransformFlagsKHR(256);
}

pub struct VkDisplaySurfaceStereoTypeNV(i32); //
impl VkDisplaySurfaceStereoTypeNV {
    pub const VK_DISPLAY_SURFACE_STEREO_TYPE_NONE_NV: i32 = 0;
    pub const VK_DISPLAY_SURFACE_STEREO_TYPE_ONBOARD_DIN_NV: i32 = 1;
    pub const VK_DISPLAY_SURFACE_STEREO_TYPE_HDMI_3D_NV: i32 = 2;
    pub const VK_DISPLAY_SURFACE_STEREO_TYPE_INBAND_DISPLAYPORT_NV: i32 = 3;
}

pub type VkSwapchainImageUsageFlagBitsANDROID = VkSwapchainImageUsageFlagsANDROID; //
impl VkSwapchainImageUsageFlagBitsANDROID {
    pub const VK_SWAPCHAIN_IMAGE_USAGE_SHARED_BIT_ANDROID: VkSwapchainImageUsageFlagsANDROID = VkSwapchainImageUsageFlagsANDROID(1);
}

pub struct VkTimeDomainKHR(i32); //
impl VkTimeDomainKHR {
    pub const VK_TIME_DOMAIN_DEVICE_KHR: i32 = 0;
    pub const VK_TIME_DOMAIN_CLOCK_MONOTONIC_KHR: i32 = 1;
    pub const VK_TIME_DOMAIN_CLOCK_MONOTONIC_RAW_KHR: i32 = 2;
    pub const VK_TIME_DOMAIN_QUERY_PERFORMANCE_COUNTER_KHR: i32 = 3;
}

pub type VkDebugReportFlagBitsEXT = VkDebugReportFlagsEXT; //
impl VkDebugReportFlagBitsEXT {
    pub const VK_DEBUG_REPORT_INFORMATION_BIT_EXT: VkDebugReportFlagsEXT = VkDebugReportFlagsEXT(1);
    pub const VK_DEBUG_REPORT_WARNING_BIT_EXT: VkDebugReportFlagsEXT = VkDebugReportFlagsEXT(2);
    pub const VK_DEBUG_REPORT_PERFORMANCE_WARNING_BIT_EXT: VkDebugReportFlagsEXT = VkDebugReportFlagsEXT(4);
    pub const VK_DEBUG_REPORT_ERROR_BIT_EXT: VkDebugReportFlagsEXT = VkDebugReportFlagsEXT(8);
    pub const VK_DEBUG_REPORT_DEBUG_BIT_EXT: VkDebugReportFlagsEXT = VkDebugReportFlagsEXT(16);
}

pub struct VkDebugReportObjectTypeEXT(i32); //
impl VkDebugReportObjectTypeEXT {
    pub const VK_DEBUG_REPORT_OBJECT_TYPE_UNKNOWN_EXT: i32 = 0;
    pub const VK_DEBUG_REPORT_OBJECT_TYPE_INSTANCE_EXT: i32 = 1;
    pub const VK_DEBUG_REPORT_OBJECT_TYPE_PHYSICAL_DEVICE_EXT: i32 = 2;
    pub const VK_DEBUG_REPORT_OBJECT_TYPE_DEVICE_EXT: i32 = 3;
    pub const VK_DEBUG_REPORT_OBJECT_TYPE_QUEUE_EXT: i32 = 4;
    pub const VK_DEBUG_REPORT_OBJECT_TYPE_SEMAPHORE_EXT: i32 = 5;
    pub const VK_DEBUG_REPORT_OBJECT_TYPE_COMMAND_BUFFER_EXT: i32 = 6;
    pub const VK_DEBUG_REPORT_OBJECT_TYPE_FENCE_EXT: i32 = 7;
    pub const VK_DEBUG_REPORT_OBJECT_TYPE_DEVICE_MEMORY_EXT: i32 = 8;
    pub const VK_DEBUG_REPORT_OBJECT_TYPE_BUFFER_EXT: i32 = 9;
    pub const VK_DEBUG_REPORT_OBJECT_TYPE_IMAGE_EXT: i32 = 10;
    pub const VK_DEBUG_REPORT_OBJECT_TYPE_EVENT_EXT: i32 = 11;
    pub const VK_DEBUG_REPORT_OBJECT_TYPE_QUERY_POOL_EXT: i32 = 12;
    pub const VK_DEBUG_REPORT_OBJECT_TYPE_BUFFER_VIEW_EXT: i32 = 13;
    pub const VK_DEBUG_REPORT_OBJECT_TYPE_IMAGE_VIEW_EXT: i32 = 14;
    pub const VK_DEBUG_REPORT_OBJECT_TYPE_SHADER_MODULE_EXT: i32 = 15;
    pub const VK_DEBUG_REPORT_OBJECT_TYPE_PIPELINE_CACHE_EXT: i32 = 16;
    pub const VK_DEBUG_REPORT_OBJECT_TYPE_PIPELINE_LAYOUT_EXT: i32 = 17;
    pub const VK_DEBUG_REPORT_OBJECT_TYPE_RENDER_PASS_EXT: i32 = 18;
    pub const VK_DEBUG_REPORT_OBJECT_TYPE_PIPELINE_EXT: i32 = 19;
    pub const VK_DEBUG_REPORT_OBJECT_TYPE_DESCRIPTOR_SET_LAYOUT_EXT: i32 = 20;
    pub const VK_DEBUG_REPORT_OBJECT_TYPE_SAMPLER_EXT: i32 = 21;
    pub const VK_DEBUG_REPORT_OBJECT_TYPE_DESCRIPTOR_POOL_EXT: i32 = 22;
    pub const VK_DEBUG_REPORT_OBJECT_TYPE_DESCRIPTOR_SET_EXT: i32 = 23;
    pub const VK_DEBUG_REPORT_OBJECT_TYPE_FRAMEBUFFER_EXT: i32 = 24;
    pub const VK_DEBUG_REPORT_OBJECT_TYPE_COMMAND_POOL_EXT: i32 = 25;
    pub const VK_DEBUG_REPORT_OBJECT_TYPE_SURFACE_KHR_EXT: i32 = 26;
    pub const VK_DEBUG_REPORT_OBJECT_TYPE_SWAPCHAIN_KHR_EXT: i32 = 27;
    pub const VK_DEBUG_REPORT_OBJECT_TYPE_DEBUG_REPORT_CALLBACK_EXT_EXT: i32 = 28;
    pub const VK_DEBUG_REPORT_OBJECT_TYPE_DEBUG_REPORT_EXT: i32 = Self::VK_DEBUG_REPORT_OBJECT_TYPE_DEBUG_REPORT_CALLBACK_EXT_EXT;
    pub const VK_DEBUG_REPORT_OBJECT_TYPE_DISPLAY_KHR_EXT: i32 = 29;
    pub const VK_DEBUG_REPORT_OBJECT_TYPE_DISPLAY_MODE_KHR_EXT: i32 = 30;
    pub const VK_DEBUG_REPORT_OBJECT_TYPE_VALIDATION_CACHE_EXT_EXT: i32 = 33;
    pub const VK_DEBUG_REPORT_OBJECT_TYPE_VALIDATION_CACHE_EXT: i32 = Self::VK_DEBUG_REPORT_OBJECT_TYPE_VALIDATION_CACHE_EXT_EXT;
}

pub struct VkDeviceMemoryReportEventTypeEXT(i32); //
impl VkDeviceMemoryReportEventTypeEXT {
    pub const VK_DEVICE_MEMORY_REPORT_EVENT_TYPE_ALLOCATE_EXT: i32 = 0;
    pub const VK_DEVICE_MEMORY_REPORT_EVENT_TYPE_FREE_EXT: i32 = 1;
    pub const VK_DEVICE_MEMORY_REPORT_EVENT_TYPE_IMPORT_EXT: i32 = 2;
    pub const VK_DEVICE_MEMORY_REPORT_EVENT_TYPE_UNIMPORT_EXT: i32 = 3;
    pub const VK_DEVICE_MEMORY_REPORT_EVENT_TYPE_ALLOCATION_FAILED_EXT: i32 = 4;
}

pub struct VkRasterizationOrderAMD(i32); //
impl VkRasterizationOrderAMD {
    pub const VK_RASTERIZATION_ORDER_STRICT_AMD: i32 = 0;
    pub const VK_RASTERIZATION_ORDER_RELAXED_AMD: i32 = 1;
}

pub type VkExternalMemoryHandleTypeFlagBitsNV = VkExternalMemoryHandleTypeFlagsNV; //
impl VkExternalMemoryHandleTypeFlagBitsNV {
    pub const VK_EXTERNAL_MEMORY_HANDLE_TYPE_OPAQUE_WIN32_BIT_NV: VkExternalMemoryHandleTypeFlagsNV = VkExternalMemoryHandleTypeFlagsNV(1);
    pub const VK_EXTERNAL_MEMORY_HANDLE_TYPE_OPAQUE_WIN32_KMT_BIT_NV: VkExternalMemoryHandleTypeFlagsNV = VkExternalMemoryHandleTypeFlagsNV(2);
    pub const VK_EXTERNAL_MEMORY_HANDLE_TYPE_D3D11_IMAGE_BIT_NV: VkExternalMemoryHandleTypeFlagsNV = VkExternalMemoryHandleTypeFlagsNV(4);
    pub const VK_EXTERNAL_MEMORY_HANDLE_TYPE_D3D11_IMAGE_KMT_BIT_NV: VkExternalMemoryHandleTypeFlagsNV = VkExternalMemoryHandleTypeFlagsNV(8);
}

pub type VkExternalMemoryFeatureFlagBitsNV = VkExternalMemoryFeatureFlagsNV; //
impl VkExternalMemoryFeatureFlagBitsNV {
    pub const VK_EXTERNAL_MEMORY_FEATURE_DEDICATED_ONLY_BIT_NV: VkExternalMemoryFeatureFlagsNV = VkExternalMemoryFeatureFlagsNV(1);
    pub const VK_EXTERNAL_MEMORY_FEATURE_EXPORTABLE_BIT_NV: VkExternalMemoryFeatureFlagsNV = VkExternalMemoryFeatureFlagsNV(2);
    pub const VK_EXTERNAL_MEMORY_FEATURE_IMPORTABLE_BIT_NV: VkExternalMemoryFeatureFlagsNV = VkExternalMemoryFeatureFlagsNV(4);
}

pub type VkClusterAccelerationStructureIndexFormatFlagBitsNV = VkClusterAccelerationStructureIndexFormatFlagsNV; //
impl VkClusterAccelerationStructureIndexFormatFlagBitsNV {
    pub const VK_CLUSTER_ACCELERATION_STRUCTURE_INDEX_FORMAT_8BIT_NV: VkClusterAccelerationStructureIndexFormatFlagsNV = VkClusterAccelerationStructureIndexFormatFlagsNV(1);
    pub const VK_CLUSTER_ACCELERATION_STRUCTURE_INDEX_FORMAT_16BIT_NV: VkClusterAccelerationStructureIndexFormatFlagsNV = VkClusterAccelerationStructureIndexFormatFlagsNV(2);
    pub const VK_CLUSTER_ACCELERATION_STRUCTURE_INDEX_FORMAT_32BIT_NV: VkClusterAccelerationStructureIndexFormatFlagsNV = VkClusterAccelerationStructureIndexFormatFlagsNV(4);
}

pub struct VkClusterAccelerationStructureTypeNV(i32); //
impl VkClusterAccelerationStructureTypeNV {
    pub const VK_CLUSTER_ACCELERATION_STRUCTURE_TYPE_CLUSTERS_BOTTOM_LEVEL_NV: i32 = 0;
    pub const VK_CLUSTER_ACCELERATION_STRUCTURE_TYPE_TRIANGLE_CLUSTER_NV: i32 = 1;
    pub const VK_CLUSTER_ACCELERATION_STRUCTURE_TYPE_TRIANGLE_CLUSTER_TEMPLATE_NV: i32 = 2;
}

pub struct VkClusterAccelerationStructureOpTypeNV(i32); //
impl VkClusterAccelerationStructureOpTypeNV {
    pub const VK_CLUSTER_ACCELERATION_STRUCTURE_OP_TYPE_MOVE_OBJECTS_NV: i32 = 0;
    pub const VK_CLUSTER_ACCELERATION_STRUCTURE_OP_TYPE_BUILD_CLUSTERS_BOTTOM_LEVEL_NV: i32 = 1;
    pub const VK_CLUSTER_ACCELERATION_STRUCTURE_OP_TYPE_BUILD_TRIANGLE_CLUSTER_NV: i32 = 2;
    pub const VK_CLUSTER_ACCELERATION_STRUCTURE_OP_TYPE_BUILD_TRIANGLE_CLUSTER_TEMPLATE_NV: i32 = 3;
    pub const VK_CLUSTER_ACCELERATION_STRUCTURE_OP_TYPE_INSTANTIATE_TRIANGLE_CLUSTER_NV: i32 = 4;
    pub const VK_CLUSTER_ACCELERATION_STRUCTURE_OP_TYPE_GET_CLUSTER_TEMPLATE_INDICES_NV: i32 = 5;
}

pub struct VkClusterAccelerationStructureOpModeNV(i32); //
impl VkClusterAccelerationStructureOpModeNV {
    pub const VK_CLUSTER_ACCELERATION_STRUCTURE_OP_MODE_IMPLICIT_DESTINATIONS_NV: i32 = 0;
    pub const VK_CLUSTER_ACCELERATION_STRUCTURE_OP_MODE_EXPLICIT_DESTINATIONS_NV: i32 = 1;
    pub const VK_CLUSTER_ACCELERATION_STRUCTURE_OP_MODE_COMPUTE_SIZES_NV: i32 = 2;
}

pub type VkClusterAccelerationStructureClusterFlagBitsNV = VkClusterAccelerationStructureClusterFlagsNV; //
impl VkClusterAccelerationStructureClusterFlagBitsNV {
    pub const VK_CLUSTER_ACCELERATION_STRUCTURE_CLUSTER_ALLOW_DISABLE_OPACITY_MICROMAPS_NV: VkClusterAccelerationStructureClusterFlagsNV = VkClusterAccelerationStructureClusterFlagsNV(1);
}

pub type VkClusterAccelerationStructureGeometryFlagBitsNV = VkClusterAccelerationStructureGeometryFlagsNV; //
impl VkClusterAccelerationStructureGeometryFlagBitsNV {
    pub const VK_CLUSTER_ACCELERATION_STRUCTURE_GEOMETRY_CULL_DISABLE_BIT_NV: VkClusterAccelerationStructureGeometryFlagsNV = VkClusterAccelerationStructureGeometryFlagsNV(1);
    pub const VK_CLUSTER_ACCELERATION_STRUCTURE_GEOMETRY_NO_DUPLICATE_ANYHIT_INVOCATION_BIT_NV: VkClusterAccelerationStructureGeometryFlagsNV = VkClusterAccelerationStructureGeometryFlagsNV(2);
    pub const VK_CLUSTER_ACCELERATION_STRUCTURE_GEOMETRY_OPAQUE_BIT_NV: VkClusterAccelerationStructureGeometryFlagsNV = VkClusterAccelerationStructureGeometryFlagsNV(4);
}

pub type VkClusterAccelerationStructureAddressResolutionFlagBitsNV = VkClusterAccelerationStructureAddressResolutionFlagsNV; //
impl VkClusterAccelerationStructureAddressResolutionFlagBitsNV {
    pub const VK_CLUSTER_ACCELERATION_STRUCTURE_ADDRESS_RESOLUTION_NONE_NV: VkClusterAccelerationStructureAddressResolutionFlagsNV = VkClusterAccelerationStructureAddressResolutionFlagsNV(0);
    pub const VK_CLUSTER_ACCELERATION_STRUCTURE_ADDRESS_RESOLUTION_INDIRECTED_DST_IMPLICIT_DATA_BIT_NV: VkClusterAccelerationStructureAddressResolutionFlagsNV = VkClusterAccelerationStructureAddressResolutionFlagsNV(1);
    pub const VK_CLUSTER_ACCELERATION_STRUCTURE_ADDRESS_RESOLUTION_INDIRECTED_SCRATCH_DATA_BIT_NV: VkClusterAccelerationStructureAddressResolutionFlagsNV = VkClusterAccelerationStructureAddressResolutionFlagsNV(2);
    pub const VK_CLUSTER_ACCELERATION_STRUCTURE_ADDRESS_RESOLUTION_INDIRECTED_DST_ADDRESS_ARRAY_BIT_NV: VkClusterAccelerationStructureAddressResolutionFlagsNV = VkClusterAccelerationStructureAddressResolutionFlagsNV(4);
    pub const VK_CLUSTER_ACCELERATION_STRUCTURE_ADDRESS_RESOLUTION_INDIRECTED_DST_SIZES_ARRAY_BIT_NV: VkClusterAccelerationStructureAddressResolutionFlagsNV = VkClusterAccelerationStructureAddressResolutionFlagsNV(8);
    pub const VK_CLUSTER_ACCELERATION_STRUCTURE_ADDRESS_RESOLUTION_INDIRECTED_SRC_INFOS_ARRAY_BIT_NV: VkClusterAccelerationStructureAddressResolutionFlagsNV = VkClusterAccelerationStructureAddressResolutionFlagsNV(16);
    pub const VK_CLUSTER_ACCELERATION_STRUCTURE_ADDRESS_RESOLUTION_INDIRECTED_SRC_INFOS_COUNT_BIT_NV: VkClusterAccelerationStructureAddressResolutionFlagsNV = VkClusterAccelerationStructureAddressResolutionFlagsNV(32);
}

pub struct VkValidationCheckEXT(i32); //
impl VkValidationCheckEXT {
    pub const VK_VALIDATION_CHECK_ALL_EXT: i32 = 0;
    pub const VK_VALIDATION_CHECK_SHADERS_EXT: i32 = 1;
}

pub struct VkValidationFeatureEnableEXT(i32); //
impl VkValidationFeatureEnableEXT {
    pub const VK_VALIDATION_FEATURE_ENABLE_GPU_ASSISTED_EXT: i32 = 0;
    pub const VK_VALIDATION_FEATURE_ENABLE_GPU_ASSISTED_RESERVE_BINDING_SLOT_EXT: i32 = 1;
    pub const VK_VALIDATION_FEATURE_ENABLE_BEST_PRACTICES_EXT: i32 = 2;
    pub const VK_VALIDATION_FEATURE_ENABLE_DEBUG_PRINTF_EXT: i32 = 3;
    pub const VK_VALIDATION_FEATURE_ENABLE_SYNCHRONIZATION_VALIDATION_EXT: i32 = 4;
}

pub struct VkValidationFeatureDisableEXT(i32); //
impl VkValidationFeatureDisableEXT {
    pub const VK_VALIDATION_FEATURE_DISABLE_ALL_EXT: i32 = 0;
    pub const VK_VALIDATION_FEATURE_DISABLE_SHADERS_EXT: i32 = 1;
    pub const VK_VALIDATION_FEATURE_DISABLE_THREAD_SAFETY_EXT: i32 = 2;
    pub const VK_VALIDATION_FEATURE_DISABLE_API_PARAMETERS_EXT: i32 = 3;
    pub const VK_VALIDATION_FEATURE_DISABLE_OBJECT_LIFETIMES_EXT: i32 = 4;
    pub const VK_VALIDATION_FEATURE_DISABLE_CORE_CHECKS_EXT: i32 = 5;
    pub const VK_VALIDATION_FEATURE_DISABLE_UNIQUE_HANDLES_EXT: i32 = 6;
    pub const VK_VALIDATION_FEATURE_DISABLE_SHADER_VALIDATION_CACHE_EXT: i32 = 7;
}

pub struct VkLayerSettingTypeEXT(i32); //
impl VkLayerSettingTypeEXT {
    pub const VK_LAYER_SETTING_TYPE_BOOL32_EXT: i32 = 0;
    pub const VK_LAYER_SETTING_TYPE_INT32_EXT: i32 = 1;
    pub const VK_LAYER_SETTING_TYPE_INT64_EXT: i32 = 2;
    pub const VK_LAYER_SETTING_TYPE_UINT32_EXT: i32 = 3;
    pub const VK_LAYER_SETTING_TYPE_UINT64_EXT: i32 = 4;
    pub const VK_LAYER_SETTING_TYPE_FLOAT32_EXT: i32 = 5;
    pub const VK_LAYER_SETTING_TYPE_FLOAT64_EXT: i32 = 6;
    pub const VK_LAYER_SETTING_TYPE_STRING_EXT: i32 = 7;
}

pub type VkSubgroupFeatureFlagBits = VkSubgroupFeatureFlags; //
impl VkSubgroupFeatureFlagBits {
    pub const VK_SUBGROUP_FEATURE_BASIC_BIT: VkSubgroupFeatureFlags = VkSubgroupFeatureFlags(1);
    pub const VK_SUBGROUP_FEATURE_VOTE_BIT: VkSubgroupFeatureFlags = VkSubgroupFeatureFlags(2);
    pub const VK_SUBGROUP_FEATURE_ARITHMETIC_BIT: VkSubgroupFeatureFlags = VkSubgroupFeatureFlags(4);
    pub const VK_SUBGROUP_FEATURE_BALLOT_BIT: VkSubgroupFeatureFlags = VkSubgroupFeatureFlags(8);
    pub const VK_SUBGROUP_FEATURE_SHUFFLE_BIT: VkSubgroupFeatureFlags = VkSubgroupFeatureFlags(16);
    pub const VK_SUBGROUP_FEATURE_SHUFFLE_RELATIVE_BIT: VkSubgroupFeatureFlags = VkSubgroupFeatureFlags(32);
    pub const VK_SUBGROUP_FEATURE_CLUSTERED_BIT: VkSubgroupFeatureFlags = VkSubgroupFeatureFlags(64);
    pub const VK_SUBGROUP_FEATURE_QUAD_BIT: VkSubgroupFeatureFlags = VkSubgroupFeatureFlags(128);
}

pub type VkIndirectCommandsLayoutUsageFlagBitsNV = VkIndirectCommandsLayoutUsageFlagsNV; //
impl VkIndirectCommandsLayoutUsageFlagBitsNV {
    pub const VK_INDIRECT_COMMANDS_LAYOUT_USAGE_EXPLICIT_PREPROCESS_BIT_NV: VkIndirectCommandsLayoutUsageFlagsNV = VkIndirectCommandsLayoutUsageFlagsNV(1);
    pub const VK_INDIRECT_COMMANDS_LAYOUT_USAGE_INDEXED_SEQUENCES_BIT_NV: VkIndirectCommandsLayoutUsageFlagsNV = VkIndirectCommandsLayoutUsageFlagsNV(2);
    pub const VK_INDIRECT_COMMANDS_LAYOUT_USAGE_UNORDERED_SEQUENCES_BIT_NV: VkIndirectCommandsLayoutUsageFlagsNV = VkIndirectCommandsLayoutUsageFlagsNV(4);
}

pub type VkIndirectStateFlagBitsNV = VkIndirectStateFlagsNV; //
impl VkIndirectStateFlagBitsNV {
    pub const VK_INDIRECT_STATE_FLAG_FRONTFACE_BIT_NV: VkIndirectStateFlagsNV = VkIndirectStateFlagsNV(1);
}

pub struct VkIndirectCommandsTokenTypeNV(i32); //
impl VkIndirectCommandsTokenTypeNV {
    pub const VK_INDIRECT_COMMANDS_TOKEN_TYPE_SHADER_GROUP_NV: i32 = 0;
    pub const VK_INDIRECT_COMMANDS_TOKEN_TYPE_STATE_FLAGS_NV: i32 = 1;
    pub const VK_INDIRECT_COMMANDS_TOKEN_TYPE_INDEX_BUFFER_NV: i32 = 2;
    pub const VK_INDIRECT_COMMANDS_TOKEN_TYPE_VERTEX_BUFFER_NV: i32 = 3;
    pub const VK_INDIRECT_COMMANDS_TOKEN_TYPE_PUSH_CONSTANT_NV: i32 = 4;
    pub const VK_INDIRECT_COMMANDS_TOKEN_TYPE_DRAW_INDEXED_NV: i32 = 5;
    pub const VK_INDIRECT_COMMANDS_TOKEN_TYPE_DRAW_NV: i32 = 6;
    pub const VK_INDIRECT_COMMANDS_TOKEN_TYPE_DRAW_TASKS_NV: i32 = 7;
}

pub type VkPrivateDataSlotCreateFlagBits = VkPrivateDataSlotCreateFlags; //
impl VkPrivateDataSlotCreateFlagBits {
}

pub type VkDescriptorSetLayoutCreateFlagBits = VkDescriptorSetLayoutCreateFlags; //
impl VkDescriptorSetLayoutCreateFlagBits {
}

pub type VkExternalMemoryHandleTypeFlagBits = VkExternalMemoryHandleTypeFlags; //
impl VkExternalMemoryHandleTypeFlagBits {
    pub const VK_EXTERNAL_MEMORY_HANDLE_TYPE_OPAQUE_FD_BIT: VkExternalMemoryHandleTypeFlags = VkExternalMemoryHandleTypeFlags(1);
    pub const VK_EXTERNAL_MEMORY_HANDLE_TYPE_OPAQUE_WIN32_BIT: VkExternalMemoryHandleTypeFlags = VkExternalMemoryHandleTypeFlags(2);
    pub const VK_EXTERNAL_MEMORY_HANDLE_TYPE_OPAQUE_WIN32_KMT_BIT: VkExternalMemoryHandleTypeFlags = VkExternalMemoryHandleTypeFlags(4);
    pub const VK_EXTERNAL_MEMORY_HANDLE_TYPE_D3D11_TEXTURE_BIT: VkExternalMemoryHandleTypeFlags = VkExternalMemoryHandleTypeFlags(8);
    pub const VK_EXTERNAL_MEMORY_HANDLE_TYPE_D3D11_TEXTURE_KMT_BIT: VkExternalMemoryHandleTypeFlags = VkExternalMemoryHandleTypeFlags(16);
    pub const VK_EXTERNAL_MEMORY_HANDLE_TYPE_D3D12_HEAP_BIT: VkExternalMemoryHandleTypeFlags = VkExternalMemoryHandleTypeFlags(32);
    pub const VK_EXTERNAL_MEMORY_HANDLE_TYPE_D3D12_RESOURCE_BIT: VkExternalMemoryHandleTypeFlags = VkExternalMemoryHandleTypeFlags(64);
}

pub type VkExternalMemoryFeatureFlagBits = VkExternalMemoryFeatureFlags; //
impl VkExternalMemoryFeatureFlagBits {
    pub const VK_EXTERNAL_MEMORY_FEATURE_DEDICATED_ONLY_BIT: VkExternalMemoryFeatureFlags = VkExternalMemoryFeatureFlags(1);
    pub const VK_EXTERNAL_MEMORY_FEATURE_EXPORTABLE_BIT: VkExternalMemoryFeatureFlags = VkExternalMemoryFeatureFlags(2);
    pub const VK_EXTERNAL_MEMORY_FEATURE_IMPORTABLE_BIT: VkExternalMemoryFeatureFlags = VkExternalMemoryFeatureFlags(4);
}

pub type VkExternalSemaphoreHandleTypeFlagBits = VkExternalSemaphoreHandleTypeFlags; //
impl VkExternalSemaphoreHandleTypeFlagBits {
    pub const VK_EXTERNAL_SEMAPHORE_HANDLE_TYPE_OPAQUE_FD_BIT: VkExternalSemaphoreHandleTypeFlags = VkExternalSemaphoreHandleTypeFlags(1);
    pub const VK_EXTERNAL_SEMAPHORE_HANDLE_TYPE_OPAQUE_WIN32_BIT: VkExternalSemaphoreHandleTypeFlags = VkExternalSemaphoreHandleTypeFlags(2);
    pub const VK_EXTERNAL_SEMAPHORE_HANDLE_TYPE_OPAQUE_WIN32_KMT_BIT: VkExternalSemaphoreHandleTypeFlags = VkExternalSemaphoreHandleTypeFlags(4);
    pub const VK_EXTERNAL_SEMAPHORE_HANDLE_TYPE_D3D12_FENCE_BIT: VkExternalSemaphoreHandleTypeFlags = VkExternalSemaphoreHandleTypeFlags(8);
    pub const VK_EXTERNAL_SEMAPHORE_HANDLE_TYPE_D3D11_FENCE_BIT: VkExternalSemaphoreHandleTypeFlags = Self::VK_EXTERNAL_SEMAPHORE_HANDLE_TYPE_D3D12_FENCE_BIT;
    pub const VK_EXTERNAL_SEMAPHORE_HANDLE_TYPE_SYNC_FD_BIT: VkExternalSemaphoreHandleTypeFlags = VkExternalSemaphoreHandleTypeFlags(16);
}

pub type VkExternalSemaphoreFeatureFlagBits = VkExternalSemaphoreFeatureFlags; //
impl VkExternalSemaphoreFeatureFlagBits {
    pub const VK_EXTERNAL_SEMAPHORE_FEATURE_EXPORTABLE_BIT: VkExternalSemaphoreFeatureFlags = VkExternalSemaphoreFeatureFlags(1);
    pub const VK_EXTERNAL_SEMAPHORE_FEATURE_IMPORTABLE_BIT: VkExternalSemaphoreFeatureFlags = VkExternalSemaphoreFeatureFlags(2);
}

pub type VkSemaphoreImportFlagBits = VkSemaphoreImportFlags; //
impl VkSemaphoreImportFlagBits {
    pub const VK_SEMAPHORE_IMPORT_TEMPORARY_BIT: VkSemaphoreImportFlags = VkSemaphoreImportFlags(1);
}

pub type VkExternalFenceHandleTypeFlagBits = VkExternalFenceHandleTypeFlags; //
impl VkExternalFenceHandleTypeFlagBits {
    pub const VK_EXTERNAL_FENCE_HANDLE_TYPE_OPAQUE_FD_BIT: VkExternalFenceHandleTypeFlags = VkExternalFenceHandleTypeFlags(1);
    pub const VK_EXTERNAL_FENCE_HANDLE_TYPE_OPAQUE_WIN32_BIT: VkExternalFenceHandleTypeFlags = VkExternalFenceHandleTypeFlags(2);
    pub const VK_EXTERNAL_FENCE_HANDLE_TYPE_OPAQUE_WIN32_KMT_BIT: VkExternalFenceHandleTypeFlags = VkExternalFenceHandleTypeFlags(4);
    pub const VK_EXTERNAL_FENCE_HANDLE_TYPE_SYNC_FD_BIT: VkExternalFenceHandleTypeFlags = VkExternalFenceHandleTypeFlags(8);
}

pub type VkExternalFenceFeatureFlagBits = VkExternalFenceFeatureFlags; //
impl VkExternalFenceFeatureFlagBits {
    pub const VK_EXTERNAL_FENCE_FEATURE_EXPORTABLE_BIT: VkExternalFenceFeatureFlags = VkExternalFenceFeatureFlags(1);
    pub const VK_EXTERNAL_FENCE_FEATURE_IMPORTABLE_BIT: VkExternalFenceFeatureFlags = VkExternalFenceFeatureFlags(2);
}

pub type VkFenceImportFlagBits = VkFenceImportFlags; //
impl VkFenceImportFlagBits {
    pub const VK_FENCE_IMPORT_TEMPORARY_BIT: VkFenceImportFlags = VkFenceImportFlags(1);
}

pub type VkSurfaceCounterFlagBitsEXT = VkSurfaceCounterFlagsEXT; //
impl VkSurfaceCounterFlagBitsEXT {
    pub const VK_SURFACE_COUNTER_VBLANK_BIT_EXT: VkSurfaceCounterFlagsEXT = VkSurfaceCounterFlagsEXT(1);
    pub const VK_SURFACE_COUNTER_VBLANK_EXT: VkSurfaceCounterFlagsEXT = Self::VK_SURFACE_COUNTER_VBLANK_BIT_EXT;
}

pub struct VkDisplayPowerStateEXT(i32); //
impl VkDisplayPowerStateEXT {
    pub const VK_DISPLAY_POWER_STATE_OFF_EXT: i32 = 0;
    pub const VK_DISPLAY_POWER_STATE_SUSPEND_EXT: i32 = 1;
    pub const VK_DISPLAY_POWER_STATE_ON_EXT: i32 = 2;
}

pub struct VkDeviceEventTypeEXT(i32); //
impl VkDeviceEventTypeEXT {
    pub const VK_DEVICE_EVENT_TYPE_DISPLAY_HOTPLUG_EXT: i32 = 0;
}

pub struct VkDisplayEventTypeEXT(i32); //
impl VkDisplayEventTypeEXT {
    pub const VK_DISPLAY_EVENT_TYPE_FIRST_PIXEL_OUT_EXT: i32 = 0;
}

pub type VkPeerMemoryFeatureFlagBits = VkPeerMemoryFeatureFlags; //
impl VkPeerMemoryFeatureFlagBits {
    pub const VK_PEER_MEMORY_FEATURE_COPY_SRC_BIT: VkPeerMemoryFeatureFlags = VkPeerMemoryFeatureFlags(1);
    pub const VK_PEER_MEMORY_FEATURE_COPY_DST_BIT: VkPeerMemoryFeatureFlags = VkPeerMemoryFeatureFlags(2);
    pub const VK_PEER_MEMORY_FEATURE_GENERIC_SRC_BIT: VkPeerMemoryFeatureFlags = VkPeerMemoryFeatureFlags(4);
    pub const VK_PEER_MEMORY_FEATURE_GENERIC_DST_BIT: VkPeerMemoryFeatureFlags = VkPeerMemoryFeatureFlags(8);
}

pub type VkMemoryAllocateFlagBits = VkMemoryAllocateFlags; //
impl VkMemoryAllocateFlagBits {
    pub const VK_MEMORY_ALLOCATE_DEVICE_MASK_BIT: VkMemoryAllocateFlags = VkMemoryAllocateFlags(1);
}

pub type VkDeviceGroupPresentModeFlagBitsKHR = VkDeviceGroupPresentModeFlagsKHR; //
impl VkDeviceGroupPresentModeFlagBitsKHR {
    pub const VK_DEVICE_GROUP_PRESENT_MODE_LOCAL_BIT_KHR: VkDeviceGroupPresentModeFlagsKHR = VkDeviceGroupPresentModeFlagsKHR(1);
    pub const VK_DEVICE_GROUP_PRESENT_MODE_REMOTE_BIT_KHR: VkDeviceGroupPresentModeFlagsKHR = VkDeviceGroupPresentModeFlagsKHR(2);
    pub const VK_DEVICE_GROUP_PRESENT_MODE_SUM_BIT_KHR: VkDeviceGroupPresentModeFlagsKHR = VkDeviceGroupPresentModeFlagsKHR(4);
    pub const VK_DEVICE_GROUP_PRESENT_MODE_LOCAL_MULTI_DEVICE_BIT_KHR: VkDeviceGroupPresentModeFlagsKHR = VkDeviceGroupPresentModeFlagsKHR(8);
}

pub type VkSwapchainCreateFlagBitsKHR = VkSwapchainCreateFlagsKHR; //
impl VkSwapchainCreateFlagBitsKHR {
}

pub struct VkViewportCoordinateSwizzleNV(i32); //
impl VkViewportCoordinateSwizzleNV {
    pub const VK_VIEWPORT_COORDINATE_SWIZZLE_POSITIVE_X_NV: i32 = 0;
    pub const VK_VIEWPORT_COORDINATE_SWIZZLE_NEGATIVE_X_NV: i32 = 1;
    pub const VK_VIEWPORT_COORDINATE_SWIZZLE_POSITIVE_Y_NV: i32 = 2;
    pub const VK_VIEWPORT_COORDINATE_SWIZZLE_NEGATIVE_Y_NV: i32 = 3;
    pub const VK_VIEWPORT_COORDINATE_SWIZZLE_POSITIVE_Z_NV: i32 = 4;
    pub const VK_VIEWPORT_COORDINATE_SWIZZLE_NEGATIVE_Z_NV: i32 = 5;
    pub const VK_VIEWPORT_COORDINATE_SWIZZLE_POSITIVE_W_NV: i32 = 6;
    pub const VK_VIEWPORT_COORDINATE_SWIZZLE_NEGATIVE_W_NV: i32 = 7;
}

pub struct VkDiscardRectangleModeEXT(i32); //
impl VkDiscardRectangleModeEXT {
    pub const VK_DISCARD_RECTANGLE_MODE_INCLUSIVE_EXT: i32 = 0;
    pub const VK_DISCARD_RECTANGLE_MODE_EXCLUSIVE_EXT: i32 = 1;
}

pub type VkSubpassDescriptionFlagBits = VkSubpassDescriptionFlags; //
impl VkSubpassDescriptionFlagBits {
}

pub struct VkPointClippingBehavior(i32); //
impl VkPointClippingBehavior {
    pub const VK_POINT_CLIPPING_BEHAVIOR_ALL_CLIP_PLANES: i32 = 0;
    pub const VK_POINT_CLIPPING_BEHAVIOR_USER_CLIP_PLANES_ONLY: i32 = 1;
}

pub struct VkSamplerReductionMode(i32); //
impl VkSamplerReductionMode {
    pub const VK_SAMPLER_REDUCTION_MODE_WEIGHTED_AVERAGE: i32 = 0;
    pub const VK_SAMPLER_REDUCTION_MODE_MIN: i32 = 1;
    pub const VK_SAMPLER_REDUCTION_MODE_MAX: i32 = 2;
}

pub struct VkTessellationDomainOrigin(i32); //
impl VkTessellationDomainOrigin {
    pub const VK_TESSELLATION_DOMAIN_ORIGIN_UPPER_LEFT: i32 = 0;
    pub const VK_TESSELLATION_DOMAIN_ORIGIN_LOWER_LEFT: i32 = 1;
}

pub struct VkSamplerYcbcrModelConversion(i32); //
impl VkSamplerYcbcrModelConversion {
    pub const VK_SAMPLER_YCBCR_MODEL_CONVERSION_RGB_IDENTITY: i32 = 0;
    pub const VK_SAMPLER_YCBCR_MODEL_CONVERSION_YCBCR_IDENTITY: i32 = 1;
    pub const VK_SAMPLER_YCBCR_MODEL_CONVERSION_YCBCR_709: i32 = 2;
    pub const VK_SAMPLER_YCBCR_MODEL_CONVERSION_YCBCR_601: i32 = 3;
    pub const VK_SAMPLER_YCBCR_MODEL_CONVERSION_YCBCR_2020: i32 = 4;
}

pub struct VkSamplerYcbcrRange(i32); //
impl VkSamplerYcbcrRange {
    pub const VK_SAMPLER_YCBCR_RANGE_ITU_FULL: i32 = 0;
    pub const VK_SAMPLER_YCBCR_RANGE_ITU_NARROW: i32 = 1;
}

pub struct VkChromaLocation(i32); //
impl VkChromaLocation {
    pub const VK_CHROMA_LOCATION_COSITED_EVEN: i32 = 0;
    pub const VK_CHROMA_LOCATION_MIDPOINT: i32 = 1;
}

pub struct VkBlendOverlapEXT(i32); //
impl VkBlendOverlapEXT {
    pub const VK_BLEND_OVERLAP_UNCORRELATED_EXT: i32 = 0;
    pub const VK_BLEND_OVERLAP_DISJOINT_EXT: i32 = 1;
    pub const VK_BLEND_OVERLAP_CONJOINT_EXT: i32 = 2;
}

pub struct VkCoverageModulationModeNV(i32); //
impl VkCoverageModulationModeNV {
    pub const VK_COVERAGE_MODULATION_MODE_NONE_NV: i32 = 0;
    pub const VK_COVERAGE_MODULATION_MODE_RGB_NV: i32 = 1;
    pub const VK_COVERAGE_MODULATION_MODE_ALPHA_NV: i32 = 2;
    pub const VK_COVERAGE_MODULATION_MODE_RGBA_NV: i32 = 3;
}

pub struct VkCoverageReductionModeNV(i32); //
impl VkCoverageReductionModeNV {
    pub const VK_COVERAGE_REDUCTION_MODE_MERGE_NV: i32 = 0;
    pub const VK_COVERAGE_REDUCTION_MODE_TRUNCATE_NV: i32 = 1;
}

pub struct VkValidationCacheHeaderVersionEXT(i32); //
impl VkValidationCacheHeaderVersionEXT {
    pub const VK_VALIDATION_CACHE_HEADER_VERSION_ONE_EXT: i32 = 1;
}

pub struct VkShaderInfoTypeAMD(i32); //
impl VkShaderInfoTypeAMD {
    pub const VK_SHADER_INFO_TYPE_STATISTICS_AMD: i32 = 0;
    pub const VK_SHADER_INFO_TYPE_BINARY_AMD: i32 = 1;
    pub const VK_SHADER_INFO_TYPE_DISASSEMBLY_AMD: i32 = 2;
}

pub struct VkQueueGlobalPriority(i32); //
impl VkQueueGlobalPriority {
    pub const VK_QUEUE_GLOBAL_PRIORITY_LOW: i32 = 128;
    pub const VK_QUEUE_GLOBAL_PRIORITY_MEDIUM: i32 = 256;
    pub const VK_QUEUE_GLOBAL_PRIORITY_HIGH: i32 = 512;
    pub const VK_QUEUE_GLOBAL_PRIORITY_REALTIME: i32 = 1024;
}

pub type VkDebugUtilsMessageSeverityFlagBitsEXT = VkDebugUtilsMessageSeverityFlagsEXT; //
impl VkDebugUtilsMessageSeverityFlagBitsEXT {
    pub const VK_DEBUG_UTILS_MESSAGE_SEVERITY_VERBOSE_BIT_EXT: VkDebugUtilsMessageSeverityFlagsEXT = VkDebugUtilsMessageSeverityFlagsEXT(1);
    pub const VK_DEBUG_UTILS_MESSAGE_SEVERITY_INFO_BIT_EXT: VkDebugUtilsMessageSeverityFlagsEXT = VkDebugUtilsMessageSeverityFlagsEXT(16);
    pub const VK_DEBUG_UTILS_MESSAGE_SEVERITY_WARNING_BIT_EXT: VkDebugUtilsMessageSeverityFlagsEXT = VkDebugUtilsMessageSeverityFlagsEXT(256);
    pub const VK_DEBUG_UTILS_MESSAGE_SEVERITY_ERROR_BIT_EXT: VkDebugUtilsMessageSeverityFlagsEXT = VkDebugUtilsMessageSeverityFlagsEXT(4096);
}

pub type VkDebugUtilsMessageTypeFlagBitsEXT = VkDebugUtilsMessageTypeFlagsEXT; //
impl VkDebugUtilsMessageTypeFlagBitsEXT {
    pub const VK_DEBUG_UTILS_MESSAGE_TYPE_GENERAL_BIT_EXT: VkDebugUtilsMessageTypeFlagsEXT = VkDebugUtilsMessageTypeFlagsEXT(1);
    pub const VK_DEBUG_UTILS_MESSAGE_TYPE_VALIDATION_BIT_EXT: VkDebugUtilsMessageTypeFlagsEXT = VkDebugUtilsMessageTypeFlagsEXT(2);
    pub const VK_DEBUG_UTILS_MESSAGE_TYPE_PERFORMANCE_BIT_EXT: VkDebugUtilsMessageTypeFlagsEXT = VkDebugUtilsMessageTypeFlagsEXT(4);
}

pub struct VkConservativeRasterizationModeEXT(i32); //
impl VkConservativeRasterizationModeEXT {
    pub const VK_CONSERVATIVE_RASTERIZATION_MODE_DISABLED_EXT: i32 = 0;
    pub const VK_CONSERVATIVE_RASTERIZATION_MODE_OVERESTIMATE_EXT: i32 = 1;
    pub const VK_CONSERVATIVE_RASTERIZATION_MODE_UNDERESTIMATE_EXT: i32 = 2;
}

pub type VkDescriptorBindingFlagBits = VkDescriptorBindingFlags; //
impl VkDescriptorBindingFlagBits {
    pub const VK_DESCRIPTOR_BINDING_UPDATE_AFTER_BIND_BIT: VkDescriptorBindingFlags = VkDescriptorBindingFlags(1);
    pub const VK_DESCRIPTOR_BINDING_UPDATE_UNUSED_WHILE_PENDING_BIT: VkDescriptorBindingFlags = VkDescriptorBindingFlags(2);
    pub const VK_DESCRIPTOR_BINDING_PARTIALLY_BOUND_BIT: VkDescriptorBindingFlags = VkDescriptorBindingFlags(4);
    pub const VK_DESCRIPTOR_BINDING_VARIABLE_DESCRIPTOR_COUNT_BIT: VkDescriptorBindingFlags = VkDescriptorBindingFlags(8);
}

pub struct VkVendorId(i32); //
impl VkVendorId {
    pub const VK_VENDOR_ID_KHRONOS: i32 = 0x10000;
    pub const VK_VENDOR_ID_VIV: i32 = 0x10001;
    pub const VK_VENDOR_ID_VSI: i32 = 0x10002;
    pub const VK_VENDOR_ID_KAZAN: i32 = 0x10003;
    pub const VK_VENDOR_ID_CODEPLAY: i32 = 0x10004;
    pub const VK_VENDOR_ID_MESA: i32 = 0x10005;
    pub const VK_VENDOR_ID_POCL: i32 = 0x10006;
    pub const VK_VENDOR_ID_MOBILEYE: i32 = 0x10007;
    pub const VK_VENDOR_ID_APE: i32 = 0x10008;
}

pub struct VkDriverId(i32); //
impl VkDriverId {
    pub const VK_DRIVER_ID_AMD_PROPRIETARY: i32 = 1;
    pub const VK_DRIVER_ID_AMD_OPEN_SOURCE: i32 = 2;
    pub const VK_DRIVER_ID_MESA_RADV: i32 = 3;
    pub const VK_DRIVER_ID_NVIDIA_PROPRIETARY: i32 = 4;
    pub const VK_DRIVER_ID_INTEL_PROPRIETARY_WINDOWS: i32 = 5;
    pub const VK_DRIVER_ID_INTEL_OPEN_SOURCE_MESA: i32 = 6;
    pub const VK_DRIVER_ID_IMAGINATION_PROPRIETARY: i32 = 7;
    pub const VK_DRIVER_ID_QUALCOMM_PROPRIETARY: i32 = 8;
    pub const VK_DRIVER_ID_ARM_PROPRIETARY: i32 = 9;
    pub const VK_DRIVER_ID_GOOGLE_SWIFTSHADER: i32 = 10;
    pub const VK_DRIVER_ID_GGP_PROPRIETARY: i32 = 11;
    pub const VK_DRIVER_ID_BROADCOM_PROPRIETARY: i32 = 12;
    pub const VK_DRIVER_ID_MESA_LLVMPIPE: i32 = 13;
    pub const VK_DRIVER_ID_MOLTENVK: i32 = 14;
    pub const VK_DRIVER_ID_COREAVI_PROPRIETARY: i32 = 15;
    pub const VK_DRIVER_ID_JUICE_PROPRIETARY: i32 = 16;
    pub const VK_DRIVER_ID_VERISILICON_PROPRIETARY: i32 = 17;
    pub const VK_DRIVER_ID_MESA_TURNIP: i32 = 18;
    pub const VK_DRIVER_ID_MESA_V3DV: i32 = 19;
    pub const VK_DRIVER_ID_MESA_PANVK: i32 = 20;
    pub const VK_DRIVER_ID_SAMSUNG_PROPRIETARY: i32 = 21;
    pub const VK_DRIVER_ID_MESA_VENUS: i32 = 22;
    pub const VK_DRIVER_ID_MESA_DOZEN: i32 = 23;
    pub const VK_DRIVER_ID_MESA_NVK: i32 = 24;
    pub const VK_DRIVER_ID_IMAGINATION_OPEN_SOURCE_MESA: i32 = 25;
    pub const VK_DRIVER_ID_MESA_HONEYKRISP: i32 = 26;
    pub const VK_DRIVER_ID_VULKAN_SC_EMULATION_ON_VULKAN: i32 = 27;
    pub const VK_DRIVER_ID_MESA_KOSMICKRISP: i32 = 28;
    pub const VK_DRIVER_ID_MESA_GFXSTREAM: i32 = 29;
    pub const VK_DRIVER_ID_APE_SOFT: i32 = 30;
    pub const VK_DRIVER_ID_RESERVED_31: i32 = 31;
}

pub type VkConditionalRenderingFlagBitsEXT = VkConditionalRenderingFlagsEXT; //
impl VkConditionalRenderingFlagBitsEXT {
    pub const VK_CONDITIONAL_RENDERING_INVERTED_BIT_EXT: VkConditionalRenderingFlagsEXT = VkConditionalRenderingFlagsEXT(1);
}

pub type VkResolveModeFlagBits = VkResolveModeFlags; //
impl VkResolveModeFlagBits {
    pub const VK_RESOLVE_MODE_NONE: VkResolveModeFlags = VkResolveModeFlags(0);
    pub const VK_RESOLVE_MODE_SAMPLE_ZERO_BIT: VkResolveModeFlags = VkResolveModeFlags(1);
    pub const VK_RESOLVE_MODE_AVERAGE_BIT: VkResolveModeFlags = VkResolveModeFlags(2);
    pub const VK_RESOLVE_MODE_MIN_BIT: VkResolveModeFlags = VkResolveModeFlags(4);
    pub const VK_RESOLVE_MODE_MAX_BIT: VkResolveModeFlags = VkResolveModeFlags(8);
}

pub struct VkShadingRatePaletteEntryNV(i32); //
impl VkShadingRatePaletteEntryNV {
    pub const VK_SHADING_RATE_PALETTE_ENTRY_NO_INVOCATIONS_NV: i32 = 0;
    pub const VK_SHADING_RATE_PALETTE_ENTRY_16_INVOCATIONS_PER_PIXEL_NV: i32 = 1;
    pub const VK_SHADING_RATE_PALETTE_ENTRY_8_INVOCATIONS_PER_PIXEL_NV: i32 = 2;
    pub const VK_SHADING_RATE_PALETTE_ENTRY_4_INVOCATIONS_PER_PIXEL_NV: i32 = 3;
    pub const VK_SHADING_RATE_PALETTE_ENTRY_2_INVOCATIONS_PER_PIXEL_NV: i32 = 4;
    pub const VK_SHADING_RATE_PALETTE_ENTRY_1_INVOCATION_PER_PIXEL_NV: i32 = 5;
    pub const VK_SHADING_RATE_PALETTE_ENTRY_1_INVOCATION_PER_2X1_PIXELS_NV: i32 = 6;
    pub const VK_SHADING_RATE_PALETTE_ENTRY_1_INVOCATION_PER_1X2_PIXELS_NV: i32 = 7;
    pub const VK_SHADING_RATE_PALETTE_ENTRY_1_INVOCATION_PER_2X2_PIXELS_NV: i32 = 8;
    pub const VK_SHADING_RATE_PALETTE_ENTRY_1_INVOCATION_PER_4X2_PIXELS_NV: i32 = 9;
    pub const VK_SHADING_RATE_PALETTE_ENTRY_1_INVOCATION_PER_2X4_PIXELS_NV: i32 = 10;
    pub const VK_SHADING_RATE_PALETTE_ENTRY_1_INVOCATION_PER_4X4_PIXELS_NV: i32 = 11;
}

pub struct VkCoarseSampleOrderTypeNV(i32); //
impl VkCoarseSampleOrderTypeNV {
    pub const VK_COARSE_SAMPLE_ORDER_TYPE_DEFAULT_NV: i32 = 0;
    pub const VK_COARSE_SAMPLE_ORDER_TYPE_CUSTOM_NV: i32 = 1;
    pub const VK_COARSE_SAMPLE_ORDER_TYPE_PIXEL_MAJOR_NV: i32 = 2;
    pub const VK_COARSE_SAMPLE_ORDER_TYPE_SAMPLE_MAJOR_NV: i32 = 3;
}

pub type VkGeometryInstanceFlagBitsKHR = VkGeometryInstanceFlagsKHR; //
impl VkGeometryInstanceFlagBitsKHR {
    pub const VK_GEOMETRY_INSTANCE_TRIANGLE_FACING_CULL_DISABLE_BIT_KHR: VkGeometryInstanceFlagsKHR = VkGeometryInstanceFlagsKHR(1);
    pub const VK_GEOMETRY_INSTANCE_TRIANGLE_FLIP_FACING_BIT_KHR: VkGeometryInstanceFlagsKHR = VkGeometryInstanceFlagsKHR(2);
    pub const VK_GEOMETRY_INSTANCE_FORCE_OPAQUE_BIT_KHR: VkGeometryInstanceFlagsKHR = VkGeometryInstanceFlagsKHR(4);
    pub const VK_GEOMETRY_INSTANCE_FORCE_NO_OPAQUE_BIT_KHR: VkGeometryInstanceFlagsKHR = VkGeometryInstanceFlagsKHR(8);
    pub const VK_GEOMETRY_INSTANCE_TRIANGLE_FRONT_COUNTERCLOCKWISE_BIT_KHR: VkGeometryInstanceFlagsKHR = Self::VK_GEOMETRY_INSTANCE_TRIANGLE_FLIP_FACING_BIT_KHR;
}

pub type VkGeometryFlagBitsKHR = VkGeometryFlagsKHR; //
impl VkGeometryFlagBitsKHR {
    pub const VK_GEOMETRY_OPAQUE_BIT_KHR: VkGeometryFlagsKHR = VkGeometryFlagsKHR(1);
    pub const VK_GEOMETRY_NO_DUPLICATE_ANY_HIT_INVOCATION_BIT_KHR: VkGeometryFlagsKHR = VkGeometryFlagsKHR(2);
}

pub type VkBuildAccelerationStructureFlagBitsKHR = VkBuildAccelerationStructureFlagsKHR; //
impl VkBuildAccelerationStructureFlagBitsKHR {
    pub const VK_BUILD_ACCELERATION_STRUCTURE_ALLOW_UPDATE_BIT_KHR: VkBuildAccelerationStructureFlagsKHR = VkBuildAccelerationStructureFlagsKHR(1);
    pub const VK_BUILD_ACCELERATION_STRUCTURE_ALLOW_COMPACTION_BIT_KHR: VkBuildAccelerationStructureFlagsKHR = VkBuildAccelerationStructureFlagsKHR(2);
    pub const VK_BUILD_ACCELERATION_STRUCTURE_PREFER_FAST_TRACE_BIT_KHR: VkBuildAccelerationStructureFlagsKHR = VkBuildAccelerationStructureFlagsKHR(4);
    pub const VK_BUILD_ACCELERATION_STRUCTURE_PREFER_FAST_BUILD_BIT_KHR: VkBuildAccelerationStructureFlagsKHR = VkBuildAccelerationStructureFlagsKHR(8);
    pub const VK_BUILD_ACCELERATION_STRUCTURE_LOW_MEMORY_BIT_KHR: VkBuildAccelerationStructureFlagsKHR = VkBuildAccelerationStructureFlagsKHR(16);
}

pub type VkAccelerationStructureCreateFlagBitsKHR = VkAccelerationStructureCreateFlagsKHR; //
impl VkAccelerationStructureCreateFlagBitsKHR {
    pub const VK_ACCELERATION_STRUCTURE_CREATE_DEVICE_ADDRESS_CAPTURE_REPLAY_BIT_KHR: VkAccelerationStructureCreateFlagsKHR = VkAccelerationStructureCreateFlagsKHR(1);
}

pub struct VkCopyAccelerationStructureModeKHR(i32); //
impl VkCopyAccelerationStructureModeKHR {
    pub const VK_COPY_ACCELERATION_STRUCTURE_MODE_CLONE_KHR: i32 = 0;
    pub const VK_COPY_ACCELERATION_STRUCTURE_MODE_COMPACT_KHR: i32 = 1;
}

pub struct VkBuildAccelerationStructureModeKHR(i32); //
impl VkBuildAccelerationStructureModeKHR {
    pub const VK_BUILD_ACCELERATION_STRUCTURE_MODE_BUILD_KHR: i32 = 0;
    pub const VK_BUILD_ACCELERATION_STRUCTURE_MODE_UPDATE_KHR: i32 = 1;
}

pub struct VkAccelerationStructureTypeKHR(i32); //
impl VkAccelerationStructureTypeKHR {
    pub const VK_ACCELERATION_STRUCTURE_TYPE_TOP_LEVEL_KHR: i32 = 0;
    pub const VK_ACCELERATION_STRUCTURE_TYPE_BOTTOM_LEVEL_KHR: i32 = 1;
    pub const VK_ACCELERATION_STRUCTURE_TYPE_GENERIC_KHR: i32 = 2;
}

pub struct VkGeometryTypeKHR(i32); //
impl VkGeometryTypeKHR {
    pub const VK_GEOMETRY_TYPE_TRIANGLES_KHR: i32 = 0;
    pub const VK_GEOMETRY_TYPE_AABBS_KHR: i32 = 1;
    pub const VK_GEOMETRY_TYPE_INSTANCES_KHR: i32 = 2;
}

pub struct VkAccelerationStructureMemoryRequirementsTypeNV(i32); //
impl VkAccelerationStructureMemoryRequirementsTypeNV {
    pub const VK_ACCELERATION_STRUCTURE_MEMORY_REQUIREMENTS_TYPE_OBJECT_NV: i32 = 0;
    pub const VK_ACCELERATION_STRUCTURE_MEMORY_REQUIREMENTS_TYPE_BUILD_SCRATCH_NV: i32 = 1;
    pub const VK_ACCELERATION_STRUCTURE_MEMORY_REQUIREMENTS_TYPE_UPDATE_SCRATCH_NV: i32 = 2;
}

pub struct VkAccelerationStructureBuildTypeKHR(i32); //
impl VkAccelerationStructureBuildTypeKHR {
    pub const VK_ACCELERATION_STRUCTURE_BUILD_TYPE_HOST_KHR: i32 = 0;
    pub const VK_ACCELERATION_STRUCTURE_BUILD_TYPE_DEVICE_KHR: i32 = 1;
    pub const VK_ACCELERATION_STRUCTURE_BUILD_TYPE_HOST_OR_DEVICE_KHR: i32 = 2;
}

pub struct VkRayTracingShaderGroupTypeKHR(i32); //
impl VkRayTracingShaderGroupTypeKHR {
    pub const VK_RAY_TRACING_SHADER_GROUP_TYPE_GENERAL_KHR: i32 = 0;
    pub const VK_RAY_TRACING_SHADER_GROUP_TYPE_TRIANGLES_HIT_GROUP_KHR: i32 = 1;
    pub const VK_RAY_TRACING_SHADER_GROUP_TYPE_PROCEDURAL_HIT_GROUP_KHR: i32 = 2;
}

pub struct VkAccelerationStructureCompatibilityKHR(i32); //
impl VkAccelerationStructureCompatibilityKHR {
    pub const VK_ACCELERATION_STRUCTURE_COMPATIBILITY_COMPATIBLE_KHR: i32 = 0;
    pub const VK_ACCELERATION_STRUCTURE_COMPATIBILITY_INCOMPATIBLE_KHR: i32 = 1;
}

pub struct VkShaderGroupShaderKHR(i32); //
impl VkShaderGroupShaderKHR {
    pub const VK_SHADER_GROUP_SHADER_GENERAL_KHR: i32 = 0;
    pub const VK_SHADER_GROUP_SHADER_CLOSEST_HIT_KHR: i32 = 1;
    pub const VK_SHADER_GROUP_SHADER_ANY_HIT_KHR: i32 = 2;
    pub const VK_SHADER_GROUP_SHADER_INTERSECTION_KHR: i32 = 3;
}

pub struct VkMemoryOverallocationBehaviorAMD(i32); //
impl VkMemoryOverallocationBehaviorAMD {
    pub const VK_MEMORY_OVERALLOCATION_BEHAVIOR_DEFAULT_AMD: i32 = 0;
    pub const VK_MEMORY_OVERALLOCATION_BEHAVIOR_ALLOWED_AMD: i32 = 1;
    pub const VK_MEMORY_OVERALLOCATION_BEHAVIOR_DISALLOWED_AMD: i32 = 2;
}

pub type VkFramebufferCreateFlagBits = VkFramebufferCreateFlags; //
impl VkFramebufferCreateFlagBits {
}

pub type VkQueryPoolCreateFlagBits = VkQueryPoolCreateFlags; //
impl VkQueryPoolCreateFlagBits {
}

pub type VkDeviceDiagnosticsConfigFlagBitsNV = VkDeviceDiagnosticsConfigFlagsNV; //
impl VkDeviceDiagnosticsConfigFlagBitsNV {
    pub const VK_DEVICE_DIAGNOSTICS_CONFIG_ENABLE_SHADER_DEBUG_INFO_BIT_NV: VkDeviceDiagnosticsConfigFlagsNV = VkDeviceDiagnosticsConfigFlagsNV(1);
    pub const VK_DEVICE_DIAGNOSTICS_CONFIG_ENABLE_RESOURCE_TRACKING_BIT_NV: VkDeviceDiagnosticsConfigFlagsNV = VkDeviceDiagnosticsConfigFlagsNV(2);
    pub const VK_DEVICE_DIAGNOSTICS_CONFIG_ENABLE_AUTOMATIC_CHECKPOINTS_BIT_NV: VkDeviceDiagnosticsConfigFlagsNV = VkDeviceDiagnosticsConfigFlagsNV(4);
    pub const VK_DEVICE_DIAGNOSTICS_CONFIG_ENABLE_SHADER_ERROR_REPORTING_BIT_NV: VkDeviceDiagnosticsConfigFlagsNV = VkDeviceDiagnosticsConfigFlagsNV(8);
}

pub type VkPipelineCreationFeedbackFlagBits = VkPipelineCreationFeedbackFlags; //
impl VkPipelineCreationFeedbackFlagBits {
    pub const VK_PIPELINE_CREATION_FEEDBACK_VALID_BIT: VkPipelineCreationFeedbackFlags = VkPipelineCreationFeedbackFlags(1);
    pub const VK_PIPELINE_CREATION_FEEDBACK_APPLICATION_PIPELINE_CACHE_HIT_BIT: VkPipelineCreationFeedbackFlags = VkPipelineCreationFeedbackFlags(2);
    pub const VK_PIPELINE_CREATION_FEEDBACK_BASE_PIPELINE_ACCELERATION_BIT: VkPipelineCreationFeedbackFlags = VkPipelineCreationFeedbackFlags(4);
}

pub struct VkFullScreenExclusiveEXT(i32); //
impl VkFullScreenExclusiveEXT {
    pub const VK_FULL_SCREEN_EXCLUSIVE_DEFAULT_EXT: i32 = 0;
    pub const VK_FULL_SCREEN_EXCLUSIVE_ALLOWED_EXT: i32 = 1;
    pub const VK_FULL_SCREEN_EXCLUSIVE_DISALLOWED_EXT: i32 = 2;
    pub const VK_FULL_SCREEN_EXCLUSIVE_APPLICATION_CONTROLLED_EXT: i32 = 3;
}

pub struct VkPerformanceCounterScopeKHR(i32); //
impl VkPerformanceCounterScopeKHR {
    pub const VK_PERFORMANCE_COUNTER_SCOPE_COMMAND_BUFFER_KHR: i32 = 0;
    pub const VK_PERFORMANCE_COUNTER_SCOPE_RENDER_PASS_KHR: i32 = 1;
    pub const VK_PERFORMANCE_COUNTER_SCOPE_COMMAND_KHR: i32 = 2;
    pub const VK_QUERY_SCOPE_COMMAND_BUFFER_KHR: i32 = Self::VK_PERFORMANCE_COUNTER_SCOPE_COMMAND_BUFFER_KHR;
    pub const VK_QUERY_SCOPE_RENDER_PASS_KHR: i32 = Self::VK_PERFORMANCE_COUNTER_SCOPE_RENDER_PASS_KHR;
    pub const VK_QUERY_SCOPE_COMMAND_KHR: i32 = Self::VK_PERFORMANCE_COUNTER_SCOPE_COMMAND_KHR;
}

pub type VkMemoryDecompressionMethodFlagBitsEXT = VkMemoryDecompressionMethodFlagsEXT; //
impl VkMemoryDecompressionMethodFlagBitsEXT {
    pub const VK_MEMORY_DECOMPRESSION_METHOD_GDEFLATE_1_0_BIT_EXT: VkMemoryDecompressionMethodFlagsEXT = VkMemoryDecompressionMethodFlagsEXT(1);
    pub const VK_MEMORY_DECOMPRESSION_METHOD_GDEFLATE_1_0_BIT_NV: VkMemoryDecompressionMethodFlagsEXT = Self::VK_MEMORY_DECOMPRESSION_METHOD_GDEFLATE_1_0_BIT_EXT;
}

pub struct VkPerformanceCounterUnitKHR(i32); //
impl VkPerformanceCounterUnitKHR {
    pub const VK_PERFORMANCE_COUNTER_UNIT_GENERIC_KHR: i32 = 0;
    pub const VK_PERFORMANCE_COUNTER_UNIT_PERCENTAGE_KHR: i32 = 1;
    pub const VK_PERFORMANCE_COUNTER_UNIT_NANOSECONDS_KHR: i32 = 2;
    pub const VK_PERFORMANCE_COUNTER_UNIT_BYTES_KHR: i32 = 3;
    pub const VK_PERFORMANCE_COUNTER_UNIT_BYTES_PER_SECOND_KHR: i32 = 4;
    pub const VK_PERFORMANCE_COUNTER_UNIT_KELVIN_KHR: i32 = 5;
    pub const VK_PERFORMANCE_COUNTER_UNIT_WATTS_KHR: i32 = 6;
    pub const VK_PERFORMANCE_COUNTER_UNIT_VOLTS_KHR: i32 = 7;
    pub const VK_PERFORMANCE_COUNTER_UNIT_AMPS_KHR: i32 = 8;
    pub const VK_PERFORMANCE_COUNTER_UNIT_HERTZ_KHR: i32 = 9;
    pub const VK_PERFORMANCE_COUNTER_UNIT_CYCLES_KHR: i32 = 10;
}

pub struct VkPerformanceCounterStorageKHR(i32); //
impl VkPerformanceCounterStorageKHR {
    pub const VK_PERFORMANCE_COUNTER_STORAGE_INT32_KHR: i32 = 0;
    pub const VK_PERFORMANCE_COUNTER_STORAGE_INT64_KHR: i32 = 1;
    pub const VK_PERFORMANCE_COUNTER_STORAGE_UINT32_KHR: i32 = 2;
    pub const VK_PERFORMANCE_COUNTER_STORAGE_UINT64_KHR: i32 = 3;
    pub const VK_PERFORMANCE_COUNTER_STORAGE_FLOAT32_KHR: i32 = 4;
    pub const VK_PERFORMANCE_COUNTER_STORAGE_FLOAT64_KHR: i32 = 5;
}

pub type VkPerformanceCounterDescriptionFlagBitsKHR = VkPerformanceCounterDescriptionFlagsKHR; //
impl VkPerformanceCounterDescriptionFlagBitsKHR {
    pub const VK_PERFORMANCE_COUNTER_DESCRIPTION_PERFORMANCE_IMPACTING_BIT_KHR: VkPerformanceCounterDescriptionFlagsKHR = VkPerformanceCounterDescriptionFlagsKHR(1);
    pub const VK_PERFORMANCE_COUNTER_DESCRIPTION_PERFORMANCE_IMPACTING_KHR: VkPerformanceCounterDescriptionFlagsKHR = Self::VK_PERFORMANCE_COUNTER_DESCRIPTION_PERFORMANCE_IMPACTING_BIT_KHR;
    pub const VK_PERFORMANCE_COUNTER_DESCRIPTION_CONCURRENTLY_IMPACTED_BIT_KHR: VkPerformanceCounterDescriptionFlagsKHR = VkPerformanceCounterDescriptionFlagsKHR(2);
    pub const VK_PERFORMANCE_COUNTER_DESCRIPTION_CONCURRENTLY_IMPACTED_KHR: VkPerformanceCounterDescriptionFlagsKHR = Self::VK_PERFORMANCE_COUNTER_DESCRIPTION_CONCURRENTLY_IMPACTED_BIT_KHR;
}

pub type VkAcquireProfilingLockFlagBitsKHR = VkAcquireProfilingLockFlagsKHR; //
impl VkAcquireProfilingLockFlagBitsKHR {
}

pub type VkShaderCorePropertiesFlagBitsAMD = VkShaderCorePropertiesFlagsAMD; //
impl VkShaderCorePropertiesFlagBitsAMD {
}

pub type VkRefreshObjectFlagBitsKHR = VkRefreshObjectFlagsKHR; //
impl VkRefreshObjectFlagBitsKHR {
}

pub struct VkPerformanceConfigurationTypeINTEL(i32); //
impl VkPerformanceConfigurationTypeINTEL {
    pub const VK_PERFORMANCE_CONFIGURATION_TYPE_COMMAND_QUEUE_METRICS_DISCOVERY_ACTIVATED_INTEL: i32 = 0;
}

pub struct VkQueryPoolSamplingModeINTEL(i32); //
impl VkQueryPoolSamplingModeINTEL {
    pub const VK_QUERY_POOL_SAMPLING_MODE_MANUAL_INTEL: i32 = 0;
}

pub struct VkPerformanceOverrideTypeINTEL(i32); //
impl VkPerformanceOverrideTypeINTEL {
    pub const VK_PERFORMANCE_OVERRIDE_TYPE_NULL_HARDWARE_INTEL: i32 = 0;
    pub const VK_PERFORMANCE_OVERRIDE_TYPE_FLUSH_GPU_CACHES_INTEL: i32 = 1;
}

pub struct VkPerformanceParameterTypeINTEL(i32); //
impl VkPerformanceParameterTypeINTEL {
    pub const VK_PERFORMANCE_PARAMETER_TYPE_HW_COUNTERS_SUPPORTED_INTEL: i32 = 0;
    pub const VK_PERFORMANCE_PARAMETER_TYPE_STREAM_MARKER_VALID_BITS_INTEL: i32 = 1;
}

pub struct VkPerformanceValueTypeINTEL(i32); //
impl VkPerformanceValueTypeINTEL {
    pub const VK_PERFORMANCE_VALUE_TYPE_UINT32_INTEL: i32 = 0;
    pub const VK_PERFORMANCE_VALUE_TYPE_UINT64_INTEL: i32 = 1;
    pub const VK_PERFORMANCE_VALUE_TYPE_FLOAT_INTEL: i32 = 2;
    pub const VK_PERFORMANCE_VALUE_TYPE_BOOL_INTEL: i32 = 3;
    pub const VK_PERFORMANCE_VALUE_TYPE_STRING_INTEL: i32 = 4;
}

pub struct VkShaderFloatControlsIndependence(i32); //
impl VkShaderFloatControlsIndependence {
    pub const VK_SHADER_FLOAT_CONTROLS_INDEPENDENCE_32_BIT_ONLY: i32 = 0;
    pub const VK_SHADER_FLOAT_CONTROLS_INDEPENDENCE_ALL: i32 = 1;
    pub const VK_SHADER_FLOAT_CONTROLS_INDEPENDENCE_NONE: i32 = 2;
}

pub struct VkPipelineExecutableStatisticFormatKHR(i32); //
impl VkPipelineExecutableStatisticFormatKHR {
    pub const VK_PIPELINE_EXECUTABLE_STATISTIC_FORMAT_BOOL32_KHR: i32 = 0;
    pub const VK_PIPELINE_EXECUTABLE_STATISTIC_FORMAT_INT64_KHR: i32 = 1;
    pub const VK_PIPELINE_EXECUTABLE_STATISTIC_FORMAT_UINT64_KHR: i32 = 2;
    pub const VK_PIPELINE_EXECUTABLE_STATISTIC_FORMAT_FLOAT64_KHR: i32 = 3;
}

pub struct VkLineRasterizationMode(i32); //
impl VkLineRasterizationMode {
    pub const VK_LINE_RASTERIZATION_MODE_DEFAULT: i32 = 0;
    pub const VK_LINE_RASTERIZATION_MODE_RECTANGULAR: i32 = 1;
    pub const VK_LINE_RASTERIZATION_MODE_BRESENHAM: i32 = 2;
    pub const VK_LINE_RASTERIZATION_MODE_RECTANGULAR_SMOOTH: i32 = 3;
}

pub type VkShaderModuleCreateFlagBits = VkShaderModuleCreateFlags; //
impl VkShaderModuleCreateFlagBits {
}

pub type VkPipelineCompilerControlFlagBitsAMD = VkPipelineCompilerControlFlagsAMD; //
impl VkPipelineCompilerControlFlagBitsAMD {
}

pub struct VkFaultLevel(i32); //
impl VkFaultLevel {
    pub const VK_FAULT_LEVEL_UNASSIGNED: i32 = 0;
    pub const VK_FAULT_LEVEL_CRITICAL: i32 = 1;
    pub const VK_FAULT_LEVEL_RECOVERABLE: i32 = 2;
    pub const VK_FAULT_LEVEL_WARNING: i32 = 3;
}

pub struct VkFaultType(i32); //
impl VkFaultType {
    pub const VK_FAULT_TYPE_INVALID: i32 = 0;
    pub const VK_FAULT_TYPE_UNASSIGNED: i32 = 1;
    pub const VK_FAULT_TYPE_IMPLEMENTATION: i32 = 2;
    pub const VK_FAULT_TYPE_SYSTEM: i32 = 3;
    pub const VK_FAULT_TYPE_PHYSICAL_DEVICE: i32 = 4;
    pub const VK_FAULT_TYPE_COMMAND_BUFFER_FULL: i32 = 5;
    pub const VK_FAULT_TYPE_INVALID_API_USAGE: i32 = 6;
}

pub struct VkFaultQueryBehavior(i32); //
impl VkFaultQueryBehavior {
    pub const VK_FAULT_QUERY_BEHAVIOR_GET_AND_CLEAR_ALL_FAULTS: i32 = 0;
}

pub type VkToolPurposeFlagBits = VkToolPurposeFlags; //
impl VkToolPurposeFlagBits {
    pub const VK_TOOL_PURPOSE_VALIDATION_BIT: VkToolPurposeFlags = VkToolPurposeFlags(1);
    pub const VK_TOOL_PURPOSE_PROFILING_BIT: VkToolPurposeFlags = VkToolPurposeFlags(2);
    pub const VK_TOOL_PURPOSE_TRACING_BIT: VkToolPurposeFlags = VkToolPurposeFlags(4);
    pub const VK_TOOL_PURPOSE_ADDITIONAL_FEATURES_BIT: VkToolPurposeFlags = VkToolPurposeFlags(8);
    pub const VK_TOOL_PURPOSE_MODIFYING_FEATURES_BIT: VkToolPurposeFlags = VkToolPurposeFlags(16);
}

pub struct VkPipelineMatchControl(i32); //
impl VkPipelineMatchControl {
    pub const VK_PIPELINE_MATCH_CONTROL_APPLICATION_UUID_EXACT_MATCH: i32 = 0;
}

pub struct VkFragmentShadingRateCombinerOpKHR(i32); //
impl VkFragmentShadingRateCombinerOpKHR {
    pub const VK_FRAGMENT_SHADING_RATE_COMBINER_OP_KEEP_KHR: i32 = 0;
    pub const VK_FRAGMENT_SHADING_RATE_COMBINER_OP_REPLACE_KHR: i32 = 1;
    pub const VK_FRAGMENT_SHADING_RATE_COMBINER_OP_MIN_KHR: i32 = 2;
    pub const VK_FRAGMENT_SHADING_RATE_COMBINER_OP_MAX_KHR: i32 = 3;
    pub const VK_FRAGMENT_SHADING_RATE_COMBINER_OP_MUL_KHR: i32 = 4;
}

pub struct VkFragmentShadingRateNV(i32); //
impl VkFragmentShadingRateNV {
    pub const VK_FRAGMENT_SHADING_RATE_1_INVOCATION_PER_PIXEL_NV: i32 = 0;
    pub const VK_FRAGMENT_SHADING_RATE_1_INVOCATION_PER_1X2_PIXELS_NV: i32 = 1;
    pub const VK_FRAGMENT_SHADING_RATE_1_INVOCATION_PER_2X1_PIXELS_NV: i32 = 4;
    pub const VK_FRAGMENT_SHADING_RATE_1_INVOCATION_PER_2X2_PIXELS_NV: i32 = 5;
    pub const VK_FRAGMENT_SHADING_RATE_1_INVOCATION_PER_2X4_PIXELS_NV: i32 = 6;
    pub const VK_FRAGMENT_SHADING_RATE_1_INVOCATION_PER_4X2_PIXELS_NV: i32 = 9;
    pub const VK_FRAGMENT_SHADING_RATE_1_INVOCATION_PER_4X4_PIXELS_NV: i32 = 10;
    pub const VK_FRAGMENT_SHADING_RATE_2_INVOCATIONS_PER_PIXEL_NV: i32 = 11;
    pub const VK_FRAGMENT_SHADING_RATE_4_INVOCATIONS_PER_PIXEL_NV: i32 = 12;
    pub const VK_FRAGMENT_SHADING_RATE_8_INVOCATIONS_PER_PIXEL_NV: i32 = 13;
    pub const VK_FRAGMENT_SHADING_RATE_16_INVOCATIONS_PER_PIXEL_NV: i32 = 14;
    pub const VK_FRAGMENT_SHADING_RATE_NO_INVOCATIONS_NV: i32 = 15;
}

pub struct VkFragmentShadingRateTypeNV(i32); //
impl VkFragmentShadingRateTypeNV {
    pub const VK_FRAGMENT_SHADING_RATE_TYPE_FRAGMENT_SIZE_NV: i32 = 0;
    pub const VK_FRAGMENT_SHADING_RATE_TYPE_ENUMS_NV: i32 = 1;
}

pub struct VkSubpassMergeStatusEXT(i32); //
impl VkSubpassMergeStatusEXT {
    pub const VK_SUBPASS_MERGE_STATUS_MERGED_EXT: i32 = 0;
    pub const VK_SUBPASS_MERGE_STATUS_DISALLOWED_EXT: i32 = 1;
    pub const VK_SUBPASS_MERGE_STATUS_NOT_MERGED_SIDE_EFFECTS_EXT: i32 = 2;
    pub const VK_SUBPASS_MERGE_STATUS_NOT_MERGED_SAMPLES_MISMATCH_EXT: i32 = 3;
    pub const VK_SUBPASS_MERGE_STATUS_NOT_MERGED_VIEWS_MISMATCH_EXT: i32 = 4;
    pub const VK_SUBPASS_MERGE_STATUS_NOT_MERGED_ALIASING_EXT: i32 = 5;
    pub const VK_SUBPASS_MERGE_STATUS_NOT_MERGED_DEPENDENCIES_EXT: i32 = 6;
    pub const VK_SUBPASS_MERGE_STATUS_NOT_MERGED_INCOMPATIBLE_INPUT_ATTACHMENT_EXT: i32 = 7;
    pub const VK_SUBPASS_MERGE_STATUS_NOT_MERGED_TOO_MANY_ATTACHMENTS_EXT: i32 = 8;
    pub const VK_SUBPASS_MERGE_STATUS_NOT_MERGED_INSUFFICIENT_STORAGE_EXT: i32 = 9;
    pub const VK_SUBPASS_MERGE_STATUS_NOT_MERGED_DEPTH_STENCIL_COUNT_EXT: i32 = 10;
    pub const VK_SUBPASS_MERGE_STATUS_NOT_MERGED_RESOLVE_ATTACHMENT_REUSE_EXT: i32 = 11;
    pub const VK_SUBPASS_MERGE_STATUS_NOT_MERGED_SINGLE_SUBPASS_EXT: i32 = 12;
    pub const VK_SUBPASS_MERGE_STATUS_NOT_MERGED_UNSPECIFIED_EXT: i32 = 13;
}

pub type VkAccessFlagBits2 = VkAccessFlags2; //
impl VkAccessFlagBits2 {
    pub const VK_ACCESS_2_NONE: VkAccessFlags2 = VkAccessFlags2(0);
    pub const VK_ACCESS_2_INDIRECT_COMMAND_READ_BIT: VkAccessFlags2 = VkAccessFlags2(1);
    pub const VK_ACCESS_2_INDEX_READ_BIT: VkAccessFlags2 = VkAccessFlags2(2);
    pub const VK_ACCESS_2_VERTEX_ATTRIBUTE_READ_BIT: VkAccessFlags2 = VkAccessFlags2(4);
    pub const VK_ACCESS_2_UNIFORM_READ_BIT: VkAccessFlags2 = VkAccessFlags2(8);
    pub const VK_ACCESS_2_INPUT_ATTACHMENT_READ_BIT: VkAccessFlags2 = VkAccessFlags2(16);
    pub const VK_ACCESS_2_SHADER_READ_BIT: VkAccessFlags2 = VkAccessFlags2(32);
    pub const VK_ACCESS_2_SHADER_WRITE_BIT: VkAccessFlags2 = VkAccessFlags2(64);
    pub const VK_ACCESS_2_COLOR_ATTACHMENT_READ_BIT: VkAccessFlags2 = VkAccessFlags2(128);
    pub const VK_ACCESS_2_COLOR_ATTACHMENT_WRITE_BIT: VkAccessFlags2 = VkAccessFlags2(256);
    pub const VK_ACCESS_2_DEPTH_STENCIL_ATTACHMENT_READ_BIT: VkAccessFlags2 = VkAccessFlags2(512);
    pub const VK_ACCESS_2_DEPTH_STENCIL_ATTACHMENT_WRITE_BIT: VkAccessFlags2 = VkAccessFlags2(1024);
    pub const VK_ACCESS_2_TRANSFER_READ_BIT: VkAccessFlags2 = VkAccessFlags2(2048);
    pub const VK_ACCESS_2_TRANSFER_WRITE_BIT: VkAccessFlags2 = VkAccessFlags2(4096);
    pub const VK_ACCESS_2_HOST_READ_BIT: VkAccessFlags2 = VkAccessFlags2(8192);
    pub const VK_ACCESS_2_HOST_WRITE_BIT: VkAccessFlags2 = VkAccessFlags2(16384);
    pub const VK_ACCESS_2_MEMORY_READ_BIT: VkAccessFlags2 = VkAccessFlags2(32768);
    pub const VK_ACCESS_2_MEMORY_WRITE_BIT: VkAccessFlags2 = VkAccessFlags2(65536);
    pub const VK_ACCESS_2_SHADER_SAMPLED_READ_BIT: VkAccessFlags2 = VkAccessFlags2(4294967296);
    pub const VK_ACCESS_2_SHADER_STORAGE_READ_BIT: VkAccessFlags2 = VkAccessFlags2(8589934592);
    pub const VK_ACCESS_2_SHADER_STORAGE_WRITE_BIT: VkAccessFlags2 = VkAccessFlags2(17179869184);
}

pub type VkPipelineStageFlagBits2 = VkPipelineStageFlags2; //
impl VkPipelineStageFlagBits2 {
    pub const VK_PIPELINE_STAGE_2_NONE: VkPipelineStageFlags2 = VkPipelineStageFlags2(0);
    pub const VK_PIPELINE_STAGE_2_TOP_OF_PIPE_BIT: VkPipelineStageFlags2 = VkPipelineStageFlags2(1);
    pub const VK_PIPELINE_STAGE_2_DRAW_INDIRECT_BIT: VkPipelineStageFlags2 = VkPipelineStageFlags2(2);
    pub const VK_PIPELINE_STAGE_2_VERTEX_INPUT_BIT: VkPipelineStageFlags2 = VkPipelineStageFlags2(4);
    pub const VK_PIPELINE_STAGE_2_VERTEX_SHADER_BIT: VkPipelineStageFlags2 = VkPipelineStageFlags2(8);
    pub const VK_PIPELINE_STAGE_2_TESSELLATION_CONTROL_SHADER_BIT: VkPipelineStageFlags2 = VkPipelineStageFlags2(16);
    pub const VK_PIPELINE_STAGE_2_TESSELLATION_EVALUATION_SHADER_BIT: VkPipelineStageFlags2 = VkPipelineStageFlags2(32);
    pub const VK_PIPELINE_STAGE_2_GEOMETRY_SHADER_BIT: VkPipelineStageFlags2 = VkPipelineStageFlags2(64);
    pub const VK_PIPELINE_STAGE_2_FRAGMENT_SHADER_BIT: VkPipelineStageFlags2 = VkPipelineStageFlags2(128);
    pub const VK_PIPELINE_STAGE_2_EARLY_FRAGMENT_TESTS_BIT: VkPipelineStageFlags2 = VkPipelineStageFlags2(256);
    pub const VK_PIPELINE_STAGE_2_LATE_FRAGMENT_TESTS_BIT: VkPipelineStageFlags2 = VkPipelineStageFlags2(512);
    pub const VK_PIPELINE_STAGE_2_COLOR_ATTACHMENT_OUTPUT_BIT: VkPipelineStageFlags2 = VkPipelineStageFlags2(1024);
    pub const VK_PIPELINE_STAGE_2_COMPUTE_SHADER_BIT: VkPipelineStageFlags2 = VkPipelineStageFlags2(2048);
    pub const VK_PIPELINE_STAGE_2_ALL_TRANSFER_BIT: VkPipelineStageFlags2 = VkPipelineStageFlags2(4096);
    pub const VK_PIPELINE_STAGE_2_TRANSFER_BIT: VkPipelineStageFlags2 = Self::VK_PIPELINE_STAGE_2_ALL_TRANSFER_BIT;
    pub const VK_PIPELINE_STAGE_2_BOTTOM_OF_PIPE_BIT: VkPipelineStageFlags2 = VkPipelineStageFlags2(8192);
    pub const VK_PIPELINE_STAGE_2_HOST_BIT: VkPipelineStageFlags2 = VkPipelineStageFlags2(16384);
    pub const VK_PIPELINE_STAGE_2_ALL_GRAPHICS_BIT: VkPipelineStageFlags2 = VkPipelineStageFlags2(32768);
    pub const VK_PIPELINE_STAGE_2_ALL_COMMANDS_BIT: VkPipelineStageFlags2 = VkPipelineStageFlags2(65536);
    pub const VK_PIPELINE_STAGE_2_COPY_BIT: VkPipelineStageFlags2 = VkPipelineStageFlags2(4294967296);
    pub const VK_PIPELINE_STAGE_2_RESOLVE_BIT: VkPipelineStageFlags2 = VkPipelineStageFlags2(8589934592);
    pub const VK_PIPELINE_STAGE_2_BLIT_BIT: VkPipelineStageFlags2 = VkPipelineStageFlags2(17179869184);
    pub const VK_PIPELINE_STAGE_2_CLEAR_BIT: VkPipelineStageFlags2 = VkPipelineStageFlags2(34359738368);
    pub const VK_PIPELINE_STAGE_2_INDEX_INPUT_BIT: VkPipelineStageFlags2 = VkPipelineStageFlags2(68719476736);
    pub const VK_PIPELINE_STAGE_2_VERTEX_ATTRIBUTE_INPUT_BIT: VkPipelineStageFlags2 = VkPipelineStageFlags2(137438953472);
    pub const VK_PIPELINE_STAGE_2_PRE_RASTERIZATION_SHADERS_BIT: VkPipelineStageFlags2 = VkPipelineStageFlags2(274877906944);
}

pub type VkSubmitFlagBits = VkSubmitFlags; //
impl VkSubmitFlagBits {
    pub const VK_SUBMIT_PROTECTED_BIT: VkSubmitFlags = VkSubmitFlags(1);
}

pub type VkEventCreateFlagBits = VkEventCreateFlags; //
impl VkEventCreateFlagBits {
}

pub type VkPipelineLayoutCreateFlagBits = VkPipelineLayoutCreateFlags; //
impl VkPipelineLayoutCreateFlagBits {
}

pub struct VkSciSyncClientTypeNV(i32); //
impl VkSciSyncClientTypeNV {
    pub const VK_SCI_SYNC_CLIENT_TYPE_SIGNALER_NV: i32 = 0;
    pub const VK_SCI_SYNC_CLIENT_TYPE_WAITER_NV: i32 = 1;
    pub const VK_SCI_SYNC_CLIENT_TYPE_SIGNALER_WAITER_NV: i32 = 2;
}

pub struct VkSciSyncPrimitiveTypeNV(i32); //
impl VkSciSyncPrimitiveTypeNV {
    pub const VK_SCI_SYNC_PRIMITIVE_TYPE_FENCE_NV: i32 = 0;
    pub const VK_SCI_SYNC_PRIMITIVE_TYPE_SEMAPHORE_NV: i32 = 1;
}

pub struct VkProvokingVertexModeEXT(i32); //
impl VkProvokingVertexModeEXT {
    pub const VK_PROVOKING_VERTEX_MODE_FIRST_VERTEX_EXT: i32 = 0;
    pub const VK_PROVOKING_VERTEX_MODE_LAST_VERTEX_EXT: i32 = 1;
}

pub struct VkPipelineCacheValidationVersion(i32); //
impl VkPipelineCacheValidationVersion {
    pub const VK_PIPELINE_CACHE_VALIDATION_VERSION_SAFETY_CRITICAL_ONE: i32 = 1;
}

pub struct VkAccelerationStructureMotionInstanceTypeNV(i32); //
impl VkAccelerationStructureMotionInstanceTypeNV {
    pub const VK_ACCELERATION_STRUCTURE_MOTION_INSTANCE_TYPE_STATIC_NV: i32 = 0;
    pub const VK_ACCELERATION_STRUCTURE_MOTION_INSTANCE_TYPE_MATRIX_MOTION_NV: i32 = 1;
    pub const VK_ACCELERATION_STRUCTURE_MOTION_INSTANCE_TYPE_SRT_MOTION_NV: i32 = 2;
}

pub type VkPipelineColorBlendStateCreateFlagBits = VkPipelineColorBlendStateCreateFlags; //
impl VkPipelineColorBlendStateCreateFlagBits {
}

pub type VkPipelineDepthStencilStateCreateFlagBits = VkPipelineDepthStencilStateCreateFlags; //
impl VkPipelineDepthStencilStateCreateFlagBits {
}

pub type VkGraphicsPipelineLibraryFlagBitsEXT = VkGraphicsPipelineLibraryFlagsEXT; //
impl VkGraphicsPipelineLibraryFlagBitsEXT {
    pub const VK_GRAPHICS_PIPELINE_LIBRARY_VERTEX_INPUT_INTERFACE_BIT_EXT: VkGraphicsPipelineLibraryFlagsEXT = VkGraphicsPipelineLibraryFlagsEXT(1);
    pub const VK_GRAPHICS_PIPELINE_LIBRARY_PRE_RASTERIZATION_SHADERS_BIT_EXT: VkGraphicsPipelineLibraryFlagsEXT = VkGraphicsPipelineLibraryFlagsEXT(2);
    pub const VK_GRAPHICS_PIPELINE_LIBRARY_FRAGMENT_SHADER_BIT_EXT: VkGraphicsPipelineLibraryFlagsEXT = VkGraphicsPipelineLibraryFlagsEXT(4);
    pub const VK_GRAPHICS_PIPELINE_LIBRARY_FRAGMENT_OUTPUT_INTERFACE_BIT_EXT: VkGraphicsPipelineLibraryFlagsEXT = VkGraphicsPipelineLibraryFlagsEXT(8);
}

pub type VkRenderingAttachmentFlagBitsKHR = VkRenderingAttachmentFlagsKHR; //
impl VkRenderingAttachmentFlagBitsKHR {
}

pub type VkResolveImageFlagBitsKHR = VkResolveImageFlagsKHR; //
impl VkResolveImageFlagBitsKHR {
}

pub type VkDeviceAddressBindingFlagBitsEXT = VkDeviceAddressBindingFlagsEXT; //
impl VkDeviceAddressBindingFlagBitsEXT {
    pub const VK_DEVICE_ADDRESS_BINDING_INTERNAL_OBJECT_BIT_EXT: VkDeviceAddressBindingFlagsEXT = VkDeviceAddressBindingFlagsEXT(1);
}

pub struct VkDeviceAddressBindingTypeEXT(i32); //
impl VkDeviceAddressBindingTypeEXT {
    pub const VK_DEVICE_ADDRESS_BINDING_TYPE_BIND_EXT: i32 = 0;
    pub const VK_DEVICE_ADDRESS_BINDING_TYPE_UNBIND_EXT: i32 = 1;
}

pub type VkFrameBoundaryFlagBitsEXT = VkFrameBoundaryFlagsEXT; //
impl VkFrameBoundaryFlagBitsEXT {
    pub const VK_FRAME_BOUNDARY_FRAME_END_BIT_EXT: VkFrameBoundaryFlagsEXT = VkFrameBoundaryFlagsEXT(1);
}

pub type VkPresentScalingFlagBitsKHR = VkPresentScalingFlagsKHR; //
impl VkPresentScalingFlagBitsKHR {
    pub const VK_PRESENT_SCALING_ONE_TO_ONE_BIT_KHR: VkPresentScalingFlagsKHR = VkPresentScalingFlagsKHR(1);
    pub const VK_PRESENT_SCALING_ONE_TO_ONE_BIT_EXT: VkPresentScalingFlagsKHR = Self::VK_PRESENT_SCALING_ONE_TO_ONE_BIT_KHR;
    pub const VK_PRESENT_SCALING_ASPECT_RATIO_STRETCH_BIT_KHR: VkPresentScalingFlagsKHR = VkPresentScalingFlagsKHR(2);
    pub const VK_PRESENT_SCALING_ASPECT_RATIO_STRETCH_BIT_EXT: VkPresentScalingFlagsKHR = Self::VK_PRESENT_SCALING_ASPECT_RATIO_STRETCH_BIT_KHR;
    pub const VK_PRESENT_SCALING_STRETCH_BIT_KHR: VkPresentScalingFlagsKHR = VkPresentScalingFlagsKHR(4);
    pub const VK_PRESENT_SCALING_STRETCH_BIT_EXT: VkPresentScalingFlagsKHR = Self::VK_PRESENT_SCALING_STRETCH_BIT_KHR;
}

pub type VkPresentGravityFlagBitsKHR = VkPresentGravityFlagsKHR; //
impl VkPresentGravityFlagBitsKHR {
    pub const VK_PRESENT_GRAVITY_MIN_BIT_KHR: VkPresentGravityFlagsKHR = VkPresentGravityFlagsKHR(1);
    pub const VK_PRESENT_GRAVITY_MIN_BIT_EXT: VkPresentGravityFlagsKHR = Self::VK_PRESENT_GRAVITY_MIN_BIT_KHR;
    pub const VK_PRESENT_GRAVITY_MAX_BIT_KHR: VkPresentGravityFlagsKHR = VkPresentGravityFlagsKHR(2);
    pub const VK_PRESENT_GRAVITY_MAX_BIT_EXT: VkPresentGravityFlagsKHR = Self::VK_PRESENT_GRAVITY_MAX_BIT_KHR;
    pub const VK_PRESENT_GRAVITY_CENTERED_BIT_KHR: VkPresentGravityFlagsKHR = VkPresentGravityFlagsKHR(4);
    pub const VK_PRESENT_GRAVITY_CENTERED_BIT_EXT: VkPresentGravityFlagsKHR = Self::VK_PRESENT_GRAVITY_CENTERED_BIT_KHR;
}

pub type VkPhysicalDeviceSchedulingControlsFlagBitsARM = VkPhysicalDeviceSchedulingControlsFlagsARM; //
impl VkPhysicalDeviceSchedulingControlsFlagBitsARM {
    pub const VK_PHYSICAL_DEVICE_SCHEDULING_CONTROLS_SHADER_CORE_COUNT_ARM: VkPhysicalDeviceSchedulingControlsFlagsARM = VkPhysicalDeviceSchedulingControlsFlagsARM(1);
    pub const VK_PHYSICAL_DEVICE_SCHEDULING_CONTROLS_DISPATCH_PARAMETERS_ARM: VkPhysicalDeviceSchedulingControlsFlagsARM = VkPhysicalDeviceSchedulingControlsFlagsARM(2);
}

pub type VkPresentStageFlagBitsEXT = VkPresentStageFlagsEXT; //
impl VkPresentStageFlagBitsEXT {
    pub const VK_PRESENT_STAGE_QUEUE_OPERATIONS_END_BIT_EXT: VkPresentStageFlagsEXT = VkPresentStageFlagsEXT(1);
    pub const VK_PRESENT_STAGE_REQUEST_DEQUEUED_BIT_EXT: VkPresentStageFlagsEXT = VkPresentStageFlagsEXT(2);
    pub const VK_PRESENT_STAGE_IMAGE_FIRST_PIXEL_OUT_BIT_EXT: VkPresentStageFlagsEXT = VkPresentStageFlagsEXT(4);
    pub const VK_PRESENT_STAGE_IMAGE_FIRST_PIXEL_VISIBLE_BIT_EXT: VkPresentStageFlagsEXT = VkPresentStageFlagsEXT(8);
}

pub type VkPastPresentationTimingFlagBitsEXT = VkPastPresentationTimingFlagsEXT; //
impl VkPastPresentationTimingFlagBitsEXT {
    pub const VK_PAST_PRESENTATION_TIMING_ALLOW_PARTIAL_RESULTS_BIT_EXT: VkPastPresentationTimingFlagsEXT = VkPastPresentationTimingFlagsEXT(1);
    pub const VK_PAST_PRESENTATION_TIMING_ALLOW_OUT_OF_ORDER_RESULTS_BIT_EXT: VkPastPresentationTimingFlagsEXT = VkPastPresentationTimingFlagsEXT(2);
}

pub type VkPresentTimingInfoFlagBitsEXT = VkPresentTimingInfoFlagsEXT; //
impl VkPresentTimingInfoFlagBitsEXT {
    pub const VK_PRESENT_TIMING_INFO_PRESENT_AT_RELATIVE_TIME_BIT_EXT: VkPresentTimingInfoFlagsEXT = VkPresentTimingInfoFlagsEXT(1);
    pub const VK_PRESENT_TIMING_INFO_PRESENT_AT_NEAREST_REFRESH_CYCLE_BIT_EXT: VkPresentTimingInfoFlagsEXT = VkPresentTimingInfoFlagsEXT(2);
}

pub type VkVideoCodecOperationFlagBitsKHR = VkVideoCodecOperationFlagsKHR; //
impl VkVideoCodecOperationFlagBitsKHR {
    pub const VK_VIDEO_CODEC_OPERATION_NONE_KHR: VkVideoCodecOperationFlagsKHR = VkVideoCodecOperationFlagsKHR(0);
}

pub type VkVideoChromaSubsamplingFlagBitsKHR = VkVideoChromaSubsamplingFlagsKHR; //Vulkan video chroma subsampling definitions
impl VkVideoChromaSubsamplingFlagBitsKHR {
    pub const VK_VIDEO_CHROMA_SUBSAMPLING_INVALID_KHR: VkVideoChromaSubsamplingFlagsKHR = VkVideoChromaSubsamplingFlagsKHR(0);
    pub const VK_VIDEO_CHROMA_SUBSAMPLING_MONOCHROME_BIT_KHR: VkVideoChromaSubsamplingFlagsKHR = VkVideoChromaSubsamplingFlagsKHR(1);
    pub const VK_VIDEO_CHROMA_SUBSAMPLING_420_BIT_KHR: VkVideoChromaSubsamplingFlagsKHR = VkVideoChromaSubsamplingFlagsKHR(2);
    pub const VK_VIDEO_CHROMA_SUBSAMPLING_422_BIT_KHR: VkVideoChromaSubsamplingFlagsKHR = VkVideoChromaSubsamplingFlagsKHR(4);
    pub const VK_VIDEO_CHROMA_SUBSAMPLING_444_BIT_KHR: VkVideoChromaSubsamplingFlagsKHR = VkVideoChromaSubsamplingFlagsKHR(8);
}

pub type VkVideoComponentBitDepthFlagBitsKHR = VkVideoComponentBitDepthFlagsKHR; //Vulkan video component bit depth definitions
impl VkVideoComponentBitDepthFlagBitsKHR {
    pub const VK_VIDEO_COMPONENT_BIT_DEPTH_INVALID_KHR: VkVideoComponentBitDepthFlagsKHR = VkVideoComponentBitDepthFlagsKHR(0);
    pub const VK_VIDEO_COMPONENT_BIT_DEPTH_8_BIT_KHR: VkVideoComponentBitDepthFlagsKHR = VkVideoComponentBitDepthFlagsKHR(1);
    pub const VK_VIDEO_COMPONENT_BIT_DEPTH_10_BIT_KHR: VkVideoComponentBitDepthFlagsKHR = VkVideoComponentBitDepthFlagsKHR(4);
    pub const VK_VIDEO_COMPONENT_BIT_DEPTH_12_BIT_KHR: VkVideoComponentBitDepthFlagsKHR = VkVideoComponentBitDepthFlagsKHR(16);
}

pub type VkVideoCapabilityFlagBitsKHR = VkVideoCapabilityFlagsKHR; //
impl VkVideoCapabilityFlagBitsKHR {
    pub const VK_VIDEO_CAPABILITY_PROTECTED_CONTENT_BIT_KHR: VkVideoCapabilityFlagsKHR = VkVideoCapabilityFlagsKHR(1);
    pub const VK_VIDEO_CAPABILITY_SEPARATE_REFERENCE_IMAGES_BIT_KHR: VkVideoCapabilityFlagsKHR = VkVideoCapabilityFlagsKHR(2);
}

pub type VkVideoSessionCreateFlagBitsKHR = VkVideoSessionCreateFlagsKHR; //
impl VkVideoSessionCreateFlagBitsKHR {
    pub const VK_VIDEO_SESSION_CREATE_PROTECTED_CONTENT_BIT_KHR: VkVideoSessionCreateFlagsKHR = VkVideoSessionCreateFlagsKHR(1);
}

pub type VkVideoSessionParametersCreateFlagBitsKHR = VkVideoSessionParametersCreateFlagsKHR; //
impl VkVideoSessionParametersCreateFlagBitsKHR {
}

pub type VkVideoDecodeH264PictureLayoutFlagBitsKHR = VkVideoDecodeH264PictureLayoutFlagsKHR; //
impl VkVideoDecodeH264PictureLayoutFlagBitsKHR {
    pub const VK_VIDEO_DECODE_H264_PICTURE_LAYOUT_PROGRESSIVE_KHR: VkVideoDecodeH264PictureLayoutFlagsKHR = VkVideoDecodeH264PictureLayoutFlagsKHR(0);
    pub const VK_VIDEO_DECODE_H264_PICTURE_LAYOUT_INTERLACED_INTERLEAVED_LINES_BIT_KHR: VkVideoDecodeH264PictureLayoutFlagsKHR = VkVideoDecodeH264PictureLayoutFlagsKHR(1);
    pub const VK_VIDEO_DECODE_H264_PICTURE_LAYOUT_INTERLACED_SEPARATE_PLANES_BIT_KHR: VkVideoDecodeH264PictureLayoutFlagsKHR = VkVideoDecodeH264PictureLayoutFlagsKHR(2);
}

pub type VkVideoCodingControlFlagBitsKHR = VkVideoCodingControlFlagsKHR; //
impl VkVideoCodingControlFlagBitsKHR {
    pub const VK_VIDEO_CODING_CONTROL_RESET_BIT_KHR: VkVideoCodingControlFlagsKHR = VkVideoCodingControlFlagsKHR(1);
}

pub struct VkQueryResultStatusKHR(i32); //
impl VkQueryResultStatusKHR {
    pub const VK_QUERY_RESULT_STATUS_ERROR_KHR: i32 = -1;
    pub const VK_QUERY_RESULT_STATUS_NOT_READY_KHR: i32 = 0;
    pub const VK_QUERY_RESULT_STATUS_COMPLETE_KHR: i32 = 1;
}

pub type VkVideoDecodeUsageFlagBitsKHR = VkVideoDecodeUsageFlagsKHR; //
impl VkVideoDecodeUsageFlagBitsKHR {
    pub const VK_VIDEO_DECODE_USAGE_DEFAULT_KHR: VkVideoDecodeUsageFlagsKHR = VkVideoDecodeUsageFlagsKHR(0);
    pub const VK_VIDEO_DECODE_USAGE_TRANSCODING_BIT_KHR: VkVideoDecodeUsageFlagsKHR = VkVideoDecodeUsageFlagsKHR(1);
    pub const VK_VIDEO_DECODE_USAGE_OFFLINE_BIT_KHR: VkVideoDecodeUsageFlagsKHR = VkVideoDecodeUsageFlagsKHR(2);
    pub const VK_VIDEO_DECODE_USAGE_STREAMING_BIT_KHR: VkVideoDecodeUsageFlagsKHR = VkVideoDecodeUsageFlagsKHR(4);
}

pub type VkVideoDecodeCapabilityFlagBitsKHR = VkVideoDecodeCapabilityFlagsKHR; //
impl VkVideoDecodeCapabilityFlagBitsKHR {
    pub const VK_VIDEO_DECODE_CAPABILITY_DPB_AND_OUTPUT_COINCIDE_BIT_KHR: VkVideoDecodeCapabilityFlagsKHR = VkVideoDecodeCapabilityFlagsKHR(1);
    pub const VK_VIDEO_DECODE_CAPABILITY_DPB_AND_OUTPUT_DISTINCT_BIT_KHR: VkVideoDecodeCapabilityFlagsKHR = VkVideoDecodeCapabilityFlagsKHR(2);
}

pub type VkVideoEncodeFlagBitsKHR = VkVideoEncodeFlagsKHR; //
impl VkVideoEncodeFlagBitsKHR {
}

pub type VkCooperativeMatrixFlagBitsEXT = VkCooperativeMatrixFlagsEXT; //
impl VkCooperativeMatrixFlagBitsEXT {
}

pub type VkVideoEncodeUsageFlagBitsKHR = VkVideoEncodeUsageFlagsKHR; //
impl VkVideoEncodeUsageFlagBitsKHR {
    pub const VK_VIDEO_ENCODE_USAGE_DEFAULT_KHR: VkVideoEncodeUsageFlagsKHR = VkVideoEncodeUsageFlagsKHR(0);
    pub const VK_VIDEO_ENCODE_USAGE_TRANSCODING_BIT_KHR: VkVideoEncodeUsageFlagsKHR = VkVideoEncodeUsageFlagsKHR(1);
    pub const VK_VIDEO_ENCODE_USAGE_STREAMING_BIT_KHR: VkVideoEncodeUsageFlagsKHR = VkVideoEncodeUsageFlagsKHR(2);
    pub const VK_VIDEO_ENCODE_USAGE_RECORDING_BIT_KHR: VkVideoEncodeUsageFlagsKHR = VkVideoEncodeUsageFlagsKHR(4);
    pub const VK_VIDEO_ENCODE_USAGE_CONFERENCING_BIT_KHR: VkVideoEncodeUsageFlagsKHR = VkVideoEncodeUsageFlagsKHR(8);
}

pub type VkVideoEncodeContentFlagBitsKHR = VkVideoEncodeContentFlagsKHR; //
impl VkVideoEncodeContentFlagBitsKHR {
    pub const VK_VIDEO_ENCODE_CONTENT_DEFAULT_KHR: VkVideoEncodeContentFlagsKHR = VkVideoEncodeContentFlagsKHR(0);
    pub const VK_VIDEO_ENCODE_CONTENT_CAMERA_BIT_KHR: VkVideoEncodeContentFlagsKHR = VkVideoEncodeContentFlagsKHR(1);
    pub const VK_VIDEO_ENCODE_CONTENT_DESKTOP_BIT_KHR: VkVideoEncodeContentFlagsKHR = VkVideoEncodeContentFlagsKHR(2);
    pub const VK_VIDEO_ENCODE_CONTENT_RENDERED_BIT_KHR: VkVideoEncodeContentFlagsKHR = VkVideoEncodeContentFlagsKHR(4);
}

pub struct VkVideoEncodeTuningModeKHR(i32); //
impl VkVideoEncodeTuningModeKHR {
    pub const VK_VIDEO_ENCODE_TUNING_MODE_DEFAULT_KHR: i32 = 0;
    pub const VK_VIDEO_ENCODE_TUNING_MODE_HIGH_QUALITY_KHR: i32 = 1;
    pub const VK_VIDEO_ENCODE_TUNING_MODE_LOW_LATENCY_KHR: i32 = 2;
    pub const VK_VIDEO_ENCODE_TUNING_MODE_ULTRA_LOW_LATENCY_KHR: i32 = 3;
    pub const VK_VIDEO_ENCODE_TUNING_MODE_LOSSLESS_KHR: i32 = 4;
}

pub type VkVideoEncodeCapabilityFlagBitsKHR = VkVideoEncodeCapabilityFlagsKHR; //
impl VkVideoEncodeCapabilityFlagBitsKHR {
    pub const VK_VIDEO_ENCODE_CAPABILITY_PRECEDING_EXTERNALLY_ENCODED_BYTES_BIT_KHR: VkVideoEncodeCapabilityFlagsKHR = VkVideoEncodeCapabilityFlagsKHR(1);
    pub const VK_VIDEO_ENCODE_CAPABILITY_INSUFFICIENT_BITSTREAM_BUFFER_RANGE_DETECTION_BIT_KHR: VkVideoEncodeCapabilityFlagsKHR = VkVideoEncodeCapabilityFlagsKHR(2);
}

pub type VkVideoEncodeFeedbackFlagBitsKHR = VkVideoEncodeFeedbackFlagsKHR; //
impl VkVideoEncodeFeedbackFlagBitsKHR {
    pub const VK_VIDEO_ENCODE_FEEDBACK_BITSTREAM_BUFFER_OFFSET_BIT_KHR: VkVideoEncodeFeedbackFlagsKHR = VkVideoEncodeFeedbackFlagsKHR(1);
    pub const VK_VIDEO_ENCODE_FEEDBACK_BITSTREAM_BYTES_WRITTEN_BIT_KHR: VkVideoEncodeFeedbackFlagsKHR = VkVideoEncodeFeedbackFlagsKHR(2);
    pub const VK_VIDEO_ENCODE_FEEDBACK_BITSTREAM_HAS_OVERRIDES_BIT_KHR: VkVideoEncodeFeedbackFlagsKHR = VkVideoEncodeFeedbackFlagsKHR(4);
}

pub type VkVideoEncodePerPartitionFeedbackFlagBitsKHR = VkVideoEncodePerPartitionFeedbackFlagsKHR; //
impl VkVideoEncodePerPartitionFeedbackFlagBitsKHR {
    pub const VK_VIDEO_ENCODE_PER_PARTITION_FEEDBACK_STATUS_BIT_KHR: VkVideoEncodePerPartitionFeedbackFlagsKHR = VkVideoEncodePerPartitionFeedbackFlagsKHR(1);
    pub const VK_VIDEO_ENCODE_PER_PARTITION_FEEDBACK_BITSTREAM_BUFFER_OFFSET_BIT_KHR: VkVideoEncodePerPartitionFeedbackFlagsKHR = VkVideoEncodePerPartitionFeedbackFlagsKHR(2);
    pub const VK_VIDEO_ENCODE_PER_PARTITION_FEEDBACK_BITSTREAM_BYTES_WRITTEN_BIT_KHR: VkVideoEncodePerPartitionFeedbackFlagsKHR = VkVideoEncodePerPartitionFeedbackFlagsKHR(4);
}

pub type VkVideoEncodeRateControlModeFlagBitsKHR = VkVideoEncodeRateControlModeFlagsKHR; //
impl VkVideoEncodeRateControlModeFlagBitsKHR {
    pub const VK_VIDEO_ENCODE_RATE_CONTROL_MODE_DEFAULT_KHR: VkVideoEncodeRateControlModeFlagsKHR = VkVideoEncodeRateControlModeFlagsKHR(0);
    pub const VK_VIDEO_ENCODE_RATE_CONTROL_MODE_DISABLED_BIT_KHR: VkVideoEncodeRateControlModeFlagsKHR = VkVideoEncodeRateControlModeFlagsKHR(1);
    pub const VK_VIDEO_ENCODE_RATE_CONTROL_MODE_CBR_BIT_KHR: VkVideoEncodeRateControlModeFlagsKHR = VkVideoEncodeRateControlModeFlagsKHR(2);
    pub const VK_VIDEO_ENCODE_RATE_CONTROL_MODE_VBR_BIT_KHR: VkVideoEncodeRateControlModeFlagsKHR = VkVideoEncodeRateControlModeFlagsKHR(4);
}

pub type VkVideoEncodeIntraRefreshModeFlagBitsKHR = VkVideoEncodeIntraRefreshModeFlagsKHR; //
impl VkVideoEncodeIntraRefreshModeFlagBitsKHR {
    pub const VK_VIDEO_ENCODE_INTRA_REFRESH_MODE_NONE_KHR: VkVideoEncodeIntraRefreshModeFlagsKHR = VkVideoEncodeIntraRefreshModeFlagsKHR(0);
    pub const VK_VIDEO_ENCODE_INTRA_REFRESH_MODE_PER_PICTURE_PARTITION_BIT_KHR: VkVideoEncodeIntraRefreshModeFlagsKHR = VkVideoEncodeIntraRefreshModeFlagsKHR(1);
    pub const VK_VIDEO_ENCODE_INTRA_REFRESH_MODE_BLOCK_BASED_BIT_KHR: VkVideoEncodeIntraRefreshModeFlagsKHR = VkVideoEncodeIntraRefreshModeFlagsKHR(2);
    pub const VK_VIDEO_ENCODE_INTRA_REFRESH_MODE_BLOCK_ROW_BASED_BIT_KHR: VkVideoEncodeIntraRefreshModeFlagsKHR = VkVideoEncodeIntraRefreshModeFlagsKHR(4);
    pub const VK_VIDEO_ENCODE_INTRA_REFRESH_MODE_BLOCK_COLUMN_BASED_BIT_KHR: VkVideoEncodeIntraRefreshModeFlagsKHR = VkVideoEncodeIntraRefreshModeFlagsKHR(8);
}

pub type VkVideoEncodeH264CapabilityFlagBitsKHR = VkVideoEncodeH264CapabilityFlagsKHR; //
impl VkVideoEncodeH264CapabilityFlagBitsKHR {
    pub const VK_VIDEO_ENCODE_H264_CAPABILITY_HRD_COMPLIANCE_BIT_KHR: VkVideoEncodeH264CapabilityFlagsKHR = VkVideoEncodeH264CapabilityFlagsKHR(1);
    pub const VK_VIDEO_ENCODE_H264_CAPABILITY_PREDICTION_WEIGHT_TABLE_GENERATED_BIT_KHR: VkVideoEncodeH264CapabilityFlagsKHR = VkVideoEncodeH264CapabilityFlagsKHR(2);
    pub const VK_VIDEO_ENCODE_H264_CAPABILITY_ROW_UNALIGNED_SLICE_BIT_KHR: VkVideoEncodeH264CapabilityFlagsKHR = VkVideoEncodeH264CapabilityFlagsKHR(4);
    pub const VK_VIDEO_ENCODE_H264_CAPABILITY_DIFFERENT_SLICE_TYPE_BIT_KHR: VkVideoEncodeH264CapabilityFlagsKHR = VkVideoEncodeH264CapabilityFlagsKHR(8);
    pub const VK_VIDEO_ENCODE_H264_CAPABILITY_B_FRAME_IN_L0_LIST_BIT_KHR: VkVideoEncodeH264CapabilityFlagsKHR = VkVideoEncodeH264CapabilityFlagsKHR(16);
    pub const VK_VIDEO_ENCODE_H264_CAPABILITY_B_FRAME_IN_L1_LIST_BIT_KHR: VkVideoEncodeH264CapabilityFlagsKHR = VkVideoEncodeH264CapabilityFlagsKHR(32);
    pub const VK_VIDEO_ENCODE_H264_CAPABILITY_PER_PICTURE_TYPE_MIN_MAX_QP_BIT_KHR: VkVideoEncodeH264CapabilityFlagsKHR = VkVideoEncodeH264CapabilityFlagsKHR(64);
    pub const VK_VIDEO_ENCODE_H264_CAPABILITY_PER_SLICE_CONSTANT_QP_BIT_KHR: VkVideoEncodeH264CapabilityFlagsKHR = VkVideoEncodeH264CapabilityFlagsKHR(128);
    pub const VK_VIDEO_ENCODE_H264_CAPABILITY_GENERATE_PREFIX_NALU_BIT_KHR: VkVideoEncodeH264CapabilityFlagsKHR = VkVideoEncodeH264CapabilityFlagsKHR(256);
}

pub type VkVideoEncodeH264StdFlagBitsKHR = VkVideoEncodeH264StdFlagsKHR; //
impl VkVideoEncodeH264StdFlagBitsKHR {
    pub const VK_VIDEO_ENCODE_H264_STD_SEPARATE_COLOR_PLANE_FLAG_SET_BIT_KHR: VkVideoEncodeH264StdFlagsKHR = VkVideoEncodeH264StdFlagsKHR(1);
    pub const VK_VIDEO_ENCODE_H264_STD_QPPRIME_Y_ZERO_TRANSFORM_BYPASS_FLAG_SET_BIT_KHR: VkVideoEncodeH264StdFlagsKHR = VkVideoEncodeH264StdFlagsKHR(2);
    pub const VK_VIDEO_ENCODE_H264_STD_SCALING_MATRIX_PRESENT_FLAG_SET_BIT_KHR: VkVideoEncodeH264StdFlagsKHR = VkVideoEncodeH264StdFlagsKHR(4);
    pub const VK_VIDEO_ENCODE_H264_STD_CHROMA_QP_INDEX_OFFSET_BIT_KHR: VkVideoEncodeH264StdFlagsKHR = VkVideoEncodeH264StdFlagsKHR(8);
    pub const VK_VIDEO_ENCODE_H264_STD_SECOND_CHROMA_QP_INDEX_OFFSET_BIT_KHR: VkVideoEncodeH264StdFlagsKHR = VkVideoEncodeH264StdFlagsKHR(16);
    pub const VK_VIDEO_ENCODE_H264_STD_PIC_INIT_QP_MINUS26_BIT_KHR: VkVideoEncodeH264StdFlagsKHR = VkVideoEncodeH264StdFlagsKHR(32);
    pub const VK_VIDEO_ENCODE_H264_STD_WEIGHTED_PRED_FLAG_SET_BIT_KHR: VkVideoEncodeH264StdFlagsKHR = VkVideoEncodeH264StdFlagsKHR(64);
    pub const VK_VIDEO_ENCODE_H264_STD_WEIGHTED_BIPRED_IDC_EXPLICIT_BIT_KHR: VkVideoEncodeH264StdFlagsKHR = VkVideoEncodeH264StdFlagsKHR(128);
    pub const VK_VIDEO_ENCODE_H264_STD_WEIGHTED_BIPRED_IDC_IMPLICIT_BIT_KHR: VkVideoEncodeH264StdFlagsKHR = VkVideoEncodeH264StdFlagsKHR(256);
    pub const VK_VIDEO_ENCODE_H264_STD_TRANSFORM_8X8_MODE_FLAG_SET_BIT_KHR: VkVideoEncodeH264StdFlagsKHR = VkVideoEncodeH264StdFlagsKHR(512);
    pub const VK_VIDEO_ENCODE_H264_STD_DIRECT_SPATIAL_MV_PRED_FLAG_UNSET_BIT_KHR: VkVideoEncodeH264StdFlagsKHR = VkVideoEncodeH264StdFlagsKHR(1024);
    pub const VK_VIDEO_ENCODE_H264_STD_ENTROPY_CODING_MODE_FLAG_UNSET_BIT_KHR: VkVideoEncodeH264StdFlagsKHR = VkVideoEncodeH264StdFlagsKHR(2048);
    pub const VK_VIDEO_ENCODE_H264_STD_ENTROPY_CODING_MODE_FLAG_SET_BIT_KHR: VkVideoEncodeH264StdFlagsKHR = VkVideoEncodeH264StdFlagsKHR(4096);
    pub const VK_VIDEO_ENCODE_H264_STD_DIRECT_8X8_INFERENCE_FLAG_UNSET_BIT_KHR: VkVideoEncodeH264StdFlagsKHR = VkVideoEncodeH264StdFlagsKHR(8192);
    pub const VK_VIDEO_ENCODE_H264_STD_CONSTRAINED_INTRA_PRED_FLAG_SET_BIT_KHR: VkVideoEncodeH264StdFlagsKHR = VkVideoEncodeH264StdFlagsKHR(16384);
    pub const VK_VIDEO_ENCODE_H264_STD_DEBLOCKING_FILTER_DISABLED_BIT_KHR: VkVideoEncodeH264StdFlagsKHR = VkVideoEncodeH264StdFlagsKHR(32768);
    pub const VK_VIDEO_ENCODE_H264_STD_DEBLOCKING_FILTER_ENABLED_BIT_KHR: VkVideoEncodeH264StdFlagsKHR = VkVideoEncodeH264StdFlagsKHR(65536);
    pub const VK_VIDEO_ENCODE_H264_STD_DEBLOCKING_FILTER_PARTIAL_BIT_KHR: VkVideoEncodeH264StdFlagsKHR = VkVideoEncodeH264StdFlagsKHR(131072);
    pub const VK_VIDEO_ENCODE_H264_STD_SLICE_QP_DELTA_BIT_KHR: VkVideoEncodeH264StdFlagsKHR = VkVideoEncodeH264StdFlagsKHR(524288);
    pub const VK_VIDEO_ENCODE_H264_STD_DIFFERENT_SLICE_QP_DELTA_BIT_KHR: VkVideoEncodeH264StdFlagsKHR = VkVideoEncodeH264StdFlagsKHR(1048576);
}

pub type VkVideoEncodeH264RateControlFlagBitsKHR = VkVideoEncodeH264RateControlFlagsKHR; //
impl VkVideoEncodeH264RateControlFlagBitsKHR {
    pub const VK_VIDEO_ENCODE_H264_RATE_CONTROL_ATTEMPT_HRD_COMPLIANCE_BIT_KHR: VkVideoEncodeH264RateControlFlagsKHR = VkVideoEncodeH264RateControlFlagsKHR(1);
    pub const VK_VIDEO_ENCODE_H264_RATE_CONTROL_REGULAR_GOP_BIT_KHR: VkVideoEncodeH264RateControlFlagsKHR = VkVideoEncodeH264RateControlFlagsKHR(2);
    pub const VK_VIDEO_ENCODE_H264_RATE_CONTROL_REFERENCE_PATTERN_FLAT_BIT_KHR: VkVideoEncodeH264RateControlFlagsKHR = VkVideoEncodeH264RateControlFlagsKHR(4);
    pub const VK_VIDEO_ENCODE_H264_RATE_CONTROL_REFERENCE_PATTERN_DYADIC_BIT_KHR: VkVideoEncodeH264RateControlFlagsKHR = VkVideoEncodeH264RateControlFlagsKHR(8);
    pub const VK_VIDEO_ENCODE_H264_RATE_CONTROL_TEMPORAL_LAYER_PATTERN_DYADIC_BIT_KHR: VkVideoEncodeH264RateControlFlagsKHR = VkVideoEncodeH264RateControlFlagsKHR(16);
}

pub type VkHostImageCopyFlagBits = VkHostImageCopyFlags; //
impl VkHostImageCopyFlagBits {
    pub const VK_HOST_IMAGE_COPY_MEMCPY_BIT: VkHostImageCopyFlags = VkHostImageCopyFlags(1);
    pub const VK_HOST_IMAGE_COPY_MEMCPY: VkHostImageCopyFlags = Self::VK_HOST_IMAGE_COPY_MEMCPY_BIT;
}

pub struct VkPartitionedAccelerationStructureOpTypeNV(i32); //
impl VkPartitionedAccelerationStructureOpTypeNV {
    pub const VK_PARTITIONED_ACCELERATION_STRUCTURE_OP_TYPE_WRITE_INSTANCE_NV: i32 = 0;
    pub const VK_PARTITIONED_ACCELERATION_STRUCTURE_OP_TYPE_UPDATE_INSTANCE_NV: i32 = 1;
    pub const VK_PARTITIONED_ACCELERATION_STRUCTURE_OP_TYPE_WRITE_PARTITION_TRANSLATION_NV: i32 = 2;
}

pub type VkPartitionedAccelerationStructureInstanceFlagBitsNV = VkPartitionedAccelerationStructureInstanceFlagsNV; //
impl VkPartitionedAccelerationStructureInstanceFlagBitsNV {
    pub const VK_PARTITIONED_ACCELERATION_STRUCTURE_INSTANCE_FLAG_TRIANGLE_FACING_CULL_DISABLE_BIT_NV: VkPartitionedAccelerationStructureInstanceFlagsNV = VkPartitionedAccelerationStructureInstanceFlagsNV(1);
    pub const VK_PARTITIONED_ACCELERATION_STRUCTURE_INSTANCE_FLAG_TRIANGLE_FLIP_FACING_BIT_NV: VkPartitionedAccelerationStructureInstanceFlagsNV = VkPartitionedAccelerationStructureInstanceFlagsNV(2);
    pub const VK_PARTITIONED_ACCELERATION_STRUCTURE_INSTANCE_FLAG_FORCE_OPAQUE_BIT_NV: VkPartitionedAccelerationStructureInstanceFlagsNV = VkPartitionedAccelerationStructureInstanceFlagsNV(4);
    pub const VK_PARTITIONED_ACCELERATION_STRUCTURE_INSTANCE_FLAG_FORCE_NO_OPAQUE_BIT_NV: VkPartitionedAccelerationStructureInstanceFlagsNV = VkPartitionedAccelerationStructureInstanceFlagsNV(8);
    pub const VK_PARTITIONED_ACCELERATION_STRUCTURE_INSTANCE_FLAG_ENABLE_EXPLICIT_BOUNDING_BOX_NV: VkPartitionedAccelerationStructureInstanceFlagsNV = VkPartitionedAccelerationStructureInstanceFlagsNV(16);
}

pub type VkImageFormatConstraintsFlagBitsFUCHSIA = VkImageFormatConstraintsFlagsFUCHSIA; //
impl VkImageFormatConstraintsFlagBitsFUCHSIA {
}

pub type VkImageConstraintsInfoFlagBitsFUCHSIA = VkImageConstraintsInfoFlagsFUCHSIA; //
impl VkImageConstraintsInfoFlagBitsFUCHSIA {
    pub const VK_IMAGE_CONSTRAINTS_INFO_CPU_READ_RARELY_FUCHSIA: VkImageConstraintsInfoFlagsFUCHSIA = VkImageConstraintsInfoFlagsFUCHSIA(1);
    pub const VK_IMAGE_CONSTRAINTS_INFO_CPU_READ_OFTEN_FUCHSIA: VkImageConstraintsInfoFlagsFUCHSIA = VkImageConstraintsInfoFlagsFUCHSIA(2);
    pub const VK_IMAGE_CONSTRAINTS_INFO_CPU_WRITE_RARELY_FUCHSIA: VkImageConstraintsInfoFlagsFUCHSIA = VkImageConstraintsInfoFlagsFUCHSIA(4);
    pub const VK_IMAGE_CONSTRAINTS_INFO_CPU_WRITE_OFTEN_FUCHSIA: VkImageConstraintsInfoFlagsFUCHSIA = VkImageConstraintsInfoFlagsFUCHSIA(8);
    pub const VK_IMAGE_CONSTRAINTS_INFO_PROTECTED_OPTIONAL_FUCHSIA: VkImageConstraintsInfoFlagsFUCHSIA = VkImageConstraintsInfoFlagsFUCHSIA(16);
}

pub type VkFormatFeatureFlagBits2 = VkFormatFeatureFlags2; //
impl VkFormatFeatureFlagBits2 {
    pub const VK_FORMAT_FEATURE_2_SAMPLED_IMAGE_BIT: VkFormatFeatureFlags2 = VkFormatFeatureFlags2(1);
    pub const VK_FORMAT_FEATURE_2_STORAGE_IMAGE_BIT: VkFormatFeatureFlags2 = VkFormatFeatureFlags2(2);
    pub const VK_FORMAT_FEATURE_2_STORAGE_IMAGE_ATOMIC_BIT: VkFormatFeatureFlags2 = VkFormatFeatureFlags2(4);
    pub const VK_FORMAT_FEATURE_2_UNIFORM_TEXEL_BUFFER_BIT: VkFormatFeatureFlags2 = VkFormatFeatureFlags2(8);
    pub const VK_FORMAT_FEATURE_2_STORAGE_TEXEL_BUFFER_BIT: VkFormatFeatureFlags2 = VkFormatFeatureFlags2(16);
    pub const VK_FORMAT_FEATURE_2_STORAGE_TEXEL_BUFFER_ATOMIC_BIT: VkFormatFeatureFlags2 = VkFormatFeatureFlags2(32);
    pub const VK_FORMAT_FEATURE_2_VERTEX_BUFFER_BIT: VkFormatFeatureFlags2 = VkFormatFeatureFlags2(64);
    pub const VK_FORMAT_FEATURE_2_COLOR_ATTACHMENT_BIT: VkFormatFeatureFlags2 = VkFormatFeatureFlags2(128);
    pub const VK_FORMAT_FEATURE_2_COLOR_ATTACHMENT_BLEND_BIT: VkFormatFeatureFlags2 = VkFormatFeatureFlags2(256);
    pub const VK_FORMAT_FEATURE_2_DEPTH_STENCIL_ATTACHMENT_BIT: VkFormatFeatureFlags2 = VkFormatFeatureFlags2(512);
    pub const VK_FORMAT_FEATURE_2_BLIT_SRC_BIT: VkFormatFeatureFlags2 = VkFormatFeatureFlags2(1024);
    pub const VK_FORMAT_FEATURE_2_BLIT_DST_BIT: VkFormatFeatureFlags2 = VkFormatFeatureFlags2(2048);
    pub const VK_FORMAT_FEATURE_2_SAMPLED_IMAGE_FILTER_LINEAR_BIT: VkFormatFeatureFlags2 = VkFormatFeatureFlags2(4096);
    pub const VK_FORMAT_FEATURE_2_TRANSFER_SRC_BIT: VkFormatFeatureFlags2 = VkFormatFeatureFlags2(16384);
    pub const VK_FORMAT_FEATURE_2_TRANSFER_DST_BIT: VkFormatFeatureFlags2 = VkFormatFeatureFlags2(32768);
    pub const VK_FORMAT_FEATURE_2_SAMPLED_IMAGE_FILTER_MINMAX_BIT: VkFormatFeatureFlags2 = VkFormatFeatureFlags2(65536);
    pub const VK_FORMAT_FEATURE_2_MIDPOINT_CHROMA_SAMPLES_BIT: VkFormatFeatureFlags2 = VkFormatFeatureFlags2(131072);
    pub const VK_FORMAT_FEATURE_2_SAMPLED_IMAGE_YCBCR_CONVERSION_LINEAR_FILTER_BIT: VkFormatFeatureFlags2 = VkFormatFeatureFlags2(262144);
    pub const VK_FORMAT_FEATURE_2_SAMPLED_IMAGE_YCBCR_CONVERSION_SEPARATE_RECONSTRUCTION_FILTER_BIT: VkFormatFeatureFlags2 = VkFormatFeatureFlags2(524288);
    pub const VK_FORMAT_FEATURE_2_SAMPLED_IMAGE_YCBCR_CONVERSION_CHROMA_RECONSTRUCTION_EXPLICIT_BIT: VkFormatFeatureFlags2 = VkFormatFeatureFlags2(1048576);
    pub const VK_FORMAT_FEATURE_2_SAMPLED_IMAGE_YCBCR_CONVERSION_CHROMA_RECONSTRUCTION_EXPLICIT_FORCEABLE_BIT: VkFormatFeatureFlags2 = VkFormatFeatureFlags2(2097152);
    pub const VK_FORMAT_FEATURE_2_DISJOINT_BIT: VkFormatFeatureFlags2 = VkFormatFeatureFlags2(4194304);
    pub const VK_FORMAT_FEATURE_2_COSITED_CHROMA_SAMPLES_BIT: VkFormatFeatureFlags2 = VkFormatFeatureFlags2(8388608);
    pub const VK_FORMAT_FEATURE_2_STORAGE_READ_WITHOUT_FORMAT_BIT: VkFormatFeatureFlags2 = VkFormatFeatureFlags2(2147483648);
    pub const VK_FORMAT_FEATURE_2_STORAGE_WRITE_WITHOUT_FORMAT_BIT: VkFormatFeatureFlags2 = VkFormatFeatureFlags2(4294967296);
    pub const VK_FORMAT_FEATURE_2_SAMPLED_IMAGE_DEPTH_COMPARISON_BIT: VkFormatFeatureFlags2 = VkFormatFeatureFlags2(8589934592);
}

pub type VkFormatFeatureFlagBits4KHR = VkFormatFeatureFlags4KHR; //
impl VkFormatFeatureFlagBits4KHR {
}

pub type VkRenderingFlagBits = VkRenderingFlags; //
impl VkRenderingFlagBits {
    pub const VK_RENDERING_CONTENTS_SECONDARY_COMMAND_BUFFERS_BIT: VkRenderingFlags = VkRenderingFlags(1);
    pub const VK_RENDERING_SUSPENDING_BIT: VkRenderingFlags = VkRenderingFlags(2);
    pub const VK_RENDERING_RESUMING_BIT: VkRenderingFlags = VkRenderingFlags(4);
}

pub type VkVideoEncodeH265CapabilityFlagBitsKHR = VkVideoEncodeH265CapabilityFlagsKHR; //
impl VkVideoEncodeH265CapabilityFlagBitsKHR {
    pub const VK_VIDEO_ENCODE_H265_CAPABILITY_HRD_COMPLIANCE_BIT_KHR: VkVideoEncodeH265CapabilityFlagsKHR = VkVideoEncodeH265CapabilityFlagsKHR(1);
    pub const VK_VIDEO_ENCODE_H265_CAPABILITY_PREDICTION_WEIGHT_TABLE_GENERATED_BIT_KHR: VkVideoEncodeH265CapabilityFlagsKHR = VkVideoEncodeH265CapabilityFlagsKHR(2);
    pub const VK_VIDEO_ENCODE_H265_CAPABILITY_ROW_UNALIGNED_SLICE_SEGMENT_BIT_KHR: VkVideoEncodeH265CapabilityFlagsKHR = VkVideoEncodeH265CapabilityFlagsKHR(4);
    pub const VK_VIDEO_ENCODE_H265_CAPABILITY_DIFFERENT_SLICE_SEGMENT_TYPE_BIT_KHR: VkVideoEncodeH265CapabilityFlagsKHR = VkVideoEncodeH265CapabilityFlagsKHR(8);
    pub const VK_VIDEO_ENCODE_H265_CAPABILITY_B_FRAME_IN_L0_LIST_BIT_KHR: VkVideoEncodeH265CapabilityFlagsKHR = VkVideoEncodeH265CapabilityFlagsKHR(16);
    pub const VK_VIDEO_ENCODE_H265_CAPABILITY_B_FRAME_IN_L1_LIST_BIT_KHR: VkVideoEncodeH265CapabilityFlagsKHR = VkVideoEncodeH265CapabilityFlagsKHR(32);
    pub const VK_VIDEO_ENCODE_H265_CAPABILITY_PER_PICTURE_TYPE_MIN_MAX_QP_BIT_KHR: VkVideoEncodeH265CapabilityFlagsKHR = VkVideoEncodeH265CapabilityFlagsKHR(64);
    pub const VK_VIDEO_ENCODE_H265_CAPABILITY_PER_SLICE_SEGMENT_CONSTANT_QP_BIT_KHR: VkVideoEncodeH265CapabilityFlagsKHR = VkVideoEncodeH265CapabilityFlagsKHR(128);
    pub const VK_VIDEO_ENCODE_H265_CAPABILITY_MULTIPLE_TILES_PER_SLICE_SEGMENT_BIT_KHR: VkVideoEncodeH265CapabilityFlagsKHR = VkVideoEncodeH265CapabilityFlagsKHR(256);
    pub const VK_VIDEO_ENCODE_H265_CAPABILITY_MULTIPLE_SLICE_SEGMENTS_PER_TILE_BIT_KHR: VkVideoEncodeH265CapabilityFlagsKHR = VkVideoEncodeH265CapabilityFlagsKHR(512);
}

pub type VkVideoEncodeH265StdFlagBitsKHR = VkVideoEncodeH265StdFlagsKHR; //
impl VkVideoEncodeH265StdFlagBitsKHR {
    pub const VK_VIDEO_ENCODE_H265_STD_SEPARATE_COLOR_PLANE_FLAG_SET_BIT_KHR: VkVideoEncodeH265StdFlagsKHR = VkVideoEncodeH265StdFlagsKHR(1);
    pub const VK_VIDEO_ENCODE_H265_STD_SAMPLE_ADAPTIVE_OFFSET_ENABLED_FLAG_SET_BIT_KHR: VkVideoEncodeH265StdFlagsKHR = VkVideoEncodeH265StdFlagsKHR(2);
    pub const VK_VIDEO_ENCODE_H265_STD_SCALING_LIST_DATA_PRESENT_FLAG_SET_BIT_KHR: VkVideoEncodeH265StdFlagsKHR = VkVideoEncodeH265StdFlagsKHR(4);
    pub const VK_VIDEO_ENCODE_H265_STD_PCM_ENABLED_FLAG_SET_BIT_KHR: VkVideoEncodeH265StdFlagsKHR = VkVideoEncodeH265StdFlagsKHR(8);
    pub const VK_VIDEO_ENCODE_H265_STD_SPS_TEMPORAL_MVP_ENABLED_FLAG_SET_BIT_KHR: VkVideoEncodeH265StdFlagsKHR = VkVideoEncodeH265StdFlagsKHR(16);
    pub const VK_VIDEO_ENCODE_H265_STD_INIT_QP_MINUS26_BIT_KHR: VkVideoEncodeH265StdFlagsKHR = VkVideoEncodeH265StdFlagsKHR(32);
    pub const VK_VIDEO_ENCODE_H265_STD_WEIGHTED_PRED_FLAG_SET_BIT_KHR: VkVideoEncodeH265StdFlagsKHR = VkVideoEncodeH265StdFlagsKHR(64);
    pub const VK_VIDEO_ENCODE_H265_STD_WEIGHTED_BIPRED_FLAG_SET_BIT_KHR: VkVideoEncodeH265StdFlagsKHR = VkVideoEncodeH265StdFlagsKHR(128);
    pub const VK_VIDEO_ENCODE_H265_STD_LOG2_PARALLEL_MERGE_LEVEL_MINUS2_BIT_KHR: VkVideoEncodeH265StdFlagsKHR = VkVideoEncodeH265StdFlagsKHR(256);
    pub const VK_VIDEO_ENCODE_H265_STD_SIGN_DATA_HIDING_ENABLED_FLAG_SET_BIT_KHR: VkVideoEncodeH265StdFlagsKHR = VkVideoEncodeH265StdFlagsKHR(512);
    pub const VK_VIDEO_ENCODE_H265_STD_TRANSFORM_SKIP_ENABLED_FLAG_SET_BIT_KHR: VkVideoEncodeH265StdFlagsKHR = VkVideoEncodeH265StdFlagsKHR(1024);
    pub const VK_VIDEO_ENCODE_H265_STD_TRANSFORM_SKIP_ENABLED_FLAG_UNSET_BIT_KHR: VkVideoEncodeH265StdFlagsKHR = VkVideoEncodeH265StdFlagsKHR(2048);
    pub const VK_VIDEO_ENCODE_H265_STD_PPS_SLICE_CHROMA_QP_OFFSETS_PRESENT_FLAG_SET_BIT_KHR: VkVideoEncodeH265StdFlagsKHR = VkVideoEncodeH265StdFlagsKHR(4096);
    pub const VK_VIDEO_ENCODE_H265_STD_TRANSQUANT_BYPASS_ENABLED_FLAG_SET_BIT_KHR: VkVideoEncodeH265StdFlagsKHR = VkVideoEncodeH265StdFlagsKHR(8192);
    pub const VK_VIDEO_ENCODE_H265_STD_CONSTRAINED_INTRA_PRED_FLAG_SET_BIT_KHR: VkVideoEncodeH265StdFlagsKHR = VkVideoEncodeH265StdFlagsKHR(16384);
    pub const VK_VIDEO_ENCODE_H265_STD_ENTROPY_CODING_SYNC_ENABLED_FLAG_SET_BIT_KHR: VkVideoEncodeH265StdFlagsKHR = VkVideoEncodeH265StdFlagsKHR(32768);
    pub const VK_VIDEO_ENCODE_H265_STD_DEBLOCKING_FILTER_OVERRIDE_ENABLED_FLAG_SET_BIT_KHR: VkVideoEncodeH265StdFlagsKHR = VkVideoEncodeH265StdFlagsKHR(65536);
    pub const VK_VIDEO_ENCODE_H265_STD_DEPENDENT_SLICE_SEGMENTS_ENABLED_FLAG_SET_BIT_KHR: VkVideoEncodeH265StdFlagsKHR = VkVideoEncodeH265StdFlagsKHR(131072);
    pub const VK_VIDEO_ENCODE_H265_STD_DEPENDENT_SLICE_SEGMENT_FLAG_SET_BIT_KHR: VkVideoEncodeH265StdFlagsKHR = VkVideoEncodeH265StdFlagsKHR(262144);
    pub const VK_VIDEO_ENCODE_H265_STD_SLICE_QP_DELTA_BIT_KHR: VkVideoEncodeH265StdFlagsKHR = VkVideoEncodeH265StdFlagsKHR(524288);
    pub const VK_VIDEO_ENCODE_H265_STD_DIFFERENT_SLICE_QP_DELTA_BIT_KHR: VkVideoEncodeH265StdFlagsKHR = VkVideoEncodeH265StdFlagsKHR(1048576);
}

pub type VkVideoEncodeH265RateControlFlagBitsKHR = VkVideoEncodeH265RateControlFlagsKHR; //
impl VkVideoEncodeH265RateControlFlagBitsKHR {
    pub const VK_VIDEO_ENCODE_H265_RATE_CONTROL_ATTEMPT_HRD_COMPLIANCE_BIT_KHR: VkVideoEncodeH265RateControlFlagsKHR = VkVideoEncodeH265RateControlFlagsKHR(1);
    pub const VK_VIDEO_ENCODE_H265_RATE_CONTROL_REGULAR_GOP_BIT_KHR: VkVideoEncodeH265RateControlFlagsKHR = VkVideoEncodeH265RateControlFlagsKHR(2);
    pub const VK_VIDEO_ENCODE_H265_RATE_CONTROL_REFERENCE_PATTERN_FLAT_BIT_KHR: VkVideoEncodeH265RateControlFlagsKHR = VkVideoEncodeH265RateControlFlagsKHR(4);
    pub const VK_VIDEO_ENCODE_H265_RATE_CONTROL_REFERENCE_PATTERN_DYADIC_BIT_KHR: VkVideoEncodeH265RateControlFlagsKHR = VkVideoEncodeH265RateControlFlagsKHR(8);
    pub const VK_VIDEO_ENCODE_H265_RATE_CONTROL_TEMPORAL_SUB_LAYER_PATTERN_DYADIC_BIT_KHR: VkVideoEncodeH265RateControlFlagsKHR = VkVideoEncodeH265RateControlFlagsKHR(16);
}

pub type VkVideoEncodeH265CtbSizeFlagBitsKHR = VkVideoEncodeH265CtbSizeFlagsKHR; //
impl VkVideoEncodeH265CtbSizeFlagBitsKHR {
    pub const VK_VIDEO_ENCODE_H265_CTB_SIZE_16_BIT_KHR: VkVideoEncodeH265CtbSizeFlagsKHR = VkVideoEncodeH265CtbSizeFlagsKHR(1);
    pub const VK_VIDEO_ENCODE_H265_CTB_SIZE_32_BIT_KHR: VkVideoEncodeH265CtbSizeFlagsKHR = VkVideoEncodeH265CtbSizeFlagsKHR(2);
    pub const VK_VIDEO_ENCODE_H265_CTB_SIZE_64_BIT_KHR: VkVideoEncodeH265CtbSizeFlagsKHR = VkVideoEncodeH265CtbSizeFlagsKHR(4);
}

pub type VkVideoEncodeH265TransformBlockSizeFlagBitsKHR = VkVideoEncodeH265TransformBlockSizeFlagsKHR; //
impl VkVideoEncodeH265TransformBlockSizeFlagBitsKHR {
    pub const VK_VIDEO_ENCODE_H265_TRANSFORM_BLOCK_SIZE_4_BIT_KHR: VkVideoEncodeH265TransformBlockSizeFlagsKHR = VkVideoEncodeH265TransformBlockSizeFlagsKHR(1);
    pub const VK_VIDEO_ENCODE_H265_TRANSFORM_BLOCK_SIZE_8_BIT_KHR: VkVideoEncodeH265TransformBlockSizeFlagsKHR = VkVideoEncodeH265TransformBlockSizeFlagsKHR(2);
    pub const VK_VIDEO_ENCODE_H265_TRANSFORM_BLOCK_SIZE_16_BIT_KHR: VkVideoEncodeH265TransformBlockSizeFlagsKHR = VkVideoEncodeH265TransformBlockSizeFlagsKHR(4);
    pub const VK_VIDEO_ENCODE_H265_TRANSFORM_BLOCK_SIZE_32_BIT_KHR: VkVideoEncodeH265TransformBlockSizeFlagsKHR = VkVideoEncodeH265TransformBlockSizeFlagsKHR(8);
}

pub type VkVideoEncodeAV1CapabilityFlagBitsKHR = VkVideoEncodeAV1CapabilityFlagsKHR; //
impl VkVideoEncodeAV1CapabilityFlagBitsKHR {
    pub const VK_VIDEO_ENCODE_AV1_CAPABILITY_PER_RATE_CONTROL_GROUP_MIN_MAX_Q_INDEX_BIT_KHR: VkVideoEncodeAV1CapabilityFlagsKHR = VkVideoEncodeAV1CapabilityFlagsKHR(1);
    pub const VK_VIDEO_ENCODE_AV1_CAPABILITY_GENERATE_OBU_EXTENSION_HEADER_BIT_KHR: VkVideoEncodeAV1CapabilityFlagsKHR = VkVideoEncodeAV1CapabilityFlagsKHR(2);
    pub const VK_VIDEO_ENCODE_AV1_CAPABILITY_PRIMARY_REFERENCE_CDF_ONLY_BIT_KHR: VkVideoEncodeAV1CapabilityFlagsKHR = VkVideoEncodeAV1CapabilityFlagsKHR(4);
    pub const VK_VIDEO_ENCODE_AV1_CAPABILITY_FRAME_SIZE_OVERRIDE_BIT_KHR: VkVideoEncodeAV1CapabilityFlagsKHR = VkVideoEncodeAV1CapabilityFlagsKHR(8);
    pub const VK_VIDEO_ENCODE_AV1_CAPABILITY_MOTION_VECTOR_SCALING_BIT_KHR: VkVideoEncodeAV1CapabilityFlagsKHR = VkVideoEncodeAV1CapabilityFlagsKHR(16);
}

pub type VkVideoEncodeAV1StdFlagBitsKHR = VkVideoEncodeAV1StdFlagsKHR; //
impl VkVideoEncodeAV1StdFlagBitsKHR {
    pub const VK_VIDEO_ENCODE_AV1_STD_UNIFORM_TILE_SPACING_FLAG_SET_BIT_KHR: VkVideoEncodeAV1StdFlagsKHR = VkVideoEncodeAV1StdFlagsKHR(1);
    pub const VK_VIDEO_ENCODE_AV1_STD_SKIP_MODE_PRESENT_UNSET_BIT_KHR: VkVideoEncodeAV1StdFlagsKHR = VkVideoEncodeAV1StdFlagsKHR(2);
    pub const VK_VIDEO_ENCODE_AV1_STD_PRIMARY_REF_FRAME_BIT_KHR: VkVideoEncodeAV1StdFlagsKHR = VkVideoEncodeAV1StdFlagsKHR(4);
    pub const VK_VIDEO_ENCODE_AV1_STD_DELTA_Q_BIT_KHR: VkVideoEncodeAV1StdFlagsKHR = VkVideoEncodeAV1StdFlagsKHR(8);
}

pub type VkVideoEncodeAV1RateControlFlagBitsKHR = VkVideoEncodeAV1RateControlFlagsKHR; //
impl VkVideoEncodeAV1RateControlFlagBitsKHR {
    pub const VK_VIDEO_ENCODE_AV1_RATE_CONTROL_REGULAR_GOP_BIT_KHR: VkVideoEncodeAV1RateControlFlagsKHR = VkVideoEncodeAV1RateControlFlagsKHR(1);
    pub const VK_VIDEO_ENCODE_AV1_RATE_CONTROL_TEMPORAL_LAYER_PATTERN_DYADIC_BIT_KHR: VkVideoEncodeAV1RateControlFlagsKHR = VkVideoEncodeAV1RateControlFlagsKHR(2);
    pub const VK_VIDEO_ENCODE_AV1_RATE_CONTROL_REFERENCE_PATTERN_FLAT_BIT_KHR: VkVideoEncodeAV1RateControlFlagsKHR = VkVideoEncodeAV1RateControlFlagsKHR(4);
    pub const VK_VIDEO_ENCODE_AV1_RATE_CONTROL_REFERENCE_PATTERN_DYADIC_BIT_KHR: VkVideoEncodeAV1RateControlFlagsKHR = VkVideoEncodeAV1RateControlFlagsKHR(8);
}

pub type VkVideoEncodeAV1SuperblockSizeFlagBitsKHR = VkVideoEncodeAV1SuperblockSizeFlagsKHR; //
impl VkVideoEncodeAV1SuperblockSizeFlagBitsKHR {
    pub const VK_VIDEO_ENCODE_AV1_SUPERBLOCK_SIZE_64_BIT_KHR: VkVideoEncodeAV1SuperblockSizeFlagsKHR = VkVideoEncodeAV1SuperblockSizeFlagsKHR(1);
    pub const VK_VIDEO_ENCODE_AV1_SUPERBLOCK_SIZE_128_BIT_KHR: VkVideoEncodeAV1SuperblockSizeFlagsKHR = VkVideoEncodeAV1SuperblockSizeFlagsKHR(2);
}

pub struct VkVideoEncodeAV1PredictionModeKHR(i32); //
impl VkVideoEncodeAV1PredictionModeKHR {
    pub const VK_VIDEO_ENCODE_AV1_PREDICTION_MODE_INTRA_ONLY_KHR: i32 = 0;
    pub const VK_VIDEO_ENCODE_AV1_PREDICTION_MODE_SINGLE_REFERENCE_KHR: i32 = 1;
    pub const VK_VIDEO_ENCODE_AV1_PREDICTION_MODE_UNIDIRECTIONAL_COMPOUND_KHR: i32 = 2;
    pub const VK_VIDEO_ENCODE_AV1_PREDICTION_MODE_BIDIRECTIONAL_COMPOUND_KHR: i32 = 3;
}

pub struct VkVideoEncodeAV1RateControlGroupKHR(i32); //
impl VkVideoEncodeAV1RateControlGroupKHR {
    pub const VK_VIDEO_ENCODE_AV1_RATE_CONTROL_GROUP_INTRA_KHR: i32 = 0;
    pub const VK_VIDEO_ENCODE_AV1_RATE_CONTROL_GROUP_PREDICTIVE_KHR: i32 = 1;
    pub const VK_VIDEO_ENCODE_AV1_RATE_CONTROL_GROUP_BIPREDICTIVE_KHR: i32 = 2;
}

pub type VkExportMetalObjectTypeFlagBitsEXT = VkExportMetalObjectTypeFlagsEXT; //
impl VkExportMetalObjectTypeFlagBitsEXT {
    pub const VK_EXPORT_METAL_OBJECT_TYPE_METAL_DEVICE_BIT_EXT: VkExportMetalObjectTypeFlagsEXT = VkExportMetalObjectTypeFlagsEXT(1);
    pub const VK_EXPORT_METAL_OBJECT_TYPE_METAL_COMMAND_QUEUE_BIT_EXT: VkExportMetalObjectTypeFlagsEXT = VkExportMetalObjectTypeFlagsEXT(2);
    pub const VK_EXPORT_METAL_OBJECT_TYPE_METAL_BUFFER_BIT_EXT: VkExportMetalObjectTypeFlagsEXT = VkExportMetalObjectTypeFlagsEXT(4);
    pub const VK_EXPORT_METAL_OBJECT_TYPE_METAL_TEXTURE_BIT_EXT: VkExportMetalObjectTypeFlagsEXT = VkExportMetalObjectTypeFlagsEXT(8);
    pub const VK_EXPORT_METAL_OBJECT_TYPE_METAL_IOSURFACE_BIT_EXT: VkExportMetalObjectTypeFlagsEXT = VkExportMetalObjectTypeFlagsEXT(16);
    pub const VK_EXPORT_METAL_OBJECT_TYPE_METAL_SHARED_EVENT_BIT_EXT: VkExportMetalObjectTypeFlagsEXT = VkExportMetalObjectTypeFlagsEXT(32);
}

pub type VkInstanceCreateFlagBits = VkInstanceCreateFlags; //
impl VkInstanceCreateFlagBits {
}

pub type VkImageCompressionFlagBitsEXT = VkImageCompressionFlagsEXT; //
impl VkImageCompressionFlagBitsEXT {
    pub const VK_IMAGE_COMPRESSION_DEFAULT_EXT: VkImageCompressionFlagsEXT = VkImageCompressionFlagsEXT(0);
    pub const VK_IMAGE_COMPRESSION_FIXED_RATE_DEFAULT_EXT: VkImageCompressionFlagsEXT = VkImageCompressionFlagsEXT(1);
    pub const VK_IMAGE_COMPRESSION_FIXED_RATE_EXPLICIT_EXT: VkImageCompressionFlagsEXT = VkImageCompressionFlagsEXT(2);
    pub const VK_IMAGE_COMPRESSION_DISABLED_EXT: VkImageCompressionFlagsEXT = VkImageCompressionFlagsEXT(4);
}

pub type VkImageCompressionFixedRateFlagBitsEXT = VkImageCompressionFixedRateFlagsEXT; //
impl VkImageCompressionFixedRateFlagBitsEXT {
    pub const VK_IMAGE_COMPRESSION_FIXED_RATE_NONE_EXT: VkImageCompressionFixedRateFlagsEXT = VkImageCompressionFixedRateFlagsEXT(0);
    pub const VK_IMAGE_COMPRESSION_FIXED_RATE_1BPC_BIT_EXT: VkImageCompressionFixedRateFlagsEXT = VkImageCompressionFixedRateFlagsEXT(1);
    pub const VK_IMAGE_COMPRESSION_FIXED_RATE_2BPC_BIT_EXT: VkImageCompressionFixedRateFlagsEXT = VkImageCompressionFixedRateFlagsEXT(2);
    pub const VK_IMAGE_COMPRESSION_FIXED_RATE_3BPC_BIT_EXT: VkImageCompressionFixedRateFlagsEXT = VkImageCompressionFixedRateFlagsEXT(4);
    pub const VK_IMAGE_COMPRESSION_FIXED_RATE_4BPC_BIT_EXT: VkImageCompressionFixedRateFlagsEXT = VkImageCompressionFixedRateFlagsEXT(8);
    pub const VK_IMAGE_COMPRESSION_FIXED_RATE_5BPC_BIT_EXT: VkImageCompressionFixedRateFlagsEXT = VkImageCompressionFixedRateFlagsEXT(16);
    pub const VK_IMAGE_COMPRESSION_FIXED_RATE_6BPC_BIT_EXT: VkImageCompressionFixedRateFlagsEXT = VkImageCompressionFixedRateFlagsEXT(32);
    pub const VK_IMAGE_COMPRESSION_FIXED_RATE_7BPC_BIT_EXT: VkImageCompressionFixedRateFlagsEXT = VkImageCompressionFixedRateFlagsEXT(64);
    pub const VK_IMAGE_COMPRESSION_FIXED_RATE_8BPC_BIT_EXT: VkImageCompressionFixedRateFlagsEXT = VkImageCompressionFixedRateFlagsEXT(128);
    pub const VK_IMAGE_COMPRESSION_FIXED_RATE_9BPC_BIT_EXT: VkImageCompressionFixedRateFlagsEXT = VkImageCompressionFixedRateFlagsEXT(256);
    pub const VK_IMAGE_COMPRESSION_FIXED_RATE_10BPC_BIT_EXT: VkImageCompressionFixedRateFlagsEXT = VkImageCompressionFixedRateFlagsEXT(512);
    pub const VK_IMAGE_COMPRESSION_FIXED_RATE_11BPC_BIT_EXT: VkImageCompressionFixedRateFlagsEXT = VkImageCompressionFixedRateFlagsEXT(1024);
    pub const VK_IMAGE_COMPRESSION_FIXED_RATE_12BPC_BIT_EXT: VkImageCompressionFixedRateFlagsEXT = VkImageCompressionFixedRateFlagsEXT(2048);
    pub const VK_IMAGE_COMPRESSION_FIXED_RATE_13BPC_BIT_EXT: VkImageCompressionFixedRateFlagsEXT = VkImageCompressionFixedRateFlagsEXT(4096);
    pub const VK_IMAGE_COMPRESSION_FIXED_RATE_14BPC_BIT_EXT: VkImageCompressionFixedRateFlagsEXT = VkImageCompressionFixedRateFlagsEXT(8192);
    pub const VK_IMAGE_COMPRESSION_FIXED_RATE_15BPC_BIT_EXT: VkImageCompressionFixedRateFlagsEXT = VkImageCompressionFixedRateFlagsEXT(16384);
    pub const VK_IMAGE_COMPRESSION_FIXED_RATE_16BPC_BIT_EXT: VkImageCompressionFixedRateFlagsEXT = VkImageCompressionFixedRateFlagsEXT(32768);
    pub const VK_IMAGE_COMPRESSION_FIXED_RATE_17BPC_BIT_EXT: VkImageCompressionFixedRateFlagsEXT = VkImageCompressionFixedRateFlagsEXT(65536);
    pub const VK_IMAGE_COMPRESSION_FIXED_RATE_18BPC_BIT_EXT: VkImageCompressionFixedRateFlagsEXT = VkImageCompressionFixedRateFlagsEXT(131072);
    pub const VK_IMAGE_COMPRESSION_FIXED_RATE_19BPC_BIT_EXT: VkImageCompressionFixedRateFlagsEXT = VkImageCompressionFixedRateFlagsEXT(262144);
    pub const VK_IMAGE_COMPRESSION_FIXED_RATE_20BPC_BIT_EXT: VkImageCompressionFixedRateFlagsEXT = VkImageCompressionFixedRateFlagsEXT(524288);
    pub const VK_IMAGE_COMPRESSION_FIXED_RATE_21BPC_BIT_EXT: VkImageCompressionFixedRateFlagsEXT = VkImageCompressionFixedRateFlagsEXT(1048576);
    pub const VK_IMAGE_COMPRESSION_FIXED_RATE_22BPC_BIT_EXT: VkImageCompressionFixedRateFlagsEXT = VkImageCompressionFixedRateFlagsEXT(2097152);
    pub const VK_IMAGE_COMPRESSION_FIXED_RATE_23BPC_BIT_EXT: VkImageCompressionFixedRateFlagsEXT = VkImageCompressionFixedRateFlagsEXT(4194304);
    pub const VK_IMAGE_COMPRESSION_FIXED_RATE_24BPC_BIT_EXT: VkImageCompressionFixedRateFlagsEXT = VkImageCompressionFixedRateFlagsEXT(8388608);
}

pub struct VkPipelineRobustnessBufferBehavior(i32); //
impl VkPipelineRobustnessBufferBehavior {
    pub const VK_PIPELINE_ROBUSTNESS_BUFFER_BEHAVIOR_DEVICE_DEFAULT: i32 = 0;
    pub const VK_PIPELINE_ROBUSTNESS_BUFFER_BEHAVIOR_DISABLED: i32 = 1;
    pub const VK_PIPELINE_ROBUSTNESS_BUFFER_BEHAVIOR_ROBUST_BUFFER_ACCESS: i32 = 2;
    pub const VK_PIPELINE_ROBUSTNESS_BUFFER_BEHAVIOR_ROBUST_BUFFER_ACCESS_2: i32 = 3;
}

pub struct VkPipelineRobustnessImageBehavior(i32); //
impl VkPipelineRobustnessImageBehavior {
    pub const VK_PIPELINE_ROBUSTNESS_IMAGE_BEHAVIOR_DEVICE_DEFAULT: i32 = 0;
    pub const VK_PIPELINE_ROBUSTNESS_IMAGE_BEHAVIOR_DISABLED: i32 = 1;
    pub const VK_PIPELINE_ROBUSTNESS_IMAGE_BEHAVIOR_ROBUST_IMAGE_ACCESS: i32 = 2;
    pub const VK_PIPELINE_ROBUSTNESS_IMAGE_BEHAVIOR_ROBUST_IMAGE_ACCESS_2: i32 = 3;
}

pub type VkOpticalFlowGridSizeFlagBitsNV = VkOpticalFlowGridSizeFlagsNV; //
impl VkOpticalFlowGridSizeFlagBitsNV {
    pub const VK_OPTICAL_FLOW_GRID_SIZE_UNKNOWN_NV: VkOpticalFlowGridSizeFlagsNV = VkOpticalFlowGridSizeFlagsNV(0);
    pub const VK_OPTICAL_FLOW_GRID_SIZE_1X1_BIT_NV: VkOpticalFlowGridSizeFlagsNV = VkOpticalFlowGridSizeFlagsNV(1);
    pub const VK_OPTICAL_FLOW_GRID_SIZE_2X2_BIT_NV: VkOpticalFlowGridSizeFlagsNV = VkOpticalFlowGridSizeFlagsNV(2);
    pub const VK_OPTICAL_FLOW_GRID_SIZE_4X4_BIT_NV: VkOpticalFlowGridSizeFlagsNV = VkOpticalFlowGridSizeFlagsNV(4);
    pub const VK_OPTICAL_FLOW_GRID_SIZE_8X8_BIT_NV: VkOpticalFlowGridSizeFlagsNV = VkOpticalFlowGridSizeFlagsNV(8);
}

pub type VkOpticalFlowUsageFlagBitsNV = VkOpticalFlowUsageFlagsNV; //
impl VkOpticalFlowUsageFlagBitsNV {
    pub const VK_OPTICAL_FLOW_USAGE_UNKNOWN_NV: VkOpticalFlowUsageFlagsNV = VkOpticalFlowUsageFlagsNV(0);
    pub const VK_OPTICAL_FLOW_USAGE_INPUT_BIT_NV: VkOpticalFlowUsageFlagsNV = VkOpticalFlowUsageFlagsNV(1);
    pub const VK_OPTICAL_FLOW_USAGE_OUTPUT_BIT_NV: VkOpticalFlowUsageFlagsNV = VkOpticalFlowUsageFlagsNV(2);
    pub const VK_OPTICAL_FLOW_USAGE_HINT_BIT_NV: VkOpticalFlowUsageFlagsNV = VkOpticalFlowUsageFlagsNV(4);
    pub const VK_OPTICAL_FLOW_USAGE_COST_BIT_NV: VkOpticalFlowUsageFlagsNV = VkOpticalFlowUsageFlagsNV(8);
    pub const VK_OPTICAL_FLOW_USAGE_GLOBAL_FLOW_BIT_NV: VkOpticalFlowUsageFlagsNV = VkOpticalFlowUsageFlagsNV(16);
}

pub struct VkOpticalFlowPerformanceLevelNV(i32); //
impl VkOpticalFlowPerformanceLevelNV {
    pub const VK_OPTICAL_FLOW_PERFORMANCE_LEVEL_UNKNOWN_NV: i32 = 0;
    pub const VK_OPTICAL_FLOW_PERFORMANCE_LEVEL_SLOW_NV: i32 = 1;
    pub const VK_OPTICAL_FLOW_PERFORMANCE_LEVEL_MEDIUM_NV: i32 = 2;
    pub const VK_OPTICAL_FLOW_PERFORMANCE_LEVEL_FAST_NV: i32 = 3;
}

pub struct VkOpticalFlowSessionBindingPointNV(i32); //
impl VkOpticalFlowSessionBindingPointNV {
    pub const VK_OPTICAL_FLOW_SESSION_BINDING_POINT_UNKNOWN_NV: i32 = 0;
    pub const VK_OPTICAL_FLOW_SESSION_BINDING_POINT_INPUT_NV: i32 = 1;
    pub const VK_OPTICAL_FLOW_SESSION_BINDING_POINT_REFERENCE_NV: i32 = 2;
    pub const VK_OPTICAL_FLOW_SESSION_BINDING_POINT_HINT_NV: i32 = 3;
    pub const VK_OPTICAL_FLOW_SESSION_BINDING_POINT_FLOW_VECTOR_NV: i32 = 4;
    pub const VK_OPTICAL_FLOW_SESSION_BINDING_POINT_BACKWARD_FLOW_VECTOR_NV: i32 = 5;
    pub const VK_OPTICAL_FLOW_SESSION_BINDING_POINT_COST_NV: i32 = 6;
    pub const VK_OPTICAL_FLOW_SESSION_BINDING_POINT_BACKWARD_COST_NV: i32 = 7;
    pub const VK_OPTICAL_FLOW_SESSION_BINDING_POINT_GLOBAL_FLOW_NV: i32 = 8;
}

pub type VkOpticalFlowSessionCreateFlagBitsNV = VkOpticalFlowSessionCreateFlagsNV; //
impl VkOpticalFlowSessionCreateFlagBitsNV {
    pub const VK_OPTICAL_FLOW_SESSION_CREATE_ENABLE_HINT_BIT_NV: VkOpticalFlowSessionCreateFlagsNV = VkOpticalFlowSessionCreateFlagsNV(1);
    pub const VK_OPTICAL_FLOW_SESSION_CREATE_ENABLE_COST_BIT_NV: VkOpticalFlowSessionCreateFlagsNV = VkOpticalFlowSessionCreateFlagsNV(2);
    pub const VK_OPTICAL_FLOW_SESSION_CREATE_ENABLE_GLOBAL_FLOW_BIT_NV: VkOpticalFlowSessionCreateFlagsNV = VkOpticalFlowSessionCreateFlagsNV(4);
    pub const VK_OPTICAL_FLOW_SESSION_CREATE_ALLOW_REGIONS_BIT_NV: VkOpticalFlowSessionCreateFlagsNV = VkOpticalFlowSessionCreateFlagsNV(8);
    pub const VK_OPTICAL_FLOW_SESSION_CREATE_BOTH_DIRECTIONS_BIT_NV: VkOpticalFlowSessionCreateFlagsNV = VkOpticalFlowSessionCreateFlagsNV(16);
}

pub type VkOpticalFlowExecuteFlagBitsNV = VkOpticalFlowExecuteFlagsNV; //
impl VkOpticalFlowExecuteFlagBitsNV {
    pub const VK_OPTICAL_FLOW_EXECUTE_DISABLE_TEMPORAL_HINTS_BIT_NV: VkOpticalFlowExecuteFlagsNV = VkOpticalFlowExecuteFlagsNV(1);
}

pub struct VkMicromapTypeEXT(i32); //
impl VkMicromapTypeEXT {
    pub const VK_MICROMAP_TYPE_OPACITY_MICROMAP_EXT: i32 = 0;
}

pub type VkBuildMicromapFlagBitsEXT = VkBuildMicromapFlagsEXT; //
impl VkBuildMicromapFlagBitsEXT {
    pub const VK_BUILD_MICROMAP_PREFER_FAST_TRACE_BIT_EXT: VkBuildMicromapFlagsEXT = VkBuildMicromapFlagsEXT(1);
    pub const VK_BUILD_MICROMAP_PREFER_FAST_BUILD_BIT_EXT: VkBuildMicromapFlagsEXT = VkBuildMicromapFlagsEXT(2);
    pub const VK_BUILD_MICROMAP_ALLOW_COMPACTION_BIT_EXT: VkBuildMicromapFlagsEXT = VkBuildMicromapFlagsEXT(4);
}

pub type VkMicromapCreateFlagBitsEXT = VkMicromapCreateFlagsEXT; //
impl VkMicromapCreateFlagBitsEXT {
    pub const VK_MICROMAP_CREATE_DEVICE_ADDRESS_CAPTURE_REPLAY_BIT_EXT: VkMicromapCreateFlagsEXT = VkMicromapCreateFlagsEXT(1);
}

pub struct VkCopyMicromapModeEXT(i32); //
impl VkCopyMicromapModeEXT {
    pub const VK_COPY_MICROMAP_MODE_CLONE_EXT: i32 = 0;
    pub const VK_COPY_MICROMAP_MODE_SERIALIZE_EXT: i32 = 1;
    pub const VK_COPY_MICROMAP_MODE_DESERIALIZE_EXT: i32 = 2;
    pub const VK_COPY_MICROMAP_MODE_COMPACT_EXT: i32 = 3;
}

pub struct VkBuildMicromapModeEXT(i32); //
impl VkBuildMicromapModeEXT {
    pub const VK_BUILD_MICROMAP_MODE_BUILD_EXT: i32 = 0;
}

pub struct VkOpacityMicromapFormatKHR(i32); //
impl VkOpacityMicromapFormatKHR {
    pub const VK_OPACITY_MICROMAP_FORMAT_2_STATE_KHR: i32 = 1;
    pub const VK_OPACITY_MICROMAP_FORMAT_4_STATE_KHR: i32 = 2;
}

pub struct VkOpacityMicromapSpecialIndexKHR(i32); //
impl VkOpacityMicromapSpecialIndexKHR {
    pub const VK_OPACITY_MICROMAP_SPECIAL_INDEX_FULLY_TRANSPARENT_KHR: i32 = -1;
    pub const VK_OPACITY_MICROMAP_SPECIAL_INDEX_FULLY_OPAQUE_KHR: i32 = -2;
    pub const VK_OPACITY_MICROMAP_SPECIAL_INDEX_FULLY_UNKNOWN_TRANSPARENT_KHR: i32 = -3;
    pub const VK_OPACITY_MICROMAP_SPECIAL_INDEX_FULLY_UNKNOWN_OPAQUE_KHR: i32 = -4;
}

pub struct VkAccelerationStructureSerializedBlockTypeKHR(i32); //
impl VkAccelerationStructureSerializedBlockTypeKHR {
    pub const VK_ACCELERATION_STRUCTURE_SERIALIZED_BLOCK_TYPE_OPACITY_MICROMAP_KHR: i32 = 0;
}

pub struct VkDepthBiasRepresentationEXT(i32); //
impl VkDepthBiasRepresentationEXT {
    pub const VK_DEPTH_BIAS_REPRESENTATION_LEAST_REPRESENTABLE_VALUE_FORMAT_EXT: i32 = 0;
    pub const VK_DEPTH_BIAS_REPRESENTATION_LEAST_REPRESENTABLE_VALUE_FORCE_UNORM_EXT: i32 = 1;
    pub const VK_DEPTH_BIAS_REPRESENTATION_FLOAT_EXT: i32 = 2;
}

pub type VkDeviceFaultFlagBitsKHR = VkDeviceFaultFlagsKHR; //
impl VkDeviceFaultFlagBitsKHR {
    pub const VK_DEVICE_FAULT_FLAG_DEVICE_LOST_KHR: VkDeviceFaultFlagsKHR = VkDeviceFaultFlagsKHR(1);
    pub const VK_DEVICE_FAULT_FLAG_MEMORY_ADDRESS_KHR: VkDeviceFaultFlagsKHR = VkDeviceFaultFlagsKHR(2);
    pub const VK_DEVICE_FAULT_FLAG_INSTRUCTION_ADDRESS_KHR: VkDeviceFaultFlagsKHR = VkDeviceFaultFlagsKHR(4);
    pub const VK_DEVICE_FAULT_FLAG_VENDOR_KHR: VkDeviceFaultFlagsKHR = VkDeviceFaultFlagsKHR(8);
    pub const VK_DEVICE_FAULT_FLAG_WATCHDOG_TIMEOUT_KHR: VkDeviceFaultFlagsKHR = VkDeviceFaultFlagsKHR(16);
    pub const VK_DEVICE_FAULT_FLAG_OVERFLOW_KHR: VkDeviceFaultFlagsKHR = VkDeviceFaultFlagsKHR(32);
}

pub struct VkDeviceFaultAddressTypeKHR(i32); //
impl VkDeviceFaultAddressTypeKHR {
    pub const VK_DEVICE_FAULT_ADDRESS_TYPE_NONE_KHR: i32 = 0;
    pub const VK_DEVICE_FAULT_ADDRESS_TYPE_READ_INVALID_KHR: i32 = 1;
    pub const VK_DEVICE_FAULT_ADDRESS_TYPE_WRITE_INVALID_KHR: i32 = 2;
    pub const VK_DEVICE_FAULT_ADDRESS_TYPE_EXECUTE_INVALID_KHR: i32 = 3;
    pub const VK_DEVICE_FAULT_ADDRESS_TYPE_INSTRUCTION_POINTER_UNKNOWN_KHR: i32 = 4;
    pub const VK_DEVICE_FAULT_ADDRESS_TYPE_INSTRUCTION_POINTER_INVALID_KHR: i32 = 5;
    pub const VK_DEVICE_FAULT_ADDRESS_TYPE_INSTRUCTION_POINTER_FAULT_KHR: i32 = 6;
}

pub struct VkDeviceFaultVendorBinaryHeaderVersionKHR(i32); //
impl VkDeviceFaultVendorBinaryHeaderVersionKHR {
    pub const VK_DEVICE_FAULT_VENDOR_BINARY_HEADER_VERSION_ONE_KHR: i32 = 1;
    pub const VK_DEVICE_FAULT_VENDOR_BINARY_HEADER_VERSION_ONE_EXT: i32 = Self::VK_DEVICE_FAULT_VENDOR_BINARY_HEADER_VERSION_ONE_KHR;
}

pub type VkIndirectCommandsLayoutUsageFlagBitsEXT = VkIndirectCommandsLayoutUsageFlagsEXT; //
impl VkIndirectCommandsLayoutUsageFlagBitsEXT {
    pub const VK_INDIRECT_COMMANDS_LAYOUT_USAGE_EXPLICIT_PREPROCESS_BIT_EXT: VkIndirectCommandsLayoutUsageFlagsEXT = VkIndirectCommandsLayoutUsageFlagsEXT(1);
    pub const VK_INDIRECT_COMMANDS_LAYOUT_USAGE_UNORDERED_SEQUENCES_BIT_EXT: VkIndirectCommandsLayoutUsageFlagsEXT = VkIndirectCommandsLayoutUsageFlagsEXT(2);
}

pub struct VkIndirectExecutionSetInfoTypeEXT(i32); //
impl VkIndirectExecutionSetInfoTypeEXT {
    pub const VK_INDIRECT_EXECUTION_SET_INFO_TYPE_PIPELINES_EXT: i32 = 0;
    pub const VK_INDIRECT_EXECUTION_SET_INFO_TYPE_SHADER_OBJECTS_EXT: i32 = 1;
}

pub type VkIndirectCommandsInputModeFlagBitsEXT = VkIndirectCommandsInputModeFlagsEXT; //
impl VkIndirectCommandsInputModeFlagBitsEXT {
    pub const VK_INDIRECT_COMMANDS_INPUT_MODE_VULKAN_INDEX_BUFFER_EXT: VkIndirectCommandsInputModeFlagsEXT = VkIndirectCommandsInputModeFlagsEXT(1);
    pub const VK_INDIRECT_COMMANDS_INPUT_MODE_DXGI_INDEX_BUFFER_EXT: VkIndirectCommandsInputModeFlagsEXT = VkIndirectCommandsInputModeFlagsEXT(2);
}

pub struct VkIndirectCommandsTokenTypeEXT(i32); //
impl VkIndirectCommandsTokenTypeEXT {
    pub const VK_INDIRECT_COMMANDS_TOKEN_TYPE_EXECUTION_SET_EXT: i32 = 0;
    pub const VK_INDIRECT_COMMANDS_TOKEN_TYPE_PUSH_CONSTANT_EXT: i32 = 1;
    pub const VK_INDIRECT_COMMANDS_TOKEN_TYPE_SEQUENCE_INDEX_EXT: i32 = 2;
    pub const VK_INDIRECT_COMMANDS_TOKEN_TYPE_INDEX_BUFFER_EXT: i32 = 3;
    pub const VK_INDIRECT_COMMANDS_TOKEN_TYPE_VERTEX_BUFFER_EXT: i32 = 4;
    pub const VK_INDIRECT_COMMANDS_TOKEN_TYPE_DRAW_INDEXED_EXT: i32 = 5;
    pub const VK_INDIRECT_COMMANDS_TOKEN_TYPE_DRAW_EXT: i32 = 6;
    pub const VK_INDIRECT_COMMANDS_TOKEN_TYPE_DRAW_INDEXED_COUNT_EXT: i32 = 7;
    pub const VK_INDIRECT_COMMANDS_TOKEN_TYPE_DRAW_COUNT_EXT: i32 = 8;
    pub const VK_INDIRECT_COMMANDS_TOKEN_TYPE_DISPATCH_EXT: i32 = 9;
}

pub struct VkDisplacementMicromapFormatNV(i32); //
impl VkDisplacementMicromapFormatNV {
    pub const VK_DISPLACEMENT_MICROMAP_FORMAT_64_TRIANGLES_64_BYTES_NV: i32 = 1;
    pub const VK_DISPLACEMENT_MICROMAP_FORMAT_256_TRIANGLES_128_BYTES_NV: i32 = 2;
    pub const VK_DISPLACEMENT_MICROMAP_FORMAT_1024_TRIANGLES_128_BYTES_NV: i32 = 3;
}

pub type VkShaderCreateFlagBitsEXT = VkShaderCreateFlagsEXT; //
impl VkShaderCreateFlagBitsEXT {
    pub const VK_SHADER_CREATE_LINK_STAGE_BIT_EXT: VkShaderCreateFlagsEXT = VkShaderCreateFlagsEXT(1);
}

pub struct VkShaderCodeTypeEXT(i32); //
impl VkShaderCodeTypeEXT {
    pub const VK_SHADER_CODE_TYPE_BINARY_EXT: i32 = 0;
    pub const VK_SHADER_CODE_TYPE_SPIRV_EXT: i32 = 1;
}

pub struct VkScopeKHR(i32); //
impl VkScopeKHR {
    pub const VK_SCOPE_DEVICE_KHR: i32 = 1;
    pub const VK_SCOPE_WORKGROUP_KHR: i32 = 2;
    pub const VK_SCOPE_SUBGROUP_KHR: i32 = 3;
    pub const VK_SCOPE_QUEUE_FAMILY_KHR: i32 = 5;
}

pub struct VkComponentTypeKHR(i32); //
impl VkComponentTypeKHR {
    pub const VK_COMPONENT_TYPE_FLOAT16_KHR: i32 = 0;
    pub const VK_COMPONENT_TYPE_FLOAT32_KHR: i32 = 1;
    pub const VK_COMPONENT_TYPE_FLOAT64_KHR: i32 = 2;
    pub const VK_COMPONENT_TYPE_SINT8_KHR: i32 = 3;
    pub const VK_COMPONENT_TYPE_SINT16_KHR: i32 = 4;
    pub const VK_COMPONENT_TYPE_SINT32_KHR: i32 = 5;
    pub const VK_COMPONENT_TYPE_SINT64_KHR: i32 = 6;
    pub const VK_COMPONENT_TYPE_UINT8_KHR: i32 = 7;
    pub const VK_COMPONENT_TYPE_UINT16_KHR: i32 = 8;
    pub const VK_COMPONENT_TYPE_UINT32_KHR: i32 = 9;
    pub const VK_COMPONENT_TYPE_UINT64_KHR: i32 = 10;
}

pub struct VkCubicFilterWeightsQCOM(i32); //
impl VkCubicFilterWeightsQCOM {
    pub const VK_CUBIC_FILTER_WEIGHTS_CATMULL_ROM_QCOM: i32 = 0;
    pub const VK_CUBIC_FILTER_WEIGHTS_ZERO_TANGENT_CARDINAL_QCOM: i32 = 1;
    pub const VK_CUBIC_FILTER_WEIGHTS_B_SPLINE_QCOM: i32 = 2;
    pub const VK_CUBIC_FILTER_WEIGHTS_MITCHELL_NETRAVALI_QCOM: i32 = 3;
}

pub struct VkBlockMatchWindowCompareModeQCOM(i32); //
impl VkBlockMatchWindowCompareModeQCOM {
    pub const VK_BLOCK_MATCH_WINDOW_COMPARE_MODE_MIN_QCOM: i32 = 0;
    pub const VK_BLOCK_MATCH_WINDOW_COMPARE_MODE_MAX_QCOM: i32 = 1;
}

pub struct VkPhysicalDeviceLayeredApiKHR(i32); //
impl VkPhysicalDeviceLayeredApiKHR {
    pub const VK_PHYSICAL_DEVICE_LAYERED_API_VULKAN_KHR: i32 = 0;
    pub const VK_PHYSICAL_DEVICE_LAYERED_API_D3D12_KHR: i32 = 1;
    pub const VK_PHYSICAL_DEVICE_LAYERED_API_METAL_KHR: i32 = 2;
    pub const VK_PHYSICAL_DEVICE_LAYERED_API_OPENGL_KHR: i32 = 3;
    pub const VK_PHYSICAL_DEVICE_LAYERED_API_OPENGLES_KHR: i32 = 4;
}

pub struct VkLayeredDriverUnderlyingApiMSFT(i32); //
impl VkLayeredDriverUnderlyingApiMSFT {
    pub const VK_LAYERED_DRIVER_UNDERLYING_API_NONE_MSFT: i32 = 0;
    pub const VK_LAYERED_DRIVER_UNDERLYING_API_D3D12_MSFT: i32 = 1;
}

pub struct VkLatencyMarkerNV(i32); //
impl VkLatencyMarkerNV {
    pub const VK_LATENCY_MARKER_SIMULATION_START_NV: i32 = 0;
    pub const VK_LATENCY_MARKER_SIMULATION_END_NV: i32 = 1;
    pub const VK_LATENCY_MARKER_RENDERSUBMIT_START_NV: i32 = 2;
    pub const VK_LATENCY_MARKER_RENDERSUBMIT_END_NV: i32 = 3;
    pub const VK_LATENCY_MARKER_PRESENT_START_NV: i32 = 4;
    pub const VK_LATENCY_MARKER_PRESENT_END_NV: i32 = 5;
    pub const VK_LATENCY_MARKER_INPUT_SAMPLE_NV: i32 = 6;
    pub const VK_LATENCY_MARKER_TRIGGER_FLASH_NV: i32 = 7;
    pub const VK_LATENCY_MARKER_OUT_OF_BAND_RENDERSUBMIT_START_NV: i32 = 8;
    pub const VK_LATENCY_MARKER_OUT_OF_BAND_RENDERSUBMIT_END_NV: i32 = 9;
    pub const VK_LATENCY_MARKER_OUT_OF_BAND_PRESENT_START_NV: i32 = 10;
    pub const VK_LATENCY_MARKER_OUT_OF_BAND_PRESENT_END_NV: i32 = 11;
}

pub struct VkOutOfBandQueueTypeNV(i32); //
impl VkOutOfBandQueueTypeNV {
    pub const VK_OUT_OF_BAND_QUEUE_TYPE_RENDER_NV: i32 = 0;
    pub const VK_OUT_OF_BAND_QUEUE_TYPE_PRESENT_NV: i32 = 1;
}

pub type VkMemoryUnmapFlagBits = VkMemoryUnmapFlags; //
impl VkMemoryUnmapFlagBits {
}

pub struct VkCompressedTriangleFormatAMDX(i32); //
impl VkCompressedTriangleFormatAMDX {
    pub const VK_COMPRESSED_TRIANGLE_FORMAT_DGF1_AMDX: i32 = 0;
}

pub type VkWaylandSurfaceCreateFlagBitsKHR = VkWaylandSurfaceCreateFlagsKHR; //
impl VkWaylandSurfaceCreateFlagBitsKHR {
}

pub struct VkDepthClampModeEXT(i32); //
impl VkDepthClampModeEXT {
    pub const VK_DEPTH_CLAMP_MODE_VIEWPORT_RANGE_EXT: i32 = 0;
    pub const VK_DEPTH_CLAMP_MODE_USER_DEFINED_RANGE_EXT: i32 = 1;
}

pub type VkAccessFlagBits3KHR = VkAccessFlags3KHR; //
impl VkAccessFlagBits3KHR {
    pub const VK_ACCESS_3_NONE_KHR: VkAccessFlags3KHR = VkAccessFlags3KHR(0);
}

pub type VkTileShadingRenderPassFlagBitsQCOM = VkTileShadingRenderPassFlagsQCOM; //
impl VkTileShadingRenderPassFlagBitsQCOM {
    pub const VK_TILE_SHADING_RENDER_PASS_ENABLE_BIT_QCOM: VkTileShadingRenderPassFlagsQCOM = VkTileShadingRenderPassFlagsQCOM(1);
    pub const VK_TILE_SHADING_RENDER_PASS_PER_TILE_EXECUTION_BIT_QCOM: VkTileShadingRenderPassFlagsQCOM = VkTileShadingRenderPassFlagsQCOM(2);
}

pub struct VkCooperativeVectorMatrixLayoutNV(i32); //
impl VkCooperativeVectorMatrixLayoutNV {
    pub const VK_COOPERATIVE_VECTOR_MATRIX_LAYOUT_ROW_MAJOR_NV: i32 = 0;
    pub const VK_COOPERATIVE_VECTOR_MATRIX_LAYOUT_COLUMN_MAJOR_NV: i32 = 1;
    pub const VK_COOPERATIVE_VECTOR_MATRIX_LAYOUT_INFERENCING_OPTIMAL_NV: i32 = 2;
    pub const VK_COOPERATIVE_VECTOR_MATRIX_LAYOUT_TRAINING_OPTIMAL_NV: i32 = 3;
}

pub type VkAddressCopyFlagBitsKHR = VkAddressCopyFlagsKHR; //
impl VkAddressCopyFlagBitsKHR {
    pub const VK_ADDRESS_COPY_DEVICE_LOCAL_BIT_KHR: VkAddressCopyFlagsKHR = VkAddressCopyFlagsKHR(1);
    pub const VK_ADDRESS_COPY_SPARSE_BIT_KHR: VkAddressCopyFlagsKHR = VkAddressCopyFlagsKHR(2);
    pub const VK_ADDRESS_COPY_PROTECTED_BIT_KHR: VkAddressCopyFlagsKHR = VkAddressCopyFlagsKHR(4);
}

pub type VkTensorCreateFlagBitsARM = VkTensorCreateFlagsARM; //
impl VkTensorCreateFlagBitsARM {
    pub const VK_TENSOR_CREATE_MUTABLE_FORMAT_BIT_ARM: VkTensorCreateFlagsARM = VkTensorCreateFlagsARM(1);
    pub const VK_TENSOR_CREATE_PROTECTED_BIT_ARM: VkTensorCreateFlagsARM = VkTensorCreateFlagsARM(2);
}

pub type VkTensorUsageFlagBitsARM = VkTensorUsageFlagsARM; //
impl VkTensorUsageFlagBitsARM {
    pub const VK_TENSOR_USAGE_SHADER_BIT_ARM: VkTensorUsageFlagsARM = VkTensorUsageFlagsARM(2);
    pub const VK_TENSOR_USAGE_TRANSFER_SRC_BIT_ARM: VkTensorUsageFlagsARM = VkTensorUsageFlagsARM(4);
    pub const VK_TENSOR_USAGE_TRANSFER_DST_BIT_ARM: VkTensorUsageFlagsARM = VkTensorUsageFlagsARM(8);
    pub const VK_TENSOR_USAGE_IMAGE_ALIASING_BIT_ARM: VkTensorUsageFlagsARM = VkTensorUsageFlagsARM(16);
}

pub struct VkTensorTilingARM(i32); //
impl VkTensorTilingARM {
    pub const VK_TENSOR_TILING_OPTIMAL_ARM: i32 = 0;
    pub const VK_TENSOR_TILING_LINEAR_ARM: i32 = 1;
}

pub type VkTensorViewCreateFlagBitsARM = VkTensorViewCreateFlagsARM; //
impl VkTensorViewCreateFlagBitsARM {
}

pub struct VkDefaultVertexAttributeValueKHR(i32); //
impl VkDefaultVertexAttributeValueKHR {
    pub const VK_DEFAULT_VERTEX_ATTRIBUTE_VALUE_ZERO_ZERO_ZERO_ZERO_KHR: i32 = 0;
    pub const VK_DEFAULT_VERTEX_ATTRIBUTE_VALUE_ZERO_ZERO_ZERO_ONE_KHR: i32 = 1;
}

pub type VkDataGraphPipelineSessionCreateFlagBitsARM = VkDataGraphPipelineSessionCreateFlagsARM; //
impl VkDataGraphPipelineSessionCreateFlagBitsARM {
    pub const VK_DATA_GRAPH_PIPELINE_SESSION_CREATE_PROTECTED_BIT_ARM: VkDataGraphPipelineSessionCreateFlagsARM = VkDataGraphPipelineSessionCreateFlagsARM(1);
}

pub struct VkDataGraphPipelineSessionBindPointARM(i32); //
impl VkDataGraphPipelineSessionBindPointARM {
    pub const VK_DATA_GRAPH_PIPELINE_SESSION_BIND_POINT_TRANSIENT_ARM: i32 = 0;
}

pub struct VkDataGraphPipelineSessionBindPointTypeARM(i32); //
impl VkDataGraphPipelineSessionBindPointTypeARM {
    pub const VK_DATA_GRAPH_PIPELINE_SESSION_BIND_POINT_TYPE_MEMORY_ARM: i32 = 0;
}

pub struct VkDataGraphPipelinePropertyARM(i32); //
impl VkDataGraphPipelinePropertyARM {
    pub const VK_DATA_GRAPH_PIPELINE_PROPERTY_CREATION_LOG_ARM: i32 = 0;
    pub const VK_DATA_GRAPH_PIPELINE_PROPERTY_IDENTIFIER_ARM: i32 = 1;
}

pub type VkDataGraphPipelineDispatchFlagBitsARM = VkDataGraphPipelineDispatchFlagsARM; //
impl VkDataGraphPipelineDispatchFlagBitsARM {
}

pub struct VkPhysicalDeviceDataGraphProcessingEngineTypeARM(i32); //
impl VkPhysicalDeviceDataGraphProcessingEngineTypeARM {
    pub const VK_PHYSICAL_DEVICE_DATA_GRAPH_PROCESSING_ENGINE_TYPE_DEFAULT_ARM: i32 = 0;
}

pub struct VkPhysicalDeviceDataGraphOperationTypeARM(i32); //
impl VkPhysicalDeviceDataGraphOperationTypeARM {
    pub const VK_PHYSICAL_DEVICE_DATA_GRAPH_OPERATION_TYPE_SPIRV_EXTENDED_INSTRUCTION_SET_ARM: i32 = 0;
}

pub struct VkDataGraphModelCacheTypeQCOM(i32); //
impl VkDataGraphModelCacheTypeQCOM {
    pub const VK_DATA_GRAPH_MODEL_CACHE_TYPE_GENERIC_BINARY_QCOM: i32 = 0;
}

pub struct VkPerfHintTypeQCOM(i32); //
impl VkPerfHintTypeQCOM {
    pub const VK_PERF_HINT_TYPE_DEFAULT_QCOM: i32 = 0;
    pub const VK_PERF_HINT_TYPE_FREQUENCY_MIN_QCOM: i32 = 1;
    pub const VK_PERF_HINT_TYPE_FREQUENCY_MAX_QCOM: i32 = 2;
    pub const VK_PERF_HINT_TYPE_FREQUENCY_SCALED_QCOM: i32 = 3;
}

pub struct VkThrottleHintTypeSEC(i32); //
impl VkThrottleHintTypeSEC {
    pub const VK_THROTTLE_HINT_TYPE_DEFAULT_SEC: i32 = 0;
    pub const VK_THROTTLE_HINT_TYPE_LOW_SEC: i32 = 1;
    pub const VK_THROTTLE_HINT_TYPE_HIGH_SEC: i32 = 2;
}

pub type VkVideoEncodeRgbModelConversionFlagBitsVALVE = VkVideoEncodeRgbModelConversionFlagsVALVE; //
impl VkVideoEncodeRgbModelConversionFlagBitsVALVE {
    pub const VK_VIDEO_ENCODE_RGB_MODEL_CONVERSION_RGB_IDENTITY_BIT_VALVE: VkVideoEncodeRgbModelConversionFlagsVALVE = VkVideoEncodeRgbModelConversionFlagsVALVE(1);
    pub const VK_VIDEO_ENCODE_RGB_MODEL_CONVERSION_YCBCR_IDENTITY_BIT_VALVE: VkVideoEncodeRgbModelConversionFlagsVALVE = VkVideoEncodeRgbModelConversionFlagsVALVE(2);
    pub const VK_VIDEO_ENCODE_RGB_MODEL_CONVERSION_YCBCR_709_BIT_VALVE: VkVideoEncodeRgbModelConversionFlagsVALVE = VkVideoEncodeRgbModelConversionFlagsVALVE(4);
    pub const VK_VIDEO_ENCODE_RGB_MODEL_CONVERSION_YCBCR_601_BIT_VALVE: VkVideoEncodeRgbModelConversionFlagsVALVE = VkVideoEncodeRgbModelConversionFlagsVALVE(8);
    pub const VK_VIDEO_ENCODE_RGB_MODEL_CONVERSION_YCBCR_2020_BIT_VALVE: VkVideoEncodeRgbModelConversionFlagsVALVE = VkVideoEncodeRgbModelConversionFlagsVALVE(16);
}

pub type VkVideoEncodeRgbRangeCompressionFlagBitsVALVE = VkVideoEncodeRgbRangeCompressionFlagsVALVE; //
impl VkVideoEncodeRgbRangeCompressionFlagBitsVALVE {
    pub const VK_VIDEO_ENCODE_RGB_RANGE_COMPRESSION_FULL_RANGE_BIT_VALVE: VkVideoEncodeRgbRangeCompressionFlagsVALVE = VkVideoEncodeRgbRangeCompressionFlagsVALVE(1);
    pub const VK_VIDEO_ENCODE_RGB_RANGE_COMPRESSION_NARROW_RANGE_BIT_VALVE: VkVideoEncodeRgbRangeCompressionFlagsVALVE = VkVideoEncodeRgbRangeCompressionFlagsVALVE(2);
}

pub type VkVideoEncodeRgbChromaOffsetFlagBitsVALVE = VkVideoEncodeRgbChromaOffsetFlagsVALVE; //
impl VkVideoEncodeRgbChromaOffsetFlagBitsVALVE {
    pub const VK_VIDEO_ENCODE_RGB_CHROMA_OFFSET_COSITED_EVEN_BIT_VALVE: VkVideoEncodeRgbChromaOffsetFlagsVALVE = VkVideoEncodeRgbChromaOffsetFlagsVALVE(1);
    pub const VK_VIDEO_ENCODE_RGB_CHROMA_OFFSET_MIDPOINT_BIT_VALVE: VkVideoEncodeRgbChromaOffsetFlagsVALVE = VkVideoEncodeRgbChromaOffsetFlagsVALVE(2);
}

pub type VkSwapchainImageUsageFlagBitsOHOS = VkSwapchainImageUsageFlagsOHOS; //
impl VkSwapchainImageUsageFlagBitsOHOS {
    pub const VK_SWAPCHAIN_IMAGE_USAGE_SHARED_BIT_OHOS: VkSwapchainImageUsageFlagsOHOS = VkSwapchainImageUsageFlagsOHOS(1);
}

pub struct VkDescriptorMappingSourceEXT(i32); //
impl VkDescriptorMappingSourceEXT {
    pub const VK_DESCRIPTOR_MAPPING_SOURCE_HEAP_WITH_CONSTANT_OFFSET_EXT: i32 = 0;
    pub const VK_DESCRIPTOR_MAPPING_SOURCE_HEAP_WITH_PUSH_INDEX_EXT: i32 = 1;
    pub const VK_DESCRIPTOR_MAPPING_SOURCE_HEAP_WITH_INDIRECT_INDEX_EXT: i32 = 2;
    pub const VK_DESCRIPTOR_MAPPING_SOURCE_HEAP_WITH_INDIRECT_INDEX_ARRAY_EXT: i32 = 3;
    pub const VK_DESCRIPTOR_MAPPING_SOURCE_RESOURCE_HEAP_DATA_EXT: i32 = 4;
    pub const VK_DESCRIPTOR_MAPPING_SOURCE_PUSH_DATA_EXT: i32 = 5;
    pub const VK_DESCRIPTOR_MAPPING_SOURCE_PUSH_ADDRESS_EXT: i32 = 6;
    pub const VK_DESCRIPTOR_MAPPING_SOURCE_INDIRECT_ADDRESS_EXT: i32 = 7;
}

pub type VkSpirvResourceTypeFlagBitsEXT = VkSpirvResourceTypeFlagsEXT; //
impl VkSpirvResourceTypeFlagBitsEXT {
    pub const VK_SPIRV_RESOURCE_TYPE_ALL_EXT: VkSpirvResourceTypeFlagsEXT = VkSpirvResourceTypeFlagsEXT(0);
    pub const VK_SPIRV_RESOURCE_TYPE_SAMPLER_BIT_EXT: VkSpirvResourceTypeFlagsEXT = VkSpirvResourceTypeFlagsEXT(1);
    pub const VK_SPIRV_RESOURCE_TYPE_SAMPLED_IMAGE_BIT_EXT: VkSpirvResourceTypeFlagsEXT = VkSpirvResourceTypeFlagsEXT(2);
    pub const VK_SPIRV_RESOURCE_TYPE_READ_ONLY_IMAGE_BIT_EXT: VkSpirvResourceTypeFlagsEXT = VkSpirvResourceTypeFlagsEXT(4);
    pub const VK_SPIRV_RESOURCE_TYPE_READ_WRITE_IMAGE_BIT_EXT: VkSpirvResourceTypeFlagsEXT = VkSpirvResourceTypeFlagsEXT(8);
    pub const VK_SPIRV_RESOURCE_TYPE_COMBINED_SAMPLED_IMAGE_BIT_EXT: VkSpirvResourceTypeFlagsEXT = VkSpirvResourceTypeFlagsEXT(16);
    pub const VK_SPIRV_RESOURCE_TYPE_UNIFORM_BUFFER_BIT_EXT: VkSpirvResourceTypeFlagsEXT = VkSpirvResourceTypeFlagsEXT(32);
    pub const VK_SPIRV_RESOURCE_TYPE_READ_ONLY_STORAGE_BUFFER_BIT_EXT: VkSpirvResourceTypeFlagsEXT = VkSpirvResourceTypeFlagsEXT(64);
    pub const VK_SPIRV_RESOURCE_TYPE_READ_WRITE_STORAGE_BUFFER_BIT_EXT: VkSpirvResourceTypeFlagsEXT = VkSpirvResourceTypeFlagsEXT(128);
}

pub type VkGpaSqShaderStageFlagBitsAMD = VkGpaSqShaderStageFlagsAMD; //
impl VkGpaSqShaderStageFlagBitsAMD {
    pub const VK_GPA_SQ_SHADER_STAGE_PS_BIT_AMD: VkGpaSqShaderStageFlagsAMD = VkGpaSqShaderStageFlagsAMD(1);
    pub const VK_GPA_SQ_SHADER_STAGE_VS_BIT_AMD: VkGpaSqShaderStageFlagsAMD = VkGpaSqShaderStageFlagsAMD(2);
    pub const VK_GPA_SQ_SHADER_STAGE_GS_BIT_AMD: VkGpaSqShaderStageFlagsAMD = VkGpaSqShaderStageFlagsAMD(4);
    pub const VK_GPA_SQ_SHADER_STAGE_ES_BIT_AMD: VkGpaSqShaderStageFlagsAMD = VkGpaSqShaderStageFlagsAMD(8);
    pub const VK_GPA_SQ_SHADER_STAGE_HS_BIT_AMD: VkGpaSqShaderStageFlagsAMD = VkGpaSqShaderStageFlagsAMD(16);
    pub const VK_GPA_SQ_SHADER_STAGE_LS_BIT_AMD: VkGpaSqShaderStageFlagsAMD = VkGpaSqShaderStageFlagsAMD(32);
    pub const VK_GPA_SQ_SHADER_STAGE_CS_BIT_AMD: VkGpaSqShaderStageFlagsAMD = VkGpaSqShaderStageFlagsAMD(64);
}

pub struct VkGpaPerfBlockAMD(i32); //
impl VkGpaPerfBlockAMD {
    pub const VK_GPA_PERF_BLOCK_CPF_AMD: i32 = 0;
    pub const VK_GPA_PERF_BLOCK_IA_AMD: i32 = 1;
    pub const VK_GPA_PERF_BLOCK_VGT_AMD: i32 = 2;
    pub const VK_GPA_PERF_BLOCK_PA_AMD: i32 = 3;
    pub const VK_GPA_PERF_BLOCK_SC_AMD: i32 = 4;
    pub const VK_GPA_PERF_BLOCK_SPI_AMD: i32 = 5;
    pub const VK_GPA_PERF_BLOCK_SQ_AMD: i32 = 6;
    pub const VK_GPA_PERF_BLOCK_SX_AMD: i32 = 7;
    pub const VK_GPA_PERF_BLOCK_TA_AMD: i32 = 8;
    pub const VK_GPA_PERF_BLOCK_TD_AMD: i32 = 9;
    pub const VK_GPA_PERF_BLOCK_TCP_AMD: i32 = 10;
    pub const VK_GPA_PERF_BLOCK_TCC_AMD: i32 = 11;
    pub const VK_GPA_PERF_BLOCK_TCA_AMD: i32 = 12;
    pub const VK_GPA_PERF_BLOCK_DB_AMD: i32 = 13;
    pub const VK_GPA_PERF_BLOCK_CB_AMD: i32 = 14;
    pub const VK_GPA_PERF_BLOCK_GDS_AMD: i32 = 15;
    pub const VK_GPA_PERF_BLOCK_SRBM_AMD: i32 = 16;
    pub const VK_GPA_PERF_BLOCK_GRBM_AMD: i32 = 17;
    pub const VK_GPA_PERF_BLOCK_GRBM_SE_AMD: i32 = 18;
    pub const VK_GPA_PERF_BLOCK_RLC_AMD: i32 = 19;
    pub const VK_GPA_PERF_BLOCK_DMA_AMD: i32 = 20;
    pub const VK_GPA_PERF_BLOCK_MC_AMD: i32 = 21;
    pub const VK_GPA_PERF_BLOCK_CPG_AMD: i32 = 22;
    pub const VK_GPA_PERF_BLOCK_CPC_AMD: i32 = 23;
    pub const VK_GPA_PERF_BLOCK_WD_AMD: i32 = 24;
    pub const VK_GPA_PERF_BLOCK_TCS_AMD: i32 = 25;
    pub const VK_GPA_PERF_BLOCK_ATC_AMD: i32 = 26;
    pub const VK_GPA_PERF_BLOCK_ATC_L2_AMD: i32 = 27;
    pub const VK_GPA_PERF_BLOCK_MC_VM_L2_AMD: i32 = 28;
    pub const VK_GPA_PERF_BLOCK_EA_AMD: i32 = 29;
    pub const VK_GPA_PERF_BLOCK_RPB_AMD: i32 = 30;
    pub const VK_GPA_PERF_BLOCK_RMI_AMD: i32 = 31;
    pub const VK_GPA_PERF_BLOCK_UMCCH_AMD: i32 = 32;
    pub const VK_GPA_PERF_BLOCK_GE_AMD: i32 = 33;
    pub const VK_GPA_PERF_BLOCK_GL1A_AMD: i32 = 34;
    pub const VK_GPA_PERF_BLOCK_GL1C_AMD: i32 = 35;
    pub const VK_GPA_PERF_BLOCK_GL1CG_AMD: i32 = 36;
    pub const VK_GPA_PERF_BLOCK_GL2A_AMD: i32 = 37;
    pub const VK_GPA_PERF_BLOCK_GL2C_AMD: i32 = 38;
    pub const VK_GPA_PERF_BLOCK_CHA_AMD: i32 = 39;
    pub const VK_GPA_PERF_BLOCK_CHC_AMD: i32 = 40;
    pub const VK_GPA_PERF_BLOCK_CHCG_AMD: i32 = 41;
    pub const VK_GPA_PERF_BLOCK_GUS_AMD: i32 = 42;
    pub const VK_GPA_PERF_BLOCK_GCR_AMD: i32 = 43;
    pub const VK_GPA_PERF_BLOCK_PH_AMD: i32 = 44;
    pub const VK_GPA_PERF_BLOCK_UTCL1_AMD: i32 = 45;
    pub const VK_GPA_PERF_BLOCK_GE1_AMD: i32 = Self::VK_GPA_PERF_BLOCK_GE_AMD;
    pub const VK_GPA_PERF_BLOCK_GE_DIST_AMD: i32 = 46;
    pub const VK_GPA_PERF_BLOCK_GE_SE_AMD: i32 = 47;
    pub const VK_GPA_PERF_BLOCK_DF_MALL_AMD: i32 = 48;
    pub const VK_GPA_PERF_BLOCK_SQ_WGP_AMD: i32 = 49;
    pub const VK_GPA_PERF_BLOCK_PC_AMD: i32 = 50;
    pub const VK_GPA_PERF_BLOCK_GL1XA_AMD: i32 = 51;
    pub const VK_GPA_PERF_BLOCK_GL1XC_AMD: i32 = 52;
    pub const VK_GPA_PERF_BLOCK_WGS_AMD: i32 = 53;
    pub const VK_GPA_PERF_BLOCK_EACPWD_AMD: i32 = 54;
    pub const VK_GPA_PERF_BLOCK_EASE_AMD: i32 = 55;
    pub const VK_GPA_PERF_BLOCK_RLCUSER_AMD: i32 = 56;
    pub const VK_GPA_PERF_BLOCK_RLCLOCAL_AMD: i32 = Self::VK_GPA_PERF_BLOCK_RLCUSER_AMD;
}

pub struct VkGpaSampleTypeAMD(i32); //
impl VkGpaSampleTypeAMD {
    pub const VK_GPA_SAMPLE_TYPE_CUMULATIVE_AMD: i32 = 0;
    pub const VK_GPA_SAMPLE_TYPE_TRACE_AMD: i32 = 1;
    pub const VK_GPA_SAMPLE_TYPE_TIMING_AMD: i32 = 2;
}

pub struct VkGpaDeviceClockModeAMD(i32); //
impl VkGpaDeviceClockModeAMD {
    pub const VK_GPA_DEVICE_CLOCK_MODE_DEFAULT_AMD: i32 = 0;
    pub const VK_GPA_DEVICE_CLOCK_MODE_QUERY_AMD: i32 = 1;
    pub const VK_GPA_DEVICE_CLOCK_MODE_PROFILING_AMD: i32 = 2;
    pub const VK_GPA_DEVICE_CLOCK_MODE_MIN_MEMORY_AMD: i32 = 3;
    pub const VK_GPA_DEVICE_CLOCK_MODE_MIN_ENGINE_AMD: i32 = 4;
    pub const VK_GPA_DEVICE_CLOCK_MODE_PEAK_AMD: i32 = 5;
}

pub type VkAddressCommandFlagBitsKHR = VkAddressCommandFlagsKHR; //
impl VkAddressCommandFlagBitsKHR {
    pub const VK_ADDRESS_COMMAND_PROTECTED_BIT_KHR: VkAddressCommandFlagsKHR = VkAddressCommandFlagsKHR(1);
    pub const VK_ADDRESS_COMMAND_FULLY_BOUND_BIT_KHR: VkAddressCommandFlagsKHR = VkAddressCommandFlagsKHR(2);
    pub const VK_ADDRESS_COMMAND_STORAGE_BUFFER_USAGE_BIT_KHR: VkAddressCommandFlagsKHR = VkAddressCommandFlagsKHR(4);
    pub const VK_ADDRESS_COMMAND_UNKNOWN_STORAGE_BUFFER_USAGE_BIT_KHR: VkAddressCommandFlagsKHR = VkAddressCommandFlagsKHR(8);
}

pub type VkDataGraphTOSAQualityFlagBitsARM = VkDataGraphTOSAQualityFlagsARM; //
impl VkDataGraphTOSAQualityFlagBitsARM {
    pub const VK_DATA_GRAPH_TOSA_QUALITY_ACCELERATED_ARM: VkDataGraphTOSAQualityFlagsARM = VkDataGraphTOSAQualityFlagsARM(1);
    pub const VK_DATA_GRAPH_TOSA_QUALITY_CONFORMANT_ARM: VkDataGraphTOSAQualityFlagsARM = VkDataGraphTOSAQualityFlagsARM(2);
    pub const VK_DATA_GRAPH_TOSA_QUALITY_EXPERIMENTAL_ARM: VkDataGraphTOSAQualityFlagsARM = VkDataGraphTOSAQualityFlagsARM(4);
    pub const VK_DATA_GRAPH_TOSA_QUALITY_DEPRECATED_ARM: VkDataGraphTOSAQualityFlagsARM = VkDataGraphTOSAQualityFlagsARM(8);
}

pub struct VkDataGraphTOSALevelARM(i32); //
impl VkDataGraphTOSALevelARM {
    pub const VK_DATA_GRAPH_TOSA_LEVEL_NONE_ARM: i32 = 0;
    pub const VK_DATA_GRAPH_TOSA_LEVEL_8K_ARM: i32 = 1;
}

pub type VkDataGraphOpticalFlowGridSizeFlagBitsARM = VkDataGraphOpticalFlowGridSizeFlagsARM; //
impl VkDataGraphOpticalFlowGridSizeFlagBitsARM {
    pub const VK_DATA_GRAPH_OPTICAL_FLOW_GRID_SIZE_UNKNOWN_ARM: VkDataGraphOpticalFlowGridSizeFlagsARM = VkDataGraphOpticalFlowGridSizeFlagsARM(0);
    pub const VK_DATA_GRAPH_OPTICAL_FLOW_GRID_SIZE_1X1_BIT_ARM: VkDataGraphOpticalFlowGridSizeFlagsARM = VkDataGraphOpticalFlowGridSizeFlagsARM(1);
    pub const VK_DATA_GRAPH_OPTICAL_FLOW_GRID_SIZE_2X2_BIT_ARM: VkDataGraphOpticalFlowGridSizeFlagsARM = VkDataGraphOpticalFlowGridSizeFlagsARM(2);
    pub const VK_DATA_GRAPH_OPTICAL_FLOW_GRID_SIZE_4X4_BIT_ARM: VkDataGraphOpticalFlowGridSizeFlagsARM = VkDataGraphOpticalFlowGridSizeFlagsARM(4);
    pub const VK_DATA_GRAPH_OPTICAL_FLOW_GRID_SIZE_8X8_BIT_ARM: VkDataGraphOpticalFlowGridSizeFlagsARM = VkDataGraphOpticalFlowGridSizeFlagsARM(8);
}

pub type VkDataGraphOpticalFlowImageUsageFlagBitsARM = VkDataGraphOpticalFlowImageUsageFlagsARM; //
impl VkDataGraphOpticalFlowImageUsageFlagBitsARM {
    pub const VK_DATA_GRAPH_OPTICAL_FLOW_IMAGE_USAGE_UNKNOWN_ARM: VkDataGraphOpticalFlowImageUsageFlagsARM = VkDataGraphOpticalFlowImageUsageFlagsARM(0);
    pub const VK_DATA_GRAPH_OPTICAL_FLOW_IMAGE_USAGE_INPUT_BIT_ARM: VkDataGraphOpticalFlowImageUsageFlagsARM = VkDataGraphOpticalFlowImageUsageFlagsARM(1);
    pub const VK_DATA_GRAPH_OPTICAL_FLOW_IMAGE_USAGE_OUTPUT_BIT_ARM: VkDataGraphOpticalFlowImageUsageFlagsARM = VkDataGraphOpticalFlowImageUsageFlagsARM(2);
    pub const VK_DATA_GRAPH_OPTICAL_FLOW_IMAGE_USAGE_HINT_BIT_ARM: VkDataGraphOpticalFlowImageUsageFlagsARM = VkDataGraphOpticalFlowImageUsageFlagsARM(4);
    pub const VK_DATA_GRAPH_OPTICAL_FLOW_IMAGE_USAGE_COST_BIT_ARM: VkDataGraphOpticalFlowImageUsageFlagsARM = VkDataGraphOpticalFlowImageUsageFlagsARM(8);
}

pub struct VkDataGraphOpticalFlowPerformanceLevelARM(i32); //
impl VkDataGraphOpticalFlowPerformanceLevelARM {
    pub const VK_DATA_GRAPH_OPTICAL_FLOW_PERFORMANCE_LEVEL_UNKNOWN_ARM: i32 = 0;
    pub const VK_DATA_GRAPH_OPTICAL_FLOW_PERFORMANCE_LEVEL_SLOW_ARM: i32 = 1;
    pub const VK_DATA_GRAPH_OPTICAL_FLOW_PERFORMANCE_LEVEL_MEDIUM_ARM: i32 = 2;
    pub const VK_DATA_GRAPH_OPTICAL_FLOW_PERFORMANCE_LEVEL_FAST_ARM: i32 = 3;
}

pub struct VkDataGraphPipelineNodeConnectionTypeARM(i32); //
impl VkDataGraphPipelineNodeConnectionTypeARM {
}

pub struct VkDataGraphPipelineNodeTypeARM(i32); //
impl VkDataGraphPipelineNodeTypeARM {
}

pub type VkDataGraphOpticalFlowCreateFlagBitsARM = VkDataGraphOpticalFlowCreateFlagsARM; //
impl VkDataGraphOpticalFlowCreateFlagBitsARM {
    pub const VK_DATA_GRAPH_OPTICAL_FLOW_CREATE_ENABLE_HINT_BIT_ARM: VkDataGraphOpticalFlowCreateFlagsARM = VkDataGraphOpticalFlowCreateFlagsARM(1);
    pub const VK_DATA_GRAPH_OPTICAL_FLOW_CREATE_ENABLE_COST_BIT_ARM: VkDataGraphOpticalFlowCreateFlagsARM = VkDataGraphOpticalFlowCreateFlagsARM(2);
    pub const VK_DATA_GRAPH_OPTICAL_FLOW_CREATE_RESERVED_30_BIT_ARM: VkDataGraphOpticalFlowCreateFlagsARM = VkDataGraphOpticalFlowCreateFlagsARM(1073741824);
}

pub type VkDataGraphOpticalFlowExecuteFlagBitsARM = VkDataGraphOpticalFlowExecuteFlagsARM; //
impl VkDataGraphOpticalFlowExecuteFlagBitsARM {
    pub const VK_DATA_GRAPH_OPTICAL_FLOW_EXECUTE_DISABLE_TEMPORAL_HINTS_BIT_ARM: VkDataGraphOpticalFlowExecuteFlagsARM = VkDataGraphOpticalFlowExecuteFlagsARM(1);
    pub const VK_DATA_GRAPH_OPTICAL_FLOW_EXECUTE_INPUT_UNCHANGED_BIT_ARM: VkDataGraphOpticalFlowExecuteFlagsARM = VkDataGraphOpticalFlowExecuteFlagsARM(2);
    pub const VK_DATA_GRAPH_OPTICAL_FLOW_EXECUTE_REFERENCE_UNCHANGED_BIT_ARM: VkDataGraphOpticalFlowExecuteFlagsARM = VkDataGraphOpticalFlowExecuteFlagsARM(4);
    pub const VK_DATA_GRAPH_OPTICAL_FLOW_EXECUTE_INPUT_IS_PREVIOUS_REFERENCE_BIT_ARM: VkDataGraphOpticalFlowExecuteFlagsARM = VkDataGraphOpticalFlowExecuteFlagsARM(8);
    pub const VK_DATA_GRAPH_OPTICAL_FLOW_EXECUTE_REFERENCE_IS_PREVIOUS_INPUT_BIT_ARM: VkDataGraphOpticalFlowExecuteFlagsARM = VkDataGraphOpticalFlowExecuteFlagsARM(16);
}

pub struct VkNeuralAcceleratorStatisticsModeARM(i32); //
impl VkNeuralAcceleratorStatisticsModeARM {
    pub const VK_NEURAL_ACCELERATOR_STATISTICS_MODE_DISABLED_ARM: i32 = 0;
    pub const VK_NEURAL_ACCELERATOR_STATISTICS_MODE_STATISTICS0_ARM: i32 = 1;
    pub const VK_NEURAL_ACCELERATOR_STATISTICS_MODE_STATISTICS1_ARM: i32 = 2;
}

pub struct VkImageTilingControlEXT(i32); //
impl VkImageTilingControlEXT {
    pub const VK_IMAGE_TILING_CONTROL_DEFAULT_EXT: i32 = 0;
    pub const VK_IMAGE_TILING_CONTROL_MIN_SIZE_EXT: i32 = 1;
    pub const VK_IMAGE_TILING_CONTROL_MAX_PERFORMANCE_EXT: i32 = 2;
}

