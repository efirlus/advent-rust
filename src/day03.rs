use regex::Regex;

pub fn 심사숙곱(입력: &str) -> i32 {
    let mut 결과값:i32 = 0;
    let 정규식 = Regex::new(r"mul\((?<첫수>\d{1,3}),(?<둘째수>\d{1,3})\)").unwrap();

    let 올바른_수식:Vec<(&str, &str)> = 정규식.captures_iter(입력).map(|수식| {
        let 첫수 = 수식.name("첫수").unwrap().as_str();
        let 둘째수 = 수식.name("둘째수").unwrap().as_str();
        (첫수, 둘째수)
    }).collect();

    for 수식 in 올바른_수식 {
        let  첫수:i32 = 수식.0.parse().unwrap();
        let 둘째수:i32 = 수식.1.parse().unwrap();
        결과값 += 첫수 * 둘째수;
    }
    return 결과값;
}

pub fn 심사숙재곱(입력: &str) -> i32 {
    let mut 결과값:i32 = 0;
    let 정규식 = Regex::new(r"mul\((?<첫수>\d{1,3}),(?<둘째수>\d{1,3})\)").unwrap();
    
    let mut 긍정_제어수식:Vec<_> = 입력.match_indices("do()").collect();
    let mut 부정_제어수식:Vec<_> = 입력.match_indices("don't()").collect();

    
    긍정_제어수식.append(&mut 부정_제어수식);
    긍정_제어수식.sort();

    let mut 올바른_수식:Vec<(&str, &str)> = 정규식.captures_iter(입력.split_at(긍정_제어수식[0].0).0).map(|수식| {
        let 첫수 = 수식.name("첫수").unwrap().as_str();
        let 둘째수 = 수식.name("둘째수").unwrap().as_str();
        (첫수, 둘째수)
    }).collect();
    let mut 다음_입력 = 입력.split_at(긍정_제어수식[0].0).1;

    let mut 이전_위치 = 긍정_제어수식[0].0;

    let mut 읽을까_말까 = false;
    if 긍정_제어수식[0].1 == "don't()" {
        읽을까_말까 = false;
    } else {
        읽을까_말까 = true;
    }
    for 수식 in &긍정_제어수식[1..] {
        let 자를_위치 = 수식.0 - 이전_위치;
        이전_위치 = 수식.0;
        let 탐색 = 다음_입력.split_at(자를_위치).0;

        if 읽을까_말까 {
            let mut 추가_수식:Vec<(&str, &str)> = 정규식.captures_iter(탐색).map(|수식| {
                let 첫수 = 수식.name("첫수").unwrap().as_str();
                let 둘째수 = 수식.name("둘째수").unwrap().as_str();
                (첫수, 둘째수)
            }).collect();
            올바른_수식.append(&mut 추가_수식);
        }
        if 수식.1 == "do()" {
            읽을까_말까 = true;
        } else {
            읽을까_말까 = false;
        }
        다음_입력 = 다음_입력.split_at(자를_위치).1;
    }
    if 읽을까_말까 {
        let mut 추가_수식:Vec<(&str, &str)> = 정규식.captures_iter(다음_입력).map(|수식| {
            let 첫수 = 수식.name("첫수").unwrap().as_str();
            let 둘째수 = 수식.name("둘째수").unwrap().as_str();
            (첫수, 둘째수)
        }).collect();
        올바른_수식.append(&mut 추가_수식);
    }

    for 수식 in 올바른_수식 {
        let  첫수:i32 = 수식.0.parse().unwrap();
        let 둘째수:i32 = 수식.1.parse().unwrap();
        결과값 += 첫수 * 둘째수;
    }

    return 결과값;
}