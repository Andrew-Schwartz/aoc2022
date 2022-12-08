$output = [string](cargo aoc input)
$day = $output.Split(' ')[6]
$day_padded = $day.PadLeft(2, '0')
$today_path = "src/day$day_padded.rs"

$lib = (Get-Content src/lib.rs) -as [Collections.ArrayList]

# idempotency
if ($lib[$lib.Count - 3] -eq "mod day$day_padded;") {
    return
}

# make the new day's rs file
Copy-Item src/dayXX.rs $today_path
(Get-Content $today_path -Raw) -replace "day","day$day" | Set-Content $today_path

# add the mod to lib.rs
#$lib = (Get-Content src/lib.rs) -as [Collections.ArrayList]
$lib.Insert($lib.Count - 2, "mod day$day_padded;")
$lib | Set-Content src/lib.rs
