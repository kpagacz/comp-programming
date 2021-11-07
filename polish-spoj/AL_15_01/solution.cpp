// link to the problem https://pl.spoj.com/problems/AL_15_01/
#include<iostream>
#include<string>

void test_case() {
	std::string darek, jarek, marek;
	std::cin >> darek >> jarek >> marek;
	int max = 0;
	for (int i = 0; i < darek.size(); ++i) {
		if (jarek[i] != darek[i] || marek[i] != darek[i]) {
			++max;
			if (jarek[i] == marek[i]) ++max;
		}
	}
	std::cout << max << "\n";
}

int main() {
	std::ios_base::sync_with_stdio(0);
	std::cin.tie(nullptr);
	test_case();
}