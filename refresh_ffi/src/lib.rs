//! Rust FFI to Refresh

mod ffi;
pub use ffi::*;

pub mod img;

#[cfg(test)]
mod test {
    use crate::ffi;

    #[test]
    fn link_test() {
        unsafe {
            println!("Refresh version: {}", ffi::Refresh_LinkedVersion());
        }
    }
}
