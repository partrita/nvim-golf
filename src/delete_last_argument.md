# Delete the Last Function Argument in Python

<!-- difficulty: beginner -->

함수 호출문의 마지막 인자와 앞의 공백을 모션 명령으로 빠르게 삭제합니다.

## Before

```py
print(pairs, len(pairs))
```

## After

```py
print(pairs,)
```

## Command

```
f,ldf)
```

1. `f,` 쉼표(,) 위치로 커서 이동
1. `l` 오른쪽 공백으로 한 칸 이동
1. `df)` 다음 닫는 소괄호())까지 공백과 마지막 인자 일괄 삭제
