use std::process::exit;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() != 4 {
        eprintln!("사용법: calc <숫자> <연산자> <숫자>   (예: calc 10 / 2)");
        exit(1);
    }

    let a: f64 = args[1].parse().unwrap_or_else(|_| {
        eprintln!("숫자가 아닙니다: {}", args[1]);
        exit(1);
    });
    let op = &args[2];
    let b: f64 = args[3].parse().unwrap_or_else(|_| {
        eprintln!("숫자가 아닙니다: {}", args[3]);
        exit(1);
    });

    let result = match op.as_str() {
        "+" => a + b,
        "-" => a - b,
        "*" => a * b,
        "/" => {
            if b == 0.0 {
                eprintln!("0으로 나눌 수 없습니다");
                exit(1);
            }
            a / b
        }
        _ => {
            eprintln!("지원하지 않는 연산자: {} (+, -, *, / 만 가능)", op);
            exit(1);
        }
    };

    println!("{}", result);
}
