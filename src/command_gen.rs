use crate::{appinput::GenOptions, gen_cmake, gen_ps1, gen_src};

pub fn gen(option: GenOptions) {
    match option {
        GenOptions::Src(options) => gen_src::gen_src(options),
        GenOptions::CMake(options) => gen_cmake::gen_cmake(options),
        GenOptions::PS1(options) => gen_ps1::gen_ps1(options),
    }
}
