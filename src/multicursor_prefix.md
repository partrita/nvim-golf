# Multicursor Prefix

<!-- difficulty: intermediate -->

Visual Block 모드로 여러 줄 앞에 접두사를 동시에 삽입합니다.

## Before

```text
apple
banana
cherry
```

## After

```text
fruit: apple
fruit: banana
fruit: cherry
```

## Command

```
<ctrl-v>2jIfruit: <esc>
```

1. `<ctrl-v>` Visual Block 모드 진입
1. `2j` 아래로 2줄 확장
1. `I` 블록 앞 삽입 모드 진입
1. `fruit: ` 접두사 입력
1. `<esc>` 노멀 모드로 복귀하여 모든 줄에 동시 적용
