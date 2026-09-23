# Align Assignments

<!-- difficulty: intermediate -->

할당 연산자를 열에 맞춰 정렬합니다.

## Before

```text
a = 1
longer = 2
mid = 3
```

## After

```text
a      = 1
longer = 2
mid    = 3
```

## Command

```
w5i <esc>jj^w3i <esc>
```

1. `w` 등호(=) 위치로 이동
1. `5i <esc>` 등호 앞에 공백 5칸 삽입 후 노멀 모드 복귀
1. `jj` 세 번째 줄로 이동
1. `^w` 줄 첫 글자에서 등호(=) 위치로 이동
1. `3i <esc>` 등호 앞에 공백 3칸 삽입 후 노멀 모드 복귀
