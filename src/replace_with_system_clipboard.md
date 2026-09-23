# Replace a Selection with the System Clipboard

<!-- difficulty: intermediate -->

레지스터를 활용하여 문자열을 복사하고 다른 위치의 문자열을 교체합니다.

## Before

```rs
fn greet() {
    println!("Hello, world!");
}

fn main() {
    println!("Hello, Rust!");
}
```

## After

```rs
fn greet() {
    println!("Hello, Rust!");
}

fn main() {
    println!("Hello, Rust!");
}
```

## Command

```
/Rust<cr>yiwgg/world<cr>viwp
```

1. `/Rust<cr>` Rust 검색 및 이동
1. `yiw` 단어(Rust) 복사
1. `gg` 파일 시작으로 이동
1. `/world<cr>` world 검색 및 이동
1. `viw` 단어(world) 비주얼 선택
1. `p` 복사해둔 텍스트로 덮어쓰기
