/*!

Wrapper of Refresh

*/

pub use refresh_ffi as ffi;

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
pub type Device = ffi::Refresh_Device;
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
    pub struct BufferUsageFlagBits: u32 {
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
    pub struct ColorComponentFlagBits: u32 {
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
    pub struct TextureUsageFlagBits: u32 {
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
