# Insert Sequence

<!-- difficulty: beginner -->

셸 출력을 커서 앞에 삽입합니다.

## Before

```text
items:
```

## After

```text
1
2
3
items:
```

## Command

```
:0read !seq 3<cr>
```

1. `:0read !seq 3` 파일 맨 앞(0번 라인)에 seq 3 명령 출력 삽입
1. `<cr>` 명령 실행
