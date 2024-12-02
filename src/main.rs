use std::fs;

fn main() {
    println!("{:?}", 어느_역사학자의_우울(파일읽기(1)));
}

fn 파일읽기(날짜:i32) -> String {
    let mut 날짜_숫자 = 날짜.to_string();
    if 날짜_숫자.len() == 1 {
        날짜_숫자 = "0".to_string()+&날짜_숫자;
    }
    let 파일경로:String = format!("{}/{}.md", 날짜_숫자, 날짜_숫자);
    let 내용 = fs::read_to_string(파일경로)
        .expect("파일이 읽히지 않아!");
    
    return 내용;
}

fn 어느_역사학자의_우울(input: String) -> i32 {
    let mut 왼쪽_메모: Vec<i32> = Vec::new();
    let mut 오른쪽_메모: Vec<i32> = Vec::new();

    let 한줄씩:Vec<&str> = input.split("\n").collect();
    for 줄 in 한줄씩 {
        let 숫자쌍:Vec<&str> = 줄.split(" ").collect();
        let 왼쪽_숫자:i32 = 숫자쌍[0].parse().unwrap();
        let 오른쪽_숫자:i32 = 숫자쌍[3].parse().unwrap();
        왼쪽_메모.push(왼쪽_숫자);
        오른쪽_메모.push(오른쪽_숫자);
    }

    왼쪽_메모.sort();
    오른쪽_메모.sort();

    let 총점 = 유사성_점수_구하기(왼쪽_메모, 오른쪽_메모);

    return 총점;
}

fn 차이점_점수_구하기(왼쪽_메모:Vec<i32>, 오른쪽_메모:Vec<i32>) -> i32 {
    let mut 총점:i32 = 0;
    for 숫자 in 0..왼쪽_메모.len() {
        let mut 차이:i32 = 왼쪽_메모[숫자] - 오른쪽_메모[숫자];
        if 차이 < 0 {
            차이 = 차이*-1;
        }
        총점 += 차이;
    }
    return 총점;
}

fn 유사성_점수_구하기(왼쪽_메모:Vec<i32>, 오른쪽_메모:Vec<i32>) -> i32 {
    let mut 총점:i32 = 0;
    

    for 숫자 in 왼쪽_메모 {
        let 이터레이터 = 오른쪽_메모.iter();
        let 개수 = 이터레이터.filter(|&x| *x == 숫자).count() as i32;
        총점 = 총점 + (숫자*개수);
    }
    return 총점;
}