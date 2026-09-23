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
