//! Embedded resources for the examples

use refresh::Resource;

const COLOR_PHASE: &[u8] = include_bytes!("color_phase_frag.spv");
const HEX_GRID: &[u8] = include_bytes!("hexagon_grid.spv");
const PASS_THROUGH: &[u8] = include_bytes!("passthrough_vert.spv");
const SEASCAPE: &[u8] = include_bytes!("seascape.spv");

pub fn pass_through_vs(device: &refresh::DeviceImpl) -> *mut refresh::ShaderModule {
    let info = refresh::ShaderModuleCreateInfo {
        byteCode: PASS_THROUGH.as_ptr() as *const _ as *mut _,
        codeSize: PASS_THROUGH.len() as u32,
    };

    refresh::ShaderModule::create(device, &info)
}
