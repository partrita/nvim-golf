# Function into Class

<!-- difficulty: advanced -->

3개의 함수를 3개의 메서드를 가진 클래스로 변환합니다.

## Before

```py
def calculate_area(length, width):
    result = length * width
    return result

def calculate_perimiter(length, width):
    result = 2 * (length + width)
    return result

def calculate_volume(length, width, height):
    result = length * width * height
    return result
```

## After

```py
class Calculator:
    @staticmethod
    def get_area(len, wid):
        return len * wid

    @staticmethod
    def get_perimiter(len, wid):
        return 2 * (len + wid)

    @staticmethod
    def get_volume(len, wid, hei):
        return len * wid * hei
```

## Command

```
:%s/calculate/get/g<cr>

:%s/def/    @staticmethod\r    def/g<cr>

:%s/length/len/g<cr>

:%s/width/wid/g<cr>

:%s/height/hei/g<cr>

:%s/    result = \(.*\)\n    return result/

        return \1/g<cr>

ggOclass Calculator:<esc>
```

1. `:%s/calculate/get/g<cr>` calculate를 get으로 변경
1. `:%s/def/    @staticmethod\r    def/g<cr>` 각 함수 위에 @staticmethod 데코레이터 추가 및 인덴트
1. `:%s/length/len/g<cr>` length 파라미터 축약
1. `:%s/width/wid/g<cr>` width 파라미터 축약
1. `:%s/height/hei/g<cr>` height 파라미터 축약
1.
    ```
    :%s/    result = \(.*\)\n    return result/        return \1/g<cr>
    ```
1. `gg` 파일 첫 줄로 이동
1. `O` 윗 줄에 새 줄 생성 후 삽입 모드 진입
1. `class Calculator:` 클래스 선언 입력
1. `<esc>` 노멀 모드 복귀
