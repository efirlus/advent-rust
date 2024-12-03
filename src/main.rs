use std::fs;
use advent_rust::심사숙곱;

fn main() {
    let 예문 = "xmul(2,4)&do()mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
    println!("{}", 심사숙곱::심사숙재곱(&파일읽기(3)));
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

