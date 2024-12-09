use std::fs;
use advent_rust::선형_공명점;


fn main() {
    println!("{}", 선형_공명점::이미테이션_미디오커_초콜렛(&파일읽기(8)));
}

fn 파일읽기(날짜:i32) -> String {
    let 파일경로:String = format!("{:02}/{:02}-input.md", 날짜, 날짜);
    let 내용 = fs::read_to_string(파일경로)
        .expect("파일이 읽히지 않아!");
    
    return 내용;
}
