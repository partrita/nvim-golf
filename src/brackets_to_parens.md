# From Brackets to Parens

<!-- difficulty: intermediate -->

배열 인덱스 접근 구문(`[i][j][k]`)을 함수 호출 형태(`(i, j, k)`)로 일괄 치환합니다.

## Before

```cpp

int main() {
	glm::vec3 umax(
		(vx[i+1][j][k]-vx[i][j][k])/2,
		(vy[i][j+1][k]-vy[i-1][j+1][k])/2,
		(vz[i][j][k+1]-vz[i-1][j][k+1])/2
	);
	glm::vec3 umin(
		(m_vx[i][j][k]-m_vx[i-1][j][k])/2,
		(m_vy[i][j][k]-m_vy[i-1][j][k])/2,
		(m_vz[i][j][k]-m_vz[i-1][j][k])/2
	);
}
```

## After

```cpp
int main() {
	glm::vec3 umax(
		(vx(i+1, j, k)-vx(i, j, k))/2,
		(vy(i, j+1, k)-vy(i-1, j+1, k))/2,
		(vz(i, j, k+1)-vz(i-1, j, k+1))/2
	);
	glm::vec3 umin(
		(m_vx(i, j, k)-m_vx(i-1, j, k))/2,
		(m_vy(i, j, k)-m_vy(i-1, j, k))/2,
		(m_vz(i, j, k)-m_vz(i-1, j, k))/2
	);
}
```

## Command

```
dd:%s/\]\[/, /g<cr>:%s/\[/(/g<cr>:%s/\]/)/g<cr>
```

1. `dd` 첫 번째 빈 줄 삭제
1. `:%s/\]\[/, /g` 인접한 대괄호 쌍(][)을 쉼표와 공백으로 치환
1. `<cr>` 치환 명령 실행
1. `:%s/\[/(/g` 남은 여는 대괄호([)를 여는 소괄호로 치환
1. `<cr>` 치환 명령 실행
1. `:%s/\]/)/g` 남은 닫는 대괄호(])를 닫는 소괄호로 치환
1. `<cr>` 치환 명령 실행
