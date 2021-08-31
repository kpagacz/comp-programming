// link to the problem https://pl.spoj.com/problems/AL_30_01/
#include<iostream>
#include<string>
#include<unordered_map>
#include<algorithm>
#include<cstring>

void test_case(int no) {
	std::string word;
	std::cin >> word;
	int no_words;
	std::cin >> no_words;
	int anagrams = 0;
	std::string copy;
	while (no_words) {
		copy = word;
		std::string another_word;
		std::cin >> another_word;
		for (char c : another_word) {
			if (copy.find(c) == std::string::npos) {
				break;
			}
			else {
				copy.erase(copy.find(c), 1);
			}
		}
		if (copy== "") {
			anagrams++;
		}
		--no_words;
	}

	std::cout << anagrams;
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