// link to the problem https://pl.spoj.com/problems/MWP2_2C/
#include<iostream>
#include<math.h>
#include<iomanip>
#include<vector>
#include<algorithm>

void test_case(int no) {
	double x1, y1, r1, x2, y2, r2;
	std::cin >> x1 >> y1 >> r1 >> x2 >> y2 >> r2;
	double distance = std::sqrt(std::pow(x1 - x2, 2) + std::pow(y1 - y2, 2));
	
	std::vector<double> radi(2);
	radi[0] = r1;
	radi[1] = r2;
	std::sort(radi.begin(), radi.end());

	if (r1 + r2 <= distance) {
		std::cout << std::fixed << std::setprecision(2) << 0.00 << "\n";
	}
	else if (radi[1] <= distance) {
		std::cout << std::fixed << std::setprecision(2) << r1 + r2 - distance << "\n";
	}
	else if (distance + radi[0] > radi[1]) {
		std::cout << std::fixed << std::setprecision(2) << radi[0] + radi[1] - distance << "\n";
	}
	else {
		std::cout << std::fixed << std::setprecision(2) << radi[0] * 2 << "\n";
	}
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