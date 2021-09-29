// link to the problem https://pl.spoj.com/problems/FR_04_08/
#include<iostream>
#include<iomanip>

int get_digits(unsigned int number) {
	if (number < 10) return 1;
	if (number < 100) return 2;
	if (number < 1000) return 3;
	if (number < 10000) return 4;
	if (number < 100000) return 5;
	if (number < 1000000) return 6;
	if (number < 10000000) return 7;
	if (number < 100000000) return 8;
	if (number < 1000000000) return 9;
	return 0;
}

void test_case(int no) {
	unsigned int a, b;
	std::cin >> a >> b;
	int** matrix = new int* [a];
	matrix[0] = new int[a * b];
	for (int i = 1; i < a; i++) matrix[i] = matrix[i - 1] + b;
	for (int i = 0; i < a; i++) for (int j = 0; j < b; j++) matrix[i][j] = 0;

	int x = 0, y = 0;
	int start_col = 0, start_row = 0, end_col = b, end_row = a;
	for (int i = 1; i <= a * b;) {
		for (auto j = start_col; j < end_col; j++) matrix[start_row][j] = i++;
		for (auto j = start_row + 1; j < end_row; j++) matrix[j][end_col - 1] = i++;
		if (end_row - 1 != start_row) for (auto j = end_col - 2; j >= start_col; j--) matrix[end_row - 1][j] = i++;
		if (end_col - 1 != start_col) for (auto j = end_row - 2; j > start_row; j--) matrix[j][start_col] = i++;
		start_row++;
		start_col++;
		end_row--;
		end_col--;
	}

	int longest_number_digits = get_digits(a * b);


	for (auto i = 0; i < a; i++) {
		for (auto j = 0; j < b - 1; j++) {
			std::cout << std::setfill('0') << std::setw(longest_number_digits) << matrix[i][j] << " ";
		}
		std::cout << std::setfill('0') << std::setw(longest_number_digits) << matrix[i][b - 1];
		std::cout << '\n';
	}

	delete[] matrix[0];
	delete[] matrix;
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