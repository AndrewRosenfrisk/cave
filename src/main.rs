use core::time;
use std::thread::sleep;

use rand::Rng;

const WIDTH: usize = 70;
const PAUSE_AMOUNT: f32 = 0.05;

fn main() {
    println!("Press Ctrl-C to stop.");

    sleep(time::Duration::from_secs(2));

    let mut left_width: usize = 20;
    let gap_width: usize = 10;

    let mut rng = rand::thread_rng();
    loop {
        let right_width = WIDTH - gap_width - left_width;
        let line =
            "#".to_string().repeat(left_width) + &" ".repeat(gap_width) + &"#".repeat(right_width);
        println!("{}", line);

        sleep(time::Duration::from_secs_f32(PAUSE_AMOUNT));

        let change = rng.gen_range(1..=6);

        if change == 1 && left_width > 1 {
            left_width -= 1;
        } else if change == 2 && left_width + gap_width < WIDTH - 1 {
            left_width += 1;
        } else {
            continue;
        }
    }
}
