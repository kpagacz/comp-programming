// link to the problem https://pl.spoj.com/problems/AL_28_03/
#include<string>
#include<iostream>

void test_case(int no) {
	int n;
	std::cin >> n;
	int longest_edge = 0;
	std::string line;
	for (int i = 0; i < n; i++) {
		std::cin >> line;
		int current_edge = 0;
		for (char c : line) {
			if (c == '.') {
				current_edge++;
				if (current_edge > longest_edge) longest_edge = current_edge;
			}
			else {
				current_edge = 0;
			}
		}
	}

	std::cout << longest_edge * longest_edge << "\n";
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