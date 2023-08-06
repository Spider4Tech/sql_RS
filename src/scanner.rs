extern crate rayon;

use rayon::prelude::*;

pub fn scanner<'a>(whole_file: &'a str, pattern: &'a str) -> Vec<Vec<&'a str>> {
    let lines: Vec<&str> = whole_file.par_lines().collect();
    let mut results: Vec<Vec<&'a str>> = Vec::new();

    for line in lines.iter() {
        if line.contains(pattern) {
            let mut group: Vec<&str> = Vec::new();
            group.push(line);

            // add next line until ;
            for next_line in lines.iter().skip_while(|&l| l != line).skip(1) {
                group.push(next_line);
                if next_line.trim_end().ends_with(';') {
                    break;
                }
            }

            results.push(group);
        }
    }

    results
}
