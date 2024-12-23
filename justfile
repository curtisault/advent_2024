# use `just work day-01 part1` to work on a specific binary fora specific day's part
work day part:
    cargo watch -w {{day}} -x "check -p {{day}}" -s "just test {{day}} {{part}}" -s "just lint {{day}}"

lint day:
    cargo clippy -p {{day}}

test day part:
    cargo nextest run -p {{day}} {{part}}

create day:
    cargo generate --path ./daily-template --name {{day}}
