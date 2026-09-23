# Sort Lines

<!-- difficulty: beginner -->

정렬되지 않은 줄들을 정렬합니다.

## Before

```text
banana
apple
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
:%!sort<cr>
```

1. `:%!` 전체 버퍼를 외부 명령에 전달
1. `sort` sort 정렬 명령
1. `<cr>` 명령 실행
