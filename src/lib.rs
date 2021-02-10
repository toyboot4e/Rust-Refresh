/*!

Wrapper of Refresh

*/

pub mod img;

pub use refresh_ffi as ffi;

use std::{ops::Deref, os::raw::c_void, rc::Rc};

// --------------------------------------------------------------------------------
// structs

pub use ffi::Refresh_Buffer as Buffer;
pub use ffi::Refresh_ColorBlendState as ColorBlendState;
pub use ffi::Refresh_ColorTargetBlendState as ColorTargetBlendState;
pub use ffi::Refresh_ColorTargetDescription as ColorTargetDescription;
pub use ffi::Refresh_CommandBuffer as CommandBuffer;
pub use ffi::Refresh_ComputePipeline as ComputePipeline;
pub use ffi::Refresh_ComputePipelineCreateInfo as ComputePipelineCreateInfo;
pub use ffi::Refresh_ComputePipelineLayoutCreateInfo as ComputePipelineLayoutCreateInfo;
pub use ffi::Refresh_DepthStencilState as DepthStencilState;
pub use ffi::Refresh_DepthStencilTargetDescription as DepthStencilTargetDescription;
pub use ffi::Refresh_DepthStencilValue as DepthStencilValue;
// pub type Device = ffi::Refresh_Device;
pub use ffi::Refresh_Framebuffer as Framebuffer;
pub use ffi::Refresh_FramebufferCreateInfo as FramebufferCreateInfo;
pub use ffi::Refresh_GraphicsPipeline as GraphicsPipeline;
pub use ffi::Refresh_GraphicsPipelineCreateInfo as GraphicsPipelineCreateInfo;
pub use ffi::Refresh_GraphicsPipelineLayoutCreateInfo as GraphicsPipelineLayoutCreateInfo;
pub use ffi::Refresh_MultisampleState as MultisampleState;
pub use ffi::Refresh_PresentationParameters as PresentationParameters;
pub use ffi::Refresh_RasterizerState as RasterizerState;
pub use ffi::Refresh_Rect as Rect;
pub use ffi::Refresh_RenderPass as RenderPass;
pub use ffi::Refresh_RenderPassCreateInfo as RenderPassCreateInfo;
pub use ffi::Refresh_RenderTarget as RenderTarget;
pub use ffi::Refresh_Sampler as Sampler;
pub use ffi::Refresh_SamplerStateCreateInfo as SamplerStateCreateInfo;
pub use ffi::Refresh_ShaderModule as ShaderModule;
pub use ffi::Refresh_ShaderModuleCreateInfo as ShaderModuleCreateInfo;
pub use ffi::Refresh_ShaderStageState as ShaderStageState;
pub use ffi::Refresh_StencilOpState as StencilOpState;
pub use ffi::Refresh_SysRenderer as SysRenderer;
pub use ffi::Refresh_Texture as Texture;
pub use ffi::Refresh_TextureCreateInfo as TextureCreateInfo;
pub use ffi::Refresh_TextureHandles as TextureHandles;
pub use ffi::Refresh_TextureSlice as TextureSlice;
/// Represents color
pub use ffi::Refresh_Vec4 as Vec4;
pub use ffi::Refresh_VertexAttribute as VertexAttribute;
pub use ffi::Refresh_VertexBinding as VertexBinding;
pub use ffi::Refresh_VertexInputState as VertexInputState;
pub use ffi::Refresh_Viewport as Viewport;
pub use ffi::Refresh_ViewportState as ViewportState;

// pub use ffi::VkDevice_T as VkDevice;
// pub use ffi::VkInstance_T as vkInstance;
// pub use ffi::VkPhysicalDevice_T as vkPhysicalDevice;

pub use ffi::REFRESH_ABI_VERSION as ABI_VERSION;
pub use ffi::REFRESH_MAJOR_VERSION as MAJOR_VERSION;
pub use ffi::REFRESH_MINOR_VERSION as MINOR_VERSION;
pub use ffi::REFRESH_PATCH_VERSION as PATCH_VERSION;

// --------------------------------------------------------------------------------
// enums

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum BlendFactor {
    ConstantAlpha = ffi::Refresh_BlendFactor_REFRESH_BLENDFACTOR_CONSTANT_ALPHA,
    ConstantColor = ffi::Refresh_BlendFactor_REFRESH_BLENDFACTOR_CONSTANT_COLOR,
    DstAlpha = ffi::Refresh_BlendFactor_REFRESH_BLENDFACTOR_DST_ALPHA,
    DstColor = ffi::Refresh_BlendFactor_REFRESH_BLENDFACTOR_DST_COLOR,
    One = ffi::Refresh_BlendFactor_REFRESH_BLENDFACTOR_ONE,
    OneMinusConstantAlpha = ffi::Refresh_BlendFactor_REFRESH_BLENDFACTOR_ONE_MINUS_CONSTANT_ALPHA,
    OneMinusConstantColor = ffi::Refresh_BlendFactor_REFRESH_BLENDFACTOR_ONE_MINUS_CONSTANT_COLOR,
    OneMinusDstAlpha = ffi::Refresh_BlendFactor_REFRESH_BLENDFACTOR_ONE_MINUS_DST_ALPHA,
    OneMinusDstColor = ffi::Refresh_BlendFactor_REFRESH_BLENDFACTOR_ONE_MINUS_DST_COLOR,
    OneMinusSrc1Alpha = ffi::Refresh_BlendFactor_REFRESH_BLENDFACTOR_ONE_MINUS_SRC1_ALPHA,
    OneMinusSrc1Color = ffi::Refresh_BlendFactor_REFRESH_BLENDFACTOR_ONE_MINUS_SRC1_COLOR,
    OneMinusSrcAlpha = ffi::Refresh_BlendFactor_REFRESH_BLENDFACTOR_ONE_MINUS_SRC_ALPHA,
    OneMinusSrcColor = ffi::Refresh_BlendFactor_REFRESH_BLENDFACTOR_ONE_MINUS_SRC_COLOR,
    Src1Alpha = ffi::Refresh_BlendFactor_REFRESH_BLENDFACTOR_SRC1_ALPHA,
    Src1Color = ffi::Refresh_BlendFactor_REFRESH_BLENDFACTOR_SRC1_COLOR,
    SrcAlpha = ffi::Refresh_BlendFactor_REFRESH_BLENDFACTOR_SRC_ALPHA,
    SrcAlphaSaturate = ffi::Refresh_BlendFactor_REFRESH_BLENDFACTOR_SRC_ALPHA_SATURATE,
    SrcColor = ffi::Refresh_BlendFactor_REFRESH_BLENDFACTOR_SRC_COLOR,
    Zero = ffi::Refresh_BlendFactor_REFRESH_BLENDFACTOR_ZERO,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum BlendOp {
    ADD = ffi::Refresh_BlendOp_REFRESH_BLENDOP_ADD,
    MAX = ffi::Refresh_BlendOp_REFRESH_BLENDOP_MAX,
    MIN = ffi::Refresh_BlendOp_REFRESH_BLENDOP_MIN,
    ReverseSubtract = ffi::Refresh_BlendOp_REFRESH_BLENDOP_REVERSE_SUBTRACT,
    Substract = ffi::Refresh_BlendOp_REFRESH_BLENDOP_SUBTRACT,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum BorderColor {
    FloatOpaqueBlack = ffi::Refresh_BorderColor_REFRESH_BORDERCOLOR_FLOAT_OPAQUE_BLACK,
    FloatOpaqueWhite = ffi::Refresh_BorderColor_REFRESH_BORDERCOLOR_FLOAT_OPAQUE_WHITE,
    FloatTransparentBlack = ffi::Refresh_BorderColor_REFRESH_BORDERCOLOR_FLOAT_TRANSPARENT_BLACK,
    IntOpaqueBlack = ffi::Refresh_BorderColor_REFRESH_BORDERCOLOR_INT_OPAQUE_BLACK,
    IntOpaqueWhite = ffi::Refresh_BorderColor_REFRESH_BORDERCOLOR_INT_OPAQUE_WHITE,
    IntTransparentBlack = ffi::Refresh_BorderColor_REFRESH_BORDERCOLOR_INT_TRANSPARENT_BLACK,
}

bitflags::bitflags! {
    pub struct BufferUsageFlags: u32 {
        const COMPUTE_BIT = ffi::Refresh_BufferUsageFlagBits_REFRESH_BUFFERUSAGE_COMPUTE_BIT;
        const INDEX_BIT = ffi::Refresh_BufferUsageFlagBits_REFRESH_BUFFERUSAGE_INDEX_BIT;
        const VERTEX_BIT = ffi::Refresh_BufferUsageFlagBits_REFRESH_BUFFERUSAGE_VERTEX_BIT;
    }
}

bitflags::bitflags! {
    pub struct ClearOptions: u32 {
        const COLOR	= ffi::Refresh_ClearOptionsBits_REFRESH_CLEAROPTIONS_COLOR;
        const DEPTH	= ffi::Refresh_ClearOptionsBits_REFRESH_CLEAROPTIONS_DEPTH;
        const STENCIL= ffi::Refresh_ClearOptionsBits_REFRESH_CLEAROPTIONS_STENCIL;
    }
}

bitflags::bitflags! {
    pub struct ColorComponentFlags: u32 {
        const A_BIT	= ffi::Refresh_ColorComponentFlagBits_REFRESH_COLORCOMPONENT_A_BIT;
        const B_BIT	= ffi::Refresh_ColorComponentFlagBits_REFRESH_COLORCOMPONENT_B_BIT;
        const G_BIT	= ffi::Refresh_ColorComponentFlagBits_REFRESH_COLORCOMPONENT_G_BIT;
        const R_BIT = ffi::Refresh_ColorComponentFlagBits_REFRESH_COLORCOMPONENT_R_BIT;
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum CompareOp {
    Always = ffi::Refresh_CompareOp_REFRESH_COMPAREOP_ALWAYS,
    Equal = ffi::Refresh_CompareOp_REFRESH_COMPAREOP_EQUAL,
    Greater = ffi::Refresh_CompareOp_REFRESH_COMPAREOP_GREATER,
    GreaterOrEqual = ffi::Refresh_CompareOp_REFRESH_COMPAREOP_GREATER_OR_EQUAL,
    Less = ffi::Refresh_CompareOp_REFRESH_COMPAREOP_LESS,
    LessOrEqual = ffi::Refresh_CompareOp_REFRESH_COMPAREOP_LESS_OR_EQUAL,
    Never = ffi::Refresh_CompareOp_REFRESH_COMPAREOP_NEVER,
    NotEqual = ffi::Refresh_CompareOp_REFRESH_COMPAREOP_NOT_EQUAL,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum CubeMapFace {
    NegativeX = ffi::Refresh_CubeMapFace_REFRESH_CUBEMAPFACE_NEGATIVEX,
    NegativeY = ffi::Refresh_CubeMapFace_REFRESH_CUBEMAPFACE_NEGATIVEY,
    NegativeZ = ffi::Refresh_CubeMapFace_REFRESH_CUBEMAPFACE_NEGATIVEZ,
    PositiveX = ffi::Refresh_CubeMapFace_REFRESH_CUBEMAPFACE_POSITIVEX,
    PositiveY = ffi::Refresh_CubeMapFace_REFRESH_CUBEMAPFACE_POSITIVEY,
    PositiveZ = ffi::Refresh_CubeMapFace_REFRESH_CUBEMAPFACE_POSITIVEZ,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum CullMode {
    Back = ffi::Refresh_CullMode_REFRESH_CULLMODE_BACK,
    Front = ffi::Refresh_CullMode_REFRESH_CULLMODE_FRONT,
    FrontAndBack = ffi::Refresh_CullMode_REFRESH_CULLMODE_FRONT_AND_BACK,
    None = ffi::Refresh_CullMode_REFRESH_CULLMODE_NONE,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum TextureFormat {
    R8G8B8A8 = ffi::Refresh_TextureFormat_REFRESH_TEXTUREFORMAT_R8G8B8A8,
    R5G6B5 = ffi::Refresh_TextureFormat_REFRESH_TEXTUREFORMAT_R5G6B5,
    A1R5G5B5 = ffi::Refresh_TextureFormat_REFRESH_TEXTUREFORMAT_A1R5G5B5,
    B4G4R4A4 = ffi::Refresh_TextureFormat_REFRESH_TEXTUREFORMAT_B4G4R4A4,
    BC1 = ffi::Refresh_TextureFormat_REFRESH_TEXTUREFORMAT_BC1,
    BC2 = ffi::Refresh_TextureFormat_REFRESH_TEXTUREFORMAT_BC2,
    BC3 = ffi::Refresh_TextureFormat_REFRESH_TEXTUREFORMAT_BC3,
    R8G8Snorm = ffi::Refresh_TextureFormat_REFRESH_TEXTUREFORMAT_R8G8_SNORM,
    R8G8B8A8Snorm = ffi::Refresh_TextureFormat_REFRESH_TEXTUREFORMAT_R8G8B8A8_SNORM,
    A2R10G10B10 = ffi::Refresh_TextureFormat_REFRESH_TEXTUREFORMAT_A2R10G10B10,
    R16G16 = ffi::Refresh_TextureFormat_REFRESH_TEXTUREFORMAT_R16G16,
    R16G16B16A16 = ffi::Refresh_TextureFormat_REFRESH_TEXTUREFORMAT_R16G16B16A16,
    R8 = ffi::Refresh_TextureFormat_REFRESH_TEXTUREFORMAT_R8,
    R32Sfloat = ffi::Refresh_TextureFormat_REFRESH_TEXTUREFORMAT_R32_SFLOAT,
    R32G32Sfloat = ffi::Refresh_TextureFormat_REFRESH_TEXTUREFORMAT_R32G32_SFLOAT,
    R32G32B32A32Sfloat = ffi::Refresh_TextureFormat_REFRESH_TEXTUREFORMAT_R32G32B32A32_SFLOAT,
    R16Sfloat = ffi::Refresh_TextureFormat_REFRESH_TEXTUREFORMAT_R16_SFLOAT,
    R16G16Sfloat = ffi::Refresh_TextureFormat_REFRESH_TEXTUREFORMAT_R16G16_SFLOAT,
    R16G16B16A16Sfloat = ffi::Refresh_TextureFormat_REFRESH_TEXTUREFORMAT_R16G16B16A16_SFLOAT,
    D16Unorm = ffi::Refresh_TextureFormat_REFRESH_TEXTUREFORMAT_D16_UNORM,
    D32Sfloat = ffi::Refresh_TextureFormat_REFRESH_TEXTUREFORMAT_D32_SFLOAT,
    D16UnormS8Uint = ffi::Refresh_TextureFormat_REFRESH_TEXTUREFORMAT_D16_UNORM_S8_UINT,
    D32SfloatS8Uint = ffi::Refresh_TextureFormat_REFRESH_TEXTUREFORMAT_D32_SFLOAT_S8_UINT,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum FillMode {
    Fill = ffi::Refresh_FillMode_REFRESH_FILLMODE_FILL,
    Line = ffi::Refresh_FillMode_REFRESH_FILLMODE_LINE,
    Point = ffi::Refresh_FillMode_REFRESH_FILLMODE_POINT,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum Filter {
    Cubic = ffi::Refresh_Filter_REFRESH_FILTER_CUBIC,
    Linear = ffi::Refresh_Filter_REFRESH_FILTER_LINEAR,
    Nearest = ffi::Refresh_Filter_REFRESH_FILTER_NEAREST,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum FrontFace {
    Clockwise = ffi::Refresh_FrontFace_REFRESH_FRONTFACE_CLOCKWISE,
    CounterClockwise = ffi::Refresh_FrontFace_REFRESH_FRONTFACE_COUNTER_CLOCKWISE,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum IndexElementSize {
    Bit16 = ffi::Refresh_IndexElementSize_REFRESH_INDEXELEMENTSIZE_16BIT,
    Bit32 = ffi::Refresh_IndexElementSize_REFRESH_INDEXELEMENTSIZE_32BIT,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum LoadOp {
    Clear = ffi::Refresh_LoadOp_REFRESH_LOADOP_CLEAR,
    DontCare = ffi::Refresh_LoadOp_REFRESH_LOADOP_DONT_CARE,
    Load = ffi::Refresh_LoadOp_REFRESH_LOADOP_LOAD,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum LogicOp {
    And = ffi::Refresh_LogicOp_REFRESH_LOGICOP_AND,
    AndInverted = ffi::Refresh_LogicOp_REFRESH_LOGICOP_AND_INVERTED,
    AndReverse = ffi::Refresh_LogicOp_REFRESH_LOGICOP_AND_REVERSE,
    Clear = ffi::Refresh_LogicOp_REFRESH_LOGICOP_CLEAR,
    Copy = ffi::Refresh_LogicOp_REFRESH_LOGICOP_COPY,
    CopyInverted = ffi::Refresh_LogicOp_REFRESH_LOGICOP_COPY_INVERTED,
    Equivalent = ffi::Refresh_LogicOp_REFRESH_LOGICOP_EQUIVALENT,
    Invert = ffi::Refresh_LogicOp_REFRESH_LOGICOP_INVERT,
    Nand = ffi::Refresh_LogicOp_REFRESH_LOGICOP_NAND,
    Nor = ffi::Refresh_LogicOp_REFRESH_LOGICOP_NOR,
    NoOp = ffi::Refresh_LogicOp_REFRESH_LOGICOP_NO_OP,
    Or = ffi::Refresh_LogicOp_REFRESH_LOGICOP_OR,
    OrInverted = ffi::Refresh_LogicOp_REFRESH_LOGICOP_OR_INVERTED,
    OrReverse = ffi::Refresh_LogicOp_REFRESH_LOGICOP_OR_REVERSE,
    Set = ffi::Refresh_LogicOp_REFRESH_LOGICOP_SET,
    Xor = ffi::Refresh_LogicOp_REFRESH_LOGICOP_XOR,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum PresentMode {
    Fifo = ffi::Refresh_PresentMode_REFRESH_PRESENTMODE_FIFO,
    FifoRelaxed = ffi::Refresh_PresentMode_REFRESH_PRESENTMODE_FIFO_RELAXED,
    Immediate = ffi::Refresh_PresentMode_REFRESH_PRESENTMODE_IMMEDIATE,
    Mailbox = ffi::Refresh_PresentMode_REFRESH_PRESENTMODE_MAILBOX,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum PrimitiveType {
    Linelist = ffi::Refresh_PrimitiveType_REFRESH_PRIMITIVETYPE_LINELIST,
    Linestrip = ffi::Refresh_PrimitiveType_REFRESH_PRIMITIVETYPE_LINESTRIP,
    Pointlist = ffi::Refresh_PrimitiveType_REFRESH_PRIMITIVETYPE_POINTLIST,
    Trianglelist = ffi::Refresh_PrimitiveType_REFRESH_PRIMITIVETYPE_TRIANGLELIST,
    Trianglestrip = ffi::Refresh_PrimitiveType_REFRESH_PRIMITIVETYPE_TRIANGLESTRIP,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum SampleCount {
    N1 = ffi::Refresh_SampleCount_REFRESH_SAMPLECOUNT_1,
    N2 = ffi::Refresh_SampleCount_REFRESH_SAMPLECOUNT_2,
    N4 = ffi::Refresh_SampleCount_REFRESH_SAMPLECOUNT_4,
    N8 = ffi::Refresh_SampleCount_REFRESH_SAMPLECOUNT_8,
    N16 = ffi::Refresh_SampleCount_REFRESH_SAMPLECOUNT_16,
    N32 = ffi::Refresh_SampleCount_REFRESH_SAMPLECOUNT_32,
    N64 = ffi::Refresh_SampleCount_REFRESH_SAMPLECOUNT_64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum SamplerAddressMode {
    ClampToBorder = ffi::Refresh_SamplerAddressMode_REFRESH_SAMPLERADDRESSMODE_CLAMP_TO_BORDER,
    ClampToEdge = ffi::Refresh_SamplerAddressMode_REFRESH_SAMPLERADDRESSMODE_CLAMP_TO_EDGE,
    MirroredRepeat = ffi::Refresh_SamplerAddressMode_REFRESH_SAMPLERADDRESSMODE_MIRRORED_REPEAT,
    Repeat = ffi::Refresh_SamplerAddressMode_REFRESH_SAMPLERADDRESSMODE_REPEAT,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum SamplerMipmapMode {
    Linear = ffi::Refresh_SamplerMipmapMode_REFRESH_SAMPLERMIPMAPMODE_LINEAR,
    Nearest = ffi::Refresh_SamplerMipmapMode_REFRESH_SAMPLERMIPMAPMODE_NEAREST,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum ShaderStageType {
    Fragment = ffi::Refresh_ShaderStageType_REFRESH_SHADERSTAGE_FRAGMENT,
    Vertex = ffi::Refresh_ShaderStageType_REFRESH_SHADERSTAGE_VERTEX,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum StencilOp {
    DecrementAndClamp = ffi::Refresh_StencilOp_REFRESH_STENCILOP_DECREMENT_AND_CLAMP,
    DecrementAndWrap = ffi::Refresh_StencilOp_REFRESH_STENCILOP_DECREMENT_AND_WRAP,
    IncrementAndClamp = ffi::Refresh_StencilOp_REFRESH_STENCILOP_INCREMENT_AND_CLAMP,
    IncrementAndWrap = ffi::Refresh_StencilOp_REFRESH_STENCILOP_INCREMENT_AND_WRAP,
    Invert = ffi::Refresh_StencilOp_REFRESH_STENCILOP_INVERT,
    Keep = ffi::Refresh_StencilOp_REFRESH_STENCILOP_KEEP,
    Replace = ffi::Refresh_StencilOp_REFRESH_STENCILOP_REPLACE,
    Zero = ffi::Refresh_StencilOp_REFRESH_STENCILOP_ZERO,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum StoreOp {
    DontCare = ffi::Refresh_StoreOp_REFRESH_STOREOP_DONT_CARE,
    Store = ffi::Refresh_StoreOp_REFRESH_STOREOP_STORE,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum SysRenderType {
    Vulkan = ffi::Refresh_SysRendererType_REFRESH_RENDERER_TYPE_VULKAN,
}

bitflags::bitflags! {
    pub struct TextureUsageFlags: u32 {
        const COLOR_TARGET_BIT = ffi::Refresh_TextureUsageFlagBits_REFRESH_TEXTUREUSAGE_COLOR_TARGET_BIT;
        const SAMPLER_BIT = ffi::Refresh_TextureUsageFlagBits_REFRESH_TEXTUREUSAGE_SAMPLER_BIT;
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum VertexElementFormat {
    Byte4 = ffi::Refresh_VertexElementFormat_REFRESH_VERTEXELEMENTFORMAT_BYTE4,
    Color = ffi::Refresh_VertexElementFormat_REFRESH_VERTEXELEMENTFORMAT_COLOR,
    HalfVector2 = ffi::Refresh_VertexElementFormat_REFRESH_VERTEXELEMENTFORMAT_HALFVECTOR2,
    HalfVector4 = ffi::Refresh_VertexElementFormat_REFRESH_VERTEXELEMENTFORMAT_HALFVECTOR4,
    NormalizedShort2 =
        ffi::Refresh_VertexElementFormat_REFRESH_VERTEXELEMENTFORMAT_NORMALIZEDSHORT2,
    NormalizedShort4 =
        ffi::Refresh_VertexElementFormat_REFRESH_VERTEXELEMENTFORMAT_NORMALIZEDSHORT4,
    Short2 = ffi::Refresh_VertexElementFormat_REFRESH_VERTEXELEMENTFORMAT_SHORT2,
    Short4 = ffi::Refresh_VertexElementFormat_REFRESH_VERTEXELEMENTFORMAT_SHORT4,
    Single = ffi::Refresh_VertexElementFormat_REFRESH_VERTEXELEMENTFORMAT_SINGLE,
    Vector2 = ffi::Refresh_VertexElementFormat_REFRESH_VERTEXELEMENTFORMAT_VECTOR2,
    Vector3 = ffi::Refresh_VertexElementFormat_REFRESH_VERTEXELEMENTFORMAT_VECTOR3,
    Vector4 = ffi::Refresh_VertexElementFormat_REFRESH_VERTEXELEMENTFORMAT_VECTOR4,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum VertexInputRate {
    Instance = ffi::Refresh_VertexInputRate_REFRESH_VERTEXINPUTRATE_INSTANCE,
    Vertex = ffi::Refresh_VertexInputRate_REFRESH_VERTEXINPUTRATE_VERTEX,
}

// --------------------------------------------------------------------------------
// functions

pub fn linked_version() -> u32 {
    unsafe { ffi::Refresh_LinkedVersion() }
}

/// `Option<unsafe extern "C" fn(msg: *const c_char)>`
pub type LogFunc = ffi::Refresh_LogFunc;

pub fn hook_log_functions(info: LogFunc, warn: LogFunc, error: LogFunc) {
    unsafe {
        ffi::Refresh_HookLogFunctions(info, warn, error);
    }
}

#[derive(Debug, Clone)]
pub struct Device {
    inner: Rc<DeviceDrop>,
}

impl Device {
    pub fn from_drop(d: DeviceDrop) -> Self {
        Self { inner: Rc::new(d) }
    }
}

impl Deref for Device {
    type Target = DeviceDrop;

    fn deref(&self) -> &Self::Target {
        self.inner.as_ref()
    }
}

#[derive(Debug)]
pub struct DeviceDrop {
    raw: *mut ffi::Refresh_Device,
}

impl Drop for DeviceDrop {
    /// Destroys a rendering context previously returned by `Refresh_CreateDevice`
    fn drop(&mut self) {
        unsafe {
            ffi::Refresh_DestroyDevice(self.raw);
        }
    }
}

impl DeviceDrop {
    /// Create a rendering context for use on the calling thread.
    ///
    /// * `presentationParameters`:
    /// 		If the windowHandle is NULL, Refresh will run in headless mode.
    /// *` debugMode`: Enable debug mode properties.
    pub fn new(params: &PresentationParameters, is_debug: bool) -> Self {
        Self {
            raw: unsafe { ffi::Refresh_CreateDevice(params as *const _ as *mut _, is_debug as u8) },
        }
    }

    /// Create a rendering context by taking an externally-initialized `VkDevice`.
    /// Only valid with Vulkan backend.
    /// Useful for piggybacking on a separate graphics library like FNA3D.
    ///
    /// * `sysRenderer`: Externally-initialized device info.
    /// * `debugMode`: Enable debug mode properties.
    ///
    pub fn from_external(sys_render: *mut SysRenderer, is_debug: bool) -> Self {
        Self {
            raw: unsafe { ffi::Refresh_CreateDeviceUsingExternal(sys_render, is_debug as u8) },
        }
    }
}

/// Drawing
impl DeviceDrop {
    /// Clears the targets of the currently bound framebuffer.
    /// If fewer colors are passed than the number of color targets in the
    /// framebuffer, this function will clear the first n color targets.
    ///
    /// NOTE:
    /// 		It is generally recommended to clear in BeginRenderPass
    /// 		rather than by calling this function unless necessary.
    ///
    /// * `clearRect`:	Area to clear.
    /// * `options`:		Bitflags to specify color/depth/stencil buffers for clearing.
    /// * `colors`:		An array of color values for the cleared color buffers.
    /// * `colorCount`:	The number of colors in the above array.
    /// * `depth`:		The new value of the cleared depth buffer.
    /// * `stencil`:		The new value of the cleared stencil buffer.
    /// * `depth_stencil`: Depth and stencil values for the cleared depth stencil buffer.
    pub fn clear(
        &self,
        cbuf: *mut CommandBuffer,
        rect: &Rect,
        opts: ClearOptions,
        colors: &Vec4,
        n_colors: u32,
        depth_stencil: DepthStencilValue,
    ) {
        unsafe {
            ffi::Refresh_Clear(
                self.raw,
                cbuf,
                rect as *const _ as *mut _,
                opts.bits(),
                colors as *const _ as *mut _,
                n_colors,
                depth_stencil,
            );
        }
    }

    /// Draws data from vertex/index buffers with instancing enabled.
    ///
    /// * `baseVertex`:			The starting offset to read from the vertex buffer.
    /// * `startIndex`:			The starting offset to read from the index buffer.
    /// * `primitiveCount`:		The number of primitives to draw.
    /// * `instanceCount`:		The number of instances that will be drawn.
    /// * `vertexParamOffset`:	The offset of the vertex shader param data.
    /// * `fragmentParamOffset`:	The offset of the fragment shader param data.
    pub fn draw_indexed_primitives(
        &self,
        cbuf: *mut CommandBuffer,
        base_vertex: u32,
        start_index: u32,
        primitive_count: u32,
        instance_count: u32,
        vertex_param_offset: u32,
        fragment_param_offset: u32,
    ) {
        unsafe {
            ffi::Refresh_DrawInstancedPrimitives(
                self.raw,
                cbuf,
                base_vertex,
                start_index,
                primitive_count,
                instance_count,
                vertex_param_offset,
                fragment_param_offset,
            );
        }
    }

    /// Draws data from vertex buffers.
    ///
    /// * `vertex_start`:				The starting offset to read from the vertex buffer.
    /// * `primitive_count`:			The number of primitives to draw.
    /// * `vertex_param_offset`:		The offset of the vertex shader param data.
    /// * `fragment_param_offset`:		The offset of the fragment shader param data.
    pub fn draw_primitive(
        &self,
        cbuf: *mut CommandBuffer,
        vertex_start: u32,
        primitive_count: u32,
        vertex_param_offset: u32,
        fragment_param_offset: u32,
    ) {
        unsafe {
            ffi::Refresh_DrawPrimitives(
                self.raw,
                cbuf,
                vertex_start,
                primitive_count,
                vertex_param_offset,
                fragment_param_offset,
            );
        }
    }

    /// Dispatches work compute items.
    ///
    /// * `group_count_x`:			Number of local workgroups to dispatch in the X dimension.
    /// * `group_count_y`:			Number of local workgroups to dispatch in the Y dimension.
    /// * `group_count_z`:			Number of local workgroups to dispatch in the Z dimension.
    /// * `compute_param_offset`:	The offset of the compute shader param data.
    pub fn dispatch_compute(&self, cbuf: *mut CommandBuffer, groups: [u32; 3], offset: u32) {
        unsafe {
            ffi::Refresh_DispatchCompute(self.raw, cbuf, groups[0], groups[1], groups[2], offset);
        }
    }
}

pub trait Resource {
    type CreateInfo;

    fn create(device: &DeviceDrop, info: &Self::CreateInfo) -> *mut Self;

    /// Sends a <resource> to be destroyed by the renderer. Note that we call it
    /// "QueueDestroy" because it may not be immediately destroyed by the renderer if
    /// this is not called from the main thread (for example, if a garbage collector
    /// deletes the resource instead of the programmer).
    fn queue_destroy(me: *mut Self, device: &DeviceDrop);
}

impl Resource for RenderPass {
    type CreateInfo = RenderPassCreateInfo;

    fn create(device: &DeviceDrop, info: &Self::CreateInfo) -> *mut Self {
        unsafe { ffi::Refresh_CreateRenderPass(device.raw, info as *const _ as *mut _) }
    }

    fn queue_destroy(me: *mut Self, device: &DeviceDrop) {
        unsafe {
            ffi::Refresh_QueueDestroyRenderPass(device.raw, me);
        }
    }
}

impl Resource for ComputePipeline {
    type CreateInfo = ComputePipelineCreateInfo;

    fn create(device: &DeviceDrop, info: &Self::CreateInfo) -> *mut Self {
        unsafe { ffi::Refresh_CreateComputePipeline(device.raw, info as *const _ as *mut _) }
    }

    fn queue_destroy(me: *mut Self, device: &DeviceDrop) {
        unsafe {
            ffi::Refresh_QueueDestroyComputePipeline(device.raw, me);
        }
    }
}

impl Resource for GraphicsPipeline {
    type CreateInfo = GraphicsPipelineCreateInfo;

    fn create(device: &DeviceDrop, info: &Self::CreateInfo) -> *mut Self {
        unsafe { ffi::Refresh_CreateGraphicsPipeline(device.raw, info as *const _ as *mut _) }
    }

    fn queue_destroy(me: *mut Self, device: &DeviceDrop) {
        unsafe {
            ffi::Refresh_QueueDestroyGraphicsPipeline(device.raw, me);
        }
    }
}

impl Resource for Sampler {
    type CreateInfo = SamplerStateCreateInfo;

    fn create(device: &DeviceDrop, info: &Self::CreateInfo) -> *mut Self {
        unsafe { ffi::Refresh_CreateSampler(device.raw, info as *const _ as *mut _) }
    }

    fn queue_destroy(me: *mut Self, device: &DeviceDrop) {
        unsafe {
            ffi::Refresh_QueueDestroySampler(device.raw, me);
        }
    }
}

impl Resource for Framebuffer {
    type CreateInfo = FramebufferCreateInfo;

    fn create(device: &DeviceDrop, info: &Self::CreateInfo) -> *mut Self {
        unsafe { ffi::Refresh_CreateFramebuffer(device.raw, info as *const _ as *mut _) }
    }

    fn queue_destroy(me: *mut Self, device: &DeviceDrop) {
        unsafe {
            ffi::Refresh_QueueDestroyFramebuffer(device.raw, me);
        }
    }
}

impl Resource for ShaderModule {
    type CreateInfo = ShaderModuleCreateInfo;

    fn create(device: &DeviceDrop, info: &Self::CreateInfo) -> *mut Self {
        unsafe { ffi::Refresh_CreateShaderModule(device.raw, info as *const _ as *mut _) }
    }

    fn queue_destroy(me: *mut Self, device: &DeviceDrop) {
        unsafe {
            ffi::Refresh_QueueDestroyShaderModule(device.raw, me);
        }
    }
}

impl Resource for Texture {
    type CreateInfo = TextureCreateInfo;

    /// Note that the contents of * the texture are undefined until SetData is called.
    fn create(device: &DeviceDrop, info: &Self::CreateInfo) -> *mut Self {
        unsafe { ffi::Refresh_CreateTexture(device.raw, info as *const _ as *mut _) }
    }

    fn queue_destroy(me: *mut Self, device: &DeviceDrop) {
        unsafe {
            ffi::Refresh_QueueDestroyTexture(device.raw, me);
        }
    }
}

/// (Non-Refresh) Additional type to Refresh for [`Resource`] crate
pub struct RenderTargetCreateInfo {
    pub texture_slice: *mut TextureSlice,
    pub multi_sample_count: SampleCount,
}

impl Resource for RenderTarget {
    type CreateInfo = RenderTargetCreateInfo;

    /// Note that the contents of * the texture are undefined until SetData is called.
    fn create(device: &DeviceDrop, info: &Self::CreateInfo) -> *mut Self {
        unsafe {
            ffi::Refresh_CreateRenderTarget(
                device.raw,
                info.texture_slice,
                info.multi_sample_count as u32,
            )
        }
    }

    fn queue_destroy(me: *mut Self, device: &DeviceDrop) {
        unsafe {
            ffi::Refresh_QueueDestroyRenderTarget(device.raw, me);
        }
    }
}

/// (Non-Refresh) Additional type to Refresh for [`Resource`] crate
pub struct BufferCreateInfo {
    pub usage_flags: BufferUsageFlags,
    pub size_in_bytes: u32,
}

impl Resource for Buffer {
    type CreateInfo = BufferCreateInfo;

    /// Note that the contents of * the texture are undefined until SetData is called.
    fn create(device: &DeviceDrop, info: &Self::CreateInfo) -> *mut Self {
        unsafe {
            ffi::Refresh_CreateBuffer(device.raw, info.usage_flags.bits(), info.size_in_bytes)
        }
    }

    fn queue_destroy(me: *mut Self, device: &DeviceDrop) {
        unsafe {
            ffi::Refresh_QueueDestroyBuffer(device.raw, me);
        }
    }
}

/// Setters
impl DeviceDrop {
    /// Uploads image data to a texture object.
    pub fn set_texture_data(&self, slice: *mut TextureSlice, data: &[u8]) {
        unsafe {
            ffi::Refresh_SetTextureData(
                self.raw,
                slice,
                data.as_ptr() as *const _ as *mut _,
                data.len() as u32,
            );
        }
    }

    /// Uploads YUV image data to three R8 texture objects.
    ///
    /// * `y`:		The texture storing the Y data.
    /// * `u`:		The texture storing the U (Cb) data.
    /// * `v`:		The texture storing the V (Cr) data.
    /// * `y_width`:	The width of the Y plane.
    /// * `y_height`:	The height of the Y plane.
    /// * `uv_width`:	The width of the U/V planes.
    /// * `uv_height`:	The height of the U/V planes.
    /// * `data`:	A pointer to the raw YUV image data.
    /// * `data_length`:	The size of the image data in bytes.
    ///
    pub fn set_texture_data_yuv(
        &self,
        yuv: [*mut Texture; 3],
        y_size: [u32; 2],
        uv_size: [u32; 2],
        data: &[u8],
    ) {
        unsafe {
            ffi::Refresh_SetTextureDataYUV(
                self.raw,
                yuv[0],
                yuv[1],
                yuv[2],
                y_size[0],
                y_size[1],
                uv_size[0],
                uv_size[1],
                data.as_ptr() as *const _ as *mut _,
                data.len() as u32,
            );
        }
    }

    /// Performs an asynchronous texture-to-texture copy.
    ///
    /// * `source_texture_slice`:		The texture slice from which to copy.
    /// * `destination_texture_slice`:	The texture slice to copy to.
    /// * `filter`:					The filter that will be used if the copy requires scaling.
    pub fn copy_texture_to_texture(
        &self,
        cbuf: *mut CommandBuffer,
        src: *mut TextureSlice,
        dst: *mut TextureSlice,
        filter: Filter,
    ) {
        unsafe {
            ffi::Refresh_CopyTextureToTexture(self.raw, cbuf, src, dst, filter as u32);
        }
    }

    /// Asynchronously copies image data from a texture slice into a buffer.
    ///
    /// NOTE:
    /// 	The buffer will not contain correct data until the command buffer
    /// 	is submitted and completed.
    pub fn copy_texture_to_buffer(
        &self,
        cbuf: *mut CommandBuffer,
        src: *mut TextureSlice,
        buf: *mut Buffer,
    ) {
        unsafe {
            ffi::Refresh_CopyTextureToBuffer(self.raw, cbuf, src, buf);
        }
    }

    /// Sets a region of the buffer with client data.
    ///
    /// NOTE:
    /// 		Calling this function on a buffer after the buffer
    /// 		has been bound without calling Submit first is an error.
    ///
    /// * `buffer`:			The vertex buffer to be updated.
    /// * `offset_in_bytes`:	The starting offset of the buffer to write into.
    pub fn set_buffer_data(&self, buf: *mut Buffer, offset_in_bytes: u32, data: &[u8]) {
        unsafe {
            ffi::Refresh_SetBufferData(
                self.raw,
                buf,
                offset_in_bytes,
                data.as_ptr() as *const _ as *mut _,
                data.len() as u32,
            );
        }
    }

    /// Pushes vertex shader params to the device.
    /// Returns a starting offset value to be used with draw calls.
    ///
    /// NOTE:
    /// 		A pipeline must be bound.
    /// 		Will use the block size of the currently bound vertex shader.
    pub fn push_vertex_shader_params(&self, pip: *mut GraphicsPipeline, data: &[u8]) {
        unsafe {
            ffi::Refresh_PushVertexShaderUniforms(
                self.raw,
                pip,
                data.as_ptr() as *const _ as *mut _,
                data.len() as u32,
            );
        }
    }

    /// Pushes fragment shader params to the device.
    /// Returns a starting offset value to be used with draw calls.
    ///
    /// NOTE:
    /// 		A graphics pipeline must be bound.
    /// 		Will use the block size of the currently bound fragment shader.
    pub fn push_fragment_shader_params(&self, pip: *mut GraphicsPipeline, data: &[u8]) {
        unsafe {
            ffi::Refresh_PushFragmentShaderUniforms(
                self.raw,
                pip,
                data.as_ptr() as *const _ as *mut _,
                data.len() as u32,
            );
        }
    }

    /// Pushes compute shader params to the device.
    /// Returns a starting offset value to be used with draw calls.
    ///
    /// NOTE:
    /// 	A compute pipeline must be bound.
    /// 	Will use the block size of the currently bound compute shader.
    pub fn push_compute_shader_params(&self, pip: *mut ComputePipeline, data: &[u8]) {
        unsafe {
            ffi::Refresh_PushComputeShaderUniforms(
                self.raw,
                pip,
                data.as_ptr() as *const _ as *mut _,
                data.len() as u32,
            );
        }
    }
}

/// Getters
impl DeviceDrop {
    /// Synchronously copies data from a buffer to a pointer.
    /// You probably want to wait for a sync point to call this.
    pub fn get_buffer_data(&self, buf: *mut Buffer, dst: &mut [u8]) {
        unsafe {
            ffi::Refresh_GetBufferData(self.raw, buf, dst.as_mut_ptr() as *mut _, dst.len() as u32);
        }
    }
}

/// Graphics State
impl DeviceDrop {
    /// Begins a render pass.
    ///
    /// * `render_pass`: The renderpass to begin.
    /// * `framebuffer`: The framebuffer to bind for the render pass.
    /// * `render_area`:
    /// 		The area affected by the render pass.
    /// 		All load, store and resolve operations are restricted
    /// 		to the given rectangle.
    /// * `clear_values`:
    /// 		A pointer to an array of Refresh_Color structures
    /// 		that contains clear values for each color target in the
    /// 		framebuffer. May be NULL.
    /// * `clear_count`: The amount of color structs in the above array.
    /// * `depth_stencil_clear_value`: The depth/stencil clear value. May be NULL.
    pub fn begin_render_pass(
        &self,
        cbuf: *mut CommandBuffer,
        rpass: *mut RenderPass,
        fbuf: *mut Framebuffer,
        area: &Rect,
        clear: Option<&[Vec4]>,
        depth_stencil_clear_value: &DepthStencilValue,
    ) {
        let (ptr, len) = match clear {
            None => (std::ptr::null_mut(), 0),
            Some(xs) => (xs.as_ptr() as *const _ as *mut _, xs.len() as u32),
        };
        unsafe {
            ffi::Refresh_BeginRenderPass(
                self.raw,
                cbuf,
                rpass,
                fbuf,
                area as *const _ as *mut _,
                ptr,
                len,
                depth_stencil_clear_value as *const _ as *mut _,
            );
        }
    }

    /// Ends the current render pass
    pub fn end_render_pass(&self, cbuf: *mut CommandBuffer) {
        unsafe {
            ffi::Refresh_EndRenderPass(self.raw, cbuf);
        }
    }

    //// Binds a graphics pipeline to the graphics bind point
    pub fn bind_graphics_pipeline(&self, cbuf: *mut CommandBuffer, gpip: *mut GraphicsPipeline) {
        unsafe {
            ffi::Refresh_BindGraphicsPipeline(self.raw, cbuf, gpip);
        }
    }

    /// Binds vertex buffers for use with subsequent draw calls
    pub fn bind_vertex_buffers(
        &self,
        cbuf: *mut CommandBuffer,
        bufs: &[*mut Buffer],
        offsets: &[usize],
    ) {
        unsafe {
            ffi::Refresh_BindVertexBuffers(
                self.raw,
                cbuf,
                // firstBinding
                0,
                // bindingCount
                bufs.len() as u32,
                bufs.as_ptr() as *const _ as *mut _,
                offsets.as_ptr() as *const _ as *mut _,
            );
        }
    }

    /// Binds an index buffer for use with subsequent draw calls
    pub fn bind_index_buffer(
        &self,
        cbuf: *mut CommandBuffer,
        buf: *mut Buffer,
        offset: u64,
        size: IndexElementSize,
    ) {
        unsafe {
            ffi::Refresh_BindIndexBuffer(self.raw, cbuf, buf, offset, size as u32);
        }
    }
    /// Sets textures/samplers for use with the currently bound vertex shader.
    ///
    /// NOTE:
    /// 		The length of the passed arrays must be equal to the number
    /// 		of sampler bindings specified by the pipeline.
    ///
    /// * `textures`:	A pointer to an array of textures.
    /// * `samplers`:	A pointer to an array of samplers.
    pub fn bind_vertex_samplers(
        &self,
        cbuf: *mut CommandBuffer,
        texs: &[*mut Texture],
        samplers: &[*mut Sampler],
    ) {
        unsafe {
            ffi::Refresh_BindVertexSamplers(
                self.raw,
                cbuf,
                texs.as_ptr() as *mut _,
                samplers.as_ptr() as *mut _,
            );
        }
    }

    /// Sets textures/samplers for use with the currently bound fragment shader.
    ///
    /// NOTE:
    ///		The length of the passed arrays must be equal to the number
    /// 		of sampler bindings specified by the pipeline.
    ///
    /// * `textures`: 	A pointer to an array of textures.
    /// * `samplers`:	A pointer to an array of samplers.
    pub fn bind_fragment_samplers(
        &self,
        cbuf: *mut CommandBuffer,
        texs: &[*mut Texture],
        samplers: &[*mut Sampler],
    ) {
        unsafe {
            ffi::Refresh_BindFragmentSamplers(
                self.raw,
                cbuf,
                texs.as_ptr() as *mut _,
                samplers.as_ptr() as *mut _,
            );
        }
    }

    /// Binds a compute pipeline to the compute bind point
    pub fn bind_compute_pipeline(&self, cbuf: *mut CommandBuffer, cpip: *mut ComputePipeline) {
        unsafe {
            ffi::Refresh_BindComputePipeline(self.raw, cbuf, cpip);
        }
    }

    /// Binds buffers for use with the currently bound compute pipeline.
    ///
    /// * `p_buffers`: An array of buffers to bind.
    /// 	Length must be equal to the number of buffers
    /// 	specified by the compute pipeline.
    pub fn bind_compute_buffers(&self, cbuf: *mut CommandBuffer, bufs: &[*mut Buffer]) {
        unsafe {
            ffi::Refresh_BindComputeBuffers(self.raw, cbuf, bufs.as_ptr() as *mut _);
        }
    }

    /// Binds textures for use with the currently bound compute pipeline.
    ///
    /// * `p_textures`: An array of textures to bind.
    /// 	Length must be equal to the number of buffers
    /// 	specified by the compute pipeline.
    pub fn bind_compute_textures(&self, cbuf: *mut CommandBuffer, texs: &[*mut Texture]) {
        unsafe {
            ffi::Refresh_BindComputeTextures(self.raw, cbuf, texs.as_ptr() as *mut _);
        }
    }
}

/// Submission/Presentation
impl DeviceDrop {
    /// Returns an allocated Refresh_CommandBuffer* object.
    /// This command buffer is managed by the implementation and
    /// should NOT be freed by the user.
    ///
    /// NOTE:
    /// 	A command buffer may only be used on the thread that
    /// 	it was acquired on. Using it on any other thread is an error.
    ///
    /// * `fixed`:
    /// 	If a command buffer is designated as fixed, it can be
    /// 	acquired once, have commands recorded into it, and
    /// 	be re-submitted indefinitely.
    pub fn acquire_command_buffer(&self, is_fixed: bool) -> *mut CommandBuffer {
        unsafe { ffi::Refresh_AcquireCommandBuffer(self.raw, is_fixed as u8) }
    }

    /// Queues an image to be presented to the screen.
    /// The image will be presented upon the next Refresh_Submit call.
    ///
    /// NOTE:
    ///		It is an error to call this function in headless mode.
    ///
    /// * `texture_slice`:			The texture slice to present.
    /// * `destination_rectangle`:	The region of the window to update. Can be NULL.
    /// * `filter`:					The filter to use if scaling is required.
    pub fn queue_present(
        &self,
        cbuf: *mut CommandBuffer,
        tex_slice: &TextureSlice,
        rect: &Rect,
        filter: Filter,
    ) {
        unsafe {
            ffi::Refresh_QueuePresent(
                self.raw,
                cbuf,
                tex_slice as *const _ as *mut _,
                rect as *const _ as *mut _,
                filter as u32,
            );
        }
    }

    ///Submits all of the enqueued commands
    pub fn submit(&self, cbufs: &[*mut CommandBuffer]) {
        unsafe {
            ffi::Refresh_Submit(self.raw, cbufs.len() as u32, cbufs.as_ptr() as *mut _);
        }
    }

    /// Waits for the previous submission to complete
    pub fn wait(&self) {
        unsafe {
            ffi::Refresh_Wait(self.raw);
        }
    }

    /// Export handles to be consumed by another API
    pub fn get_texture_handles(&self, tex: *mut Texture, handles: &mut TextureHandles) {
        unsafe {
            ffi::Refresh_GetTextureHandles(self.raw, tex, handles as *mut _);
        }
    }
}
