pub mod data_bin;
pub mod regfile;
pub mod images;
pub mod shared_types;
mod stream_utils;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
