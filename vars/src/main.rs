use std::env::args;

fn main() {
    println!("{}", circle_area(5.0));
    let args: Vec<String> = args().skip(1).collect();
    for arg in args {
        if arg == "count" {
            count(arg);
        }
    }
}

fn circle_area(r: f64) -> f64 {
    r * r * 3.14
}

fn count(arg: String) {
    let mut count: i8 = 0;

    'outerloop: loop {
        println!("{} {} times", arg, count + 1);
        count += 1;
        if count >= 8 { break 'outerloop }
    }
}