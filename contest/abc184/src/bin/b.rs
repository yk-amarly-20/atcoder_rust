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
        let parsed_line = line
            .split_whitespace()
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

fn solve(initial_score: i32, x: &String) -> i32 {

    let mut score: i32 = initial_score;
    for c in x.chars() {
        if c == 'o' {
            score += 1;
        } else if c == 'x' && score > 0 {
            score -= 1;
        }
    }

    score
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let nx = utils::read_line::<i32>()?;
    let _ = nx[0];
    let x = nx[1];
    let s = &utils::read_line::<String>()?[0];

    println!("{}", solve(x, s));

    Ok(())
}

fn main() {
    match run() {
        Err(err) => panic!("{}", err),
        _ => (),
    };
}
