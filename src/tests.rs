#![cfg(test)]
use super::*;
/// it probably doesn't work 😢
#[test]
fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
}
#[test]
fn windows_rs_io_safety_apitranslation() {
        assert!(HANDLE(0isize).is_invalid() && HANDLE(-1isize).is_invalid());
}
#[test]
fn example_test() {
        todo!();
        // let path = PathBuf::from(format!(r"\\.\pipe\LOCAL\rust_testing_exmaple_{}", ::std::process::id()));
        // let server = DuplexServer::open(&path);
}
