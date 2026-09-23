# Format JSON with jq

<!-- difficulty: beginner -->

한 줄 JSON을 셸 필터로 보기 좋게 펼칩니다.

## Before

```json
{"name": "neovim", "stars": 100}
```

## After

```json
{
  "name": "neovim",
  "stars": 100
}
```

## Command

```
:%!jq .<cr>
```

1. `:%!` 전체 버퍼를 외부 명령의 표준 입출력으로 전달
1. `jq .` jq 포맷팅 도구 실행
1. `<cr>` 명령 실행
