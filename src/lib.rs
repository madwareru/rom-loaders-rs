#[macro_use]
extern crate bitflags;

pub mod data_bin;
pub mod regfile;
pub mod images;
pub mod shared_types;
pub mod multimedia;
pub mod alm;

mod stream_utils;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
