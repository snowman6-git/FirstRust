use sysinfo::{
    // Components, Disks, Networks, +
    System,
};
fn togb(value: u64) -> f64 {
    (value as f64) / 1024_f64.powf(3.0)
}

fn main(){
    let mut sys = System::new_all();
    sys.refresh_all();
    let system = match System::name() {
        Some(value) => value,
        _ => String::from("Lost"),
    };
    println!("System Name: {}", system);
    println!("Memory: {:.1} GB", togb(sys.total_memory()));
    println!("Used: {:.1} GB", togb(sys.used_memory()));

    // println!("요약 {:.1}/{:.1} GB", togb(sys.used_memory()), togb(sys.total_memory()));

    
    // let mut input = String::new();
    // std::io::stdin()
    // .read_line(&mut input)
    // .expect("can not read user input");
}