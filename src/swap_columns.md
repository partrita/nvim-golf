# Swap Columns

<!-- difficulty: beginner -->

두 열의 순서를 바꿉니다.

## Before

```text
apple 1
banana 2
```

## After

```text
1 apple
2 banana
```

## Command

```
:%s/\(.*\) \(.*\)/\2 \1/<cr>
```

1. `:%s/\(.*\) \(.*\)/` 공백 기준 두 열 매칭 및 그룹 캡처
1. `\2 \1/` 두 번째 열과 첫 번째 열의 위치 교환
1. `<cr>` 명령 실행
