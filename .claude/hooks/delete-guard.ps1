# delete-guard.ps1 - PreToolUse(Bash) 훅: 위험한 삭제 명령 차단

$raw = [Console]::In.ReadToEnd()

# 메시지를 깔끔하게 뽑으려고 command 필드를 먼저 시도
$cmd = ""
try {
    $data = $raw | ConvertFrom-Json
    $cmd = [string]$data.tool_input.command
} catch { }

# 못 뽑으면 원문 전체를 검사 (파싱 실패해도 삭제가 못 빠져나감)
$target = if ($cmd) { $cmd } else { $raw }

$patterns = @('\brm\b', '\bdel\b', '\brmdir\b', '\brd\b', 'Remove-Item')

foreach ($p in $patterns) {
    if ($target -match $p) {
        [Console]::Error.WriteLine("[delete-guard] blocked delete command: $target")
        exit 2
    }
}

exit 0