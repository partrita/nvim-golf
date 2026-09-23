# YAML to dotenv

<!-- difficulty: beginner -->

YAML 설정 파일에서 환경 변수 치환 구문(`!ENV`)을 찾아 `.env` 키 목록으로 변환합니다.

## Before

```yaml
vimgolf:
  logging:
    level: INFO
app:
  postgres:
    host: !ENV {POSTGRES_HOST}
    port: !ENV {POSTGRES_PORT}
  pulsar:
    host: !ENV ${PULSAR_HOST}
    port: !ENV ${PULSAR_PORT}
    namespace: vimgolf
    topic: !ENV ${PULSAR_TOPIC}
```

## After

```sh
POSTGRES_HOST=
POSTGRES_PORT=
PULSAR_HOST=
PULSAR_PORT=
PULSAR_TOPIC=
```

## Command

```
:v/!ENV/d<cr>:%s/.*{\([^}]*\)}.*/\1=/<cr>
```

1. `:v/!ENV/d` !ENV 지시어가 없는 모든 설정 줄 삭제
1. `<cr>` 명령 실행
1. `:%s/.*{\([^}]*\)}.*/\1=/` 중괄호 안의 환경 변수 이름을 추출하여 VAR= 형식으로 치환
1. `<cr>` 치환 명령 실행
