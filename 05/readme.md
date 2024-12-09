## 인쇄대기열

세레스에서 만족스러운 수색..유희를 즐긴 학자 군단은 이제 제17지하기지의 문구류 더미를 수색하자고 한다

북극의 인쇄청은 크리스마스 시즌에 무지막지하게 바쁜데, 그 와중에 역사학자들이 이 역사적으로 아주 중요한 시설을 뛰놀고,
그동안 (2017년 1일차에서 본) 아주 낯익은 프린터기를 관리하는 요정과 눈이 마주쳤다.

이 요정은 너무 바빠서 지금 문제가 무언지 설명할 틈 조차 없다. 이미 한 번 문제를 해결해 본 나를 찾는게 당연하댄다

썰매 발사 안전 규정의 업데이트 사항이 제대로 인쇄되지 않는 게 문제인데
안전규정은 반드시 특정 순서로 인쇄돼야 한다

`X|Y` 구문은 두 수가 모두 존재하 ㄹ경우, x가 먼저 인쇄돼야 한다는 이야기다

```룰
47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13
```

```인쇄
75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47
```

룰에 따르면, `75,47,61,53,29`는 맞게 인쇄됐다. 2, 3도 마찬가지
근데 4번, `75,97,47,61,53`는 틀렸다. 룰 중에 `97|75`를 어겨서 그렇다
5번도 `29|13`을 어겨서 실패, 6번도 `29|13`, `47|13`, `47|29`를 어겨서 실패다

결론적으론, 맞는 인쇄에서 한 가운데 수를 찾아서 모두 더한 값을 달라는 이야기다


### 접근
일단 2번 읽어서 앞은 페이지 규칙, 뒤는 인쇄물로 설정하고, 페이지규칙을 정렬한 뒤, 인쇄물을 읽으면서 각 줄을 모두 메모리에 얹고, 앞 뒤를 판단하게 한 다음
ok 뜨면 미들값 출력

여기서 궁금한 건, vec<vec>에 contains가 먹히냐는 것. 혹은 필터나

### 실행

1. 해보다 실패한 흔적
안전수칙을 만들면서 바로 규칙을 적용해 필터링할 수 있을 줄 알았다... 하지만 이 코드는 실패
```rust
    let 페이지_정렬_규칙:Vec<Vec<_>> = 입력.lines()
        .filter(|줄|
            줄.contains("|")
        ).map(|줄|
            줄.split("|").collect()
        )
        .collect();
    let 업데이트_안전_수칙:Vec<Vec<_>> = 입력.lines()
        .filter(|줄|
            줄.contains(",")
        ).map(|줄| {
            let 분할:Vec<_> = 줄.split(",").collect();
            let 알맞은_규칙:Vec<_> = Vec::new();
            for 수 in 분할 {
                알맞은_규칙.push(페이지_정렬_규칙.iter().filter(|숫자| 숫자.contains(&수)).collect());
            }
            println!("{:?}", 알맞은_규칙);
            return 알맞은_규칙;
        },
        
        )
        .collect();
```

2. 이번에도 고민 실패의 흔적
```rust
    for 수칙 in 업데이트_안전_수칙 {
        println!("이번 수칙은 {:?} 이에요!", 수칙);
        let mut 딱_맞는_규칙:Vec<Vec<&str>> = Vec::new();
        
        let test = 페이지_정렬_규칙.iter()
            .filter(|x| x.iter().all(|x| 수칙.contains(x)));
        println!("여기에 맞는 규정은 {:?} 이 있답니다!!", test);

/* 이걸 구현해보고 싶었는데, 위에 테스트가 안먹혔다 ... 필터링이 아예 안돼서...
        for 규칙 in 페이지_정렬_규칙.clone() {
            if 규칙.iter().all(|수| 수칙.contains(수)) {
                딱_맞는_규칙.push(규칙);
            }
        }
        println!("여기에 맞는 규정은 {:?} 이 있답니다!!", 딱_맞는_규칙);
 */
```

3. 3번째 도전 실패
아래껄 구현해볼라고 위에를 썼는뎨 잘 안됐다. 정렬규칙.clone.iter 까지만 하면 all(|수|) 가 Vec<str>로 처리된다.. 필요한 건 각 str인데...

```rust
        페이지_정렬_규칙.clone().iter().for_each(|규칙| &규칙.....);
            .all(|수| 문서.contains(수))
            .then(||
                문서.iter().position(|페이지| 페이지 == &규칙[0]) < 문서.iter().position(|페이지| 페이지 == &규칙[1])
            ).and_then(
                총점 += 문서[(문서.len()/2)+1]
            );
        
        
        for 규칙 in 페이지_정렬_규칙.clone() {
            if 규칙.iter().all(|수| 문서.contains(수)) {
                
                if 문서.iter().position(|페이지| 페이지 == &규칙[0]) < 문서.iter().position(|페이지| 페이지 == &규칙[1]) {
                    u+=1;
                }
                딱_맞는_규칙.push(규칙);
            }
        }
```

하여튼 그래도 첫날에 비하면 아래꺼도 충분히 rusti-sh 하다


## 파트2
요정놈들이 결과물 들고 희희낙락하면서 일하는 동안 시간이 좀 남았다고
틀린 목록을 고쳐보고자 한다... 아 벌써 좆되는데 ㅋㅋㅋㅋㅋㅋㅋㅋㅋㅋㅋㅋㅋㅋㅋㅋㅋㅋㅋㅋㅋ


### 시도
알고리즘이 제일 어렵다 진짜...


1. 첫시도
```rust
for (순번,  &ref 규칙) in  딱_맞는_규칙.iter().enumerate() {
    규칙.iter().enumerate().for_each(|숫자| {
        if 숫자.0 == 0 && !문서.contains(숫자.1) {
            문서.push(숫자.1);
            return 문서;
        } else if 숫자.0 == 1 && !문서.contains(숫자.1) {
            문서.push(숫자.1);
            return 문서;
        } else if 숫자.0 == 0 && 문서.contains(숫자.1) {
            return 문서;
        } else if 숫자.0 == 1 && 문서.contains(숫자.1) {
            문서.remove(문서.iter().position(|&수| 수 == *숫자.1).expect("인덱스 못찾음"));
            문서.push(숫자.1);
            return 문서;
        }
        println!("아직 안 만든 조건이 있네요!");
```
이건 그래도 제법 빠르게 구상했다고 할 수 있다
규칙 하나씩 빼서 인덱스 받아서 각 규칙 항마다 처리한다는 구상은 할 수 있지
이제 헤맸던 건, "처음부터 돌리기"를 어떻게 하냐였고, 그걸로 머리를 쥐어짜다가

2. 일단 완성
```rust
    'outer: loop {
        'inner: for (순번,  &ref 규칙) in  딱_맞는_규칙.iter().enumerate() {
            let mut 다시_시작 = false;
            let mut 자리수 = 0;
            규칙.iter().enumerate().for_each(|숫자| {
                if 숫자.0 == 0 {
                    if 문서.contains(숫자.1) {
                        // return ;
                    } else {
                        문서.push(숫자.1);
                    }
                    자리수 = 문서.iter().position(|&수| 수 == *숫자.1).expect("인덱스 못찾음"); 
                } else {
                    if 문서.contains(숫자.1) {
                        if 자리수 >  문서.iter().position(|&수| 수 == *숫자.1).expect("인덱스 못찾음") {
                            문서.remove(문서.iter().position(|&수| 수 == *숫자.1).expect("인덱스 못찾음"));
                            문서.push(숫자.1);
                            다시_시작 = true;
                        }
                    } else {
                        문서.push(숫자.1);
                    }
                }
            });
            if 순번 == 딱_맞는_규칙.len() - 1 {
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
```
이너 아우터 태그, 그리고 처음 써보는 loop가 점철된 새로운 알고리즘을 만든거지
이것도 여러번 갈아엎은 건데... 하여튼 퍼포먼스 적으로는 마음에 든다

3. 사실 헤멘 이유는
```rust
fn 재정렬을_해볼까<'a>(mut 문서:Vec<&'a str>, 딱_맞는_규칙:Vec<Vec<&'a str>>, mut 총점:i32) -> (Vec<&'a str>, i32) {
```
보이나 저 덕지덕지 붙은 달팽이와 `'a` 태그들?
도대체 이해가 안간다 저게 뭔지 아직도 모르겠다 ㅋㅋㅋㅋㅋㅋㅋㅋㅋㅋㅋㅋㅋㅋㅋㅋㅋㅋ

4. 총점 모으는 걸 어디서 하나로도 헤메네 ㅋㅋㅋㅋㅋㅋㅋㅋㅋㅋㅋㅋㅋㅋㅋㅋ
가장 위에서 모아야지 밑에서 모으면 0으로 리셋된다...

5. 이 알고리즘은 틀린거같은데.... 그래서 원본문서를 받아오는 걸로 했는데도 안된다, 아마 맞은 규칙 개수를 세봐야 알거같다

6. 드디어 성공시킨 inner 알고리즘
```rust
    let mut 다시_시작 = false;
    let mut 앞_숫자_자리 = 0;
    규칙.iter().enumerate().for_each(|숫자| {
        if 숫자.0 == 0 {
            앞_숫자_자리 = 문서.iter().position(|&수| 수 == *숫자.1).expect("인덱스 못찾음");
        } else {
            let 뒷_숫자_자리 = 문서.iter().position(|&수| 수 == *숫자.1).expect("인덱스 못찾음");
            if 앞_숫자_자리 > 뒷_숫자_자리 {
                문서.remove(뒷_숫자_자리);
                문서.push(숫자.1);
                다시_시작 = true;
            } else {
                규칙_개수-=1;
            }
        }
    });
```
주요 변경점이라면, push 파트를 아예 제거해버리고, 규칙이 맞을 때 마다 하나씩 차감해서 0일 때 outer 탈출시킨 것 정도
그 외엔 뭐가 바뀐지 모르겠는데 하여튼 승리했다 ㅋㅋㅋ



## 추가

ai가 준 대안

```rust

pub fn process_print_queue(input: &str) -> i32 {
    input.lines()
        .fold((Vec::new(), Vec::new(), 0), |(mut rules, mut docs, score), line| {
            match line {
                line if line.contains("|") => {
                    rules.push(line.split("|").collect::<Vec<_>>());
                    (rules, docs, score)
                },
                line if line.contains(",") => {
                    let document = line.split(",").collect::<Vec<_>>();
                    let new_score = process_document(&document, &rules);
                    (rules, docs, score + new_score)
                },
                _ => (rules, docs, score)
            }
        }).2  // Return final score
}

pub fn process_print_queue(input: &str) -> i32 {
    input.lines()
        .filter(|line| line.contains("|") || line.contains(","))
        .fold(
            (Vec::new(), 0), // (rules, total_score)
            |(mut rules, mut score), line| {
                if line.contains("|") {
                    rules.push(line.split("|").collect::<Vec<_>>());
                    (rules, score)
                } else {
                    let document: Vec<_> = line.split(",").collect();
                    let violated_count = rules.iter()
                        .filter(|rule| {
                            rule.iter().all(|page| document.contains(page)) &&
                            document.iter().position(|p| p == &rule[0]) >
                            document.iter().position(|p| p == &rule[1])
                        })
                        .count();
                    
                    (rules, score + if violated_count > 0 { 1 } else { 0 })
                }
            }
        )
        .1  // Get the score from the tuple
}
```

둘다 틀린 답이 나오지만 몇가지 배울 점이 있음
fold 안에 스코프를 열고 match, push 적용 하는 법
`.fold(먼저 출력형을 선언 |출력형의 변수명 지정| {수식})` 이라는 기본 구조를 배울 수 있었음