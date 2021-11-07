// link to the problem https://pl.spoj.com/problems/AL_23_05/
#include<iostream>
#include<vector>

uint32_t MODULO = 1000000009;

uint32_t sum_routes(int i, int j, const std::vector < std::vector<uint32_t>>& counts) {
	uint32_t sum = 0;
	if (i > 0) sum += counts[i - 1][j];
	if (j > 0) sum += counts[i][j - 1];
	sum += counts[i][j];
	return sum % MODULO;
}

void test_case(int no) {
	uint32_t square_size;
	std::cin >> square_size;
	std::vector<std::vector<char>> square(square_size);
	for (auto& row : square) {
		row = std::vector<char>(square_size);
	}

	for (auto& row : square) {
		for (auto& c : row) {
			std::cin >> c;
		}
	}

	std::vector<std::vector<uint32_t>> routes_count(square_size, std::vector<uint32_t>(square_size, 0));
	routes_count[0][0] = 1;
	for (int i = 0; i < square_size; i++) {
		for (int j = 0; j < square_size; j++) {
			if (square[i][j] == 'x') routes_count[i][j] = 0; else routes_count[i][j] = sum_routes(i, j, routes_count);
		}
	}

	std::cout << routes_count[square_size - 1][square_size - 1] << "\n";
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