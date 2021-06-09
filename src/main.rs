use std::fs;
use std::path::{Path, PathBuf};

fn main() {
    let basedir = "/sys/class/hwmon";
    for entry in fs::read_dir(basedir).expect("Unable to read dir") {
        let hwmon = HwMonTemperatures::new(entry.unwrap().path());

        let hwmon_name = hwmon.read_name().unwrap_or("<undef>".to_string());
        for sensor in hwmon {
            println!(
                "{}-{}: {}°C",
                hwmon_name,
                sensor.label.clone().unwrap_or("<undef>".to_string()),
                sensor.temp / 1000
            );
        }
    }
}

struct TempData {
    label: Option<String>,
    temp: i32,
}

struct HwMonTemperatures {
    num: u8,
    path: PathBuf,
}

impl HwMonTemperatures {
    fn new(path: PathBuf) -> Self {
        Self { num: 0, path } // Num is incremented before first use, files start at temp1.
    }

    fn read_name(&self) -> Option<String> {
        Some(
            fs::read_to_string(self.path.join("name")).ok()?.trim().to_string()
            )
    }
}

impl Iterator for HwMonTemperatures {
    type Item = TempData;

    fn next(&mut self) -> Option<Self::Item> {
        self.num += 1;
        let path = std::path::Path::new(&self.path);
        Some(TempData {
            temp: read_temp(path, self.num)?,
            label: read_label(path, self.num),
        })
    }
}


fn read_temp(path: &Path, num: u8) -> Option<i32> {
    let filename = format!("temp{}_input", num);
    Some(
        fs::read_to_string(path.join(filename))
            .ok()?
            .trim()
            .parse()
            .ok()?,
    )
}

fn read_label(path: &Path, num: u8) -> Option<String> {
    let filename = format!("temp{}_label", num);
    Some(
        fs::read_to_string(path.join(filename))
            .ok()?
            .trim()
            .to_string(),
    )
}
