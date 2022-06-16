// Each crate can be built indepedant of the workspace and natively with `cargo build`
fn main() {
    std::process::Command::new("sudo")
        .args(["apt-get", "update"])
        .output()
        .unwrap();
    std::process::Command::new("sudo")
        .args([
            "apt-get",
            "install",
            "binutils-dev",
            "clang",
            "cmake",
            "build-essential",
            "zlib1g-dev",
        ])
        .output()
        .unwrap();
}
