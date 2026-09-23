# Neovim golf

Neovim은 텍스트 편집에 _매우_ 강력한 에디터입니다. 이 웹사이트는 Neovim을 활용하여 코드 스니펫을 어떻게 리팩토링했는지 보여주는 예제들을 모아둔 곳입니다.

## 키 표기법 안내 (Key Legend)

예제의 `Command`에 표시되는 특수 키 표기법은 다음과 같습니다:

| 표기 | 키 | 설명 |
| :--- | :--- | :--- |
| `<cr>` 또는 `<ret>` | Enter / Return | 엔터 키 (명령줄 실행 또는 줄바꿈) |
| `<esc>` | Esc | Escape 키 (일반 모드로 복귀 또는 작업 취소) |
| `<space>` | Space | 스페이스(공백) 키 |
| `<tab>` | Tab | 탭 키 |
| `<bs>` | Backspace | 백스페이스 키 (이전 글자 삭제) |
| `<C-v>` | Ctrl + v | 시각적 블록(Visual Block) 모드 진입 |
| `<C-a>` | Ctrl + a | 커서 위치의 숫자 1 증가 |
| `<C-x>` | Ctrl + x | 커서 위치의 숫자 1 감소 |
| `<C-r>` | Ctrl + r | 명령줄/입력 모드 레지스터 삽입 또는 Redo |

## 기여하기 (Contributing)

새로운 예제를 제안하고 싶다면 이슈를 생성해 주세요. 확인 후 추가하겠습니다.

---

직접 새 예제를 추가하고 싶다면 아래 템플릿을 사용하여 `src/your_example.md` 파일을 생성하세요.

````md
# Title

`h`를 대문자로 변경하고 느낌표를 추가했습니다.

## Before

```
hello world
```

## After

```
Hello world!
```

## Command

```
~A!
```

1. `~` 선택 영역의 대소문자 전환
1. `A` 줄 끝으로 이동하고 삽입 모드 진입
1. `!` 느낌표 입력
````

### 필수 프로그램 (Dependencies)

- [Neovim](https://neovim.io/)
- [Rust](https://www.rust-lang.org/tools/install)
- [mdbook](https://rust-lang.github.io/mdBook/guide/installation.html)
- [VHS](https://github.com/charmbracelet/vhs?tab=readme-ov-file#installation) (데모 파일 생성 및 예제 정확성 테스트용)

위 도구들을 직접 설치하지 않고도 기여하고 싶다면, [`src/`](src/) 폴더의 마크다운 예제 파일을 수정한 뒤 Pull Request를 보내면 GitHub CI가 자동으로 PR을 테스트합니다.

### 유효성 검증 (Validate)

프로젝트 루트에서 다음 명령어를 실행하여 예제 구조가 올바른지 확인하세요:

```sh
cargo validate
```

### 데모 생성 (Generate Demos)

다음 명령어를 실행하여 각 예제의 데모를 생성하고 테스트할 수 있습니다:

```sh
cargo generate-demos
```

특정 데모만 지정해서 생성할 수도 있습니다:

```sh
cargo generate-demos export_from_mod
```

### 로컬에서 실행하기 (Running locally)

다음 명령어로 웹사이트를 로컬에서 실행할 수 있습니다:

```sh
mdbook serve
```

`http://localhost:3000`에서 접속할 수 있습니다.

