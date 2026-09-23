# Swap Quoted Strings

<!-- difficulty: beginner -->

두 따옴표 내용의 위치를 바꿉니다.

## Before

```text
"foo" "bar"
```

## After

```text
"bar" "foo"
```

## Command

```
:s/"\([^"]*\)" "\([^"]*\)"/"\2" "\1"/<cr>
```

1. `:s/"\([^"]*\)" "\([^"]*\)"/` 두 개의 큰따옴표 문자열 매칭 및 캡처
1. `"\2" "\1"/` 두 문자열의 위치 교환
1. `<cr>` 명령 실행
