use std::time::Instant;
use ewts::EwtsConverter;
use std::fs;

const ITERATION_COUNT: u64 = 33;

fn main() {
    let ewts_text = fs::read_to_string("./sample_ewts_text.txt").unwrap();
    let fsize = fs::metadata("./sample_ewts_text.txt").unwrap().len();
    let converter = EwtsConverter::create();

    #[allow(unused_variables)]
    let mut total_chars_count = 0;

    let now = Instant::now();
    for _ in 0..ITERATION_COUNT {
        let r = converter.ewts_to_unicode(&ewts_text);
        total_chars_count += r.chars().count();
    }

    let elapsed_ms = now.elapsed().as_millis();
    let elapsed_s = now.elapsed().as_secs_f64();
    let speed = (fsize * ITERATION_COUNT / 1024) as f64 / elapsed_s;

    println!("ewts-rs (rust): speed - {:.3} Kb/s; launches - {}; time - {} ms", speed, ITERATION_COUNT, elapsed_ms);

}
