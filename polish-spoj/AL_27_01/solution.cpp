// link to the problem https://pl.spoj.com/problems/AL_27_01/
#define _CRT_SECURE_NO_WARNINGS
#include<iostream>
#include<string>
#include<unordered_map>

void test_case() {
	std::string a, b, c;
	std::getline(std::cin, a);
	std::getline(std::cin, b);
	std::getline(std::cin, c);
	std::unordered_map<char, bool> already_seen;
	for (char c : a) already_seen.insert({ c, true });

	for (char c : b) if (already_seen.find(c) != std::end(already_seen)) {
		printf("TAK\n"); return;
	}
	for (char c : b) already_seen.insert({ c, true });

	for (char c_ : c) if (already_seen.find(c_) != std::end(already_seen)) {
		printf("TAK\n"); return;
	}

	printf("NIE\n");
}

int main() {
	int tests;
	scanf(" %d\n", &tests);
	for (int i = 0; i < tests; ++i) {
		test_case();
	}
}