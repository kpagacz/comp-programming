// link to the problem https://pl.spoj.com/problems/MWP2_1A/
#include<iostream>
#include<vector>
#include<algorithm>

int rotate_and_count_matching(std::vector<int>& numbers) {
	int matching = 0;
	for (auto i = 0; i < numbers.size(); i++) {
		if (numbers[i] == i + 1) matching++;
	}
	std::rotate(numbers.begin(), numbers.begin() + 1, numbers.end());
	return matching;
}

void test_case(int no) {
	int numbers_count;
	std::cin >> numbers_count;
	std::vector<int> numbers(numbers_count);
	for (auto& n : numbers) std::cin >> n;

	int match_max = 0;
	int match;
	int max_rot = 0;
	for (auto current_rotation = 0; current_rotation < numbers_count; current_rotation++) {
		match = rotate_and_count_matching(numbers);
		if (match > match_max) {
			match_max = match;
			max_rot = current_rotation;
		}
		if (match_max > numbers_count / 2) break;
	}

	std::rotate(numbers.begin(), numbers.begin() + max_rot, numbers.end());
	for (auto& n : numbers) std::cout << n << " ";
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