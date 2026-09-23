# Delete Surround

<!-- difficulty: beginner -->

여러 줄의 감싸는 괄호를 한 번에 제거합니다.

## Before

```text
(use)
(use)
```

## After

```text
use
use
```

## Command

```
:%s/[()]//g<cr>
```

1. `:%s` 전체 파일 대상 치환
1. `/[()]//` 여는 괄호와 닫는 괄호를 빈 문자열로 삭제
1. `g` 모든 일치 항목에 적용
1. `<cr>` 명령 실행
