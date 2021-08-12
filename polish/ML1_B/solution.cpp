// link to the problem https://pl.spoj.com/problems/ML1_B/
#include <iostream>
#include <string>

int main() {
	int words;
	std::cin >> words;
	std::string word;
	int for_loop_counter = 0;
	int for_loop_counter_max = 0;
	for (int i = 0; i < words; i++) {
		std::cin >> word;
		if (word == "for") ++for_loop_counter; else if (word == "end") --for_loop_counter;
		if (for_loop_counter > for_loop_counter_max) for_loop_counter_max = for_loop_counter;
	}

	if (for_loop_counter_max == 0) {
		std::cout << "O(1)";
	}
	else if (for_loop_counter_max == 1) {
		std::cout << "O(n)";
	}
	else {
		std::cout << "O(n^" << for_loop_counter_max << ")";
	}
}