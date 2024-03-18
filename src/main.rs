use std::{thread, time, io};

fn input(var: &mut String) -> &str{
    let mut buf: &mut String = var;
    io::stdin().read_line(buf).expect("Error to read line");
    return buf.trim();
}

fn get_time() -> (u32, u32, u32){
    let mut hour: String = String::new();
    let mut mins: String = String::new();
    let mut seconds: String = String::new();

    let hour: u32 = input(&mut hour).parse().expect("Error parse number");
    let mins: u32 = input(&mut mins).parse().expect("Error parse number");
    let seconds: u32 = input(&mut seconds).parse().expect("Error parse number");

    return (hour, mins, seconds);
}

fn timer(hour: u32, mins: u32, seconds: u32){

    loop{

    }

}

fn main() {
    loop {
        let (hour, mins, seconds) = get_time();
        println!("{hour}, {mins}, {seconds}")

    }
}
