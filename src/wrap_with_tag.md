# Wrap with Tag

<!-- difficulty: beginner -->

각 줄을 여는 태그와 닫는 태그로 감쌉니다.

## Before

```text
apple
banana
```

## After

```text
<li>apple</li>
<li>banana</li>
```

## Command

```
:%s/.*/<lt>li>&<lt>\/li>/<cr>
```

1. `:%s/.*/` 전체 파일의 각 줄 내용 매칭
1. `<lt>li>` 줄 앞에 여는 태그 삽입
1. `&` 매칭된 원래 텍스트
1. `<lt>\/li>/` 줄 뒤에 닫는 태그 삽입
1. `<cr>` 명령 실행
