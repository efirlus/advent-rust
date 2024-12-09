## 다리고치기
22년 9일차에 봤던 눈에 익은 줄다리의 바로 그 정글에 도착했다
다리를 건너는 중에, 한 무리의 인부들이 다리를 고치고 있는 걸 발견했다. (당연하지만 이 다리는 심심하면 무너진다)

이걸 다 고쳐야 지나가겠구나 하는 걸 직감적으로 깨달은 나는 결국 이걸 도와주기로 했다

그런데 문제는 "지나가던 아기코끼리들"이 다리의 미세조정에 쓸 방정식에서 연산자를 몽땅 털어가버린 거다

```
190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20
```

결국 완성은 누가? 내가
일단 후보군을 좁히기 위해 코끼리들을 자세히 관찰하자, 그 녀석들이 갖고 노는 연산자가 +, * 2개 뿐인 걸 알 수 있다

인부들의 말로는 연산의 우선순위 같은 개념은 여기엔 통하지 않고, 오직 좌에서 우로만 계산을 한다고 한다

자, 예를 보면 1번은 당연하게 10*19다
두번째 건 원칙적으로는 81*40 --> + 27이어야 겠지만, 왼-오 룰에 따르면 그냥 81+40  --> * 27도 된다
나머진 조합이 안되고
쭉쭉 내려가서 마지막 건 어떤 식으로 메쳐봐도 11 + 6 --> * 16 --> + 20이어야만 된다

자, 인부들이 원하는 건 "가능한 조합이 하나라도 나오는 수"의 합만 있으면 된다고 한다...

구해보자

### 접근법
자.... 보자... 일단 수가 엄청 많다, 850줄 한줄에 5~12개의 랜덤 수가 있다

+를 0으로 *를 1로 잡고 세도 숫자가 11개면 연산을 1024번 해야된다... 와 이거 쉽지 않은데

2개는 거를 수 있다, all +랑 all *를 해서 최소수 최대수 처리를 하면 아웃라인을 좁힐 수 있다
예를 들어 16, 10, 13을 다 곱해도 20ㅌㅌ기 때문에 이건 더 안해봐도 탈락이다

나머지는, 음, 2^(list.len-1) 숫자를 만들어서 0부터 이 숫자까질 나열하고
해당 숫자를 2진코드화하고, 그걸 enumerate해서 쭉 for돌리자
내 머리론 안떠오른다



### 실행

1. 약간 헤메서 빡종하고 밥먹고 오긴 했지만, 그래도 비교적 빠르게 완성함
아직도 for_each를 쓸지, map이나 filter를 쓸지 헷갈림
그리고 솔직히 지금 enumerate 아예 쓸 필요 없는데...
>> 아 ㄹㅇ 안쓰고 바로 for_each 하니까 되네 ㅋㅋㅋ

```rust
let mut 수식모음:Vec<(i32, Vec<i32>)> = Vec::new();
    입력.lines()
        .for_each(|한줄| {
            let mut 임시 = 한줄
                .split_ascii_whitespace();
            let 임시_결과값:i32 = 임시.next()
                .unwrap()
                .split(":")
                .next()
                .unwrap()
                .parse()
                .unwrap();
            let 임시_수식구성:Vec<i32> = 임시.into_iter()
                .map(|각_수|
                    각_수.parse().unwrap()
                ).collect();
            수식모음.push((임시_결과값, 임시_수식구성));
        });
```
나중에 이걸, 좀 더 간결하게 만들 수 있는지 물어보자

2. 이제 이렇게 받은 수식 모음을 for 하......기 전에, 바이너리 만들기를 해볼까
>> 아 쉽네, "{}"에 저렇게 많은 포멧이 있는 걸 처음 알았다 ㅋㅋㅋㅋㅋㅋ
일단 2가진 기억할만 하다 
a. "{:0n}" = 숫자 자리수를 n으로 맞추기 위해 앞에 0을 붙여라
b. "{인수:바이너리}" = 인수를 바이너리로 적어라

3. 하, 참 나, i32보다 큰 수가 등장할 줄은 몰랐네 ㅋㅋㅋㅋ i64는 프로그래밍 인생에서도 처음 쓰는데

4. fold 가 덧셈만 하는게 아니네, 그냥 연쇄 수식에 다 적용되네 오늘처럼ㅋㅋㅋ
연쇄합, 연쇄곱 다 됨 굿

5. 새대가리
> [!quote]
> 나머지는, 음, 2^(list.len-1) 숫자를 만들어서
분명히 계획 세워놓고 뻘짓하고 앉았네 ㅋㅋㅋㅋㅋㅋㅋㅋㅋㅋㅋㅋ
자고 일어나보니 이모양이네 ㅋㅋㅋ어제밤엔 왜 안되지 하다 잤는데 ㅋㅋㅋㅋㅋㅋㅋ

6. format!의 단점(내가 모르는?)과 대안
`format!("{0길이b}", 변경할수)`가 안된다
원하는 길이에 맞게 0을 넣을 수가 없는 것
그래서 넉넉하게 024b 처리하고, split_off(24-원하는수)로 앞에서부터 잘랐다

7. for_each의 단점
아 이건 for랑 다른 점이 있네, 무조건 최후의 순간까지 달려야됨. break이 없음
try_for_each라는 메쏘드도 있긴 한데 이건 에러 계산에 관한거라 훨씬 어렵다

8. 당황스럽네 왜 안되지....
예문값은 분명하게 3749 딱 나와...
내머리로는 최소값 *전체합계* < 범위 < 최대값 *전체곱* 안에서만 추리면 될거같은데
뭐가 누락된거지....

9. 머리를 쥐어짜도 안돼서 ai한테 물어봄
ai도 알고리즘만으론 도무지 뭐가 문젠지 모르는 걸 보면, 이번엔 알고리즘만큼은 훌륭하다는 걸 알 수 있다, 문제는 문법적 이슈 같은데
---
씨발 개빡쳐서 그냥 풀로 돌리기로 함, ai도 나도 2시간을 씨름해도 답이 안나옴 ㅋㅋㅋㅋㅋㅋㅋㅋㅋ
코드는 아까우니까 저장해두자 
```rust
pub fn 수리수리_다리_수리(입력: &str) -> i128 {
    // 일단 입력값 ㅇㅇㅇㅇ: ㅌㅌ ㅊㅊ ㅍㅍ ㅁㅁ를 Vec(ㅇㅇㅇㅇ [ㅌㅌ ㅊㅊ ㅍㅍ ㅁㅁ]) 형태로
    // println!("입력받은 수식은 아래와 같아요!\n{입력}");
    let mut 수식모음:Vec<(i64, Vec<i64>)> = Vec::new();
    입력.lines()
        .for_each(|한줄| {
            let mut 임시 = 한줄
                .split_ascii_whitespace();
            let 임시_결과값:i64 = 임시.next()
                .unwrap()
                .split(":")
                .next()
                .unwrap()
                .parse()
                .expect(&format!("{:?}에서 에러가 났네요", 한줄));
            let 임시_수식구성:Vec<i64> = 임시.into_iter()
                .map(|각_수|
                    각_수.parse().unwrap()
                ).collect();
            수식모음.push((임시_결과값, 임시_수식구성));
        });
    // println!("모은 숫자들은 어떻게 됐나 볼까요?\n{:?}",수식모음);

    // 본격적인 결론부인데, 일단 각 수식에 대해 나누고
    let mut 결과:i128 = 0;
    수식모음.iter().for_each(|수식| { 
        // for 를 따로 하지 말고 한번에 lazy로 돌려보자
        println!("Expected Result: {}\nGiven Numbers in Order: {:?}", 수식.0, 수식.1);
        let 모든_수_합 = 수식.1.iter()
            .fold(0, |mut 합, 각_수| {
                합 += 각_수;
                합
            });
        let 모든_수_곱:i64 = 수식.1.iter()
            .fold(1, |mut 곱:i64, 각_수| {
                곱*=각_수;
                곱
            });
        if 수식.0 < 모든_수_합 || 수식.0 > 모든_수_곱 {
            println!("Result {} outs boundary of {} to {}\n\n", 수식.0, 모든_수_합, 모든_수_곱);
        } else if 수식.0 == 모든_수_합 {
            println!("Result {} was All-Addition of {:?}\n\n", 수식.0, 수식.1);
            결과 += 모든_수_합 as i128;
        } else if 수식.0 == 모든_수_곱 {
            println!("Result {} was All-Multiplication of {:?}\n\n", 수식.0, 수식.1);
            결과 += 모든_수_곱 as i128;
        } else {
            //쉬운계산 끝, 이제 어려운 계산 ㄱ
            println!("Result {} is in the boundary of {} to {}", 수식.0, 모든_수_합, 모든_수_곱);

            // 일단 이진법으로 경우의 수를 만들고
            let 경우의수:i32 = 2_i32.pow((수식.1.len() as u32) - 1 );
            let mut 한번만_더해야지:i64 = 0;
            if 경우의수 > 2 {
                'inner: for 이번_경우 in 1..경우의수-1 {
                    let mut 이진수 = format!("{:0width$b}", 이번_경우, width = 수식.1.len() - 1);
                    print!("Possibility {} ->", 이진수);
                    
                    // 예상결과 수집
                    let 예상결과 = 수식.1.iter()
                    // 숫자목록을 이터레이션해서, fold로 넘기고
                    .fold(0, |mut 중간결과, 계산할_수| {
                        // 만약 이게 첫 수일 때 처리하고
                        if 중간결과 == 0 {
                            print!(" {}",계산할_수);
                            중간결과 += 계산할_수
                            
                        } else {
                            // 둘째 수부터, 연산자를 하나씩 꺼내가지고
                            let 다음_계산 = 이진수.split_off(1);
                            // 매치시킨 다음에
                            match 이진수.as_str() {
                                "0" => {
                                    print!(" + {}", 계산할_수);
                                    중간결과 = 중간결과.checked_add(*계산할_수).expect(&format!(" -> overflow here!"));
                                }
                                "1" => {
                                    print!(" * {}", 계산할_수);
                                    중간결과 = 중간결과.checked_mul(*계산할_수).expect(&format!(" -> overflow here!"));
                                }
                                _ => {}
                            }

                            // split_off 에 대한 남은 처리 잊지 말고
                            이진수 = 다음_계산
                        }
                        // 리턴은 중간결과, 이것만 리턴하면 알아서 fold해줌
                        중간결과
                    });
                    print!(" = {}", 예상결과);
                    if 예상결과 == 수식.0 {
                        print!(" -> Perfect Match!!");
                        한번만_더해야지 = 예상결과;
                    }
                    print!("\n")
                }
            }
            결과 += 한번만_더해야지 as i128;
            print!("\n")
        }
    });
    return 결과;
}
```

10. 571 차이

```rust
pub fn 수리수리_다리_수리(입력: &str) -> i128 {
    // 일단 입력값 ㅇㅇㅇㅇ: ㅌㅌ ㅊㅊ ㅍㅍ ㅁㅁ를 Vec(ㅇㅇㅇㅇ [ㅌㅌ ㅊㅊ ㅍㅍ ㅁㅁ]) 형태로
    // println!("입력받은 수식은 아래와 같아요!\n{입력}");
    let mut 수식모음:Vec<(i64, Vec<i64>)> = Vec::new();
    입력.lines()
        .for_each(|한줄| {
            let mut 임시 = 한줄
                .split_ascii_whitespace();
            let 임시_결과값:i64 = 임시.next()
                .unwrap()
                .split(":")
                .next()
                .unwrap()
                .parse()
                .expect(&format!("{:?}에서 에러가 났네요", 한줄));
            let 임시_수식구성:Vec<i64> = 임시.into_iter()
                .map(|각_수|
                    각_수.parse().unwrap()
                ).collect();
            수식모음.push((임시_결과값, 임시_수식구성));
        });
    // println!("모은 숫자들은 어떻게 됐나 볼까요?\n{:?}",수식모음);

    // 본격적인 결론부인데, 일단 각 수식에 대해 나누고
    let mut 결과:i128 = 0;
    수식모음.iter().for_each(|수식| { 
        // for 를 따로 하지 말고 한번에 lazy로 돌려보자
        // println!("Expected Result: {}\nGiven Numbers in order: {:?}", 수식.0, 수식.1);
        // 일단 이진법으로 경우의 수를 만들고
        let 경우의수:i32 = 2_i32.pow((수식.1.len() as u32) - 1 );
        let mut 한번만_더해야지:i64 = 0;
        
        for 이번_경우 in 0..경우의수 {
            let mut 이진수 = format!("{:0width$b}", 이번_경우, width = 수식.1.len() - 1);
            // print!("Possibility {} ->", 이진수);
            
            // 예상결과 수집
            let 예상결과 = 수식.1.iter()
            // 숫자목록을 이터레이션해서, fold로 넘기고
            .fold(0, |mut 중간결과, 계산할_수| {
                // 만약 이게 첫 수일 때 처리하고
                if 중간결과 == 0 {
                    // print!(" {}",계산할_수);
                    중간결과 += 계산할_수
                    
                } else {
                    // 둘째 수부터, 연산자를 하나씩 꺼내가지고
                    let 다음_계산 = 이진수.split_off(1);
                    // 매치시킨 다음에
                    match 이진수.as_str() {
                        "0" => {
                            // print!(" + {}", 계산할_수);
                            중간결과 = 중간결과.checked_add(*계산할_수).expect(&format!(" -> overflow here!"));
                        }
                        "1" => {
                            // print!(" * {}", 계산할_수);
                            중간결과 = 중간결과.checked_mul(*계산할_수).expect(&format!(" -> overflow here!"));
                        }
                        _ => {}
                    }

                    // split_off 에 대한 남은 처리 잊지 말고
                    이진수 = 다음_계산
                }
                // 리턴은 중간결과, 이것만 리턴하면 알아서 fold해줌
                중간결과
            });
            // print!(" = {}", 예상결과);
            if 예상결과 == 수식.0 {
                한번만_더해야지 = 예상결과;
            }
            // print!("\n")
        }
        if 한번만_더해야지 != 0 {
            println!("{}: {:?} -> Perfect Match!!", 수식.0, 수식.1);
        }
        결과 += 한번만_더해야지 as i128;
        // print!("\n")
    });
    return 결과;
}
```
답은 6392012777720 라고 한다..
뭘 놓쳤을까 ㅋㅋㅋㅋㅋㅋㅋㅋㅋㅋㅋㅋㅋㅋㅋㅋ

6392012777149랑 비교해서 어떤 수가 미스난건지 알고 싶은데 진짜 ㅋㅋㅋㅋㅋㅋㅋㅋㅋㅋㅋㅋ
---
하나 놓쳤네 ㅋㅋㅋㅋㅋㅋㅋㅋㅋㅋㅋㅋㅋㅋㅋㅋㅋㅋㅋㅋㅋㅋㅋㅋㅋㅋㅋㅋㅋㅋㅋㅋㅋ
`571: [1, 447, 124]`
"1*447+124" 상황에서 전체합보다 1 작댄다 미친



## 파트 2
아 존나 힘든 고난 끝에 답을 줬더니 기준치에 한참 미달이라고 걱정이다
그 때, 내 눈에 보여선 안될, 코끼리 코를 닮아서 안보였던 연산자가 보였다

`||` 연산자는 concatenation을 시행한다
`12 || 345 = 12345`다

자, 이제 나머진 모두 같다고 칠 때, 답은 무엇일까.......



### 접근법

접근법 자체만 보면 이건 쉽다 비교적.
2진법을 3진법으로 고치고, '2'일 때, 대강 이런 알고리즘을 넣으면 된다

```rust
중간결과 = 중간결과 * 10.pow(계산할수.len()) + 계산할수
```
이제 문제는 3진법 구현이지




### 실행

1. 눈에 확 띄는 발견 `.ilog10()`
수학 배운 건 이럴 때 쓰는거임
```rust
중간결과 = (중간결과*(10_i128.pow(계산할_수.ilog10()+1))) + 계산할_수 
```
깔끔한 구현



2. 3진수 만들기...
물어본 바, 찾아본 바 그런 함수는 없다. ternary operator라고 "a:b?c" 그런거만 나온다

함수를 직접 만들어보자
이 참에 method 함수를 만드는 것도 좋은 선택일듯?

pow()처럼, 24.ternary() 이런 식으로

3. impl
메쏘드는 기존형 자료에는 추가가 안되는 걸로 보임
지금 고르기 가장 좋은 방식은 struct로 감싼 다음 그걸 impl하는 법
rust의 모든 type은 기본적으로는 노란색(즉 struct)로 눈에 보이니까 딱히 틀린 짓은 아님

자 이제 문제는 알고리즘인데....
아까 잠깐 본게 insert였는데

---
docs에 따르면 string.insert(usize, 'char') 방식이다
근데 한글자 string을 char로 만드는 법을 몰라서 결국 연산을 낭비했다

```rust
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
```

---
아 근데 이게 생각보다 지저분함
```rust
let 진법화_대상:경우의_수_변환기 = 경우의_수_변환기{가짓수:이번_경우};
진법화 = format!("{:0>너비$}", 진법화_대상.삼진법(), 너비 = 수식.1.len() - 1);
```
method를 만든다는 아이디어는 좋았지만, 깨끗하진 않았다 정도로...




## 요약

1. format!("{}")의 다양성
일단 배운것들은 이렇다
- {:b} == 2진수
- {:023} == 0이 23개 앞에 붙어서 자릿수 처리
- {:0변수$b}, 변수 = usize 숫자 == 변수값 만큼 0 추가가
- {:x>5} == x를 인쇄할 변수 앞에 채워서 5자리 만들기

2. fold의 유용성
.fold( 변수의 시작형 선언 | 변수명 여러개 | { 코드 } )
이렇게 누적으로 유용한 함수는 못봤다. 코드 맨 마지막에 반환형을 시작형하고 맞춰서 딱 한 줄 선언하면 끝이다

3. for_each에는 break이 없다.. 너무 아쉽다

4. 수학적 알고리즘의 세계는 오묘하다
4시간을 알고리즘 때문에 내다 버렸다

5. 그래도 배운 것 안에선 허투루 쓰지 않는다
ilog10 같이 자리수 파악에 유용한 메소드도 발견했다

6. 3진수 만들기...
물어본 바, 찾아본 바 그런 함수는 없다. ternary operator라고 "a:b?c" 그런거만 나온다
그래서 method 함수를 만들어보고 싶었다. pow()처럼, 24.ternary() 이런 식으로
그런데 메쏘드는 기존에 존재하는 자료형에는 추가가 안되는 걸로 보임

내가 고른 유일한 아는 법은, 6일차에 default를 선언해본 것에 따라
struct로 감싼 다음 그걸 impl하는 법

```rust
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
```

근데 이 코드나, 사용 방식이나 여러모로 내 예상보다 지저분했다

```rust
let 진법화_대상:경우의_수_변환기 = 경우의_수_변환기{가짓수:이번_경우};
진법화 = format!("{:0>너비$}", 진법화_대상.삼진법(), 너비 = 수식.1.len() - 1);
```
method를 만든다는 아이디어는 좋았지만, 깨끗하진 않았다 정도로...



## 추가
```rust
// Define a trait for ternary conversion
pub trait ToTernary {
    fn ternary(&self) -> String;
}

// Implement for i32
impl ToTernary for i32 {
    fn ternary(&self) -> String {
        if *self == 0 {
            return "0".to_string();
        }
        
        let mut num = *self;
        let mut result = String::new();
        
        while num > 0 {
            result.insert(0, char::from_digit((num % 3) as u32, 10).unwrap());
            num /= 3;
        }
        
        result
    }
}

// Usage example:
fn main() {
    let number = 42;
    println!("{} in ternary: {}", number, number.ternary());  // Will print: 42 in ternary: 1120
    
    // You can use it directly in expressions
    let ternary_string = 15.ternary();  // Will return: "120"
}
```

1.
trait라는 걸 ai가 알려줬다. 자주 써먹게 될 거 같다

2.
`char::from_digit( 들어갈 숫자 , 자릿수 ).unwrap(에러핸들링);`
숫자가 char로 변한다고 한다
insert랑 엮어서 while을 돌리는게 핵심