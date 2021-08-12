// link to the problem https://pl.spoj.com/problems/DDZ5/
#include <iostream>

int main() {
	int numbers_count;
	std::cin >> numbers_count;
	int fibonacci_like_count = 0;

	if (numbers_count >= 3) {
		int n1, n2, n3;
		std::cin >> n1 >> n2;
		while (std::cin >> n3) {
			if (n1 + n2 == n3) ++fibonacci_like_count;
			n1 = n2;
			n2 = n3;
		}
	}
	std::cout << fibonacci_like_count;
}