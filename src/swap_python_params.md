# Swap Python Function Parameters

<!-- difficulty: beginner -->

함수 정의의 매개변수 순서를 바꾸고, 코드 내의 모든 주석을 삭제합니다.

## Before

```py
# caculate area
def calculate_area(width, height): # width, height
    return width * height # return area

area1 = calculate_area(5, 10)
area2 = calculate_area(8, 12)
```

## After

```py
def calculate_area(height, width):
    return width * height

area1 = calculate_area(5, 10)
area2 = calculate_area(8, 12)
```

## Command

```
dd:%s/ *#.*//<cr>:1s/width, height/height, width/<cr>
```

1. `dd` 첫 번째 줄의 단독 주석 삭제
1. `:%s/ *#.*//` 코드 전체에서 인라인 주석(# 이후) 제거
1. `<cr>` 치환 명령 실행
1. `:1s/width, height/height, width/` 1행 함수 정의의 매개변수 순서 맞바꾸기
1. `<cr>` 치환 명령 실행
