mod utils {
    use std::error::Error;
    use std::io::stdin;
    use std::str::FromStr;

    #[allow(dead_code)]
    pub fn read_line<T>() -> Result<Vec<T>, Box<dyn Error>>
    where
        T: FromStr,
        T::Err: 'static + Error,
    {
        let mut line = String::new();
        let _ = stdin().read_line(&mut line)?;
        let parsed_line = line.split_whitespace()
            .map(|x| x.parse::<T>())
            .collect::<Result<Vec<T>, T::Err>>()?;

        Ok(parsed_line)
    }

    #[allow(dead_code)]
    pub fn read_lines<T>(n: usize) -> Result<Vec<Vec<T>>, Box<dyn Error>>
    where
        T: FromStr,
        T::Err: 'static + Error,
    {
        (0..n).map(|_| read_line()).collect()
    }
}

fn solve(sx: f64, sy: f64, gx: f64, gy: f64) -> f64 {

    (sy * gx + sx * gy) / (sy + gy)
}

fn run() -> Result<(), Box<dyn std::error::Error>> {

    let p = utils::read_line::<f64>()?;

    println!("{}", solve(p[0], p[1], p[2], p[3]));

    Ok(())
}

fn main() {
    match run() {
        Err(err) => panic!("{}", err),
        _ => (),
    };
}
