
use std::fs::{read_dir};

fn get_devices() -> Vec<String> {
    read_dir("/sys/class/net/").unwrap().map(|res| res.unwrap().file_name().into_string().unwrap()).collect()
}


fn main() {
    println!("{:?}", get_devices());
}
