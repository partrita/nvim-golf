# Indent Lines

<!-- difficulty: beginner -->

여러 줄을 한 번에 들여씁니다.

## Before

```text
apple
banana
cherry
```

## After

```text
  apple
  banana
  cherry
```

## Command

```
:%s/^/  /g<cr>
```

1. `:%s` 전체 파일 치환
1. `/^/` 각 줄의 시작(^) 위치 매칭
1. `  /` 두 칸 공백 삽입
1. `g` 전역 적용
1. `<cr>` 명령 실행
