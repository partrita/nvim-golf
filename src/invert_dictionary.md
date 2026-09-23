# Invert Dictionary

<!-- difficulty: advanced -->

딕셔너리의 키-값 쌍을 반전시킵니다.

## Before

```gdscript
var color_to_points = {
    "red" = 0,
    "orange" = 5,
    "yellow" = 10,
    "green" = 15,
    "blue" = 20,
    "purple" = 30,
    "black" = 50,
}
```

## After

```gdscript
var points_to_color = {
    0 = "red",
    5 = "orange",
    10 = "yellow",
    15 = "green",
    20 = "blue",
    30 = "purple",
    50 = "black",
}
```

## Command

```
:1s/color_to_points/points_to_color/<cr>

:%s/\("[^"]*"\) = \([0-9]*\)/\2 = \1/g<cr>
```

1. `:1s/color_to_points/points_to_color/<cr>` 변수 이름 반전
1. `:%s/\("[^"]*"\) = \([0-9]*\)/\2 = \1/g<cr>` 키 문자열과 값 숫자의 위치 맞바꾸기
