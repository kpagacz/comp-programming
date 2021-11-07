// link to the problem https://pl.spoj.com/problems/FR_12_09/
#include <iostream>

int main() {
	char c;
	int kot_encountered = 0;
	bool waiting_for_k, waiting_for_o, waiting_for_t;
	waiting_for_k = true;
	waiting_for_o = waiting_for_t = false;

	while (std::cin.get(c)) {
		if (c == 'k' && waiting_for_k) {
			waiting_for_k = false;
			waiting_for_o = true;
		}
		else if (c == 'o' && waiting_for_o) {
			waiting_for_o = false;
			waiting_for_t = true;
		}
		else if (c == 't' && waiting_for_t) {
			waiting_for_t = false;
			waiting_for_k = true;
			++kot_encountered;
		}
	}

	if (kot_encountered > 0) std::cout << kot_encountered; else std::cout << "NIE";
}