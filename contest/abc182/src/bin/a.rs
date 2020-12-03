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

fn solve(a: i32, b: i32) -> i32 {

    let max_follower_num = 2 * a + 100;

    max_follower_num - b
}

fn run() -> Result<(), Box<dyn std::error::Error>> {

    let v = utils::read_line::<i32>()?;
    let a = v[0];
    let b = v[1];

    println!("{}", solve(a, b));

    Ok(())
}

fn main() {
    match run() {
        Err(err) => panic!("{}", err),
        _ => (),
    };
}
