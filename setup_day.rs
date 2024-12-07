#!/usr/bin/env rust-script
use std::fs::{self, File};
use std::io::Write;
use std::path::Path;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let day = if args.len() > 1 {
        args[1].parse::<u32>().unwrap_or(0)
    } else {
        // If no argument provided, get the next day number based on existing files
        get_next_day_number()
    };

    let day_padded = format!("{:02}", day);
    
    // Create directories if they don't exist
    fs::create_dir_all("src").unwrap();
    fs::create_dir_all(format!("{}", day_padded)).unwrap();

    // Create the Rust source file
    let rs_path = format!("src/day{}.rs", day_padded);
    if !Path::new(&rs_path).exists() {
        let mut file = File::create(&rs_path).unwrap();
        write!(file, r###"pub fn part1(input: &str) -> String {{
    todo!("Implement part 1")
}}

pub fn part2(input: &str) -> String {{
    todo!("Implement part 2")
}}

#[cfg(test)]
mod tests {{
    use super::*;

    const EXAMPLE: &str = r#"
"#;

    #[test]
    fn test_part1() {{
        assert_eq!(part1(EXAMPLE.trim()), "");
    }}

    #[test]
    fn test_part2() {{
        assert_eq!(part2(EXAMPLE.trim()), "");
    }}
}}"###).unwrap();
    }

    // readme.md 만들기 // 자작 개조
    let rs_path = format!("{}/readme.md", day_padded);
    if !Path::new(&rs_path).exists() {
        let mut file = File::create(&rs_path).unwrap();
        write!(file, r####"## 제목




### 접근법




### 실행




## 파트 2




### 접근법




### 실행




"####).unwrap();
    }
    // Create the input file
    let input_path = format!("{}/{}-input.md", day_padded, day_padded);
    if !Path::new(&input_path).exists() {
        File::create(input_path).unwrap();
    }

    // Update lib.rs
    let lib_path = "src/lib.rs";
    let mut lib_content = if Path::new(lib_path).exists() {
        fs::read_to_string(lib_path).unwrap_or(String::new())
    } else {
        String::new()
    };

    // Check if the module is already declared
    let module_declaration = format!("#[path = \"day{}.rs\"]\npub mod day{};\n", day_padded, day_padded);
    if !lib_content.contains(&module_declaration) {
        if !lib_content.is_empty() && !lib_content.ends_with('\n') {
            lib_content.push('\n');
        }
        lib_content.push_str(&module_declaration);
        lib_content.push('\n');
        fs::write(lib_path, lib_content).unwrap();
    }

    println!("✨ Created files for Day {}:", day);
    println!("  → src/day{}.rs", day_padded);
    println!("  → {}/{}-input.md", day_padded, day_padded);
    println!("  → {}/readme.md", day_padded);
    println!("  → Updated src/lib.rs");
}

fn get_next_day_number() -> u32 {
    let src_dir = Path::new("src");
    if !src_dir.exists() {
        return 1;
    }

    let mut max_day = 0;
    if let Ok(entries) = fs::read_dir(src_dir) {
        for entry in entries {
            if let Ok(entry) = entry {
                let file_name = entry.file_name().to_string_lossy().to_string();
                if file_name.starts_with("day") && file_name.ends_with(".rs") {
                    if let Ok(day) = file_name[3..5].parse::<u32>() {
                        max_day = max_day.max(day);
                    }
                }
            }
        }
    }
    max_day + 1
}