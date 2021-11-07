// link to the problem 
#include<iostream>
#include<math.h>
#include<iomanip>

void test_case(int no) {
	int x, y, int_a, int_b;
	std::cin >> x >> y >> int_a >> int_b;
	double a = 1.0 * int_a / 100;
	double b = 1.0 * int_b / 100;

	double res = (-std::sqrt((b - 1) * (4 * a * x * y + b * x * x - 2 * b * x * y + b * y * y - x * x - 2 * x * y - y * y)) - b * x - b * y + x + y) / (2 * (b - 1));
	std::cout << std::fixed << std::setprecision(4) << res << '\n';
}

int main() {
	std::ios_base::sync_with_stdio(0);
	std::cin.tie(nullptr);
	int tests;
	std::cin >> tests;
	while (tests) {
		test_case(tests);
		--tests;
	}
}