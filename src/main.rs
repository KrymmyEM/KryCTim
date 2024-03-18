use std::{thread, time, io};

fn input(var: &mut String) -> &str{
    let mut buf: &mut String = var;
    io::stdin().read_line(buf).expect("Error to read line");
    return buf.trim();
}

fn get_time() -> (u64, u64, u64){
    let mut hour: String = String::new();
    let mut mins: String = String::new();
    let mut seconds: String = String::new();

    let hour: u64 = input(&mut hour).parse().expect("Error parse number");
    let mins: u64 = input(&mut mins).parse().expect("Error parse number");
    let seconds: u64 = input(&mut seconds).parse().expect("Error parse number");

    return (hour, mins, seconds);
}

fn timer(hour: u64, mins: u64, seconds: u64, type_timer: u8) -> bool{
    let hour_on_seconds: u64 = hour * 3_600;
    let mins_on_seconds: u64 = mins * 60;
    let full_seconds: u64 = hour_on_seconds + mins_on_seconds + seconds;
    let sleep_seconds = time::Duration::new(full_seconds, 0);
    let second = time::Duration::new(1, 0);

    match type_timer {
        1 => {
            thread::sleep(sleep_seconds);
            return true;
        },
        2 => {
            let mut counter: u64 = 1;
            
            while counter < full_seconds {
                thread::sleep(second);
                counter+=1;
            }
            return true;
        },
        _ => {
            println!("Error type timer");
            return false;
        }
    }
}

fn main() {
    let type_timer: u8 = input(&mut String::new()).parse().expect("Error parse number");
    let (hour, mins, seconds) = get_time();
    timer(hour, mins, seconds, type_timer);
}
