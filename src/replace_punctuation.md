# Replace Punctuation

<!-- difficulty: beginner -->

여러 줄의 앞 기호를 한 번에 바꿉니다.

## Before

```text
- apple
- banana
- cherry
```

## After

```text
* apple
* banana
* cherry
```

## Command

```
:%s/^-/*/<cr>
```

1. `:%s/^-/` 줄 시작 부분의 하이픈(-) 매칭
1. `*/` 별표(*)로 교체
1. `<cr>` 명령 실행
