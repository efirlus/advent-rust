use std::fs;
use advent_rust::어느_역사학자의_우울;

fn main() {
    println!("{:?}", 어느_역사학자의_우울::우울한_역사학자들의_메모(파일읽기(1)));
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
