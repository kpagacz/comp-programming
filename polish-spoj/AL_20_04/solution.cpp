// link to the problem https://pl.spoj.com/problems/AL_20_04/
#include<iostream>
#include<vector>
#include<algorithm>
#include<cstring>

void test_case(const int& d) {
	bool* intervals = new bool[d];
	std::memset(intervals, false, d);
	intervals[0] = true;
	int n;
	std::cin >> n;
	std::vector<int> numbers(n);
	for (auto& no : numbers) {
		std::cin >> no;
		intervals[no] = true;
		intervals[d - no] = true;
	}

	for (int i = 0; i < n; ++i) {
		for (int j = i + 1; j < n; ++j) {
			intervals[numbers[j] - numbers[i]] = true;
		}
	}

	bool all_intervals_present = true;
	for (int i = 0; i < d; i++) if (intervals[i] == false) {
		all_intervals_present = false;
		break;
	}

	if (all_intervals_present) {
		std::cout << "Tak\n";
	}
	else {
		std::cout << "Nie\n";
	}
	

	delete[] intervals;
}

int main() {
	std::ios_base::sync_with_stdio(0);
	std::cin.tie(nullptr);
	int d;
	while (std::cin >> d) {
		test_case(d);
	}
}