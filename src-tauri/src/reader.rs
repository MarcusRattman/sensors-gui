#![allow(dead_code, unused_variables)]
use std::collections::BTreeMap;
use std::fs::{read_dir, read_to_string};

pub fn get_devices() -> BTreeMap<String, usize> {
    let hwmon = "/sys/class/hwmon";
    let devices = read_dir(hwmon).unwrap().filter_map(Result::ok);
    let mut device_temp = BTreeMap::new();

    let mut sensors = devices
        .map(|device| {
            read_dir(device.path())
                .unwrap()
                .filter_map(Result::ok)
                .map(|file| file.path().to_string_lossy().to_string())
                .filter(|name| {
                    name.contains("temp") && (name.contains("label") || name.contains("input"))
                })
                .collect::<Vec<_>>()
        })
        .flatten()
        .collect::<Vec<_>>();

    sensors.sort();

    sensors.chunks(2).for_each(|chunk| {
        let label = read_to_string(chunk[1].clone()).unwrap().trim().to_string();

        let temp = read_to_string(chunk[0].clone())
            .unwrap()
            .trim()
            .parse::<usize>()
            .unwrap()
            / 1000;

        device_temp.insert(label, temp);
    });

    device_temp
}

#[cfg(test)]
#[test]
fn is_read() {
    println!("{:?}", get_devices());
}
