// link to the problem https://pl.spoj.com/problems/AL_08_10/
#include<iostream>
#include<string>

void test_case(int no) {
	int n;
	std::cin >> n;
	std::string results;
	std::cin >> results;
	int it = 1;
	int answer = 0;
	n /= 2;
	int temp;
	while (it <= results.size()) {
		temp = results[results.size() - it] - 'A';
		it = 2 * it + (temp == 0);
		answer += temp * n;
		n /= 2;
	}

	std::cout << answer + 1<< "\n";
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