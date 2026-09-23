//! Validate that all of the examples adhere to a certain structure

use std::{collections::HashSet, env, fmt::Write as _, fs};

use crate::{
    command::{GENERATED_DIR, ROOT_DIR},
    parse_example::{Difficulty, Example},
};
use miette::miette;

/// Make sure each example has the required structure.
///
/// Also regenerates `SUMMARY.md` and `introduction.md`, grouping the examples
/// by [`Difficulty`]. When only a subset of examples is requested (e.g.
/// `cargo validate <name>`), the generated index files are left untouched so
/// that a filtered run cannot clobber the full book index.
pub fn validate() -> miette::Result<Vec<Example>> {
    // If user passes any examples, those will be the only ones that are included.
    //
    // If no examples are passed, then include everything
    let only_include_these_examples: HashSet<_> = env::args()
        // 1. skip binary name
        // 2. skip argument type
        .skip(2)
        .collect();

    if only_include_these_examples.is_empty() {
        fs::create_dir_all(&*GENERATED_DIR)
            .and_then(|()| fs::remove_dir_all(&*GENERATED_DIR))
            .and_then(|()| fs::create_dir_all(&*GENERATED_DIR))
            .map_err(|err| miette!("failed cleaning the generated directory: {err}"))?;
    } else {
        fs::create_dir_all(&*GENERATED_DIR)
            .map_err(|err| miette!("failed creating the generated directory: {err}"))?;
    }

    let mut examples = Example::parse_all(&ROOT_DIR, &only_include_these_examples)?;

    // We want to sort examples from smallest command count to largest
    examples.sort_by_key(|a| a.key_events.len());

    // Group by difficulty, keeping the shortest-command-first order within each group.
    // `sort_by_key` is stable, so pushing in order preserves it per group.
    let mut grouped: [Vec<&Example>; 3] = [Vec::new(), Vec::new(), Vec::new()];
    for example in &examples {
        let level = example
            .difficulty
            .unwrap_or_else(|| Difficulty::fallback(example.key_events.len()));
        grouped[level as usize].push(example);
    }

    // A filtered run (e.g. `cargo validate <name>`) must not clobber the full
    // book index that a previous unfiltered run generated.
    if only_include_these_examples.is_empty() {
        let mut summary_md = String::from(
            "<!-- @generated This file is generated. Do not edit it by hand. -->

# Summary

- [Neovim Golf - Introduction](introduction.md)\n",
        );
        let mut all_previews = String::new();
        let mut md_file_with_everything = String::new();

        for (level, group) in Difficulty::ALL.iter().zip(grouped.iter()) {
            if group.is_empty() {
                continue;
            }

            writeln!(summary_md, "\n# {}\n", level.title())
                .map_err(|err| miette!("failed to add part title to SUMMARY.md: {err}"))?;

            writeln!(all_previews, "## {}\n", level.title())
                .map_err(|err| miette!("failed to add section to introduction.md: {err}"))?;

            for example in group {
                let name = &example.name;
                let title = &example.title;

                writeln!(summary_md, "- [{title}]({name}.md)").map_err(|err| {
                    miette!("failed to add line to SUMMARY.md for example `{name}`: {err}",)
                })?;

                writeln!(
                    all_previews,
                    "### [{title}]({name}.md)

{desc}

<video autoplay controls loop>
  <source src=\"generated/{name}.mp4\">
</video>\n",
                    desc = example.description.as_deref().unwrap_or("")
                )
                .map_err(|err| {
                    miette!("failed to add preview to introduction.md for example `{name}`: {err}",)
                })?;
            }
        }

        // The offline copy keeps every example in shortest-command-first order.
        for example in &examples {
            writeln!(md_file_with_everything, "{}", example.contents).map_err(|err| {
                miette!(
                    "failed to add entire example to \
                    introduction.md for example `{name}`: {err}",
                    name = example.name
                )
            })?;
        }

        fs::write(ROOT_DIR.join("SUMMARY.md"), summary_md)
            .map_err(|err| miette!("Failed to write `SUMMARY.md`: {err}"))
            .map(|()| {
                fs::write(
                    ROOT_DIR.join("introduction.md"),
                    format!(
                        "<!-- @generated This file is generated. Do not edit it by hand. -->

# Neovim Golf

Neovim Golf는 차세대 확장형 텍스트 에디터인
[Neovim](https://github.com/neovim/neovim)을 사용한 리팩토링 및 텍스트 조작 예제 모음입니다.

각 예제는 상세히 설명되어 있으며 최신 버전의 Neovim으로 테스트되었고 비디오 데모가 포함되어 있습니다.
예제들은 단순한 가상이 아니며, 모두 실제 개발/편집 상황에서 유용한 조작들로 구성되었습니다.

모달 편집(Modal Editing), 텍스트 객체(Text Objects), 레지스터, 글로벌 치환, 필터 명령 등 Neovim의 강력한 기능들을
통해 더 효율적이고 직관적으로 텍스트를 편집하는 방법을 배울 수 있습니다.

생산성이 향상될 뿐만 아니라 문제를 최소한의 키스트로크로 해결하는 재미를 느낄 수 있습니다!

예제는 난이도별로 세 그룹으로 나뉩니다. 처음이라면 Beginner부터 순서대로 풀어보세요.

## 키 표기법 안내 (Key Legend)

예제의 `Command`에 표시되는 특수 키 표기법은 다음과 같습니다:

| 표기 | 키 | 설명 |
| :--- | :--- | :--- |
| `<cr>` 또는 `<ret>` | Enter / Return | 엔터 키 (명령줄 실행 또는 줄바꿈) |
| `<esc>` | Esc | Escape 키 (일반 모드로 복귀 또는 취소) |
| `<space>` | Space | 스페이스(공백) 키 |
| `<tab>` | Tab | 탭 키 |
| `<bs>` | Backspace | 백스페이스 키 (이전 글자 삭제) |
| `<C-v>` | Ctrl + v | 시각적 블록(Visual Block) 모드 진입 |
| `<C-a>` | Ctrl + a | 커서 위치의 숫자 1 증가 |
| `<C-x>` | Ctrl + x | 커서 위치의 숫자 1 감소 |
| `<C-r>` | Ctrl + r | 명령줄/입력 모드 레지스터 삽입 또는 Redo |

# 각 예제 데모

<details>

<summary>모든 예제는 하나의 코드 블록으로도 제공됩니다</summary>

인터넷 없이도 Neovim에 복사하여 붙여넣고 직접 연습해 볼 수 있습니다!

````````````md
{md_file_with_everything}
````````````

</details>

{all_previews}"
                    ),
                )
            })?
            .map_err(|err| miette!("Failed to write `introduction.md`: {err}"))?;
    }

    Ok(examples)
}
