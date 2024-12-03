use std::i32;

pub fn 붉은_코_보고서(입력: String) -> i32 {
    let mut 보고_수량:i32 = 0;
    let mut 불안전:i32 = 0;
    for 줄 in 입력.lines() {
        let 보고서 = 줄.split_ascii_whitespace();
        let mut 보고정보:Vec<i32>= vec![];
        for 숫자 in 보고서 {
            보고정보.push(숫자.parse().unwrap());
        }

        if !보고서_메뉴얼(보고정보) {
            불안전 += 1;
        }
        보고_수량 = 보고_수량 + 1;
    }

    println!("총 보고수량 = {보고_수량}\n위험 보고 = {불안전}\n안전 보고 = {}", 보고_수량 - 불안전);
    return 보고_수량 - 불안전;
}


// TODO: 불안한데/못참아 처리법은 여기엔 안통할것같다. 지금 보니 접근이 잘못됨
// 해야할 접근법은 전전수를 만들어서 만약 전전수랑 ㅇㅋ면 한 번 봐주는 방식을
// 해야하는 것 같다. 일단 자고 일어나자
pub fn 문제없음_처리장치(입력: String) -> i32 {
    let mut 보고_수량:i32 = 0;
    let mut 불안전:i32 = 0;
    for 줄 in 입력.lines() {
        let 보고서 = 줄.split_ascii_whitespace();
        let mut 보고정보:Vec<i32> = vec![];
        for 숫자 in 보고서 {
            보고정보.push(숫자.parse().unwrap());
        }

        if !보고서_메뉴얼(보고정보.clone()) {
            let mut 재검토 = 0;
            
            'outer: for 순서 in 0..보고정보.len() {
                let mut 문제없음_보고정보:Vec<i32> = vec![];
                for 내부순서 in 0..보고정보.len() {
                    if 내부순서 == 순서 {
                        continue;
                    } else {
                        문제없음_보고정보.push(보고정보[내부순서]);
                    }
                }

                if 보고서_메뉴얼(문제없음_보고정보.clone()) {
                    break 'outer;
                }
                재검토 += 1;
            }
            if 재검토 == 보고정보.len() {
                불안전 += 1;
            }
        }

        보고_수량 = 보고_수량 + 1;
    }

    println!("총 보고수량 = {보고_수량}\n위험 보고 = {불안전}\n안전 보고 = {}", 보고_수량 - 불안전);
    return 보고_수량 - 불안전;
}

fn 보고서_메뉴얼(보고정보:Vec<i32>) -> bool {
    let mut 이전_수:i32 = 0;
    let mut 직전_수차:&str = "";
    for 순서 in 0..보고정보.len() {
        if 순서 == 0 {
            이전_수 = 보고정보[순서];
            직전_수차 = ""
        } else {
            let 수차 = 이전_수 - 보고정보[순서];
            
            if (수차 == 0) || (수차 > 3) || (수차 < -3) || 
                ((0 < 수차) && (수차 <= 3) && 직전_수차 == "m") || 
                ((0 > 수차) && (수차 >= -3) && 직전_수차 == "p") {
                return false;
            }
            이전_수 = 보고정보[순서];
            if 수차.is_negative() {
                직전_수차 = "m";
            } else {
                직전_수차 = "p";
            }
        }
    }

    return true;
}