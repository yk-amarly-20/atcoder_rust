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

fn solve(n: i32) -> String {

    if n % 2 == 0 {
        "White".to_string()
    } else {
        "Black".to_string()
    }
}

fn run() -> Result<(), Box<dyn std::error::Error>> {

    let n = utils::read_line::<i32>()?[0];

    println!("{}", solve(n));

    Ok(())
}

fn main() {
    match run() {
        Err(err) => panic!("{}", err),
        _ => (),
    };
}
