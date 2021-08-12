// link to the problem https://pl.spoj.com/problems/WWO_01_01/
#include <iostream>
#include <string>
int main() {
	int a, b, c;
	std::cin >> a >> b >> c;
	std::string name;
	int boys = 0;
	int girls = 0;
	while (std::cin >> name) {
		if (name.back() == 'a') ++girls; else ++boys;
	}

	if (boys > girls) std::cout << girls; else std::cout << boys;
}