# Add Semicolons

<!-- difficulty: beginner -->

각 줄 끝에 세미콜론을 붙입니다.

## Before

```js
const a = 1
const b = 2
const c = 3
```

## After

```js
const a = 1;
const b = 2;
const c = 3;
```

## Command

```
:%s/$/;/g<cr>
```

1. `:%s` 전체 파일 대상 치환 명령
1. `/$/` 각 줄의 끝($) 위치를 검색
1. `;/` 세미콜론으로 치환
1. `g` 모든 일치 항목에 적용
1. `<cr>` 명령 실행
