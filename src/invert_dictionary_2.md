# Invert Dictionary 2

<!-- difficulty: advanced -->

딕셔너리의 키-값 쌍을 반전시키는 또 다른 방법입니다.

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

:2,8s/\(\s*\)\(.*\) = \(.*\),/\1\3 = \2,/<cr>
```

1. `:1s/color_to_points/points_to_color/<cr>` 첫 줄 변수명 치환
1. `:2,8s/\(\s*\)\(.*\) = \(.*\),/\1\3 = \2,/<cr>` 2~8행 들여쓰기와 쉼표를 유지하며 등호 앞뒤 내용 치환
