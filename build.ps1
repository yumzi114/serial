[Console]::OutputEncoding = [System.Text.Encoding]::UTF8
# 릴리즈 빌드
cargo build --release

# 바이너리 이름 추출
$binName = (Get-Content .\Cargo.toml | Select-String -Pattern '^name *= *"(.+)"' | ForEach-Object { ($_ -match '"(.+)"') | Out-Null; $matches[1] })

# 소스 경로
$srcPath = "target\release\$binName.exe"

# 대상 경로 (공백/한글 포함되므로 Resolve-Path 사용)
$dest = "$HOME\OneDrive\바탕 화면"

# 경로 정리 후 복사
Copy-Item (Resolve-Path $srcPath) -Destination (Resolve-Path $dest) -Force

Write-Host "✅ 복사 완료: $binName.exe → 바탕 화면"