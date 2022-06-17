const PACKAGES: [&str; 4] = [
    "build-essential",
    "zlib1g-dev",
    "libncurses5-dev",
    "libgdbm-dev",
];

// Each crate can be built indepedant of the workspace and natively with `cargo build`
fn main() {
    build_util::install(&PACKAGES);
}
