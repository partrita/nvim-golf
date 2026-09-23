<!-- @generated This file is generated. Do not edit it by hand. -->

# Neovim Golf

Neovim Golf는 차세대 확장형 텍스트 에디터인
[Neovim](https://github.com/neovim/neovim)을 사용한 리팩토링 및 텍스트 조작 예제 모음입니다.

각 예제는 상세히 설명되어 있으며 최신 버전의 Neovim으로 테스트되었고 비디오 데모가 포함되어 있습니다.
예제들은 단순한 가상이 아니며, 모두 실제 개발/편집 상황에서 유용한 조작들로 구성되었습니다.

모달 편집(Modal Editing), 텍스트 객체(Text Objects), 레지스터, 글로벌 치환, 필터 명령 등 Neovim의 강력한 기능들을
통해 더 효율적이고 직관적으로 텍스트를 편집하는 방법을 배울 수 있습니다.

생산성이 향상될 뿐만 아니라 문제를 최소한의 키스트로크로 해결하는 재미를 느낄 수 있습니다!

예제는 난이도별로 세 그룹으로 나뉩니다. 처음이라면 Beginner부터 순서대로 풀어보세요.

# 각 예제 데모

<details>

<summary>모든 예제는 하나의 코드 블록으로도 제공됩니다</summary>

인터넷 없이도 Neovim에 복사하여 붙여넣고 직접 연습해 볼 수 있습니다!

````````````md
# Join Lines

<!-- difficulty: beginner -->

두 줄을 한 줄로 합칩니다.

## Before

```text
hello
world
```

## After

```text
hello world
```

## Command

```
J
```

1. `J` 현재 줄과 다음 줄을 공백을 두고 합치기

# Delete Blank Line

<!-- difficulty: beginner -->

빈 줄을 삭제합니다.

## Before

```text
apple

banana
```

## After

```text
apple
banana
```

## Command

```
jdd
```

1. `j` 빈 줄로 이동
1. `dd` 현재 줄 삭제

# Swap Lines

<!-- difficulty: beginner -->

두 줄의 순서를 바꿉니다.

## Before

```text
first
second
```

## After

```text
second
first
```

## Command

```
ddp
```

1. `dd` 첫 번째 줄을 잘라내기
1. `p` 아래 줄에 붙여넣기

# Rotate Main Selection

<!-- difficulty: beginner -->

마지막 숫자만 바꿉니다.

## Before

```text
a1 a2 a3
```

## After

```text
a1 a2 a9
```

## Command

```
$r9
```

1. `$` 줄 끝으로 이동
1. `r` 한 글자 교체
1. `9` 교체할 문자 9 입력

# Extend Delete Words

<!-- difficulty: beginner -->

앞의 두 단어를 지웁니다.

## Before

```text
one two three four
```

## After

```text
three four
```

## Command

```
2dw
```

1. `2dw` 두 단어를 삭제

# Duplicate Line

<!-- difficulty: beginner -->

한 줄을 복사해 세 줄로 늘립니다.

## Before

```text
log
```

## After

```text
log
log
log
```

## Command

```
yyp.
```

1. `yy` 현재 줄 복사
1. `p` 아래 줄에 붙여넣기
1. `.` 이전 작업(붙여넣기) 반복

# Toggle Word Case

<!-- difficulty: beginner -->

단어의 대소문자를 뒤집습니다.

## Before

```text
hello
```

## After

```text
HELLO
```

## Command

```
gUiw
```

1. `gUiw` 현재 단어를 모두 대문자로 변환

# Increment Numbers

<!-- difficulty: beginner -->

여러 숫자를 한 번에 1씩 증가시킵니다.

## Before

```text
0
0
0
```

## After

```text
1
1
1
```

## Command

```
<ctrl-a>j<ctrl-a>j<ctrl-a>
```

1. `<ctrl-a>` 첫 번째 줄 숫자 1 증가
1. `j` 다음 줄로 이동
1. `<ctrl-a>` 두 번째 줄 숫자 1 증가
1. `j` 다음 줄로 이동
1. `<ctrl-a>` 세 번째 줄 숫자 1 증가

# Toggle Comment

<!-- difficulty: beginner -->

여러 줄을 한 번에 주석으로 바꿉니다.

## Before

```py
print("a")
print("b")
print("c")
```

## After

```py
# print("a")
# print("b")
# print("c")
```

## Command

```
<ctrl-v>2jI# <esc>
```

1. `<ctrl-v>` Visual Block 모드 진입
1. `2j` 아래 2줄 확장 선택
1. `I` 블록 앞 삽입 모드 진입
1. `# ` 주석 기호와 공백 입력
1. `<esc>` 노멀 모드로 복귀하여 전체 적용

# Format JSON with jq

<!-- difficulty: beginner -->

한 줄 JSON을 셸 필터로 보기 좋게 펼칩니다.

## Before

```json
{"name": "helix", "stars": 100}
```

## After

```json
{
  "name": "helix",
  "stars": 100
}
```

## Command

```
:%!jq .<cr>
```

1. `:%!` 전체 버퍼를 외부 명령의 표준 입출력으로 전달
1. `jq .` jq 포맷팅 도구 실행
1. `<cr>` 명령 실행

# Sort Lines

<!-- difficulty: beginner -->

정렬되지 않은 줄들을 정렬합니다.

## Before

```text
banana
apple
cherry
```

## After

```text
apple
banana
cherry
```

## Command

```
:%!sort<cr>
```

1. `:%!` 전체 버퍼를 외부 명령에 전달
1. `sort` sort 정렬 명령
1. `<cr>` 명령 실행

# Replace an Identifier

<!-- difficulty: beginner -->

커서가 놓인 변수 이름을 다른 이름으로 변경합니다.

## Before

```rs
count
```

## After

```rs
total
```

## Command

```
ciwtotal<esc>
```

1. `ciw` 단어 내부를 삭제하고 삽입 모드로 전환
1. `total` 새 변수 이름 입력
1. `<esc>` 노멀 모드로 복귀

# Add Semicolons

<!-- difficulty: beginner -->

각 줄 끝에 세미콜론을 붙입니다.

## Before

```js
const a = 1
const b = 2
const c = 3
```

## After

```js
const a = 1;
const b = 2;
const c = 3;
```

## Command

```
:%s/$/;/g<cr>
```

1. `:%s` 전체 파일 대상 치환 명령
1. `/$/` 각 줄의 끝($) 위치를 검색
1. `;/` 세미콜론으로 치환
1. `g` 모든 일치 항목에 적용
1. `<cr>` 명령 실행

# CSV to Lines

<!-- difficulty: beginner -->

쉼표로 구분된 한 줄을 여러 줄로 나눕니다.

## Before

```text
apple,banana,cherry
```

## After

```text
apple
banana
cherry
```

## Command

```
:s/,/\r/g<cr>
```

1. `:s` 현재 줄 치환 명령
1. `/,/` 쉼표(,) 검색
1. `\r/` 줄바꿈(\r)으로 교체
1. `g` 줄 내 모든 일치 항목에 적용
1. `<cr>` 명령 실행

# Replace Punctuation

<!-- difficulty: beginner -->

여러 줄의 앞 기호를 한 번에 바꿉니다.

## Before

```text
- apple
- banana
- cherry
```

## After

```text
* apple
* banana
* cherry
```

## Command

```
:%s/^-/*/<cr>
```

1. `:%s/^-/` 줄 시작 부분의 하이픈(-) 매칭
1. `*/` 별표(*)로 교체
1. `<cr>` 명령 실행

# Indent Lines

<!-- difficulty: beginner -->

여러 줄을 한 번에 들여씁니다.

## Before

```text
apple
banana
cherry
```

## After

```text
  apple
  banana
  cherry
```

## Command

```
:%s/^/  /g<cr>
```

1. `:%s` 전체 파일 치환
1. `/^/` 각 줄의 시작(^) 위치 매칭
1. `  /` 두 칸 공백 삽입
1. `g` 전역 적용
1. `<cr>` 명령 실행

# Multicursor Prefix

<!-- difficulty: intermediate -->

Visual Block 모드로 여러 줄 앞에 접두사를 동시에 삽입합니다.

## Before

```text
apple
banana
cherry
```

## After

```text
fruit: apple
fruit: banana
fruit: cherry
```

## Command

```
<ctrl-v>2jIfruit: <esc>
```

1. `<ctrl-v>` Visual Block 모드 진입
1. `2j` 아래로 2줄 확장
1. `I` 블록 앞 삽입 모드 진입
1. `fruit: ` 접두사 입력
1. `<esc>` 노멀 모드로 복귀하여 모든 줄에 동시 적용

# Fix Typo with Search

<!-- difficulty: beginner -->

검색으로 오타를 찾아 고칩니다.

## Before

```text
roses are red
violets are blu
sugar is sweet
```

## After

```text
roses are red
violets are blue
sugar is sweet
```

## Command

```
/blu<cr>cwblue<esc>
```

1. `/blu<cr>` 오타 검색 후 이동
1. `cw` 단어를 삭제하고 삽입 모드로 전환
1. `blue` 올바른 단어 입력
1. `<esc>` 노멀 모드로 복귀

# Delete Surround

<!-- difficulty: beginner -->

여러 줄의 감싸는 괄호를 한 번에 제거합니다.

## Before

```text
(use)
(use)
```

## After

```text
use
use
```

## Command

```
:%s/[()]//g<cr>
```

1. `:%s` 전체 파일 대상 치환
1. `/[()]//` 여는 괄호와 닫는 괄호를 빈 문자열로 삭제
1. `g` 모든 일치 항목에 적용
1. `<cr>` 명령 실행

# Align Assignments

<!-- difficulty: intermediate -->

할당 연산자를 열에 맞춰 정렬합니다.

## Before

```text
a = 1
longer = 2
mid = 3
```

## After

```text
a      = 1
longer = 2
mid    = 3
```

## Command

```
w5i <esc>jj^w3i <esc>
```

1. `w` 등호(=) 위치로 이동
1. `5i <esc>` 등호 앞에 공백 5칸 삽입 후 노멀 모드 복귀
1. `jj` 세 번째 줄로 이동
1. `^w` 줄 첫 글자에서 등호(=) 위치로 이동
1. `3i <esc>` 등호 앞에 공백 3칸 삽입 후 노멀 모드 복귀

# Insert Sequence

<!-- difficulty: beginner -->

셸 출력을 커서 앞에 삽입합니다.

## Before

```text
items:
```

## After

```text
1
2
3
items:
```

## Command

```
:0read !seq 3<cr>
```

1. `:0read !seq 3` 파일 맨 앞(0번 라인)에 seq 3 명령 출력 삽입
1. `<cr>` 명령 실행

# Replace a Regex-Sensitive Literal

<!-- difficulty: intermediate -->

문서 전체에서 정규식 메타문자가 포함된 문자열을 다른 문자열로 한 번에 변경합니다.

## Before

```text
C++ is widely used.
I learned C++ before Rust.
This project does not use C++ anymore.
```

## After

```text
Rust is widely used.
I learned Rust before Rust.
This project does not use Rust anymore.
```

## Command

```
:%s/C++/Rust/g<cr>
```

1. `:%s/` 전체 대상 치환
1. `C++/` C++ 검색
1. `Rust/` Rust로 치환
1. `g` 줄 내 모든 일치 항목 적용
1. `<cr>` 명령 실행

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

# Replace a Selection with the System Clipboard

<!-- difficulty: intermediate -->

레지스터를 활용하여 문자열을 복사하고 다른 위치의 문자열을 교체합니다.

## Before

```rs
fn greet() {
    println!("Hello, world!");
}

fn main() {
    println!("Hello, Rust!");
}
```

## After

```rs
fn greet() {
    println!("Hello, Rust!");
}

fn main() {
    println!("Hello, Rust!");
}
```

## Command

```
/Rust<cr>yiwgg/world<cr>viwp
```

1. `/Rust<cr>` Rust 검색 및 이동
1. `yiw` 단어(Rust) 복사
1. `gg` 파일 시작으로 이동
1. `/world<cr>` world 검색 및 이동
1. `viw` 단어(world) 비주얼 선택
1. `p` 복사해둔 텍스트로 덮어쓰기

# Replace Surrounding Characters

<!-- difficulty: beginner -->

괄호를 대괄호로 변경합니다.

## Before

```text
(use)
(use)
```

## After

```text
[use]
[use]
```

## Command

```
:%s/(\([^)]*\))/[\1]/g<cr>
```

1. `:%s/(\([^)]*\))/` 소괄호와 내부 내용 매칭 및 캡처
1. `[\1]/g` 대괄호로 감싸도록 교체
1. `<cr>` 명령 실행

# Swap Columns

<!-- difficulty: beginner -->

두 열의 순서를 바꿉니다.

## Before

```text
apple 1
banana 2
```

## After

```text
1 apple
2 banana
```

## Command

```
:%s/\(.*\) \(.*\)/\2 \1/<cr>
```

1. `:%s/\(.*\) \(.*\)/` 공백 기준 두 열 매칭 및 그룹 캡처
1. `\2 \1/` 두 번째 열과 첫 번째 열의 위치 교환
1. `<cr>` 명령 실행

# Text into Array

<!-- difficulty: intermediate -->

줄바꿈으로 구분된 데이터를 문자열 배열로 결합합니다.

## Before

```text
Hello
This
Is
Helix
```

## After

```js
["Hello", "This", "Is", "Helix"]
```

## Command

```
:%s/.*/"&"/<cr>:%s/\n\ze./, /g<cr>I[<esc>A]<esc>
```

1. `:%s/.*/"&"/<cr>` 각 줄을 큰따옴표로 감싸기
1. `:%s/\n\ze./, /g<cr>` 다음 줄이 존재하는 줄바꿈만 쉼표와 공백으로 치환
1. `I[` 줄 맨 앞에 여는 대괄호([) 삽입
1. `<esc>` 노멀 모드 복귀
1. `A]` 줄 맨 끝에 닫는 대괄호(]) 삽입
1. `<esc>` 노멀 모드 복귀

# Swap Quoted Strings

<!-- difficulty: beginner -->

두 따옴표 내용의 위치를 바꿉니다.

## Before

```text
"foo" "bar"
```

## After

```text
"bar" "foo"
```

## Command

```
:s/"\([^"]*\)" "\([^"]*\)"/"\2" "\1"/<cr>
```

1. `:s/"\([^"]*\)" "\([^"]*\)"/` 두 개의 큰따옴표 문자열 매칭 및 캡처
1. `"\2" "\1"/` 두 문자열의 위치 교환
1. `<cr>` 명령 실행

# Export from Rust Module

<!-- difficulty: intermediate -->

각 모듈에 포함된 함수를 re-export(pub use)합니다.

## Before

```rs
mod generate_demos;
mod mdbook_preprocessor;
mod validate;
```

## After

```rs
mod generate_demos;
mod mdbook_preprocessor;
mod validate;

pub use generate_demos::generate_demos;
pub use mdbook_preprocessor::mdbook_preprocessor;
pub use validate::validate;
```

## Command

```
:%t$<cr>:4s/^/\r/<cr>:5,$s/mod \(.*\);/pub use \1::\1;/g<cr>
```

1. `:%t$<cr>` 전체 내용을 버퍼 맨 끝($)으로 복제
1. `:4s/^/\r/<cr>` 4번째 줄 앞에 빈 줄 삽입
1. `:5,$s/mod \(.*\);/pub use \1::\1;/g<cr>` 5행부터 끝까지 mod 구문을 pub use 모듈::함수로 치환

# Object into Array

<!-- difficulty: intermediate -->

객체를 2차원 배열로 변환합니다.

## Before

```js
const palette = {
  apricot: "#f47868",
  lightning: "#ffcd1c",
  delta: "6f44f0",
};
```

## After

```js
const palette = [
  ["apricot", "#f47868"],
  ["lightning", "#ffcd1c"],
  ["delta", "6f44f0"],
];
```

## Command

```
:%s/{/[/<cr>:%s/}/]/<cr>

:%s/  \([a-z]*\): \([^,]*\),/  ["\1", \2],/g<cr>
```

1. `:%s/{/[/<cr>` 여는 중괄호를 여는 대괄호로 치환
1. `:%s/}/]/<cr>` 닫는 중괄호를 닫는 대괄호로 치환
1. `:%s/  \([a-z]*\): \([^,]*\),/  ["\1", \2],/g<cr>` 키-값 쌍을 배열 요소 형식으로 치환

# snake_case to camelCase

<!-- difficulty: intermediate -->

모든 필드명을 camelCase로 변경합니다. (이메일 주소의 밑줄 제외)

## Before

```js
const user_profile = {
  first_name: "John",
  last_name: "Doe",
  birth_date: "1990-05-15",
  email_address: "john_doe@example.com",
  phone_number: "555-123-4567",
  mailing_address: {
    street_name: "Main Street",
    house_number: 123,
    apartment_unit: "4B",
    zip_code: "10001",
    city_name: "New York",
  },
};
```

## After

```js
const userProfile = {
  firstName: "John",
  lastName: "Doe",
  birthDate: "1990-05-15",
  emailAddress: "john_doe@example.com",
  phoneNumber: "555-123-4567",
  mailingAddress: {
    streetName: "Main Street",
    houseNumber: 123,
    apartmentUnit: "4B",
    zipCode: "10001",
    cityName: "New York",
  },
};
```

## Command

```
:%s/_\([a-z]\)\([^@]*:\)/\u\1\2/g<cr>

:%s/user_profile/userProfile/<cr>
```

1. `:%s/_\([a-z]\)\([^@]*:\)/\u\1\2/g<cr>` 콜론(:) 앞의 키 필드에 있는 밑줄(_)을 뒤의 문자를 대문자(\u\1)로 변환하며 치환 (이메일 제외)
1. `:%s/user_profile/userProfile/<cr>` 변수명 camelCase 치환

# Invert Dictionary

<!-- difficulty: advanced -->

딕셔너리의 키-값 쌍을 반전시킵니다.

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

:%s/\("[^"]*"\) = \([0-9]*\)/\2 = \1/g<cr>
```

1. `:1s/color_to_points/points_to_color/<cr>` 변수 이름 반전
1. `:%s/\("[^"]*"\) = \([0-9]*\)/\2 = \1/g<cr>` 키 문자열과 값 숫자의 위치 맞바꾸기

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

# CSV to SQL

<!-- difficulty: advanced -->

CSV 형식 데이터를 SQL INSERT 문으로 변환합니다.

## Before

```csv
id 1,Item 1,cost 1,location 1
id 2,Item 2,cost 2,location 2
id 10,Item 10,cost 10,location 10
```

## After

```sql
INSERT INTO `database`.`table` (`id` ,`item` ,`cost` ,`location`) VALUES ('id 1','Item 1','cost 1','Location 1');
INSERT INTO `database`.`table` (`id` ,`item` ,`cost` ,`location`) VALUES ('id 2','Item 2','cost 2','Location 2');
INSERT INTO `database`.`table` (`id` ,`item` ,`cost` ,`location`) VALUES ('id 10','Item 10','cost 10','Location 10');
```

## Command

```
:%s/\([^,]*\),\([^,]*\),\([^,]*\),location \(.*\)/

INSERT INTO `database`.`table` (`id` ,`item` ,`cost` ,

`location`) VALUES ('\1','\2','\3','Location \4');/<cr>
```

1. `:%s/\([^,]*\),\([^,]*\),\([^,]*\),location \(.*\)/` 네 개 열을 매칭하고 캡처
1.
    ```
    INSERT INTO `database`.`table` (`id` ,`item` ,`cost` ,`location`) VALUES ('\1','\2','\3','Location \4');/<cr>
    ```

# Enumerate and Align

<!-- difficulty: advanced -->

각 객체에 1부터 시작하여 증가하는 `rank` 필드를 추가하고, 보기 좋게 필드를 정렬합니다.

## Before

```js
[
  { word: "a", count: 2565 },
  { word: "and", count: 1777 },
  { word: "of", count: 1331 },
  { word: "that", count: 1263 },
  { word: "to", count: 1030 },
  { word: "in", count: 1027 },
  { word: "it", count: 754 },
  { word: "as", count: 730 },
  { word: "was", count: 687 },
  { word: "you", count: 652 },
  { word: "for", count: 630 },
];
```

## After

```js
[
  { rank:  1, word: "a",    count: 2565 },
  { rank:  2, word: "and",  count: 1777 },
  { rank:  3, word: "of",   count: 1331 },
  { rank:  4, word: "that", count: 1263 },
  { rank:  5, word: "to",   count: 1030 },
  { rank:  6, word: "in",   count: 1027 },
  { rank:  7, word: "it",   count:  754 },
  { rank:  8, word: "as",   count:  730 },
  { rank:  9, word: "was",  count:  687 },
  { rank: 10, word: "you",  count:  652 },
  { rank: 11, word: "for",  count:  630 },
];
```

## Command

```
:2,12s/word/\=printf("rank: %2d, word", line(".")-1)/<cr>

:2,12s/word: \([^,]*\), count: \(\d\+\)/

\=printf("word: %-7s count: %4d",

 submatch(1).",", str2nr(submatch(2)))/<cr>
```

1. `:2,12s/word/\=printf("rank: %2d, word", line(".")-1)/<cr>` 2~12행에 증가하는 번호의 rank 필드 삽입
1.
    ```
    :2,12s/word: \([^,]*\), count: \(\d\+\)/\=printf("word: %-7s count: %4d", submatch(1).",", str2nr(submatch(2)))/<cr>
    ```

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

# Reverse Golf Example

<!-- difficulty: advanced -->

예제의 "Before"와 "After" 케이스를 서로 맞바꿉니다.

## Before

````md
# snake_case to camelCase

Rename all fields to be camelCase.

## Before

```js
const user_profile = {first_name: "John"};
```

## After

```js
const userProfile = {firstName: "John"};
```
````

## After

````md
# camelCase to snake_case

Rename all fields to be snake_case.

## Before

```js
const userProfile = {firstName: "John"};
```

## After

```js
const user_profile = {first_name: "John"};
```
````

## Command

```
:1,3s/snake_case/TEMP_CASE/g<cr>

:1,3s/camelCase/snake_case/g<cr>

:1,3s/TEMP_CASE/camelCase/g<cr>

:7,9s/user_profile/userProfile/g<cr>

:7,9s/first_name/firstName/g<cr>

:13,15s/userProfile/user_profile/g<cr>

:13,15s/firstName/first_name/g<cr>
```

1. `:1,3s/snake_case/TEMP_CASE/g<cr>` 헤더 부분 임시 치환
1. `:1,3s/camelCase/snake_case/g<cr>` camelCase를 snake_case로 치환
1. `:1,3s/TEMP_CASE/camelCase/g<cr>` 임시값을 camelCase로 치환
1. `:7,9s/user_profile/userProfile/g<cr>` Before 코드 블록 변수명을 userProfile로 치환
1. `:7,9s/first_name/firstName/g<cr>` Before 코드 블록 필드명을 firstName으로 치환
1. `:13,15s/userProfile/user_profile/g<cr>` After 코드 블록 변수명을 user_profile로 치환
1. `:13,15s/firstName/first_name/g<cr>` After 코드 블록 필드명을 first_name으로 치환


````````````

</details>

## Beginner (초급)

### [Join Lines](join_lines.md)

두 줄을 한 줄로 합칩니다.

<video autoplay controls loop>
  <source src="generated/join_lines.mp4">
</video>

### [Delete Blank Line](delete_blank_line.md)

빈 줄을 삭제합니다.

<video autoplay controls loop>
  <source src="generated/delete_blank_line.mp4">
</video>

### [Swap Lines](swap_lines.md)

두 줄의 순서를 바꿉니다.

<video autoplay controls loop>
  <source src="generated/swap_lines.mp4">
</video>

### [Rotate Main Selection](rotate_main_selection.md)

마지막 숫자만 바꿉니다.

<video autoplay controls loop>
  <source src="generated/rotate_main_selection.mp4">
</video>

### [Extend Delete Words](extend_delete_words.md)

앞의 두 단어를 지웁니다.

<video autoplay controls loop>
  <source src="generated/extend_delete_words.mp4">
</video>

### [Duplicate Line](duplicate_line.md)

한 줄을 복사해 세 줄로 늘립니다.

<video autoplay controls loop>
  <source src="generated/duplicate_line.mp4">
</video>

### [Toggle Word Case](toggle_word_case.md)

단어의 대소문자를 뒤집습니다.

<video autoplay controls loop>
  <source src="generated/toggle_word_case.mp4">
</video>

### [Increment Numbers](increment_numbers.md)

여러 숫자를 한 번에 1씩 증가시킵니다.

<video autoplay controls loop>
  <source src="generated/increment_numbers.mp4">
</video>

### [Toggle Comment](toggle_comment.md)

여러 줄을 한 번에 주석으로 바꿉니다.

<video autoplay controls loop>
  <source src="generated/toggle_comment.mp4">
</video>

### [Format JSON with jq](format_json_jq.md)

한 줄 JSON을 셸 필터로 보기 좋게 펼칩니다.

<video autoplay controls loop>
  <source src="generated/format_json_jq.mp4">
</video>

### [Sort Lines](sort_lines.md)

정렬되지 않은 줄들을 정렬합니다.

<video autoplay controls loop>
  <source src="generated/sort_lines.mp4">
</video>

### [Replace an Identifier](replace_identifier.md)

커서가 놓인 변수 이름을 다른 이름으로 변경합니다.

<video autoplay controls loop>
  <source src="generated/replace_identifier.mp4">
</video>

### [Add Semicolons](add_semicolons.md)

각 줄 끝에 세미콜론을 붙입니다.

<video autoplay controls loop>
  <source src="generated/add_semicolons.mp4">
</video>

### [CSV to Lines](csv_to_lines.md)

쉼표로 구분된 한 줄을 여러 줄로 나눕니다.

<video autoplay controls loop>
  <source src="generated/csv_to_lines.mp4">
</video>

### [Replace Punctuation](replace_punctuation.md)

여러 줄의 앞 기호를 한 번에 바꿉니다.

<video autoplay controls loop>
  <source src="generated/replace_punctuation.mp4">
</video>

### [Indent Lines](indent_lines.md)

여러 줄을 한 번에 들여씁니다.

<video autoplay controls loop>
  <source src="generated/indent_lines.mp4">
</video>

### [Fix Typo with Search](fix_typo_with_search.md)

검색으로 오타를 찾아 고칩니다.

<video autoplay controls loop>
  <source src="generated/fix_typo_with_search.mp4">
</video>

### [Delete Surround](delete_surround.md)

여러 줄의 감싸는 괄호를 한 번에 제거합니다.

<video autoplay controls loop>
  <source src="generated/delete_surround.mp4">
</video>

### [Insert Sequence](insert_sequence.md)

셸 출력을 커서 앞에 삽입합니다.

<video autoplay controls loop>
  <source src="generated/insert_sequence.mp4">
</video>

### [Wrap with Tag](wrap_with_tag.md)

각 줄을 여는 태그와 닫는 태그로 감쌉니다.

<video autoplay controls loop>
  <source src="generated/wrap_with_tag.mp4">
</video>

### [Replace Surrounding Characters](replace_surround.md)

괄호를 대괄호로 변경합니다.

<video autoplay controls loop>
  <source src="generated/replace_surround.mp4">
</video>

### [Swap Columns](swap_columns.md)

두 열의 순서를 바꿉니다.

<video autoplay controls loop>
  <source src="generated/swap_columns.mp4">
</video>

### [Swap Quoted Strings](swap_quoted_strings.md)

두 따옴표 내용의 위치를 바꿉니다.

<video autoplay controls loop>
  <source src="generated/swap_quoted_strings.mp4">
</video>

## Intermediate (중급)

### [Multicursor Prefix](multicursor_prefix.md)

Visual Block 모드로 여러 줄 앞에 접두사를 동시에 삽입합니다.

<video autoplay controls loop>
  <source src="generated/multicursor_prefix.mp4">
</video>

### [Align Assignments](align_assignments.md)

할당 연산자를 열에 맞춰 정렬합니다.

<video autoplay controls loop>
  <source src="generated/align_assignments.mp4">
</video>

### [Replace a Regex-Sensitive Literal](replace_regex_literal.md)

문서 전체에서 정규식 메타문자가 포함된 문자열을 다른 문자열로 한 번에 변경합니다.

<video autoplay controls loop>
  <source src="generated/replace_regex_literal.mp4">
</video>

### [Replace a Selection with the System Clipboard](replace_with_system_clipboard.md)

레지스터를 활용하여 문자열을 복사하고 다른 위치의 문자열을 교체합니다.

<video autoplay controls loop>
  <source src="generated/replace_with_system_clipboard.mp4">
</video>

### [Text into Array](text_into_array.md)

줄바꿈으로 구분된 데이터를 문자열 배열로 결합합니다.

<video autoplay controls loop>
  <source src="generated/text_into_array.mp4">
</video>

### [Export from Rust Module](export_from_mod.md)

각 모듈에 포함된 함수를 re-export(pub use)합니다.

<video autoplay controls loop>
  <source src="generated/export_from_mod.mp4">
</video>

### [Object into Array](object_into_array.md)

객체를 2차원 배열로 변환합니다.

<video autoplay controls loop>
  <source src="generated/object_into_array.mp4">
</video>

### [snake_case to camelCase](snake_case_to_camel_case.md)

모든 필드명을 camelCase로 변경합니다. (이메일 주소의 밑줄 제외)

<video autoplay controls loop>
  <source src="generated/snake_case_to_camel_case.mp4">
</video>

## Advanced (고급)

### [Invert Dictionary](invert_dictionary.md)

딕셔너리의 키-값 쌍을 반전시킵니다.

<video autoplay controls loop>
  <source src="generated/invert_dictionary.mp4">
</video>

### [Invert Dictionary 2](invert_dictionary_2.md)

딕셔너리의 키-값 쌍을 반전시키는 또 다른 방법입니다.

<video autoplay controls loop>
  <source src="generated/invert_dictionary_2.mp4">
</video>

### [CSV to SQL](csv_to_sql.md)

CSV 형식 데이터를 SQL INSERT 문으로 변환합니다.

<video autoplay controls loop>
  <source src="generated/csv_to_sql.mp4">
</video>

### [Enumerate and Align](enumerate_and_align.md)

각 객체에 1부터 시작하여 증가하는 `rank` 필드를 추가하고, 보기 좋게 필드를 정렬합니다.

<video autoplay controls loop>
  <source src="generated/enumerate_and_align.mp4">
</video>

### [Function into Class](function_into_class.md)

3개의 함수를 3개의 메서드를 가진 클래스로 변환합니다.

<video autoplay controls loop>
  <source src="generated/function_into_class.mp4">
</video>

### [Reverse Golf Example](reverse_golf_example.md)

예제의 "Before"와 "After" 케이스를 서로 맞바꿉니다.

<video autoplay controls loop>
  <source src="generated/reverse_golf_example.mp4">
</video>

