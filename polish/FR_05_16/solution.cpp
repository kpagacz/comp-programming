// link to the problem https://pl.spoj.com/problems/FR_05_16/
#include<iostream>
#include<stack>

void test_case(int no) {
	std::stack<int> stack;
	int q;
	std::cin >> q;
	int decider;
	char operation;
	int number;
	while (q) {
		std::cin >> decider;
		switch (decider) {
		case(0):
			std::cin >> operation;
			int a, b;
			a = stack.top();
			stack.pop();
			b = stack.top();
			stack.pop();
			switch (operation) {
			case('+'):
				stack.push(a + b);
				break;
			case('-'):
				stack.push(b - a);
				break;
			case('*'):
				stack.push(a * b);
				break;
			case('/'):
				stack.push(b / a);
				break;
			}
			break;
		case(1):
			std::cin >> number;
			stack.push(number);
			break;
		}

		--q;
	}

	std::cout << stack.top() << "\n" << "ONP";
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