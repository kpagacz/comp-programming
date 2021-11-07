// link to the problem https://pl.spoj.com/problems/MWP4_2E/
#include<iostream>
#include<vector>

void test_case(int no) {
	int people, messages, source;
	std::cin >> people >> messages >> source;

	std::vector<bool> infected(people + 1, false);
	infected[source] = true;
	std::cout << source;

	int from, to;
	for (auto i = 0; i < messages; i++) {
		std::cin >> from >> to;
		if (infected[from] && !infected[to]) {
			std::cout << " " << to;
			infected[to] = true;
		}
	}

	std::cout << '\n';
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