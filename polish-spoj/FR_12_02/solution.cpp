// link to the problem https://pl.spoj.com/problems/FR_12_02/
#include <iostream>
int main() {
	char x1, y1, x2, y2;
	scanf("%c%c %c%c", &x1, &y1, &x2, &y2);

	if (x1 == x2 || y1 == y2) std::cout << "TAK"; else std::cout << "NIE";
}