const PACKAGES: [&str; 5] = [
    "binutils-dev",
    "clang",
    "cmake",
    "build-essential",
    "zlib1g-dev",
];

// Each crate can be built indepedant of the workspace and natively with `cargo build`
fn main() {
    build_util::install(&PACKAGES);
}
