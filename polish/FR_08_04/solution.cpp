// link to the problem https://pl.spoj.com/problems/FR_08_04/
#include<iostream>
#include<math.h>
#include<iomanip>

const double PI = 3.14159265;

void test_case(int no) {
	int segments;
	std::cin >> segments;
	double up = 0, down = 0;
	double angle, distance;
	for (int i = 0; i < segments; i++) {
		std::cin >> angle >> distance;
		double change = sin((angle + 360) * PI / 180) * distance;
		if (change > 0) {
			up += change;
		}
		else {
			down -= change;
		}
	}

	std::cout << std::fixed << std::setprecision(2) << up << " "
		<< std::fixed << std::setprecision(2) << down << '\n';
}

int main() {
	std::ios_base::sync_with_stdio(0);
	std::cin.tie(nullptr);
	int tests;
	tests = 1;
	while (tests) {
		test_case(tests);
		--tests;
	}
}