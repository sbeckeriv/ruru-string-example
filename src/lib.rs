#[macro_use]
extern crate ruru;
use ruru::{AnyObject, Boolean, Class, Fixnum, RString, VM};
use ruru::types::Argc;

#[no_mangle]
pub extern fn initialize_rust_color() {
    methods!(
        RString, // type of `self` object
        itself, // name of `self` object which will be used in methods

        fn string_is_blank() -> Boolean {
            Boolean::new(itself.to_string().chars().all(|c| c.is_whitespace()))
        }

        fn string_length_equals(expected_length: Fixnum) -> Boolean {
            let real_length = itself.to_string().len() as i64;

            Boolean::new(expected_length.to_i64() == real_length)
        }
    );

    Class::from_existing("String").define(|itself| {
        itself.def("becker", string_is_blank);
        itself.def("length_equals?", string_length_equals);
    });
}
//}
