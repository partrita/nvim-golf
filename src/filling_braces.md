# Filling Braces

<!-- difficulty: beginner -->

중괄호 안으로 이동하여 지정된 속성값을 입력합니다.

## Before

```css
.ocean {}
.land {}
.sky {}
```

## After

```css
.ocean {blue}
.land {green}
.sky {lightblue}
```

## Command

```
ci}blue<esc>jci}green<esc>jci}lightblue<esc>
```

1. `ci}` 현재 줄의 중괄호({}) 내부로 점프하여 변경 모드로 전환
1. `blue` 첫 번째 중괄호 내용 입력
1. `<esc>` 일반 모드로 복귀
1. `j` 다음 줄로 이동
1. `ci}` 다음 줄의 중괄호 내부로 전환
1. `green` 두 번째 중괄호 내용 입력
1. `<esc>` 일반 모드로 복귀
1. `j` 다음 줄로 이동
1. `ci}` 다음 줄의 중괄호 내부로 전환
1. `lightblue` 세 번째 중괄호 내용 입력
1. `<esc>` 일반 모드로 복귀
