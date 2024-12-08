use std::fs;
use advent_rust::수리수리_다리_수리;


fn main() {
    println!("{}", 수리수리_다리_수리::수리수리_다리_수리(&파일읽기(7), "삼진"));
}

fn 파일읽기(날짜:i32) -> String {
    let 파일경로:String = format!("{:02}/{:02}-input.md", 날짜, 날짜);
    let 내용 = fs::read_to_string(파일경로)
        .expect("파일이 읽히지 않아!");
    
    return 내용;
}
