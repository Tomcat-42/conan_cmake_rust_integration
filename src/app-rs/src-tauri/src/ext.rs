use autocxx::prelude::*;

include_cpp! {
    #include "deep_thought/answer.hpp"
    safety!(unsafe_ffi)
    generate!("deep_thought::answer")
}

pub fn answer() -> i32 {
    ffi::deep_thought::answer().into()
}
