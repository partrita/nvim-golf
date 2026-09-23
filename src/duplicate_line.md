# Duplicate Line

<!-- difficulty: beginner -->

한 줄을 복사해 세 줄로 늘립니다.

## Before

```text
log
```

## After

```text
log
log
log
```

## Command

```
yyp.
```

1. `yy` 현재 줄 복사
1. `p` 아래 줄에 붙여넣기
1. `.` 이전 작업(붙여넣기) 반복
