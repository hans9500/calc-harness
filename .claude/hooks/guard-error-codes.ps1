# guard-error-codes.ps1 - PreToolUse: 예약 범위 9xxx 편집 차단

# stdin을 UTF-8로 명시해서 읽기 (PowerShell 5.1 인코딩 문제 회피)
$stdin = [Console]::OpenStandardInput()
$reader = New-Object IO.StreamReader($stdin, [Text.Encoding]::UTF8)
$raw = $reader.ReadToEnd()

$path = ""
$newText = ""
try {
    $data = $raw | ConvertFrom-Json
    $path = [string]$data.tool_input.file_path
    $newText = [string]$data.tool_input.new_string + [string]$data.tool_input.content
} catch {
    # 파싱 실패 = 검사 불가. 가드는 안전하게 "차단" 쪽으로 기운다 (fail-closed)
    [Console]::Error.WriteLine("[guard-error-codes] input parse failed - blocking to be safe")
    exit 2
}

# .rs 파일이 아니면 검사 불필요 -> 통과
if ($path -notmatch '\.rs$') { exit 0 }

# 예약 범위 9xxx 탐지
if ($newText -match 'E9\d{3}' -or $newText -match '\b9\d{3}\s*=') {
    [Console]::Error.WriteLine("[guard-error-codes] 9xxx is a reserved range. Use 1xxx (input) or 2xxx (math) instead.")
    exit 2
}

exit 0