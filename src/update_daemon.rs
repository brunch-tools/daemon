use duct::cmd;

pub fn update_daemon() {
    let output1 = cmd!("curl", "-Ls", "https://brunch.tools/install.sh").pipe(cmd!("sudo","bash")).read().unwrap();
    println!("{}", output1);
}