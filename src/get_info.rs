use tungstenite::Message;
use std::fs::File;
use std::io::Read;
use json::JsonValue;

pub fn get_info() -> Message {
    let mut data = json::JsonValue::new_object();
    data["daemon_version"] = JsonValue::from(get_toolkit_daemon_ver());
    data["toolkit_version"] = JsonValue::from(get_toolkit_ver());
    data["brunch_version"] = JsonValue::from(get_brunch_ver());
    data["device_type"] = JsonValue::from(get_device_type());
    return Message::from(json::stringify(data));
}

pub fn get_toolkit_ver() -> String {
    return "NONE".parse().unwrap();
}

pub fn get_toolkit_daemon_ver() -> String {
    return String::from(env!("CARGO_PKG_VERSION"));
}

pub fn get_brunch_ver() -> String {
    let file = match File::open("/etc/brunch_version") {
        Ok(file) => Some(file),
        Err(_err) => {
            None
        }
    };
    return if !file.is_none() {
        let mut contents = "".to_string();
        match file.unwrap().read_to_string(&mut contents) {
            Ok(file) => Some(file),
            Err(_err) => {
                None
            }
        };
        String::from(contents.replace("\n",""))
    } else {
        String::from("NONE")
    }
}

pub fn get_device_type() -> String {
    let sys_vendor_file = match File::open("/sys/devices/virtual/dmi/id/sys_vendor") {
        Ok(file) => Some(file),
        Err(_err) => {
            None
        }
    };
    let mut sys_vendor = "".to_string();
    if !sys_vendor_file.is_none() {
        match sys_vendor_file.unwrap().read_to_string(&mut sys_vendor) {
            Ok(file) => Some(file),
            Err(_err) => {
                None
            }
        };
    } else {
        sys_vendor= "Generic".parse().unwrap();
    }
    let product_family_file = match File::open("/sys/devices/virtual/dmi/id/product_family") {
        Ok(file) => Some(file),
        Err(_err) => {
            None
        }
    };
    let mut product_family = "".to_string();
    if !product_family_file.is_none() {
        match product_family_file.unwrap().read_to_string(&mut product_family) {
            Ok(file) => Some(file),
            Err(_err) => {
                None
            }
        };
    } else {
        product_family= "Brunchbook".parse().unwrap()
    }
    return sys_vendor.replace("\n","")+" "+&*product_family.replace("\n","");
}