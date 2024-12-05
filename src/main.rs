use std::fs;
use advent_rust::케레스_수색;

fn main() {
    let 예문 = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";
    케레스_수색::저런_x_mas였다네요(&파일읽기(4));
}

fn 파일읽기(날짜:i32) -> String {
    let mut 날짜_숫자 = 날짜.to_string();
    if 날짜_숫자.len() == 1 {
        날짜_숫자 = "0".to_string()+&날짜_숫자;
    }
    let 파일경로:String = format!("{}/{}-input.md", 날짜_숫자, 날짜_숫자);
    let 내용 = fs::read_to_string(파일경로)
        .expect("파일이 읽히지 않아!");
    
    return 내용;
}

