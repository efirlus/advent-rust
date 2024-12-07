use std::fs;
use advent_rust::경비_피하기_경기;

fn main() {
    println!("{}", 경비_피하기_경기::경비_괴롭히기_경기(&파일읽기(6)));
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

