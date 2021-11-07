// link to the problem https://pl.spoj.com/problems/FR_AA_03/
#include <iostream>
#include <string>

int main() {
	int stack = 0;
	char c;
	bool had_mark = false;
	std::string sequence;

	std::cin >> sequence;
	for (char c : sequence) {
		if (c == '+') ++stack; else --stack;
		if (stack == 3) {
			if (had_mark) std::cout << " ";
			had_mark = true;
			std::cout << "5";
			stack = 0;
		}
		if (stack == -3) {
			if (had_mark) std::cout << " ";
			had_mark = true;
			std::cout << "1";
			stack = 0;
		}		
	}

	if (!had_mark) {
		std::cout << "BRAK";
	}
}