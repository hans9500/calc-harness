// 에러코드 체계
//   1xxx = 입력/파싱 오류
//   2xxx = 수학 오류
//   9xxx = 예약(사용 금지)
// 모든 에러는 stderr에 "[E<코드>] <메시지>" 형식으로 출력하고 종료한다.
//
// 사용 중인 코드:
//   1000 = 사용법 오류(인자 개수 불일치)
//   1001 = 숫자 아님
//   1002 = 지원하지 않는 연산자
//   2001 = 0으로 나누기

use std::process::exit;

/// 에러코드와 메시지를 "[E<코드>] <메시지>" 형식으로 stderr에 출력하고 종료한다.
fn die(code: u32, message: &str) -> ! {
    eprintln!("[E{}] {}", code, message);
    exit(1);
}

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() != 4 {
        die(1000, "사용법: calc <숫자> <연산자> <숫자>   (예: calc 10 / 2)");
    }

    let a: f64 = args[1]
        .parse()
        .unwrap_or_else(|_| die(1001, &format!("숫자가 아닙니다: {}", args[1])));
    let op = &args[2];
    let b: f64 = args[3]
        .parse()
        .unwrap_or_else(|_| die(1001, &format!("숫자가 아닙니다: {}", args[3])));

    let result = match op.as_str() {
        "+" => a + b,
        "-" => a - b,
        "*" => a * b,
        "/" => {
            if b == 0.0 {
                die(2001, "0으로 나눌 수 없습니다");
            }
            a / b
        }
        _ => die(1002, &format!("지원하지 않는 연산자: {} (+, -, *, / 만 가능)", op)),
    };

    println!("{}", result);
}
