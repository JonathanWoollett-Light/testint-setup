/// By default we do not install system dependencies.
const INSTALL: bool = false;

pub fn install(packages: &[&str]) {
    if INSTALL {
        std::process::Command::new("sudo")
            .args(
                ["apt-get", "update"]
                    .iter()
                    .chain(packages.iter())
                    .collect::<Vec<_>>(),
            )
            .output()
            .unwrap();
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
        let output = std::process::Command::new("apt-cache")
            .args(
                ["--quiet=0", "policy"]
                    .iter()
                    .chain(packages.iter())
                    .collect::<Vec<_>>(),
            )
            .output()
            .unwrap();
        for missing in std::str::from_utf8(&output.stderr)
            .unwrap()
            .trim()
            .split('\n')
        {
            // TODO: Change this to `println!("cargo:error={}",missing);` when this features becomes
            // avaible.
            eprintln!("{}", missing);
        }
        if !output.stderr.is_empty() {
            std::process::exit(1)
        }
    }
}
