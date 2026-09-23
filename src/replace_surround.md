# Replace Surrounding Characters

<!-- difficulty: beginner -->

괄호를 대괄호로 변경합니다.

## Before

```text
(use)
(use)
```

## After

```text
[use]
[use]
```

## Command

```
:%s/(\([^)]*\))/[\1]/g<cr>
```

1. `:%s/(\([^)]*\))/` 소괄호와 내부 내용 매칭 및 캡처
1. `[\1]/g` 대괄호로 감싸도록 교체
1. `<cr>` 명령 실행
