어제 거는 15년 19일차 게 나오더니 이번엔 20년 2일차가 나오네 ㅋㅋㅋㅋㅋ
북극토보간(납작한 다인승 나무썰매) 대여점이 3일차의 수색처다.

학자들이 수색을 진행하는 동안 나는 가게 주인장의 부탁이나 들어주자
포스기 프로그램이 뭔가 에러가 난다길래 확인해 보니 메모리가 손상돼서 알고리즘이 뒤죽박죽이 됐다

이 프로그램은 원래는 mul(n,m) 형식으로 n이랑 m을 곱하는 프로그램이다
근데 무언가가 꼬여서 가짜 수식이 늘어났다고 한다

오직 `mul(n,m)` 이외에 그 어떤 문자도, 심지어 스페이스라도 있으면 전부 가짜 수식이다

`xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))`

이 예에선 
mul(2,4)
mul(5,5)
mul(11,8)
mul(8,5)
이렇게 4개만 맞는 식이다

이 답들을 합치는게 문제


### 접근법
그냥 regex 쓰면 될 거 같은데... rust는 정규식이 std가 아니네 ㅅㅂㅋㅋㅋㅋㅋ
크레이트 추가하지 뭐, 일이 편할거같은데


### 1. 타입타입타입
- String으로 읽는 인풋이 참 애물단지였는데, 함수(&인풋) 하면 저절로 &str로 변한다는 걸 알게 됨. 굿
- 근데 왜케 analyzer가 말썽이냐 ㅅㅂ;

### 2. 문서화 하나는 일등공신
- 개인적으로 이 부분이랑 rust-analyzer 두개는, 그래도 장점이 많음. 문서화가 철저하고 예문이 많아서 에지간하면 읽어보면 응용 가능함.. 아직 regex를 숫자로 받는 건 못하는 중이긴 한데;;

## 파트2
개간나 새끼.... 이러면 정규식이 안되네 ㅋㅋㅋㅋㅋㅋㅋㅋㅋㅋㅋㅋㅅㅂ 꼼수금지

`do()`라는 구ㅜㅁㄴ이 앞에 등장하면 계산 하고, `don't()` 구문이 등장하면 다음 do가 등장할 때 까지 멈춰야 된다

`xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))`

이 예문에서 맨앞에 mul(2,4) 는 ㅇㅋ, 그 뒤에 don't() 때문에 mul(5,5) 랑 mul(11,8)는 무시, do() 다음에 나온 mul(8,5)는 다시 ㅇㅋ다

와 이거 쉽지가 않은데...

일단 regex crate를 잘 읽어보니까 `RegexSet`이라는 게 있다. 한 번 보자
-- 개뿔 아무것도 안되어서 그냥 밀어버림
이게 안좋은 건 아님, 단지 "각 매치별로 하나씩만" 찾아줌 ㅅㅂ;;


다음 찾아본 건 match_indices 라는 기본 함수임

먼저 각 매칭의 첫 튜플을 꺼내서 값을 비교한 후 다음 것부터 돌리면 될듯

---

이야 ㅋㅋㅋㅋㅋㅋㅋㅋㅋㅋㅋㅋ 러스트 좋네 ㅋㅋㅋㅋㅋㅋㅋㅋㅋㅋㅋㅋㅋ
vec.append().sort() 하면 그게 (i, str) 형식 이라도 i 기준으로 정렬해줌 ㅋㅋㅋㅋㅋㅋㅋㅋㅋㅋㅋㅋㅋ아 좋다 ㅋㅋㅋㅋㅋㅋㅋㅋㅋㅋㅋㅋㅋㅋㅋㅋ

그나저나 문법 존나 적응 안되네 ㅋㅋㅋ `vec[1..4]` 이런 식임, `:` 아님

---

씨벌 ㅋㅋㅋㅋㅋㅋㅋㅋㅋㅋㅋ for 안에 if continue 하면 for 전체를 한바퀴 캔슬시켜버리는 걸 몰랐네 아 난 그 밑에는 읽을 줄 알았지 ㅅㅂㅋㅋㅋㅋㅋㅋ
하여튼 해결한 김에 오늘의 총정리를 해보자


1. for 안에 if continue 하면 for 전체를 한바퀴 캔슬시켜버리는 걸 몰랐네 아 난 그 밑에는 읽을 줄 알았지

2. 일단 가장 훌륭한 부분은 regex crate의 태그 기능이다. `(?<태그이름>\패턴)`으로 처리하면 `$1` 처럼 일일이 숫자를 셀 필요가 없어짐. 이건 아주 장점임
태그 이름은 `매치.name("태그이름")`으로 꺼내서 필요에 따라 variable로 저장할 수 있는거지
또 유의사항은, 반드시 unwrap 해줘야 한다는 거, 왜인지는 모르겠네

3. 튜플의 문법이 상상외로 간결한 것도 놀라움. 튜플.0 1 2 이렇게 쓰면 됨

4. 오늘 날 가장 골머리 썩게 한 건 match_indice. 
regex 적용은 안되지만 일치패턴의 "주소"를 줌, 이건 좋아보임. 천만다행으로 `do()` & `don't()`는 regex없이 매칭 가능
결과값은 벡터로 나오고, 벡터 안에 "주소"와 "매칭값"이 튜플로 묶여서 한 튜플이 한 엘레멘트임
오늘은 했지만 다음에 match 안 먹히는 건 어떻게 될지 모름 ㅋㅋㅋㅋㅋㅋㅋㅋㅋㅋㅋㅋㅋㅋ

5. vec.append().sort() 하면 그게 (i, str) 형식 이라도 i 기준으로 정렬해 주는 건 참 신기함, 어떻게 tuple 내부값을 정렬해주지


