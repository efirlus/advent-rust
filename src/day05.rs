pub fn 프린터_대기열(입력:&str) -> i32 {
    let 페이지_정렬_규칙:Vec<Vec<_>> = 입력.lines()
        .filter(|줄|
            줄.contains("|")
        ).map(|줄|
            줄.split("|").collect()
        )
        .collect();
    let 출력할_안전_문서들:Vec<Vec<_>> = 입력.lines()
        .filter(|줄|
            줄.contains(",")
        ).map(|줄|
            줄.split(",").collect()
        )
        .collect();

    let mut 총점:i32 = 0;

    for 문서 in 출력할_안전_문서들 {
        let mut 딱_맞는_규칙:Vec<Vec<&str>> = Vec::new();
        let mut 규칙_적용_개수:usize = 0;

        
        for 규칙 in 페이지_정렬_규칙.clone() {
            if 규칙.iter().all(|수| 문서.contains(수)) {
                
                if 문서.iter().position(|페이지| 페이지 == &규칙[0]) < 문서.iter().position(|페이지| 페이지 == &규칙[1]) {
                    규칙_적용_개수+=1;
                }
                딱_맞는_규칙.push(규칙);
            }
        }
        if 딱_맞는_규칙.len() == 규칙_적용_개수 {
            // let 점수:i32 = 문서[문서.len()/2].parse().unwrap();
            // 총점 += 점수;
            continue;
        } else {
            // println!("{:?} 는 {}개의 규칙 중 {}개를 위반했네요! 고쳐볼까요?", 문서, 딱_맞는_규칙.len(), 딱_맞는_규칙.len() - 규칙_적용_개수);
            총점 += 시간이_조금_남는구만(문서, 딱_맞는_규칙);

        }
    }
    return 총점;
}

fn 시간이_조금_남는구만<'b>(mut 문서:Vec<&'b str>, 딱_맞는_규칙:Vec<Vec<&'b str>>) -> i32 {
    let mut 총점:i32 = 0;

    (문서, 총점) = 재정렬을_해볼까(문서, 딱_맞는_규칙);

    return 총점;
}

fn 재정렬을_해볼까<'a>(mut 문서:Vec<&'a str>, 딱_맞는_규칙:Vec<Vec<&'a str>>) -> (Vec<&'a str>, i32) {
    'outer: loop {
        let mut 규칙_개수:usize = 딱_맞는_규칙.len();
        // println!("현재 작업중인 문서는 {:?} 랍니다~", 문서);
        'inner: for (순번,  &ref 규칙) in 딱_맞는_규칙.iter().enumerate() {
            // println!("{}번째 규칙은 {:?}!", 순번+1, 규칙);
            let mut 다시_시작 = false;
            let mut 앞_숫자_자리 = 0;
            규칙.iter().enumerate().for_each(|숫자| {
                if 숫자.0 == 0 {
                    앞_숫자_자리 = 문서.iter().position(|&수| 수 == *숫자.1).expect("인덱스 못찾음");
                    // print!("앞 숫자 {}의 자리는 {} 이고... ", 숫자.1, 앞_숫자_자리);
                } else {
                    let 뒷_숫자_자리 = 문서.iter().position(|&수| 수 == *숫자.1).expect("인덱스 못찾음");
                    if 앞_숫자_자리 > 뒷_숫자_자리 {
                        문서.remove(뒷_숫자_자리);
                        문서.push(숫자.1);
                        다시_시작 = true;
                        // print!("뒷 숫자 {}의 자리는 {} 라서 이 규칙은 틀렸군요, 다시 시작하죠!\n", 숫자.1, 뒷_숫자_자리);
                    } else {
                        규칙_개수-=1;
                        // print!("뒷 숫자 {}의 자리는 {} 라서 이 규칙은 맞습니다!\n", 숫자.1, 뒷_숫자_자리);
                    }
                }
            });
            if 순번 == 딱_맞는_규칙.len() - 1 && 규칙_개수 == 0 {
                break 'outer;
            } else {
                if 다시_시작 {
                    break 'inner;
                } else {
                    continue;
                }
            }
        }
    }
    // println!("다시 완성된 문서는 {:?}에요!\n중간값은 {}번째 자리에 위치한 {}!!", 문서, 문서.len()/2+1, 문서[문서.len()/2]);
    let 총점 = 문서[문서.len()/2].parse::<i32>().unwrap();

    return (문서, 총점);
}