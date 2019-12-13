
use std::fs::{read_dir, read_to_string};

fn get_devices() -> Vec<String> {
    read_dir("/sys/class/net/").unwrap().map(|res| res.unwrap().file_name().into_string().unwrap()).collect()
}

fn read_entry(device: &String, entry: &str) -> String {
    read_to_string(format!("/sys/class/net/{}/statistics/{}", device, entry)).unwrap()
}

fn main() {
    for device in get_devices() {
        println!("Printing statistics for device \"{}\":", device);
        print!("Total Bytes Sent - {}", read_entry(&device, "tx_bytes"));
        print!("Total Bytes Received - {}", read_entry(&device, "rx_bytes"));
        print!("Total Packets Sent - {}", read_entry(&device, "tx_packets"));
        println!("Total Packets Received - {}", read_entry(&device, "rx_packets"));
    }
}
