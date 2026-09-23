# Contributing Neovim Golf Examples

이 저장소에 새로운 Neovim Golf 문제(예제)를 추가할 때는 아래 규칙을 따라주세요.

## 문제 구성

- 모든 문제는 `src/<problem_name>.md`에 추가합니다.
- 파일 이름은 소문자 `snake_case`를 사용하고, 예제의 핵심 작업을 짧게 표현합니다.
- 제목은 사용자가 수행할 편집 작업을 명확하게 설명합니다.
- 설명은 한두 문장으로 작성하고, Neovim의 특정 기능이나 편집 목표가 드러나게 합니다.
- 반드시 다음 세 개의 섹션을 순서대로 포함합니다.
  1. `## Before`
  2. `## After`
  3. `## Command`
- 제목 바로 아래에 난이도 주석을 추가합니다. 값은 `beginner`, `intermediate`, `advanced` 중 하나입니다.
  ```md
  # Sort Lines

  <!-- difficulty: intermediate -->
  ```
  주석을 생략하면 명령 길이를 기준으로 자동 분류됩니다.
- `## Steps`는 필요할 때만 추가합니다. 기존 예제처럼 `Command` 아래 번호 목록으로 단계를 설명해도 됩니다.

## Before / After

- `Before`는 실제로 Neovim에서 편집할 원본이어야 합니다.
- `After`는 문제에서 요구하는 최종 상태여야 합니다.
- 코드 블록의 언어 태그를 실제 파일 형식에 맞게 지정합니다. 예: `rs`, `py`, `js`, `toml`.
- 가능하면 하나의 명확한 편집 목표에 집중합니다. 지나치게 많은 조작을 한 문제에 넣지 않습니다.
- `Before`와 `After`의 차이는 Neovim의 편집 기능을 연습할 수 있도록 충분히 의미 있어야 합니다.
- `Before`와 `After`의 공백, 줄바꿈, 들여쓰기도 실제 결과와 정확히 일치해야 합니다. 데모 테스트는 실행 결과를 문자열로 비교합니다.

## Command

### 가장 중요한 규칙: 실제 Neovim에서 검증할 것

- Neovim의 기본 키맵, 모달 편집(Normal, Insert, Visual, Visual Block 등), Ex 명령어(`:%s/.../`, `:%!sort` 등), 텍스트 객체(`iw`, `i"`, `a)` 등)을 적절히 활용합니다.
- `Command`에 넣는 키 입력은 문서나 기억에 의존해서 작성하지 말고, 반드시 이 프로젝트의 Neovim/VHS 환경에서 실제로 실행하여 확인합니다.
- 실제 Neovim에서 재현 가능한 키 입력만 기록합니다.
- 여러 키 입력을 한 줄에 이어 쓸 수 있으며, `<space>`, `<esc>`, `<ret>`, `<cr>`, `<C-v>`, `<ctrl-v>`처럼 특수 키는 명확한 표기를 사용합니다.
- 명령은 반드시 `Before`에서 `After`로 정확히 변환되어야 합니다.
- 복잡한 명령을 추가했다면 번호 목록으로 각 키 입력의 역할을 설명합니다.

### Command 코드 블록 형식

- `## Command` 바로 아래에는 **명령을 담은 fenced code block이 하나 있어야 합니다.**
- 번호 목록이나 설명 문장을 `## Command`와 명령 코드 블록 사이에 넣지 않습니다.
- 실행 명령은 코드 블록 안에만 작성합니다.
- 설명이 필요하면 명령 코드 블록 **뒤에** 번호 목록으로 작성합니다.
- 명령 코드 블록에는 실제 입력하는 키만 넣고, 설명이나 주석을 넣지 않습니다.

올바른 예:

```md
## Command

```
ciwtotal<esc>
```

1. `ciw` 단어 내부를 변경 모드로 전환
1. `total` 새 값을 입력
1. `<esc>` 일반 모드로 복귀
```

## 검증

문제를 추가하거나 수정한 뒤 프로젝트 루트에서 다음 명령을 실행합니다.

### 1. Markdown 구조 검증

```sh
cargo validate
```

반드시 성공해야 합니다.

### 2. 실제 데모 실행 검증

```sh
cargo generate-demos <problem_name>
```

이 단계는 선택 사항이 아닙니다. `cargo validate`가 성공하더라도 실제 Neovim 키 입력이 잘못되면 데모 생성 단계에서 실패하거나 `Before`와 `After`가 일치하지 않을 수 있습니다.

`cargo generate-demos`는 다음을 모두 검증하는 최종 테스트로 취급합니다.

- Markdown에서 Command가 올바르게 파싱되는지
- VHS가 Command의 키 입력을 실제로 실행하는지
- 실행 결과가 `After`와 정확히 일치하는지
- 공백과 줄바꿈까지 포함하여 최종 파일 내용이 정확한지

### 3. 실패 시 제출하지 않기

- `cargo validate` 또는 `cargo generate-demos`가 실패한 상태로 문제를 제출하지 않습니다.
- 실패 메시지의 예상 결과와 실제 결과를 비교하여 **Command를 수정한 후 다시 실행**합니다.
- 단순히 `After`를 실제 실행 결과에 맞춰 바꾸지 마세요. 문제의 의도에 맞는 올바른 Neovim 명령을 먼저 찾아야 합니다.
- GitHub Actions의 `Build`가 실패하면 PR을 완료된 것으로 간주하지 않습니다. 로컬에서 동일한 `cargo generate-demos <problem_name>` 검증을 수행하고 수정합니다.

## 체크리스트

새 문제를 제출하기 전에 모두 확인하세요.

- [ ] `src/<problem_name>.md` 파일명이 `snake_case`인가?
- [ ] 제목 바로 아래에 난이도 주석(`<!-- difficulty: ... -->`)이 있는가?
- [ ] 제목과 설명만 읽어도 편집 목표가 이해되는가?
- [ ] `Before`와 `After`가 공백/줄바꿈까지 정확히 대응하는가?
- [ ] `Command` 바로 아래에 명령 fenced code block이 있는가?
- [ ] Command 코드 블록 앞에 설명이나 번호 목록이 없는가?
- [ ] Command 코드 블록에는 실제 키 입력만 있는가?
- [ ] 각 명령이 실제 Neovim에서 동작하는 것을 확인했는가?
- [ ] 각 명령의 목적을 설명했는가?
- [ ] `cargo validate`가 통과하는가?
- [ ] `cargo generate-demos <problem_name>`가 통과하는가?
- [ ] 데모 실행 결과가 `After`와 정확히 일치하는가?
- [ ] GitHub Actions `Build`가 성공하는가?
