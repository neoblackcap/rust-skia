use skia_bindings::{GrVkImageInfo, VkImage, VkImageTiling, VkImageLayout, VkFormat, GrVkYcbcrConversionInfo, GrVkAlloc, VkSamplerYcbcrModelConversion, VkChromaLocation, VkSamplerYcbcrRange, VkFilter };
use std::ffi::c_void;
use super::alloc::Alloc;

#[derive(Debug)]
pub struct ImageInfo {
    pub(crate) native: GrVkImageInfo
}

impl ImageInfo {

    pub unsafe fn new(
        image: *mut c_void,
        alloc: &Alloc,
        image_tiling: VkImageTiling,
        image_layout: VkImageLayout,
        format: VkFormat,
        level_count: u32) -> ImageInfo
    {
        // originally defined as a C macro in vulkan_core.h
        // and therefore not present in the bindings.
        const VK_QUEUE_FAMILY_IGNORED : u32 = 0;

        Self::from_raw(GrVkImageInfo {
            fImage: image as VkImage,
            fAlloc: alloc.native,
            fImageTiling: image_tiling,
            fImageLayout: image_layout,
            fFormat: format,
            fLevelCount: level_count,
            fCurrentQueueFamily: VK_QUEUE_FAMILY_IGNORED,
            fYcbcrConversionInfo: GrVkYcbcrConversionInfo {
                fYcbcrModel: VkSamplerYcbcrModelConversion::VK_SAMPLER_YCBCR_MODEL_CONVERSION_RGB_IDENTITY,
                fYcbcrRange: VkSamplerYcbcrRange::VK_SAMPLER_YCBCR_RANGE_ITU_FULL,
                fXChromaOffset: VkChromaLocation::VK_CHROMA_LOCATION_COSITED_EVEN,
                fYChromaOffset: VkChromaLocation::VK_CHROMA_LOCATION_COSITED_EVEN,
                fChromaFilter: VkFilter::VK_FILTER_NEAREST,
                fForceExplicitReconstruction: 0,
                fExternalFormat: 0,
                fExternalFormatFeatures: 0
            }
        })
    }

    pub(crate) unsafe fn from_raw(image_info: GrVkImageInfo) -> ImageInfo {
        ImageInfo { native: image_info }
    }

    #[inline]
    pub fn image(&self) -> VkImage {
        self.native.fImage
    }

    #[inline]
    pub fn alloc(&self) -> GrVkAlloc {
        self.native.fAlloc
    }

    #[inline]
    pub fn tiling(&self) -> VkImageTiling {
        self.native.fImageTiling
    }

    #[inline]
    pub fn layout(&self) -> VkImageLayout {
        self.native.fImageLayout
    }

    #[inline]
    pub fn format(&self) -> VkFormat {
        self.native.fFormat
    }

    #[inline]
    pub fn level_count(&self) -> u32 {
        self.native.fLevelCount
    }

    #[inline]
    pub fn current_queue_family(&self) -> u32 {
        self.native.fCurrentQueueFamily
    }

    #[inline]
    pub fn ycbcr_conversion_info(&self) -> GrVkYcbcrConversionInfo {
        self.native.fYcbcrConversionInfo
    }
}