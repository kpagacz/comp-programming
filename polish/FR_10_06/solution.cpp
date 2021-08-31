// link to the problem https://pl.spoj.com/problems/FR_10_06/
#include<iostream>
#include<vector>
#include<algorithm>
#include<math.h>

bool is_clashing(const std::vector<int> positions, int new_position) {
	for (const auto& pos : positions) {
		if (pos == new_position) return true;
	}
	int new_position_index = positions.size();
	for (int i = 0; i < positions.size(); ++i) {
		if (std::abs(new_position - positions[i]) == std::abs(i - new_position_index)) return true;
	}

	return false;
}

void test_case(int no) {
	std::vector<int> positions;
	bool is_good = true;
	for (int i = 0; i < 8; ++i) {
		int new_pos;
		std::cin >> new_pos;
		if (is_good) {
			if (is_clashing(positions, new_pos)) {
				is_good = false;
			}
			positions.push_back(new_pos);
		}

	}
	if (is_good) {
		std::cout << "TAK\n";
	}
	else {
		std::cout << "NIE\n";
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