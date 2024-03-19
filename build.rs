use std::fmt::format;
use std::path::PathBuf;

fn main() {
    if cfg!(feature = "cpu") {
        println!("cargo:rerun-if-changed=yrrid-cpu-msm/Main.cpp");
        cc::Build::new()
            .cpp(true)
            .std("c++17")
            .opt_level(3)
            .file("yrrid-cpu-msm/Main.cpp")
            .compile("cpu-msm");
    }

    if cfg!(feature = "gpu") {
        cc::Build::new()
            .cpp(true)
            .std("c++17")
            .flag("-fopenmp")
            .file("yrrid-fpga-msm/Main.cpp")
            .define("__BLS12_381_XCLBIN_PATH__", &*bls12_381_xclbin_str)
            .define("__BLS12_377_XCLBIN_PATH__", &*bls12_377_xclbin_str)
            .include("/opt/xilinx/xrt/include/")
            .opt_level(3)
            .compile("fpga-msm");
    }
}
