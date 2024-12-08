pub fn 수리수리_다리_수리(입력: &str, 조건:&str) -> i128 {
    let mut 수식모음:Vec<(i128, Vec<i128>)> = Vec::new();
    입력.lines()
        .for_each(|한줄| {
            let mut 임시 = 한줄
                .split_ascii_whitespace();
            let 임시_결과값:i128 = 임시.next()
                .unwrap()
                .split(":")
                .next()
                .unwrap()
                .parse()
                .expect(&format!("{:?}에서 에러가 났네요", 한줄));
            let 임시_수식구성:Vec<i128> = 임시.into_iter()
                .map(|각_수|
                    각_수.parse().unwrap()
                ).collect();
            수식모음.push((임시_결과값, 임시_수식구성));
        });

    let mut 결과:i128 = 0;
    수식모음.iter().for_each(|수식| { 
        // println!("Expected Result: {} ---- Condition: {}\nGiven Numbers in order: {:?}", 수식.0, 조건, 수식.1);
        let mut 경우의수:i32 = 0;
        if 조건 == "이진" {
            경우의수 = 2_i32.pow((수식.1.len() as u32) - 1 );
        } else if 조건 == "삼진" {
            경우의수 = 3_i32.pow((수식.1.len() as u32) - 1 );
        }
        // println!("Total Possibilities are {}", 경우의수);
        
        let mut 한번만_더해야지:i128 = 0;
        
        'outer: for 이번_경우 in 0..경우의수 {
            let mut 진법화 = "".to_string();
            if 조건 == "이진" {
                진법화 = format!("{:0너비$b}", 이번_경우, 너비 = 수식.1.len() - 1);
            } else if 조건 == "삼진" {
                let 진법화_대상:경우의_수_변환기 = 경우의_수_변환기{가짓수:이번_경우};
                진법화 = format!("{:0>너비$}", 진법화_대상.삼진법(), 너비 = 수식.1.len() - 1);
            }

            // print!("P {} -> ", 진법화);
            
            let 예상결과 = 수식.1.iter()
                .fold(0, |mut 중간결과, 계산할_수| {
                    if 중간결과 == 0 {
                        // print!("{} ", 계산할_수);
                        중간결과 += 계산할_수
                    } else {
                        let 다음_계산 = 진법화.split_off(1);
                        match 진법화.as_str() {
                            "0" => {
                                // print!("+ {} ", 계산할_수);
                                중간결과 = 중간결과.checked_add(*계산할_수).expect(&format!("으악 숫자가 넘친다 -> {}", 수식.0));
                            }
                            "1" => {
                                // print!("* {} ", 계산할_수);
                                중간결과 = 중간결과.checked_mul(*계산할_수).expect(&format!("으악 숫자가 넘친다 -> {}", 수식.0));
                            }
                            "2" => {
                                // print!("| {} ", 계산할_수);
                                중간결과 = (중간결과*(10_i128.pow(계산할_수.ilog10()+1))) + 계산할_수 
                            }
                            _ => {}
                        }
                        진법화 = 다음_계산
                    }
                    중간결과
                });
            // print!("= {}", 예상결과);
            if 예상결과 == 수식.0 {
                // print!(" -> Perfect match\n");
                한번만_더해야지 = 예상결과;
                break 'outer;
            } else {
                // print!("\n")
            }
        }
        결과 += 한번만_더해야지 as i128;
        // println!("");
    });
    return 결과;
}


struct 경우의_수_변환기 {
    가짓수: i32
}

impl 경우의_수_변환기 {
    fn 삼진법(self) -> String {
        let mut 결과:String = "".to_string();
        let mut 몫 = self.가짓수;
        // println!("========== 변환할 숫자 = {} ==========", 몫);
        'outer: loop {
            let 나머지 = format!("{}", 몫%3);
            // print!("{}", 나머지);
            if 몫 / 3 < 1 {
                결과.insert(0,  나머지.chars().next().unwrap() );
                break 'outer;
            } else {
                결과.insert(0, 나머지.chars().next().unwrap() );
            }
            몫 /= 3;
        }
        return 결과;
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    const 예문: &str = r#"190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20"#;

    #[test]
    fn test_part1() {
        assert_eq!(수리수리_다리_수리(예문.trim(), "이진"), 3749);
    }

    #[test]
    fn test_part2() {
        assert_eq!(수리수리_다리_수리(예문.trim(), "삼진"), 11387);
    }
}