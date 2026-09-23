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
