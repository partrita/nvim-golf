# Export from Rust Module

<!-- difficulty: intermediate -->

각 모듈에 포함된 함수를 re-export(pub use)합니다.

## Before

```rs
mod generate_demos;
mod mdbook_preprocessor;
mod validate;
```

## After

```rs
mod generate_demos;
mod mdbook_preprocessor;
mod validate;

pub use generate_demos::generate_demos;
pub use mdbook_preprocessor::mdbook_preprocessor;
pub use validate::validate;
```

## Command

```
:%t$<cr>:4s/^/\r/<cr>:5,$s/mod \(.*\);/pub use \1::\1;/g<cr>
```

1. `:%t$<cr>` 전체 내용을 버퍼 맨 끝($)으로 복제
1. `:4s/^/\r/<cr>` 4번째 줄 앞에 빈 줄 삽입
1. `:5,$s/mod \(.*\);/pub use \1::\1;/g<cr>` 5행부터 끝까지 mod 구문을 pub use 모듈::함수로 치환
