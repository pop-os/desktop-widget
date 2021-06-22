use std::{
    env,
    fs::{self, File},
    io::{self, Write},
};

const PKGCONFIG: &str = r#"Name: ${name}
Description: {description}
Version: {version}
Cflags: -I${includedir}
Libs: -L${libdir} -l${name}"#;

fn main() -> io::Result<()> {
    let mut args = env::args();

    let app = args.nth(1).expect("requires APP argument");
    let libdir = args.next().expect("requires libdir argument");
    let includedir = args.next().expect("requires includedir argument");

    fs::create_dir_all("target/")?;

    let target = ["target/", &app, ".pc"].concat();
    let mut file = File::create(&target).expect("unable to create pkgconfig file");

    let cargo = fs::read_to_string("Cargo.toml").expect("no parent Cargo.toml");

    let version = cargo
        .lines()
        .find(|line| line.starts_with("version ="))
        .expect("no version found in parent Cargo.toml")
        .split_whitespace()
        .nth(2)
        .expect("no version string on version key in Cargo.toml");

    let config = PKGCONFIG.replace("{version}", version);

    writeln!(&mut file, "libdir={}\nincludedir={}\nname={}\n{}", libdir, includedir, app, config)
}
