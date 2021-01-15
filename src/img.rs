/*!

Refresh image module powered by `stb_image.h`

* TODO: load image from memory (needs change in Refresh)

*/

use refresh_ffi::img as ffi;
use std::ffi::CStr;

/// TODO: create comfortable API
pub struct Image {
    pub pixels: Vec<u8>,
    pub w: u32,
    pub h: u32,
    pub n_channels: u32,
}

impl Image {
    pub fn load_name(name: &CStr) -> Self {
        let (mut w, mut h, mut n_channels) = (0u32, 0u32, 0u32);

        let ptr = unsafe {
            ffi::Refresh_Image_Load(
                name.as_ptr(),
                &mut w as *const _ as *mut _,
                &mut h as *const _ as *mut _,
                &mut n_channels as *const _ as *mut _,
            )
        };

        let len = (w * h) as usize;
        let pixels = unsafe { Vec::from_raw_parts(ptr, len, len) };

        Self {
            pixels,
            w,
            h,
            n_channels,
        }
    }

    pub fn save_png(&self, name: &CStr) {
        self::save_png(name, self.w, self.h, &self.pixels);
    }
}

// `Vec` frees the memory so we don't need C `free` actually
// pub fn free(mem: &[u8]) {
//     unsafe {
//         ffi::Refresh_Image_Free(mem.as_ptr() as *const _ as *mut _);
//     }
// }

pub fn save_png(name: &CStr, w: u32, h: u32, data: &[u8]) {
    unsafe {
        ffi::Refresh_Image_SavePNG(
            name.as_ptr(),
            w as i32,
            h as i32,
            data.as_ptr() as *const _ as *mut _,
        );
    }
}
