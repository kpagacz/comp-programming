// link to the problem https://pl.spoj.com/problems/UPS12_A/
#include<iostream>
#include<vector>
#include<string>


void test_case(int no) {
	char c;
	std::vector<int> letters_counts(26, 0);
	std::string input;
	std::getline(std::cin, input);
	for (auto& letter : input) {
		letters_counts[letter - 'a']++;
	}

	bool pangram = true;
	bool perfect_pangram = true;
	int letter_occurence = letters_counts[0];

	for (auto& i : letters_counts) {
		if (i == 0) {
			pangram = false;
			break;
		}

		if (i != letter_occurence) perfect_pangram = false;
	}

	if (pangram) {
		if (perfect_pangram) std::cout << "PANGRAM PERFEKCYJNY\n"; else std::cout << "PANGRAM\n";
	} else {
		std::cout << "NIE\n";
	}
}

int main() {
	std::ios_base::sync_with_stdio(0);
	std::cin.tie(nullptr);
	int tests;
	tests = 1;
	while (tests) {
		test_case(tests);
		--tests;
	}
}