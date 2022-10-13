use std::path::Path;

const COMPILER_PATH: &str = "clang++";
const VORO_DIR: &str = "src/voro/src";
const VORO_SRC: &[&str] = &[
    "cell.cc",
    "common.cc",
    "container.cc",
    "unitcell.cc",
    "v_compute.cc",
    "c_loops.cc",
    "v_base.cc",
    "wall.cc",
    "pre_container.cc",
    "container_prd.cc",
];
const CORE_PROCEDURES: &[&str] = &["src/voro_orthogonal_container.cpp"];
const LIB_NAME: &str = "voro_dependency";

fn main() {
    let complete_voro_paths: Vec<String> = VORO_SRC
        .iter()
        .map(|file| format!("{}/{}", VORO_DIR, file))
        .collect();

    cc::Build::new()
        .compiler(COMPILER_PATH)
        .files(complete_voro_paths.iter().map(|path| Path::new(path)))
        .files(CORE_PROCEDURES.iter().map(|path| Path::new(path)))
        .compile(LIB_NAME)
}
