# Toggle Comment

<!-- difficulty: beginner -->

여러 줄을 한 번에 주석으로 바꿉니다.

## Before

```py
print("a")
print("b")
print("c")
```

## After

```py
# print("a")
# print("b")
# print("c")
```

## Command

```
<ctrl-v>2jI# <esc>
```

1. `<ctrl-v>` Visual Block 모드 진입
1. `2j` 아래 2줄 확장 선택
1. `I` 블록 앞 삽입 모드 진입
1. `# ` 주석 기호와 공백 입력
1. `<esc>` 노멀 모드로 복귀하여 전체 적용
