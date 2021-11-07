// link to the problem https://pl.spoj.com/problems/KWACIK/
#include<iostream>
#include<array>
#include<algorithm>

bool is_broken(const std::array<int, 9>& digits, const int& threshold) {
	if (std::any_of(digits.begin(), digits.end(), [](int value) {return value > 9; })) return true;
	if (digits[0] + digits[1] + digits[3] + digits[4] > threshold) return true;
	if (digits[1] + digits[2] + digits[4] + digits[5] > threshold) return true;
	if (digits[3] + digits[4] + digits[6] + digits[7] > threshold) return true;
	if (digits[4] + digits[5] + digits[7] + digits[8] > threshold) return true;
	return false;
}

bool is_good(const std::array<int, 9>& digits, const int& threshold) {
	if (digits[0] + digits[1] + digits[3] + digits[4] == threshold && 
		digits[1] + digits[2] + digits[4] + digits[5] == threshold && 
		digits[3] + digits[4] + digits[6] + digits[7] == threshold &&
		digits[4] + digits[5] + digits[7] + digits[8] == threshold) return true;
	return false;
}

void count_squares(std::array<int, 9>& digits, int& square_count, const int& threshold, int increment_index) {
	if (increment_index < 0) return;
	digits[increment_index]++;
	if (is_broken(digits, threshold)) {

	}
	else {
		count_squares(digits, square_count, threshold, increment_index);
	}
}

void test_case(int no) {
	int n;
	std::cin >> n;
	if (n > 36) {
		std::cout << 0 << "\n";
		return;
	}

	std::array<int, 9> digits;
	std::fill(digits.begin(), digits.end(), 0);

	int square_count = 0;
	count_squares(digits, square_count, n, 8);
	std::cout << square_count << '\n';
}

int main() {
	int tests;
	std::cin >> tests;
	while (tests) {
		test_case(tests);
		--tests;
	}
}