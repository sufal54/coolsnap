use std::{env, fs, os::unix::fs::symlink, path::Path};

static OUTPUT_PATH: &str = "/etc/systemd/system/coolsnap.service";
static BIN_PATH: &str = "/opt";

fn main() {
    let current_dir = env::current_dir().unwrap();

    if Path::new(OUTPUT_PATH).exists() {
        fs::remove_file(OUTPUT_PATH).unwrap();
    }

    fs::copy(
        format!("{}/config/coolsnap.service", current_dir.display()),
        OUTPUT_PATH,
    )
    .unwrap();

    let home = format!("{}/coolsnap", BIN_PATH);

    if Path::new(&home).exists() {
        fs::remove_dir_all(&home).unwrap();
    }

    fs::create_dir_all(format!("{}/bin", home)).unwrap();

    fs::write(format!("{}/temp_limit", home), "50").unwrap();

    fs::copy(
        format!("{}/target/release/coolsnap", current_dir.display()),
        format!("{}/bin/coolsnap", home),
    )
    .unwrap();

    fs::copy(
        format!("{}/script/temptool.sh", current_dir.display()),
        format!("{}/bin/temptool.sh", home),
    )
    .unwrap();

    let link = "/usr/local/bin/coolsnap";

    if Path::new(link).exists() {
        fs::remove_file(link).unwrap();
    }

    symlink("/opt/coolsnap/bin/temptool.sh", link).unwrap();
}
