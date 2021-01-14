/*!

Wrapper of Refresh

*/

pub use refresh_ffi as ffi;

use std::{ops::Deref, rc::Rc};

// --------------------------------------------------------------------------------
// structs

pub type Buffer = ffi::Refresh_Buffer;
pub type Color = ffi::Refresh_Color;
pub type ColorBlendState = ffi::Refresh_ColorBlendState;
pub type ColorTarget = ffi::Refresh_ColorTarget;
pub type ColorTargetBlendState = ffi::Refresh_ColorTargetBlendState;
pub type ColorTargetDescription = ffi::Refresh_ColorTargetDescription;
pub type CommandBuffer = ffi::Refresh_CommandBuffer;
pub type ComputePipeline = ffi::Refresh_ComputePipeline;
pub type ComputePipelineCreateInfo = ffi::Refresh_ComputePipelineCreateInfo;
pub type ComputePipelineLayoutCreateInfo = ffi::Refresh_ComputePipelineLayoutCreateInfo;
pub type DepthStencilState = ffi::Refresh_DepthStencilState;
pub type DepthStencilTarget = ffi::Refresh_DepthStencilTarget;
pub type DepthStencilTargetDescription = ffi::Refresh_DepthStencilTargetDescription;
pub type DepthStencilValue = ffi::Refresh_DepthStencilValue;
// pub type Device = ffi::Refresh_Device;
pub type Framebuffer = ffi::Refresh_Framebuffer;
pub type FramebufferCreateInfo = ffi::Refresh_FramebufferCreateInfo;
pub type GraphicsPipeline = ffi::Refresh_GraphicsPipeline;
pub type GraphicsPipelineCreateInfo = ffi::Refresh_GraphicsPipelineCreateInfo;
pub type GraphicsPipelineLayoutCreateInfo = ffi::Refresh_GraphicsPipelineLayoutCreateInfo;
pub type MultisampleState = ffi::Refresh_MultisampleState;
pub type PresentationParameters = ffi::Refresh_PresentationParameters;
pub type RasterizerState = ffi::Refresh_RasterizerState;
pub type Rect = ffi::Refresh_Rect;
pub type RenderPass = ffi::Refresh_RenderPass;
pub type RenderPassCreateInfo = ffi::Refresh_RenderPassCreateInfo;
pub type Sampler = ffi::Refresh_Sampler;
pub type SamplerStateCreateInfo = ffi::Refresh_SamplerStateCreateInfo;
pub type ShaderModule = ffi::Refresh_ShaderModule;
pub type ShaderModuleCreateInfo = ffi::Refresh_ShaderModuleCreateInfo;
pub type ShaderStageState = ffi::Refresh_ShaderStageState;
pub type StencilOpState = ffi::Refresh_StencilOpState;
pub type SysRenderer = ffi::Refresh_SysRenderer;
pub type Texture = ffi::Refresh_Texture;
pub type TextureHandles = ffi::Refresh_TextureHandles;
pub type TextureSlice = ffi::Refresh_TextureSlice;
pub type Vec4 = ffi::Refresh_Vec4;
pub type VertexAttribute = ffi::Refresh_VertexAttribute;
pub type VertexBinding = ffi::Refresh_VertexBinding;
pub type VertexInputState = ffi::Refresh_VertexInputState;
pub type Viewport = ffi::Refresh_Viewport;
pub type ViewportState = ffi::Refresh_ViewportState;

// pub type VkDevice = ffi::VkDevice_T;
// pub type VkInstance = ffi::VkInstance_T;
// pub type VkPhysicalDevice = ffi::VkPhysicalDevice_T;

pub const ABI_VERSION: u32 = ffi::REFRESH_ABI_VERSION;
pub const MAJOR_VERSION: u32 = ffi::REFRESH_MAJOR_VERSION;
pub const MINOR_VERSION: u32 = ffi::REFRESH_MINOR_VERSION;
pub const PATCH_VERSION: u32 = ffi::REFRESH_PATCH_VERSION;

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
pub enum ColorFormat {
    A1R5G5B5 = ffi::Refresh_ColorFormat_REFRESH_COLORFORMAT_A1R5G5B5,
    A2R10G10B10 = ffi::Refresh_ColorFormat_REFRESH_COLORFORMAT_A2R10G10B10,
    B4G4R4A4 = ffi::Refresh_ColorFormat_REFRESH_COLORFORMAT_B4G4R4A4,
    BC1 = ffi::Refresh_ColorFormat_REFRESH_COLORFORMAT_BC1,
    BC2 = ffi::Refresh_ColorFormat_REFRESH_COLORFORMAT_BC2,
    BC3 = ffi::Refresh_ColorFormat_REFRESH_COLORFORMAT_BC3,
    R5G6B5 = ffi::Refresh_ColorFormat_REFRESH_COLORFORMAT_R5G6B5,
    R8 = ffi::Refresh_ColorFormat_REFRESH_COLORFORMAT_R8,
    R8G8B8A8 = ffi::Refresh_ColorFormat_REFRESH_COLORFORMAT_R8G8B8A8,
    R8G8B8A8Snorm = ffi::Refresh_ColorFormat_REFRESH_COLORFORMAT_R8G8B8A8_SNORM,
    R8G8Snorm = ffi::Refresh_ColorFormat_REFRESH_COLORFORMAT_R8G8_SNORM,
    R16G16 = ffi::Refresh_ColorFormat_REFRESH_COLORFORMAT_R16G16,
    R16G16B16A16 = ffi::Refresh_ColorFormat_REFRESH_COLORFORMAT_R16G16B16A16,
    R16G16B16A16fFloat = ffi::Refresh_ColorFormat_REFRESH_COLORFORMAT_R16G16B16A16_SFLOAT,
    R16G16fFloat = ffi::Refresh_ColorFormat_REFRESH_COLORFORMAT_R16G16_SFLOAT,
    R16fFloat = ffi::Refresh_ColorFormat_REFRESH_COLORFORMAT_R16_SFLOAT,
    R32G32B32A32fFloat = ffi::Refresh_ColorFormat_REFRESH_COLORFORMAT_R32G32B32A32_SFLOAT,
    R32G32fFloat = ffi::Refresh_ColorFormat_REFRESH_COLORFORMAT_R32G32_SFLOAT,
    R32fFloat = ffi::Refresh_ColorFormat_REFRESH_COLORFORMAT_R32_SFLOAT,
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
pub enum DepthFormat {
    D16Unorm = ffi::Refresh_DepthFormat_REFRESH_DEPTHFORMAT_D16_UNORM,
    D16UnormS8Uint = ffi::Refresh_DepthFormat_REFRESH_DEPTHFORMAT_D16_UNORM_S8_UINT,
    D32Sfloat = ffi::Refresh_DepthFormat_REFRESH_DEPTHFORMAT_D32_SFLOAT,
    D32SfloatS8Uint = ffi::Refresh_DepthFormat_REFRESH_DEPTHFORMAT_D32_SFLOAT_S8_UINT,
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
    /// * clear_rect:	Area to clear.
    /// * options:		Bitflags to specify color/depth/stencil buffers for clearing.
    /// * colors:		An array of color values for the cleared color buffers.
    /// * color_count:	The number of colors in the above array.
    /// * depth:		The new value of the cleared depth buffer.
    /// * stencil:		The new value of the cleared stencil buffer.
    ///
    pub fn clear(
        &self,
        cbuf: *mut CommandBuffer,
        rect: &Rect,
        opts: ClearOptions,
        colors: &Color,
        n_colors: u32,
        depth: f32,
        stencil: i32,
    ) {
        unsafe {
            ffi::Refresh_Clear(
                self.raw,
                cbuf,
                rect as *const _ as *mut _,
                opts.bits(),
                colors as *const _ as *mut _,
                n_colors,
                depth,
                stencil,
            );
        }
    }

    /// Draws data from vertex/index buffers with instancing enabled.
    ///
    /// base_vertex:			The starting offset to read from the vertex buffer.
    /// start_index:			The starting offset to read from the index buffer.
    /// primitive_count:		The number of primitives to draw.
    /// instance_count:		The number of instances that will be drawn.
    /// vertex_param_offset:	The offset of the vertex shader param data.
    /// fragment_param_offset:	The offset of the fragment shader param data.
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

// /* Creates a 2D texture.
//  *
//  * format:		The pixel format of the texture data.
//  * width:		The width of the texture image.
//  * height: 		The height of the texture image.
//  * levelCount:	The number of mipmap levels to allocate.
//  * usageFlags:	Specifies how the texture will be used.
//  *
//  * Returns an allocated Refresh_Texture* object. Note that the contents of
//  * the texture are undefined until SetData is called.
//  */
// REFRESHAPI Refresh_Texture* Refresh_CreateTexture2D(
// 	Refresh_Device *device,
// 	Refresh_ColorFormat format,
// 	uint32_t width,
// 	uint32_t height,
// 	uint32_t levelCount,
// 	Refresh_TextureUsageFlags usageFlags
// );

// /* Creates a 3D texture.
//  *
//  * format:		The pixel format of the texture data.
//  * width:		The width of the texture image.
//  * height: 		The height of the texture image.
//  * depth: 		The depth of the texture image.
//  * levelCount: 	The number of mipmap levels to allocate.
//  * usageFlags:	Specifies how the texture will be used.
//  *
//  * Returns an allocated Refresh_Texture* object. Note that the contents of
//  * the texture are undefined until SetData is called.
//  */
// REFRESHAPI Refresh_Texture* Refresh_CreateTexture3D(
// 	Refresh_Device *device,
// 	Refresh_ColorFormat format,
// 	uint32_t width,
// 	uint32_t height,
// 	uint32_t depth,
// 	uint32_t levelCount,
// 	Refresh_TextureUsageFlags usageFlags
// );

// /* Creates a texture cube.
//  *
//  * format:		The pixel format of the texture data.
//  * size: 		The length of the cube side.
//  * levelCount: 	The number of mipmap levels to allocate.
//  * usageFlags:	Specifies how the texture will be used.
//  *
//  * Returns an allocated Refresh_Texture* object. Note that the contents of
//  * the texture are undefined until SetData is called.
//  */
// REFRESHAPI Refresh_Texture* Refresh_CreateTextureCube(
// 	Refresh_Device *device,
// 	Refresh_ColorFormat format,
// 	uint32_t size,
// 	uint32_t levelCount,
// 	Refresh_TextureUsageFlags usageFlags
// );

// /* Creates a color target.
//  *
//  * multisampleCount:	The MSAA value for the color target.
//  * textureSlice: 		The texture slice that the color target will resolve to.
//  */
// REFRESHAPI Refresh_ColorTarget* Refresh_CreateColorTarget(
// 	Refresh_Device *device,
// 	Refresh_SampleCount multisampleCount,
// 	Refresh_TextureSlice *textureSlice
// );

// /* Creates a depth/stencil target.
//  *
//  * width:	The width of the depth/stencil target.
//  * height: 	The height of the depth/stencil target.
//  * format:	The storage format of the depth/stencil target.
//  */
// REFRESHAPI Refresh_DepthStencilTarget* Refresh_CreateDepthStencilTarget(
// 	Refresh_Device *device,
// 	uint32_t width,
// 	uint32_t height,
// 	Refresh_DepthFormat format
// );

// /* Creates a buffer.
//  *
//  * usageFlags:	Specifies how the buffer will be used.
//  * sizeInBytes:	The length of the buffer.
//  */
// REFRESHAPI Refresh_Buffer* Refresh_CreateBuffer(
// 	Refresh_Device *device,
// 	Refresh_BufferUsageFlags usageFlags,
// 	uint32_t sizeInBytes
// );

// /* Setters */
// /* Uploads image data to a texture object.
//  *
//  * 	textureSlice:		The texture slice to be updated.
//  * 	data:				A pointer to the image data.
//  * 	dataLengthInBytes:	The size of the image data.
//  */
// REFRESHAPI void Refresh_SetTextureData(
// 	Refresh_Device *driverData,
// 	Refresh_TextureSlice *textureSlice,
// 	void *data,
// 	uint32_t dataLengthInBytes
// );

// /* Uploads YUV image data to three R8 texture objects.
//  *
//  * y:		The texture storing the Y data.
//  * u:		The texture storing the U (Cb) data.
//  * v:		The texture storing the V (Cr) data.
//  * yWidth:	The width of the Y plane.
//  * yHeight:	The height of the Y plane.
//  * uvWidth:	The width of the U/V planes.
//  * uvHeight:	The height of the U/V planes.
//  * data:	A pointer to the raw YUV image data.
//  * dataLength:	The size of the image data in bytes.
//  */
// REFRESHAPI void Refresh_SetTextureDataYUV(
// 	Refresh_Device *driverData,
// 	Refresh_Texture *y,
// 	Refresh_Texture *u,
// 	Refresh_Texture *v,
// 	uint32_t yWidth,
// 	uint32_t yHeight,
// 	uint32_t uvWidth,
// 	uint32_t uvHeight,
// 	void* data,
// 	uint32_t dataLength
// );

// /* Performs an asynchronous texture-to-texture copy.
//  *
//  * sourceTextureSlice:		The texture slice from which to copy.
//  * destinationTextureSlice:	The texture slice to copy to.
//  * filter:					The filter that will be used if the copy requires scaling.
//  */
// REFRESHAPI void Refresh_CopyTextureToTexture(
// 	Refresh_Device *driverData,
// 	Refresh_CommandBuffer *commandBuffer,
// 	Refresh_TextureSlice *sourceTextureSlice,
// 	Refresh_TextureSlice *destinationTextureSlice,
// 	Refresh_Filter filter
// );

// /* Asynchronously copies image data from a texture slice into a buffer.
//  *
//  * NOTE:
//  * 	The buffer will not contain correct data until the command buffer
//  * 	is submitted and completed.
//  *
//  * textureSlice:	The texture object being copied.
//  * buffer:			The buffer being filled with the image data.
//  */
// REFRESHAPI void Refresh_CopyTextureToBuffer(
// 	Refresh_Device *device,
// 	Refresh_CommandBuffer *commandBuffer,
// 	Refresh_TextureSlice *textureSlice,
// 	Refresh_Buffer *buffer
// );

// /* Sets a region of the buffer with client data.
//  *
//  * NOTE:
//  * 		Calling this function on a buffer after the buffer
//  * 		has been bound without calling Submit first is an error.
//  *
//  * buffer:			The vertex buffer to be updated.
//  * offsetInBytes:	The starting offset of the buffer to write into.
//  * data:			The client data to write into the buffer.
//  * dataLength:		The length of data from the client buffer to write.
//  */
// REFRESHAPI void Refresh_SetBufferData(
// 	Refresh_Device *device,
// 	Refresh_Buffer *buffer,
// 	uint32_t offsetInBytes,
// 	void* data,
// 	uint32_t dataLength
// );

// /* Pushes vertex shader params to the device.
//  * Returns a starting offset value to be used with draw calls.
//  *
//  * NOTE:
//  * 		A pipeline must be bound.
//  * 		Will use the block size of the currently bound vertex shader.
//  *
//  * data: 				The client data to write into the buffer.
//  * paramBlockCount: 	The number of param-sized blocks from the client buffer to write.
//  */
// REFRESHAPI uint32_t Refresh_PushVertexShaderParams(
// 	Refresh_Device *device,
// 	Refresh_CommandBuffer *commandBuffer,
// 	void *data,
// 	uint32_t paramBlockCount
// );

// /* Pushes fragment shader params to the device.
//  * Returns a starting offset value to be used with draw calls.
//  *
//  * NOTE:
//  * 		A graphics pipeline must be bound.
//  * 		Will use the block size of the currently bound fragment shader.
//  *
//  * data: 				The client data to write into the buffer.
//  * paramBlockCount: 	The number of param-sized blocks from the client buffer to write.
//  */
// REFRESHAPI uint32_t Refresh_PushFragmentShaderParams(
// 	Refresh_Device *device,
// 	Refresh_CommandBuffer *commandBuffer,
// 	void *data,
// 	uint32_t paramBlockCount
// );

// /* Pushes compute shader params to the device.
//  * Returns a starting offset value to be used with draw calls.
//  *
//  * NOTE:
//  * 	A compute pipeline must be bound.
//  * 	Will use the block size of the currently bound compute shader.
//  *
//  * data:			The client data to write into the buffer.
//  * paramBlockData:	The number of param-sized blocks from the client buffer to write.
//  */
// REFRESHAPI uint32_t Refresh_PushComputeShaderParams(
// 	Refresh_Device *device,
// 	Refresh_CommandBuffer *commandBuffer,
// 	void *data,
// 	uint32_t paramBlockCount
// );

// /* Getters */
// /* Synchronously copies data from a buffer to a pointer.
//  * You probably want to wait for a sync point to call this.
//  *
//  * buffer: 				The buffer to copy data from.
//  * data:				The pointer to copy data to.
//  * dataLengthInBytes:	The length of data to copy.
//  */
// REFRESHAPI void Refresh_GetBufferData(
// 	Refresh_Device *device,
// 	Refresh_Buffer *buffer,
// 	void *data,
// 	uint32_t dataLengthInBytes
// );

// /* Disposal */
// /* Sends a texture to be destroyed by the renderer. Note that we call it
//  * "QueueDestroy" because it may not be immediately destroyed by the renderer if
//  * this is not called from the main thread (for example, if a garbage collector
//  * deletes the resource instead of the programmer).
//  *
//  * texture: The Refresh_Texture to be destroyed.
//  */
// REFRESHAPI void Refresh_QueueDestroyTexture(
// 	Refresh_Device *device,
// 	Refresh_Texture *texture
// );

// /* Sends a sampler to be destroyed by the renderer. Note that we call it
//  * "QueueDestroy" because it may not be immediately destroyed by the renderer if
//  * this is not called from the main thread (for example, if a garbage collector
//  * deletes the resource instead of the programmer).
//  *
//  * texture: The Refresh_Sampler to be destroyed.
//  */
// REFRESHAPI void Refresh_QueueDestroySampler(
// 	Refresh_Device *device,
// 	Refresh_Sampler *sampler
// );

// /* Sends a buffer to be destroyed by the renderer. Note that we call it
//  * "QueueDestroy" because it may not be immediately destroyed by the renderer if
//  * this is not called from the main thread (for example, if a garbage collector
//  * deletes the resource instead of the programmer).
//  *
//  * buffer: The Refresh_Buffer to be destroyed.
//  */
// REFRESHAPI void Refresh_QueueDestroyBuffer(
// 	Refresh_Device *device,
// 	Refresh_Buffer *buffer
// );

// /* Sends a color target to be destroyed by the renderer. Note that we call it
//  * "QueueDestroy" because it may not be immediately destroyed by the renderer if
//  * this is not called from the main thread (for example, if a garbage collector
//  * deletes the resource instead of the programmer).
//  *
//  * colorTarget: The Refresh_ColorTarget to be destroyed.
//  */
// REFRESHAPI void Refresh_QueueDestroyColorTarget(
// 	Refresh_Device *device,
// 	Refresh_ColorTarget *colorTarget
// );

// /* Sends a depth/stencil target to be destroyed by the renderer. Note that we call it
//  * "QueueDestroy" because it may not be immediately destroyed by the renderer if
//  * this is not called from the main thread (for example, if a garbage collector
//  * deletes the resource instead of the programmer).
//  *
//  * depthStencilTarget: The Refresh_DepthStencilTarget to be destroyed.
//  */
// REFRESHAPI void Refresh_QueueDestroyDepthStencilTarget(
// 	Refresh_Device *device,
// 	Refresh_DepthStencilTarget *depthStencilTarget
// );

// /* Graphics State */
// /* Begins a render pass.
//  *
//  * renderPass: The renderpass to begin.
//  * framebuffer: The framebuffer to bind for the render pass.
//  * renderArea:
//  * 		The area affected by the render pass.
//  * 		All load, store and resolve operations are restricted
//  * 		to the given rectangle.
//  * clearValues:
//  * 		A pointer to an array of Refresh_Color structures
//  * 		that contains clear values for each color target in the
//  * 		framebuffer. May be NULL.
//  * clearCount: The amount of color structs in the above array.
//  * depthStencilClearValue: The depth/stencil clear value. May be NULL.
//  */
// REFRESHAPI void Refresh_BeginRenderPass(
// 	Refresh_Device *device,
// 	Refresh_CommandBuffer *commandBuffer,
// 	Refresh_RenderPass *renderPass,
// 	Refresh_Framebuffer *framebuffer,
// 	Refresh_Rect renderArea,
// 	Refresh_Color *pColorClearValues,
// 	uint32_t colorClearCount,
// 	Refresh_DepthStencilValue *depthStencilClearValue
// );

// /* Ends the current render pass. */
// REFRESHAPI void Refresh_EndRenderPass(
// 	Refresh_Device *device,
// 	Refresh_CommandBuffer *commandBuffer
// );

// /* Binds a graphics pipeline to the graphics bind point. */
// REFRESHAPI void Refresh_BindGraphicsPipeline(
// 	Refresh_Device *device,
// 	Refresh_CommandBuffer *commandBuffer,
// 	Refresh_GraphicsPipeline *graphicsPipeline
// );

// /* Binds vertex buffers for use with subsequent draw calls. */
// REFRESHAPI void Refresh_BindVertexBuffers(
// 	Refresh_Device *device,
// 	Refresh_CommandBuffer *commandBuffer,
// 	uint32_t firstBinding,
// 	uint32_t bindingCount,
// 	Refresh_Buffer **pBuffers,
// 	uint64_t *pOffsets
// );

// /* Binds an index buffer for use with subsequent draw calls. */
// REFRESHAPI void Refresh_BindIndexBuffer(
// 	Refresh_Device *device,
// 	Refresh_CommandBuffer *commandBuffer,
// 	Refresh_Buffer *buffer,
// 	uint64_t offset,
// 	Refresh_IndexElementSize indexElementSize
// );

// /* Sets textures/samplers for use with the currently bound vertex shader.
//  *
//  * NOTE:
//  * 		The length of the passed arrays must be equal to the number
//  * 		of sampler bindings specified by the pipeline.
//  *
//  * textures:	A pointer to an array of textures.
//  * samplers:	A pointer to an array of samplers.
//  */
// REFRESHAPI void Refresh_BindVertexSamplers(
// 	Refresh_Device *device,
// 	Refresh_CommandBuffer *commandBuffer,
// 	Refresh_Texture **pTextures,
// 	Refresh_Sampler **pSamplers
// );

// /* Sets textures/samplers for use with the currently bound fragment shader.
//  *
//  * NOTE:
//  *		The length of the passed arrays must be equal to the number
//  * 		of sampler bindings specified by the pipeline.
//  *
//  * textures: 	A pointer to an array of textures.
//  * samplers:	A pointer to an array of samplers.
//  */
// REFRESHAPI void Refresh_BindFragmentSamplers(
// 	Refresh_Device *device,
// 	Refresh_CommandBuffer *commandBuffer,
// 	Refresh_Texture **pTextures,
// 	Refresh_Sampler **pSamplers
// );

// /* Binds a compute pipeline to the compute bind point. */
// REFRESHAPI void Refresh_BindComputePipeline(
// 	Refresh_Device *device,
// 	Refresh_CommandBuffer *commandBuffer,
// 	Refresh_ComputePipeline *computePipeline
// );

// /* Binds buffers for use with the currently bound compute pipeline.
//  *
//  * pBuffers: An array of buffers to bind.
//  * 	Length must be equal to the number of buffers
//  * 	specified by the compute pipeline.
//  */
// REFRESHAPI void Refresh_BindComputeBuffers(
// 	Refresh_Device *device,
// 	Refresh_CommandBuffer *commandBuffer,
// 	Refresh_Buffer **pBuffers
// );

// /* Binds textures for use with the currently bound compute pipeline.
//  *
//  * pTextures: An array of textures to bind.
//  * 	Length must be equal to the number of buffers
//  * 	specified by the compute pipeline.
//  */
// REFRESHAPI void Refresh_BindComputeTextures(
// 	Refresh_Device *device,
// 	Refresh_CommandBuffer *commandBuffer,
// 	Refresh_Texture **pTextures
// );

// /* Submission/Presentation */
// /* Returns an allocated Refresh_CommandBuffer* object.
//  * This command buffer is managed by the implementation and
//  * should NOT be freed by the user.
//  *
//  * NOTE:
//  * 	A command buffer may only be used on the thread that
//  * 	it was acquired on. Using it on any other thread is an error.
//  *
//  * fixed:
//  * 	If a command buffer is designated as fixed, it can be
//  * 	acquired once, have commands recorded into it, and
//  * 	be re-submitted indefinitely.
//  *
//  */
// REFRESHAPI Refresh_CommandBuffer* Refresh_AcquireCommandBuffer(
// 	Refresh_Device *device,
// 	uint8_t fixed
// );

// /* Queues an image to be presented to the screen.
//  * The image will be presented upon the next Refresh_Submit call.
//  *
//  * NOTE:
//  *		It is an error to call this function in headless mode.
//  *
//  * textureSlice:			The texture slice to present.
//  * destinationRectangle:	The region of the window to update. Can be NULL.
//  * filter:					The filter to use if scaling is required.
//  */
// REFRESHAPI void Refresh_QueuePresent(
// 	Refresh_Device *device,
// 	Refresh_CommandBuffer *commandBuffer,
// 	Refresh_TextureSlice *textureSlice,
// 	Refresh_Rect *destinationRectangle,
// 	Refresh_Filter filter
// );

// /* Submits all of the enqueued commands. */
// REFRESHAPI void Refresh_Submit(
// 	Refresh_Device* device,
// 	uint32_t commandBufferCount,
// 	Refresh_CommandBuffer **pCommandBuffers
// );

// /* Waits for the previous submission to complete. */
// REFRESHAPI void Refresh_Wait(
// 	Refresh_Device *device
// );

// /* Export handles to be consumed by another API */
// REFRESHAPI void Refresh_GetTextureHandles(
// 	Refresh_Device* device,
// 	Refresh_Texture* texture,
// 	Refresh_TextureHandles* handles
// );
