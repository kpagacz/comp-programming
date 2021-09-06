// link to the problem https://pl.spoj.com/problems/FR_06_01/
#include<iomanip>
#include<iostream>

void test_case(int no) {
	double m1, k1, m3, k3;
	std::cin >> m1 >> k1 >> m3 >> k3;
	int cm1, ck1, cm3, ck3;
	std::cin >> cm1 >> ck1 >> cm3 >> ck3;

	std::cout << "K16K36: " << std::fixed << std::setprecision(2) << (k1 * ck1 + k3 * ck3) / (ck1 + ck3) << "\n";
	std::cout << "M16M36: " << std::fixed << std::setprecision(2) << (m1 * cm1 + m3 * cm3) / (cm1 + cm3) << "\n";
	std::cout << "M16K16M36K36: " << std::fixed << std::setprecision(2) << ((k1 * ck1 + k3 * ck3) + (m1 * cm1 + m3 * cm3)) / (ck1 + ck3 + cm1 + cm3) << "\n";
}

int main() {
	std::ios_base::sync_with_stdio(0);
	std::cin.tie(nullptr);
	int tests = 1;
	while (tests) {
		test_case(tests);
		--tests;
	}
}