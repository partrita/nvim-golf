# Enumerate and Align

<!-- difficulty: advanced -->

각 객체에 1부터 시작하여 증가하는 `rank` 필드를 추가하고, 보기 좋게 필드를 정렬합니다.

## Before

```js
[
  { word: "a", count: 2565 },
  { word: "and", count: 1777 },
  { word: "of", count: 1331 },
  { word: "that", count: 1263 },
  { word: "to", count: 1030 },
  { word: "in", count: 1027 },
  { word: "it", count: 754 },
  { word: "as", count: 730 },
  { word: "was", count: 687 },
  { word: "you", count: 652 },
  { word: "for", count: 630 },
];
```

## After

```js
[
  { rank:  1, word: "a",    count: 2565 },
  { rank:  2, word: "and",  count: 1777 },
  { rank:  3, word: "of",   count: 1331 },
  { rank:  4, word: "that", count: 1263 },
  { rank:  5, word: "to",   count: 1030 },
  { rank:  6, word: "in",   count: 1027 },
  { rank:  7, word: "it",   count:  754 },
  { rank:  8, word: "as",   count:  730 },
  { rank:  9, word: "was",  count:  687 },
  { rank: 10, word: "you",  count:  652 },
  { rank: 11, word: "for",  count:  630 },
];
```

## Command

```
:2,12s/word/\=printf("rank: %2d, word", line(".")-1)/<cr>

:2,12s/word: \([^,]*\), count: \(\d\+\)/

\=printf("word: %-7s count: %4d",

 submatch(1).",", str2nr(submatch(2)))/<cr>
```

1. `:2,12s/word/\=printf("rank: %2d, word", line(".")-1)/<cr>` 2~12행에 증가하는 번호의 rank 필드 삽입
1.
    ```
    :2,12s/word: \([^,]*\), count: \(\d\+\)/\=printf("word: %-7s count: %4d", submatch(1).",", str2nr(submatch(2)))/<cr>
    ```
