# Object into Array

<!-- difficulty: intermediate -->

객체를 2차원 배열로 변환합니다.

## Before

```js
const palette = {
  apricot: "#f47868",
  lightning: "#ffcd1c",
  delta: "6f44f0",
};
```

## After

```js
const palette = [
  ["apricot", "#f47868"],
  ["lightning", "#ffcd1c"],
  ["delta", "6f44f0"],
];
```

## Command

```
:%s/{/[/<cr>:%s/}/]/<cr>

:%s/  \([a-z]*\): \([^,]*\),/  ["\1", \2],/g<cr>
```

1. `:%s/{/[/<cr>` 여는 중괄호를 여는 대괄호로 치환
1. `:%s/}/]/<cr>` 닫는 중괄호를 닫는 대괄호로 치환
1. `:%s/  \([a-z]*\): \([^,]*\),/  ["\1", \2],/g<cr>` 키-값 쌍을 배열 요소 형식으로 치환
