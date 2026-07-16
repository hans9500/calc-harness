// 빌드된 calc-harness 실행파일을 직접 돌려서 stdout/stderr를 검사하는 통합 테스트.
// 실행파일 경로는 Cargo가 넘겨주는 CARGO_BIN_EXE_<bin-name> 환경변수로 얻는다.

use std::process::Command;

/// calc-harness를 주어진 인자로 실행하고 (성공여부, stdout, stderr)를 돌려준다.
fn run(args: &[&str]) -> (bool, String, String) {
    let output = Command::new(env!("CARGO_BIN_EXE_calc-harness"))
        .args(args)
        .output()
        .expect("실행파일을 실행하지 못했습니다");

    let stdout = String::from_utf8_lossy(&output.stdout).trim().to_string();
    let stderr = String::from_utf8_lossy(&output.stderr).trim().to_string();
    (output.status.success(), stdout, stderr)
}

#[test]
fn addition() {
    let (ok, stdout, _) = run(&["3", "+", "4"]);
    assert!(ok, "성공 종료여야 합니다");
    assert_eq!(stdout, "7");
}

#[test]
fn division() {
    let (ok, stdout, _) = run(&["10", "/", "2"]);
    assert!(ok, "성공 종료여야 합니다");
    assert_eq!(stdout, "5");
}

#[test]
fn division_by_zero() {
    let (ok, _, stderr) = run(&["10", "/", "0"]);
    assert!(!ok, "실패 종료여야 합니다");
    assert!(
        stderr.starts_with("[E2001]"),
        "stderr에 [E2001]이 있어야 합니다: {stderr}"
    );
}

#[test]
fn not_a_number() {
    let (ok, _, stderr) = run(&["abc", "+", "2"]);
    assert!(!ok, "실패 종료여야 합니다");
    assert!(
        stderr.starts_with("[E1001]"),
        "stderr에 [E1001]이 있어야 합니다: {stderr}"
    );
}
