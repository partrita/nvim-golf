# CSV to Lines

<!-- difficulty: beginner -->

쉼표로 구분된 한 줄을 여러 줄로 나눕니다.

## Before

```text
apple,banana,cherry
```

## After

```text
apple
banana
cherry
```

## Command

```
:s/,/\r/g<cr>
```

1. `:s` 현재 줄 치환 명령
1. `/,/` 쉼표(,) 검색
1. `\r/` 줄바꿈(\r)으로 교체
1. `g` 줄 내 모든 일치 항목에 적용
1. `<cr>` 명령 실행
