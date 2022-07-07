/// By default we do not install system dependencies.
/// In application this would be an enviroment variable.
const INSTALL: bool = false;

pub fn install(packages: &[&str]) {
    if INSTALL {
        // Updates `apt-get`.
        std::process::Command::new("sudo")
            .args(
                ["apt-get", "update"]
                    .iter()
                    .chain(packages.iter())
                    .collect::<Vec<_>>(),
            )
            .output()
            .unwrap();
        // Installs dependencies from `apt-get`.
        std::process::Command::new("sudo")
            .args(
                ["apt-get", "install"]
                    .iter()
                    .chain(packages.iter())
                    .collect::<Vec<_>>(),
            )
            .output()
            .unwrap();
    } else {
        // Checks dependencies with `apt-cache`.
        let output = std::process::Command::new("apt-cache")
            .args(
                ["--quiet=0", "policy"]
                    .iter()
                    .chain(packages.iter())
                    .collect::<Vec<_>>(),
            )
            .output()
            .unwrap();
        // Prints each missing dependency.
        for missing in std::str::from_utf8(&output.stderr)
            .unwrap()
            .trim()
            .split('\n')
        {
            // TODO: Change this to `println!("cargo:error={}",missing);` when this features becomes
            // avaible.
            eprintln!("{}", missing);
        }
        // Exits with non-zero error code if any depdencies where missing.
        if !output.stderr.is_empty() {
            std::process::exit(1)
        }
    }
}
