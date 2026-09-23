# Create JSON from a .env File

<!-- difficulty: intermediate -->

환경 변수 파일(`.env`)의 주석과 빈 줄을 정리하고 유효한 JSON 형식으로 변환합니다.

## Before

```sh
# API Settings
JOBS_API_URL=http://localhost:5000
JOBS_BASE_URL=http://localhost:8000
SCRAPERS_BASE_URL=http://localhost:9900

# Database Settings
JOBS_DATABASE_URI=mongodb://mongouser:mongopassword@127.0.0.1:27017/app

# Redis Settings
JOBS_REDIS_DSN=redis://127.0.0.1:6000

# Data API
DATA_BASE_URL=http://127.0.0.1:8900

# Minio config
JOBS_MINIO_SECURE=false
JOBS_MINIO_ACCESS_KEY=miniouser
JOBS_MINIO_SECRET_KEY=miniosecret
JOBS_MINIO_HOST=127.0.0.1:9500
JOBS_MINIO_DEFAULT_BUCKET=jobs
JOBS_MINIO_REGION=us-west-1
JOBS_MINIO_CREATE_BUCKETS=false
JOBS_MINIO_RESULTS_FORMAT=results/{job.id}.json
JOBS_MINIO_ARGUMENTS_FORMAT=arguments/{job.id}.json
JOBS_MINIO_SERVICES_PATH=services/{job.service.bucket_name}
JOBS_MINIO_LOGS_BUCKET=

# RabbitMQ Settings
JOBS_RABBITMQ_URI=amqp://rabbitmquser:rabbitmqpassword@127.0.0.1:5672

# Package registry
REGISTRY_TOKEN=g_dka000111222333444

LOG_FORMAT=text

# Slack notifications
SLACK_TOKEN=
LOGGING_CHANNEL=

# Metadata API
TEST_METADATA_BASE_URL=http://127.0.0.1:8801
```

## After

```json
{
    "JOBS_API_URL": "http://localhost:5000",
    "JOBS_BASE_URL": "http://localhost:8000",
    "SCRAPERS_BASE_URL": "http://localhost:9900",
    "JOBS_DATABASE_URI": "mongodb://mongouser:mongopassword@127.0.0.1:27017/app",
    "JOBS_REDIS_DSN": "redis://127.0.0.1:6000",
    "DATA_BASE_URL": "http://127.0.0.1:8900",
    "JOBS_MINIO_SECURE": "false",
    "JOBS_MINIO_ACCESS_KEY": "miniouser",
    "JOBS_MINIO_SECRET_KEY": "miniosecret",
    "JOBS_MINIO_HOST": "127.0.0.1:9500",
    "JOBS_MINIO_DEFAULT_BUCKET": "jobs",
    "JOBS_MINIO_REGION": "us-west-1",
    "JOBS_MINIO_CREATE_BUCKETS": "false",
    "JOBS_MINIO_RESULTS_FORMAT": "results/{job.id}.json",
    "JOBS_MINIO_ARGUMENTS_FORMAT": "arguments/{job.id}.json",
    "JOBS_MINIO_SERVICES_PATH": "services/{job.service.bucket_name}",
    "JOBS_MINIO_LOGS_BUCKET": "",
    "JOBS_RABBITMQ_URI": "amqp://rabbitmquser:rabbitmqpassword@127.0.0.1:5672",
    "REGISTRY_TOKEN": "g_dka000111222333444",
    "LOG_FORMAT": "text",
    "SLACK_TOKEN": "",
    "LOGGING_CHANNEL": "",
    "TEST_METADATA_BASE_URL": "http://127.0.0.1:8801"
}
```

## Command

```
:v/=/d<cr>

ggO{<esc>Go}<esc>

:2,$-1s/^\([^=]*\)=\(.*\)$/    "\1": "\2",/<cr>

:$-1s/,$//<cr>
```

1. `:v/=/d` 등호(=)가 없는 모든 줄(주석 및 빈 줄) 삭제
1. `<cr>` 명령 실행
1. `gg` 문서의 첫 줄로 이동
1. `O` 윗 줄에 새 줄 추가하고 입력 모드 전환
1. `{` 여는 중괄호 입력
1. `<esc>` 일반 모드로 복귀
1. `G` 문서의 마지막 줄로 이동
1. `o` 아랫 줄에 새 줄 추가하고 입력 모드 전환
1. `}` 닫는 중괄호 입력
1. `<esc>` 일반 모드로 복귀
1. `:2,$-1s/^\([^=]*\)=\(.*\)$/    "\1": "\2",/` 2번 줄부터 마지막 직전 줄까지 KEY=VALUE 형식을 JSON 키-값으로 치환
1. `<cr>` 치환 명령 실행
1. `:$-1s/,$//` 마지막 데이터 줄의 불필요한 쉼표 제거
1. `<cr>` 치환 명령 실행
