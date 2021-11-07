// link to the problem 
#include<iostream>
#include<string>
#include<unordered_map>

void test_case(int no) {
	std::string trainings;
	std::cin >> trainings;
	std::unordered_map<std::string, std::unordered_map<int, int>> fours_counts;
	for (auto i = 0; i < trainings.size() - 3; i++) {
		auto four = trainings.substr(i, 4);
		auto modulo = i % 4;
		if (fours_counts.find(four) != fours_counts.end() && fours_counts[four].find(modulo) != fours_counts[four].end()) {
			fours_counts[four][modulo]++;
		} else {
			if (fours_counts.find(four) == fours_counts.end()) {
				std::unordered_map<int, int> new_map({ { modulo, 1 } });
				fours_counts.insert({ four, new_map});
			}
			//else {
			//	fours_counts[four].insert({ modulo, 0 });
			//}
		}
	}

	for (auto& value : fours_counts) {
		for (auto& val2 : fours_counts[value.first]) {
			std::cout << value.first << " " << val2.first << " " << val2.second << "\n";
		}
	}
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