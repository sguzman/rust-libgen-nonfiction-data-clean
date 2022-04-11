extern crate cached;

use cached::proc_macro::cached;

#[cached]
fn load_file(file: String) -> String {
    use std::fs;

    fs::read_to_string(file)
        .expect("Could not open file")
}

fn split_at_nl(content: &String) -> Vec<&str> {
    content.lines().collect()
}

fn count_commas_in_line(line: &str) -> usize {
    line.matches(",").count()
}


fn print(ss: &str) {
    if count_commas_in_line(&ss) == 46 {
        println!("{}", ss);
    }
}

fn args() -> String {
    let args: Vec<String> = std::env::args().collect();

    args[1].clone()
}

fn main() {
    let arg: String = args();
    let content: String = load_file(arg);
    let lines: Vec<&str> = split_at_nl(&content);

    println!("{}", "hi");
    for l in lines {
        print(&l);
    }
    println!("{}", "bye");
}
