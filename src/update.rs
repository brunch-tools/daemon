use duct::cmd;

pub fn update_toolkit() {
    let output1 = cmd!("curl", "-l", "https://raw.githubusercontent.com/WesBosch/brunch-toolkit/main/brunch-toolkit","-o","/tmp/brunch-toolkit").read().unwrap();
    println!("{}", output1);
    let output2 = cmd!("install", "-Dt","/usr/local/bin","-m","755","/tmp/brunch-toolkit").read().unwrap();
    println!("{}", output2);
    let output2 = cmd!("rm", "/tmp/brunch-toolkit").read().unwrap();
    println!("{}", output2);
}

pub fn update_framework() {
    let output1 = cmd!("/usr/local/bin/brunch-toolkit","-pwa-au").read().unwrap();
    println!("{}", output1);
}

pub fn update_daemon() {
    let output1 = cmd!("curl", "-Ls", "https://brunch.tools/install.sh").pipe(cmd!("sudo","bash")).read().unwrap();
    println!("{}", output1);
}