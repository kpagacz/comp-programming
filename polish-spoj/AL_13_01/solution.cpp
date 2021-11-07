// link to the problem https://pl.spoj.com/problems/AL_13_01/
#include<iostream>
#include<list>
#include<algorithm>

void test_case(int no) {
	std::list<int> numbers;
	int n;
	std::cin >> n;
	numbers.push_front(n);
	for (int i = n - 1; i > 0; i--) {
		int temp = numbers.back();
		numbers.pop_back();
		numbers.push_front(temp);
		numbers.push_front(i);
	}



	for (const int& n : numbers) {
		std::cout << n << " ";
	}
	std::cout << "\n";
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