#[derive(Clone)]
struct 경비_소녀 {
    위치:(i32, i32),
    방향:char,
}

impl Default for 경비_소녀 {
    fn default() -> Self {
        Self{위치:(0,0), 방향:'중'}
    }
}


fn 지도_읽기(입력: &str) -> (경비_소녀, Vec<(i32, i32)>, (i32, i32)) {
    let mut 경비:경비_소녀 = 경비_소녀::default();
    let mut 장애물들:Vec<(i32, i32)> = Vec::new();
    let mut 지도_크기:(i32, i32) = (0,0);
    입력.lines().enumerate()
        .for_each(|한줄씩| {
            한줄씩.1.chars().enumerate()
                .for_each(|한글자씩| {
                    match 한글자씩.1 {
                        '#' => {
                            // println!("({},{})에는 장애물이 있네요!", 한줄씩.0, 한글자씩.0);
                            장애물들.push((한줄씩.0 as i32, 한글자씩.0 as i32));
                        },
                        '^' => {
                            경비.위치 = (한줄씩.0 as i32, 한글자씩.0 as i32);
                            경비.방향 = '상'
                        },
                        '<' => {
                            경비.위치 = (한줄씩.0 as i32, 한글자씩.0 as i32);
                            경비.방향 = '좌'
                        },
                        '>' => {
                            경비.위치 = (한줄씩.0 as i32, 한글자씩.0 as i32);
                            경비.방향 = '우'
                        },
                        'v' => {
                            경비.위치 = (한줄씩.0 as i32, 한글자씩.0 as i32);
                            경비.방향 = '하'
                        },
                        _ => {},
                    };
                    지도_크기 = (한줄씩.0 as i32, 한글자씩.0 as i32)
                });
        });

    return (경비, 장애물들, 지도_크기);
}

fn 경비_메뉴얼(mut 경비:경비_소녀, 지시사항:&str) -> 경비_소녀 {
    match 경비.방향 {
        '상' => {
            match 지시사항 {
                "전진" => {
                    경비.위치.0-=1;
                }
                "회전" => {
                    경비.방향 = '우';
                    경비.위치.0+=1;
                    경비.위치.1+=1;
                }
                _ => {}
            }
        }
        '하' => {
            match 지시사항 {
                "전진" => {
                    경비.위치.0+=1;
                }
                "회전" => {
                    경비.방향 = '좌';
                    경비.위치.0-=1;
                    경비.위치.1-=1;
                }
                _ => {}
            }
            
        }
        '좌' => {
            match 지시사항 {
                "전진" => {
                    경비.위치.1-=1;
                }
                "회전" => {
                    경비.방향 = '상';
                    경비.위치.0-=1;
                    경비.위치.1+=1;
                }
                _ => {}
            }
            
        }
        '우' => {
            match 지시사항 {
                "전진" => {
                    경비.위치.1+=1;
                }
                "회전" => {
                    경비.방향 = '하';
                    경비.위치.0+=1;
                    경비.위치.1-=1;
                }
                _ => {}
            }
        }
        _ => {}
    }
    return 경비;
}

pub fn 경비_피하기_경기 (입력:&str) -> i32 {
    let (mut 경비, 장애물들, 지도_크기) = 지도_읽기(입력);
    // prinln!("이 지도는 총 가로 {:?} 칸, 세로 {:?} 칸이에요!", 지도_크기.1+1, 지도_크기.0+1);
    // println!("작은 경비소녀는 {:?}에 서서 {}측 방향을 보고 있구요!", 경비.위치, 경비.방향);
    // println!("장애물은 총 {} 곳에 있고, 위치는 차례대로 {:?}에요!", 장애물들.len(), 장애물들);

    let mut 방문한_장소 = Vec::new();
    
    'outer: loop {
        if 경비.위치.0 < 0 || 경비.위치.1 < 0 || 경비.위치.0 > 지도_크기.0 || 경비.위치.1 > 지도_크기.1 {
            break 'outer;
        } else if !장애물들.contains(&경비.위치) {
            // println!("{:?} : 이 곳엔 아무것도 없어요! {:?}측으로 전진합니다!!", &경비.위치, &경비.방향);
            경비 = 경비_메뉴얼(경비.clone(), "전진");
            let 이번_방문_장소 = 경비.위치.clone();
            방문한_장소.push(이번_방문_장소.clone());
        } else if 장애물들.contains(&경비.위치) {
            // println!("{:?} : 아코! 쿵했어요! 90도 회전해서 {:?}측으로 전진합니다!!", &경비.위치, &경비.방향);
            경비 = 경비_메뉴얼(경비, "회전");
            방문한_장소.pop();
            let 이번_방문_장소 = 경비.위치.clone();
            방문한_장소.push(이번_방문_장소.clone());
        }
    }

    let mut 중복_제거 = 방문한_장소.to_vec();
    중복_제거.sort();
    중복_제거.dedup();

    return 중복_제거.len() as i32 - 1;
}

pub fn 경비_괴롭히기_경기 (입력:&str) -> i32 {
    let (mut 원본_경비, 원본_장애물, 지도_크기) = 지도_읽기(입력);
    // prinln!("이 지도는 총 가로 {:?} 칸, 세로 {:?} 칸이에요!", 지도_크기.1+1, 지도_크기.0+1);
    // println!("작은 경비소녀는 {:?}에 서서 {}측 방향을 보고 있구요!", 경비.위치, 경비.방향);
    // println!("장애물은 총 {} 곳에 있고, 위치는 차례대로 {:?}에요!", 장애물들.len(), 장애물들);

    // 일단 한 바퀴를 먼저 돌아서 경비의 경로, 약 5천개를 찾아내자
    let mut 원래_방문할_장소 = Vec::new();
    
    'outer: loop {
        if 원본_경비.위치.0 < 0 || 원본_경비.위치.1 < 0 || 원본_경비.위치.0 > 지도_크기.0 || 원본_경비.위치.1 > 지도_크기.1 {
            break 'outer;
        } else if !원본_장애물.contains(&원본_경비.위치) {
            // println!("{:?} : 이 곳엔 아무것도 없어요! {:?}측으로 전진합니다!!", &원본_경비.위치, &원본_경비.방향);
            원본_경비 = 경비_메뉴얼(원본_경비.clone(), "전진");
            let 이번_방문_장소 = 원본_경비.위치.clone();
            원래_방문할_장소.push(이번_방문_장소.clone());
        } else if 원본_장애물.contains(&원본_경비.위치) {
            // println!("{:?} : 아코! 쿵했어요! 90도 회전해서 {:?}측으로 전진합니다!!", &원본_경비.위치, &원본_경비.방향);
            원본_경비 = 경비_메뉴얼(원본_경비, "회전");
            원래_방문할_장소.pop();
            let 이번_방문_장소 = 원본_경비.위치.clone();
            원래_방문할_장소.push(이번_방문_장소.clone());
        }
    }

    let mut 중복_제거 = 원래_방문할_장소.to_vec();
    중복_제거.sort();
    중복_제거.dedup();

    // 이제 저걸 for로 돌려서 처리하자
    let mut 여기다_이걸_놓으면_재밌겠다:i32 = 0;
    // 변수를 다시 받아와야 되네 경비 좌표가 밖으로 고정돼있네 ㅋㅋㅋ
    let (시뮬레이션_경비, 장애물, 지도_크기) = 지도_읽기(입력);
    
    for 어디가_좋을까 in 중복_제거 {
        let mut 방문한_장소 = Vec::new();
        let mut 헤메이는_발걸음:i32 = 0;
        let mut 자_이제_게임을_시작하지 = 시뮬레이션_경비.clone();
        let mut 인공장애물 = 장애물.clone();

        if !인공장애물.contains(&어디가_좋을까) {
            println!("이번에는 ({}, {}) 지점에 장애물을 설치해볼까~?", 어디가_좋을까.0, 어디가_좋을까.1);
            인공장애물.push(어디가_좋을까);
            println!("이제 장애물은 총 {}개랍니다!", 인공장애물.len());
        }
        
        'inner: loop {
            if 자_이제_게임을_시작하지.위치.0 < 0 || 자_이제_게임을_시작하지.위치.1 < 0 || 자_이제_게임을_시작하지.위치.0 > 지도_크기.0 || 자_이제_게임을_시작하지.위치.1 > 지도_크기.1 {
                println!("이런, 미꾸라지처럼 빠져나갔잖아!");
                break 'inner;
            } else if 헤메이는_발걸음 > 방문한_장소.len() as i32 *3 {
                여기다_이걸_놓으면_재밌겠다+=1;
                println!("하하 드디어 잡았다!");
                break 'inner;
            } else if !인공장애물.contains(&자_이제_게임을_시작하지.위치) {
                // println!("{:?} : 이 곳엔 아무것도 없어요! {:?}측으로 전진합니다!!", &자_이제_게임을_시작하지.위치, &자_이제_게임을_시작하지.방향);
                자_이제_게임을_시작하지 = 경비_메뉴얼(자_이제_게임을_시작하지.clone(), "전진");
                let 이번_방문_장소 = 자_이제_게임을_시작하지.위치.clone();
                방문한_장소.push(이번_방문_장소.clone());
            } else if 인공장애물.contains(&자_이제_게임을_시작하지.위치) {
                // println!("{:?} : 아코! 쿵했어요! 90도 회전해서 {:?}측으로 전진합니다!!", &시뮬레이션_경비.위치, &시뮬레이션_경비.방향);
                자_이제_게임을_시작하지 = 경비_메뉴얼(자_이제_게임을_시작하지, "회전");
                방문한_장소.pop();
                let 이번_방문_장소 = 자_이제_게임을_시작하지.위치.clone();
                방문한_장소.push(이번_방문_장소.clone());
            }

            방문한_장소 = 방문한_장소.to_vec();
            방문한_장소.sort();
            방문한_장소.dedup();
            헤메이는_발걸음 += 1;
        }
    }


    println!("쓸만한 장애물 설치 장소는 총 {} 곳이네요!", 여기다_이걸_놓으면_재밌겠다);

    return 여기다_이걸_놓으면_재밌겠다;
}


#[cfg(test)]
mod tests {
    use super::*;

    const 예문: &str = r##"....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#..."##;

    #[test]
    fn test_part1() {
        assert_eq!(경비_피하기_경기(예문.trim()), 41);
    }

    #[test]
    fn test_part2() {
        assert_eq!(경비_괴롭히기_경기(예문.trim()), 6);
    }
}