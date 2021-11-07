// link to the problem https://pl.spoj.com/problems/FR_07_11/
#include<vector>
#include<iostream>

int main() {
	int numbers;
	std::cin >> numbers;
	std::vector<std::vector<int>> vectors(3);
	int number;
	for (int i = 0; i < numbers; ++i) {
		std::cin >> number;
		vectors[i % 3].push_back(number);
	}

	std::vector<int>::reverse_iterator first_v_rev = std::rbegin(vectors[0]);
	std::vector<int>::iterator second_v_forward = std::begin(vectors[1]);
	std::vector<int>::iterator third_v_forward = std::begin(vectors[2]);

	for (int i = 0; i < numbers; ++i) {
		switch (i % 3) {
		case 0: 
			std::cout << *first_v_rev++ << " ";
			break;
		case 1:
			std::cout << *second_v_forward++ << " ";
			break;
		case 2:
			std::cout << *third_v_forward++ << " ";
			break;
		}
	}
	std::cout << std::endl;
}