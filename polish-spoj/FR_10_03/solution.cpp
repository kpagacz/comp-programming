// link to the problem https://pl.spoj.com/problems/FR_10_03/
#include<iostream>
#include<vector>
#include<algorithm>
#include<iterator>

void test_case(int no) {
	int32_t collectioners_count;
	std::cin >> collectioners_count;
	std::vector<uint32_t> wanted(collectioners_count);
	std::vector<uint32_t> have(collectioners_count);
	for (auto& n : wanted) std::cin >> n;
	for (auto& n : have) std::cin >> n;

	std::vector<uint32_t> difference;
	std::sort(wanted.begin(), wanted.end());
	std::sort(have.begin(), have.end());
	std::set_difference(wanted.begin(), wanted.end(), have.begin(), have.end(), std::inserter(difference, difference.begin()));

	if (difference.empty()) std::cout << "TAK\n"; else std::cout << "NIE\n";
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