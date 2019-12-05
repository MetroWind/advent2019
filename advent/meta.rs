#![allow(non_snake_case)]

use std::fs;
use std::vec::Vec;
use std::path::Path;

extern crate proc_macro;
use proc_macro::TokenStream;

extern crate regex;
use regex::Regex;

struct Day
{
    mod_name: String,
    num: u8,
}

impl Day
{
    fn new(file: &Path) -> Result<Day, ()>
    {
        let day_file_pattern = Regex::new(r"^day(\d{2})\.rs$").unwrap();
        let basename = file.file_name().ok_or(())?.to_str().ok_or(())?;

        let day = Day
        {
            // filename: file.to_path_buf(),
            mod_name: String::from(&basename[..basename.len()-3]),
            num: day_file_pattern.captures(basename).ok_or(())?
                .get(1).ok_or(())?.as_str()
                .parse().map_err(|_| ())?
        };
        Ok(day)
    }

    fn vec_from_dir(dir: &str) -> Result<Vec<Day>, ()>
    {
        let days: Vec<Day> = fs::read_dir(dir).map_err(|_| ())?
            .map(|entry| Day::new(&entry.map_err(|_| ())?.path()))
            .filter(|day_maybe| day_maybe.is_ok())
            .map(|day_maybe|
                 {
                     let day = day_maybe.unwrap();
                     day
                 }).collect();
        Ok(days)
    }

}

#[proc_macro]
pub fn importAdvent(_: TokenStream) -> TokenStream
{
    let days = Day::vec_from_dir("advent").unwrap();
    let code: String = format!(
        "mod advent
{{
{}
}}",
        days.iter().map(|day| format!("    pub mod {};", day.mod_name))
            .collect::<Vec<String>>().join("\n"));

    println!("Days imported with\n\n{}\n", code);

    code.parse().unwrap()
}

fn matchPart(day: &Day, part: &str, input: &str) -> String
{
    format!("match {part}
    {{
        1 => advent::{day}::part1({input}).to_string(),
        2 => advent::{day}::part2({input}).to_string(),
        _ => panic!(\"There's no part {{}}.\", part),
    }}",
            day=day.mod_name, part=part, input=input)
}

#[proc_macro]
pub fn defineRun(_: TokenStream) -> TokenStream
{
    let day_var = "day";
    let part_var = "part";
    let input_var = "input";

    let days = Day::vec_from_dir("advent").unwrap();
    let code: String = format!(
        "fn run({day}: u8, {part}: u8, {input}: &str) -> String
{{
match {day}
{{
{body},
    _ => panic!(\"There's no day {{}}.\", {day}),
}}
}}",
        day=day_var,
        part=part_var,
        input=input_var,
        body=days.iter().map(
            |day| format!("    {} => {}",
                          day.num,
                          matchPart(&day, &part_var[..], &input_var[..])))
            .collect::<Vec<String>>().join(",\n"));

    println!("The run() function is defined as\n\n{}\n", code);
    code.parse().unwrap()
}
