# Fix Typo with Search

<!-- difficulty: beginner -->

검색으로 오타를 찾아 고칩니다.

## Before

```text
roses are red
violets are blu
sugar is sweet
```

## After

```text
roses are red
violets are blue
sugar is sweet
```

## Command

```
/blu<cr>cwblue<esc>
```

1. `/blu<cr>` 오타 검색 후 이동
1. `cw` 단어를 삭제하고 삽입 모드로 전환
1. `blue` 올바른 단어 입력
1. `<esc>` 노멀 모드로 복귀
