## 제목

16년 25일차에 들렀던  부활절토끼비밀기지 옥상에 다시 떨어졌다

학자님들 노시는 동안 봐도 봐도 존나 큰 안테나를 다시 한 번 살펴봤더니
누군진 몰라도 부활절토끼들의 위장기업체인 이미테이션메디오크르 초콜렛을 크리스마스 선물로 구입할 마음을 0.1% 증가시키는 전파를 방출하도록 안테나를 조절해놨다

세상에나.......

주변 도시를 둘러보자 안테나가 여기저기 퍼져있는게 보인다. 각 안테나를 맵으로 보면
```
............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............
```

대충 이런데, 각 글자가 바로 주파수다 (대문자, 소문자, 숫자 가능)

안테나의 고유한 알고리즘, 공명주파수 기반 안티노드에 나쁜맘 생성 효과를 만들어주는 그런건데
같은 주파수의 안테나끼리의 거리의 2배 되는 지점에 안티노드가 형성된다

```
..........      ..........
...#......      ...#......
..........      #.........
....a.....      ....a.....
..........      ........a.
.....a....      .....a....
..........      ..#.......
......#...      ......#...
..........      ..........
..........      ..........
```
이런 느낌
안티노드는 안테나가 있는 그 자리에도 형성될 수 있다

결론적으로 위의 예시에서 안티노드는 이 정도다

```
0123456789ab
......#....#
...#....0...
....#0....#.
..#....0....
....0....#..
.#....A.....
...#........
#......#....
........A...
.........A..
..........#.
..........#.
```

자... 형성된 안티노드를 세보자




### 접근법

1. 인풋을 보니
생각보다 엄청 종류가 다양하다. 이건 무조건 struct가 필요해 보인다
여기에 해시셋도 하나 만들자, "주파수의 종류"를 체크해서 for를 돌리자

2. 보자... 각 주파수의 유니크 종류별로
해당 주파수의 모든 "쌍"의 종류를 구하고
(예를 들어 예제의 0는 4개니까 각각 ab, ac, ad, bc, bd, cd 총 6종, 이건 팩토리얼이다)

해당 쌍별로 좌표간 거리 (방향연산)를 구하고
그 자리가 맵 안인지 체크해서, hashset 하나 더 만들자

3. 거리연산은 어떻게 하느냐....
예를 들어 (4, 3) v (7, 2) 의 경우
좌우3 상하1 차이니까, (4,3) 기준 (1,4) // (7,2) 기준 (10,1) 이렇게 2개다

i32 음수도 들어가네, 그러면 "기준"을 기준으로 해서
4,3 - 7,2 = -3,1, 기호를 그대로 / 대상에게는 *-1
아니면 그냥 7,2 - 4,3 = 3,-1 이런 식으로 기준별역연산



### 실행

1. 한번에 엮을라캤는데 쉽게 안되네.... 하 for_each 두번 쓰는거 지겨워서 map을 좀 써보는데, 정말 쉽지 않아

2. 안되는 이유 추정
```rust
let 안테나들:Vec<안테나> = 입력.lines().enumerate().map(|(u, x)|
        x.chars().enumerate().filter(|(i, x)| *x != '.')
            .map(|x|안테나{위치:(x.0, u), 주파수:x.1} ).collect()
    ).collect();
```
지금 가만 보면, chars에서 collect하면 이게 "vec"으로 나오니까 vec<vec>이 되는거같은데...

현재 chars filter map 결과물이 `the trait "FromIterator<안테나>"`
니까 그 가설이 맞을듯

그럼 방법은, 안에서 생성된 Vec을 바깥에 "이어주는" 메소드를 찾아야 한큐가 된다는 거임

3. 밤중에 EXTEND라는 걸 보긴 했는데, 썩 아름답진 않게 응용됨, 난 안에서 콜렉, 밖에서 익스텐이 될 줄 알았지...

```rust
let mut 안테나들:Vec<안테나> = Vec::new();
    입력.lines().enumerate()
        .for_each(|(u, x)| {
            let 한줄씩:Vec<안테나> = x.chars().enumerate()
                .filter(|x| x.1 != '.')
                .map(|x|  안테나{위치:(x.0, u), 주파수:x.1} )
                .collect();
            안테나들.extend(한줄씩);
        });
```

4. 중요 포인트
안테나 목록은 "순서"가 아주 중요함
for 돌릴때마다 순서가 바뀌면 개낭패기 때문, 따라서 vec을 쓰는 게 맞음


5. 아 솔직히 이건 좀...
"마지막줄"이 안 세진다, 즉 세로 크기가 제대로 측정이 안됐단 이야기다
딱 한줄 문제인거같아서 그냥 >= 로 처리했다.. 하



## 파트 2

어깨 너머로 흘깃 처다보던 학자분 한분이 "혹시 공명배음 효과는 제대로 계산했느냐"고 넌지시 물었다

아뿔싸........

학자가 다르긴 하다

계산식을 업데이트 해보니, 안티노드는 두 안테나간 거리만큼 "여러차례" 발생한다

```
T....#....
...T......
.T....#...
.........#
..#.......
..........
...#......
..........
....#.....
..........
```

이런 느낌.
따라서 처음 예제는
```
##....#....#
.#.#....0...
..#.#0....#.
..##...0....
....0....#..
.#...#A....#
...#..#.....
#....#.#....
..#.....A...
....#....A..
.#........#.
...#......##
```

총 34곳이 되는거다
하아... 학교를 다시 다니든 해야지




### 접근법

1. 앞안테나 뒤안테나 계산 파트를 함수로 빼내자
그 다음에, 똑같은 함수지만 for를 넣은 걸 만들자

그래서 그냥 밖으로 나갈 때 까지 while을 돌리면 되겠다



### 실행

1. 덧셈뺄셈, 숫자 잘못, 역계산, 변수 실수야 하루 이틀 일이 아니라지만
이건 너무하네... 번역 실수라니 ㅋㅋㅋㅋㅋㅋㅋㅋㅋㅋㅋㅋㅋㅋㅋㅋ
"안테나 본인"도 포함된댄다 ㅋㅋㅋㅋㅋ
아 왜 안되나 했네



## 추가

1. 한큐 해결법
```rust
let 안테나들:Vec<안테나> = 입력.lines().enumerate()
        .flat_map(|(세로, 줄)| {
            줄.chars().enumerate()
                .filter(|(_, 글자)| *글자 != '.')
                .map(move |(가로, 주파수)| 안테나 {
                        위치:(가로 as i32, 세로 as i32), 주파수:주파수
                    })
        }).collect();
```
키포인트는 바깥의 flat, 안의 move
무브는 바깥에 선언된 변수를 안으로 들고 들어오는 것
flat은 안에서 나오는 이터레이터들의 "뭉치"를 한 이터레이터로 쫙 연결하는 것, 굿


