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
