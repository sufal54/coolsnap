use std::{fs, thread::sleep, time::Duration};

static LIMIT_LOC: &str = "/opt/coolsnap/temp_limit";
static CORE_DIR: &str = "/sys/class/thermal/";

fn read_limit() -> usize {
    fs::read_to_string(LIMIT_LOC)
        .unwrap_or_else(|_| "50".to_string())
        .trim()
        .parse::<usize>()
        .unwrap_or(50)
}

fn read_tem(core_loc: &Vec<String>) -> usize {
    let mut total_temp = 0;
    for path in core_loc.iter() {
        let raw = fs::read_to_string(format!("{}/temp", path)).unwrap();

        let curr_temp = raw.trim().parse::<usize>().unwrap();

        total_temp += curr_temp / 1000;
    }
    total_temp / core_loc.len()
}

fn main() {
    let mut core_loc = Vec::<String>::new();

    let core_list = fs::read_dir(CORE_DIR).unwrap();

    for entry in core_list {
        let entry = entry.unwrap();
        let name = entry.file_name().into_string().unwrap();

        if name.starts_with("thermal_zone") {
            core_loc.push(entry.path().display().to_string());
        }
    }

    loop {
        let temprature = read_tem(&core_loc);

        let limit = read_limit();

        println!("{}", temprature);
        if temprature < limit {
            break;
        }

        sleep(Duration::from_secs(2));
    }
}
