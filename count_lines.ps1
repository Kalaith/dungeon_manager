$files = Get-ChildItem -Recurse -Filter "*.rs" | Where-Object { $_.FullName -notmatch "\\target\\" }
$results = $files | Select-Object @{Name="File"; Expression={$_.FullName.Substring($PWD.Path.Length + 1)}}, @{Name="Lines"; Expression={ (Get-Content $_.FullName | Measure-Object -Line).Lines }} | Sort-Object Lines -Descending

$results | Format-Table -AutoSize

$total = ($results | Measure-Object -Property Lines -Sum).Sum
Write-Host "Total Lines in .rs files: $total" -ForegroundColor Green
