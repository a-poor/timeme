use clap::{App, Arg};
use std::process::Command;
use std::time::{Duration, Instant};

const VERSION: &str = "0.2.0";

fn sum(data: &mut Vec<f64>) -> f64 {
    let mut sum: f64 = 0.0;
    for d in data {
        sum += *d;
    }
    sum
}

fn mean(data: &mut Vec<f64>) -> f64 {
    sum(data) / data.len() as f64
}

fn std(data: &mut Vec<f64>) -> f64 {
    let mean = mean(data);
    let total = data.iter().map(|d| (d - mean).powi(2)).sum::<f64>();
    (total / (data.len() as f64)).sqrt()
}

fn main() {
    let app = App::new("timeme")
        .version(VERSION)
        .author("Austin Poor <a-poor@users.noreply.github.com>")
        .about("Times the execution of a command.")
        .arg(Arg::new("number")
            .short('n')
            .long("number")
            .about("Number of times to run the command. Note: Set to <= 0 to run for at least 0.2s.")
            .default_value("0")
        )
        // .arg(Arg::new("output")
        //     .short('o')
        //     .long("output")
        //     .about("Display the output of the final run.")
        //     .required(true)
        //     .index(1)
        // )
        .arg(Arg::new("CMD")
            .about("The command to time")
            .required(true)
            .index(1)
        )
        .arg(Arg::new("ARGS")
            .about("The arguments to pass to CMD")
            .multiple_values(true)
            .required(false)
            .index(2)
        );

    // Parse the command line arguments
    let matches = app.get_matches();

    // Get the number of times to run the command
    let mut n: i32 = 0;
    if let Some(sn) = matches.value_of("number") {
        let new_n: i32 = sn.parse().unwrap();
        if new_n > 0 {
            n = new_n;
        }
    }
    let guess_time = n == 0;

    // Get the command to run
    let cmd_name = matches.value_of("CMD").unwrap();

    // Get the arguments to pass to the command
    let cmd_args: Vec<_> = matches.values_of("ARGS").unwrap().collect();

    // Build the command to run
    let mut cmd = Command::new(cmd_name);
    cmd.args(cmd_args);

    let mut times: Vec<Duration> = Vec::new();

    let mut n_loops = 0;
    if guess_time {
        let loop_start = Instant::now();
        while n_loops < 1 || loop_start.elapsed().as_secs_f64() < 0.2 {
            let start = Instant::now();
            let _output = cmd.output().expect("Error running the command");
            times.push(start.elapsed());
            n_loops += 1;
        }
    } else {
        for _ in 0..n {
            let start = Instant::now();
            let _output = cmd.output().expect("Error running the command");
            times.push(start.elapsed());
            n_loops += 1;
        }
    }

    // Convert times to float
    let mut ftimes: Vec<f64> = times.iter().map(|d| d.as_secs_f64()).collect();

    // Calculate mean & std
    let sec_avg = mean(&mut ftimes);
    let delta_avg = if sec_avg > 0. {
        Duration::from_secs_f64(sec_avg)
    } else {
        Duration::ZERO
    };

    let sec_std = std(&mut ftimes);
    let delta_std = if sec_std > 0. {
        Duration::from_secs_f64(sec_std)
    } else {
        Duration::ZERO
    };

    // Format the results
    println!(
        "{:?} (+/- {:?}) for {} loops",
        delta_avg, delta_std, n_loops
    );
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_sum() {
        let mut data: Vec<f64> = vec![];
        assert_eq!(0.0, super::sum(&mut data));

        let mut data: Vec<f64> = vec![1.0];
        assert_eq!(1.0, super::sum(&mut data));

        let mut data: Vec<f64> = vec![1.0, 2.5, 3.0];
        assert_eq!(6.5, super::sum(&mut data));

        let mut data: Vec<f64> = vec![-1.0, 0.0, 1.0];
        assert_eq!(0.0, super::sum(&mut data));
    }

    #[test]
    fn test_mean() {
        let mut data: Vec<f64> = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        assert_eq!(3.0, super::mean(&mut data));

        let mut data: Vec<f64> = vec![1.0];
        assert_eq!(1.0, super::mean(&mut data));
    }

    #[test]
    fn test_std() {
        let mut data: Vec<f64> = vec![1.0, 1.0, 1.0];
        assert_eq!(0.0, super::std(&mut data));

        let mut data: Vec<f64> = vec![1.0, 2.0];
        assert_eq!(0.5, super::std(&mut data));
    }
}
