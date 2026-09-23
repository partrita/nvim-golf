# Text into Array

<!-- difficulty: intermediate -->

줄바꿈으로 구분된 데이터를 문자열 배열로 결합합니다.

## Before

```text
Hello
This
Is
Helix
```

## After

```js
["Hello", "This", "Is", "Helix"]
```

## Command

```
:%s/.*/"&"/<cr>:%s/\n\ze./, /g<cr>I[<esc>A]<esc>
```

1. `:%s/.*/"&"/<cr>` 각 줄을 큰따옴표로 감싸기
1. `:%s/\n\ze./, /g<cr>` 다음 줄이 존재하는 줄바꿈만 쉼표와 공백으로 치환
1. `I[` 줄 맨 앞에 여는 대괄호([) 삽입
1. `<esc>` 노멀 모드 복귀
1. `A]` 줄 맨 끝에 닫는 대괄호(]) 삽입
1. `<esc>` 노멀 모드 복귀
