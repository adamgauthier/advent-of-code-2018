use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

fn main() -> std::io::Result<()> {
    let file = File::open("input.txt")?;
    let buf_reader = BufReader::new(file);

    let mut calibration = 0;

    for line_res in buf_reader.lines() {
        let line = line_res?;
        let mut line = line.chars();

        let sign = line.next().unwrap();
        let number = line.as_str().parse::<i64>().unwrap();

        if sign == '+' {
            calibration += number;
        } else {
            calibration -= number;
        }
    }

    println!("Result: {}", calibration);

    Ok(())
}
