// link to the problem https://pl.spoj.com/problems/FR_AA_04/
#include <iostream>
#include <cstring>

int main() {
	int count;
	std::cin >> count;
	
	bool* temperatures = new bool[101];
	std::memset(temperatures, false, sizeof(bool) * 101);
	int different_temperatures = 0;

	int temperature_instance;
	while (std::cin >> temperature_instance) {
		if (temperatures[temperature_instance + 50]) continue; else {
			temperatures[temperature_instance + 50] = true;
			++different_temperatures;
		}
	}
	std::cout << different_temperatures;
}