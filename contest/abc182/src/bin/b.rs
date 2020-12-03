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

fn solve(a: Vec<i32>) -> i32 {

    let mut result = 2;
    let mut count = 0;

    for i in 2..1001 {
        let mut sub_count = 0;
        for j in a.iter() {
            if (j >= &i) && (j % i == 0) {
                sub_count += 1;
            }
        }

        if sub_count > count {
            result = i;
            count = sub_count;
        }
    }

    result
}

fn run() -> Result<(), Box<dyn std::error::Error>> {

    let _ = utils::read_line::<i32>()?[0];
    let a = utils::read_line::<i32>()?;

    println!("{}", solve(a));

    Ok(())
}

fn main() {
    match run() {
        Err(err) => panic!("{}", err),
        _ => (),
    };
}
