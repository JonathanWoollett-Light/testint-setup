// Each crate can be built indepedant of the workspace and natively with `cargo build`
fn main() {
    std::process::Command::new("sudo").args(["apt-get","update"]).output().unwrap();
    std::process::Command::new("sudo").args([
        "apt-get","install","libgdbm-dev","libnss3-dev","libreadline-dev","libffi-dev"
    ]).output().unwrap();
}