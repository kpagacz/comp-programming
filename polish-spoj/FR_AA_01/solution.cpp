#include<iostream>

int main() {
	int stationary_competitors = 0;
	for (int i = 0; i < 4; i++) {
		int total_number;
		std::cin >> total_number;
		int remote_number;
		std::cin >> remote_number;
		stationary_competitors += total_number - remote_number;
	}
	std::cout << stationary_competitors;
}