# auto-fmt.ps1 - PostToolUse: .rs 파일 편집 후 cargo fmt 실행

$raw = [Console]::In.ReadToEnd()
$path = ""
try {
    $data = $raw | ConvertFrom-Json
    $path = [string]$data.tool_input.file_path
} catch { exit 0 }

# .rs 파일이 아니면 아무것도 안 함
if ($path -notmatch '\.rs$') { exit 0 }

# 프로젝트 전체 서식 정리 (조용히)
cargo fmt 2>&1 | Out-Null

exit 0