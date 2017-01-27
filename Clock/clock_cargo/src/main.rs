extern crate time;
extern crate schedule_recv;

fn main() {

    let tick = schedule_recv::periodic_ms(1000);

    loop {
        tick.recv().unwrap();
        println!("{}", time::now().strftime("%Y-%m-%d %H:%M:%S").unwrap());
    }
}
