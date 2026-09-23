# Rearrange Array to One Line

<!-- difficulty: intermediate -->

여러 줄로 펼쳐진 자바스크립트 객체 배열을 한 줄로 병합하고 후행 쉼표를 제거합니다.

## Before

```js
const data = [
  {
    goal: 400,
  },
  {
    goal: 300,
  },
  {
    goal: 200,
  },
  {
    goal: 300,
  },
  {
    goal: 200,
  },
  {
    goal: 278,
  },
  {
    goal: 189,
  },
  {
    goal: 239,
  },
  {
    goal: 300,
  },
  {
    goal: 200,
  },
  {
    goal: 278,
  },
  {
    goal: 189,
  },
  {
    goal: 349,
  },
]
```

## After

```js
const data = [
  { goal: 400 }, { goal: 300 }, { goal: 200 }, { goal: 300 }, { goal: 200 }, { goal: 278 }, { goal: 189 }, { goal: 239 }, { goal: 300 }, { goal: 200 }, { goal: 278 }, { goal: 189 }, { goal: 349 }
]
```

## Command

```
:2,$-1j<cr>:2s/,\s*}/ }/g<cr>:2s/,\s*$//<cr>
```

1. `:2,$-1j` 2행부터 마지막 직전 행까지의 모든 객체 라인을 한 줄로 결합
1. `<cr>` 명령 실행
1. `:2s/,\s*}/ }/g` 각 객체 내부 프로퍼티 뒤의 후행 쉼표 제거
1. `<cr>` 치환 명령 실행
1. `:2s/,\s*$//` 배열 마지막 원소 뒤에 붙은 후행 쉼표 제거
1. `<cr>` 치환 명령 실행
