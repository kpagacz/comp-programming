// link to the problem https://pl.spoj.com/problems/AL_25_01/
#include<iostream>
#include<vector>
#include<algorithm>

void test_case(int no) {
	int n, p, k;
	std::cin >> n >> p >> k;
	int bounces = 1, current_pos = p - 1;
	std::vector<int> distances(n);
	std::vector<bool> visited(n, false);

	for (auto& no : distances) std::cin >> no;
	if (p == k) {
		std::cout << "1";
		return;
	}
	visited[current_pos] = true;
	current_pos = (current_pos + distances[current_pos]) % n;
	++bounces;
	while (current_pos != k - 1) {
		if (visited[current_pos]) {
			std::cout << "NIGDY";
			return;
		}
		visited[current_pos] = true;
		current_pos = (current_pos + distances[current_pos]) % n;
		++bounces;
	}

	std::cout << bounces;
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