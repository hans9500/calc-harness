# calc-harness

Claude Code의 **하네스 엔지니어링**(훅 · 스킬 · 에이전트)을 하나씩 직접 만들며 익히기 위한 학습용 프로젝트다. 간단한 Rust CLI 계산기를 소재로, `.claude/` 디렉토리에 훅 3개 · 스킬 1개 · 에이전트 1개를 구성했다.

> 이 저장소의 Claude Code 관련 설명은 모두 [공식 문서](https://code.claude.com/docs)에서 확인한 내용을 기준으로 한다.

## 계산기

숫자 · 연산자 · 숫자를 받아 계산하고, 오류는 에러코드로 표현한다.

```cmd
cargo run -- 10 / 2      REM -> 5
cargo run -- 3 + 4       REM -> 7
cargo run -- 10 / 0      REM -> [E2001] 0으로 나눌 수 없습니다
cargo run -- abc + 2     REM -> [E1001] 숫자가 아닙니다: abc
```

테스트:

```cmd
cargo test
```

## 프로젝트 구조

```
calc-harness/
├── src/main.rs                       계산기 (에러코드 체계 + die() 헬퍼)
├── tests/cli.rs                      통합 테스트
└── .claude/                          == 하네스 ==
    ├── settings.json                 훅 등록
    ├── hooks/
    │   ├── delete-guard.ps1          위험 삭제 차단        (PreToolUse)
    │   ├── guard-error-codes.ps1     9xxx 예약범위 차단     (PreToolUse)
    │   └── auto-fmt.ps1              편집 후 cargo fmt      (PostToolUse)
    ├── skills/
    │   └── add-error-code/SKILL.md   에러코드 추가 규약
    └── agents/
        └── test-analyst.md           테스트 분석 에이전트 (읽기 전용)
```

## 하네스 구성

| 요소 | 파일 | 역할 | 성격 |
|---|---|---|---|
| 훅 | `delete-guard.ps1` | `rm`/`del`/`Remove-Item` 등 삭제 명령 차단 | 결정론 · 자동 |
| 훅 | `guard-error-codes.ps1` | `.rs`에 예약범위 `9xxx` 편집 시 차단 | 결정론 · 자동 |
| 훅 | `auto-fmt.ps1` | `.rs` 편집 후 `cargo fmt` 자동 실행 | 결정론 · 자동 |
| 스킬 | `add-error-code` | 새 에러코드 추가 규약 안내 | 확률적 · 요청 시 |
| 에이전트 | `test-analyst` | 별도 컨텍스트에서 `cargo test` 실행 후 결론만 회수 | 독립 · 위임 시 |

**핵심 구분:** 훅은 이벤트에 예외 없이 자동 실행되고(결정론적), 스킬·에이전트는 필요할 때 판단으로 적용된다(확률적). 진짜 지켜야 할 규칙은 스킬로 안내하고 훅으로 강제한다(예: `add-error-code` 스킬 + `guard-error-codes` 훅).

### errorCode 번호 범위

| 범위 | 용도 | 사용 중 |
|---|---|---|
| `1xxx` | 입력/파싱 오류 | 1000 사용법 · 1001 숫자 아님 · 1002 연산자 |
| `2xxx` | 수학 오류 | 2001 0으로 나누기 · 2002 오버플로우 |
| `9xxx` | 예약(사용 금지) | 없음 (가드 훅이 차단) |

## 훅 검증

훅은 배선하기 전에 스크립트 단독으로 검증한다(가짜 입력 JSON을 stdin으로 흘려 넣고 종료코드 확인). 위험/예약은 `exit=2`(차단), 안전/정상은 `exit=0`(통과)이어야 한다.

```cmd
echo {"tool_input":{"command":"Remove-Item -Recurse -Force target"}}> test-input.json
type test-input.json | powershell -NoProfile -ExecutionPolicy Bypass -File .claude\hooks\delete-guard.ps1
echo exit=%errorlevel%
```

`/hooks`로 등록된 훅을, `@agent-test-analyst`로 에이전트를 확인할 수 있다.

## 전체 가이드

개념 · 0~6단계 실습 · 실패 스토리(fail-open, matcher 구멍, 한글 인코딩) · 다이어그램까지 담은 상세 가이드는 `harness_tutorial_guide.html`(단일 HTML, 다크/라이트 토글)에 있다.

## 참고 사항

- 작업 환경은 **Windows + cmd**, 훅 스크립트는 **PowerShell**로 작성했다. 설정/스킬/에이전트 파일은 **BOM 없이 UTF-8**로 저장해야 한다.
- PowerShell 5.1은 stdin을 UTF-8로 명시해 읽어야 한글이 깨지지 않는다. 보안 성격의 훅은 파싱 실패 시 통과가 아니라 차단(fail-closed)으로 둔다.

## 공식 문서

- 훅 — <https://code.claude.com/docs/en/hooks>
- 스킬 — <https://code.claude.com/docs/en/skills>
- 서브에이전트 — <https://code.claude.com/docs/en/sub-agents>
