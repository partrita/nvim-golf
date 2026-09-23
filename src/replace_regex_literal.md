# Replace a Regex-Sensitive Literal

<!-- difficulty: intermediate -->

문서 전체에서 정규식 메타문자가 포함된 문자열을 다른 문자열로 한 번에 변경합니다.

## Before

```text
C++ is widely used.
I learned C++ before Rust.
This project does not use C++ anymore.
```

## After

```text
Rust is widely used.
I learned Rust before Rust.
This project does not use Rust anymore.
```

## Command

```
:%s/C++/Rust/g<cr>
```

1. `:%s/` 전체 대상 치환
1. `C++/` C++ 검색
1. `Rust/` Rust로 치환
1. `g` 줄 내 모든 일치 항목 적용
1. `<cr>` 명령 실행
