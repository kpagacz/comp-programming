// link to the problem https://pl.spoj.com/problems/AL_20_05/
#include<iostream>
#include<vector>

const unsigned int MODULO = 1000000009;

void test_case(int no, const std::vector<unsigned int>& ways) {
	int number;
	std::cin >> number;
	std::cout << ways[number + 2] << '\n';
}

int main() {
	std::ios_base::sync_with_stdio(0);
	std::cin.tie(nullptr);
	std::vector<unsigned int> ways(1000000 + 3, 0);
	ways[0] = ways[1] = ways[2] = 0;
	ways[3] = 1;
	for (auto i = 4; i < ways.size(); i++) {
		ways[i] += (ways[i - 3] + ways[i - 2] + ways[i - 1]) % MODULO;
	}
	int tests;
	std::cin >> tests;
	while (tests) {
		test_case(tests, ways);
		--tests;
	}
}