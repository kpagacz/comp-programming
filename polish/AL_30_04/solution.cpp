// link to the problem https://pl.spoj.com/problems/AL_30_04/
#include<iostream>
#include<vector>

void test_case(int no) {
	int n_heaps;
	std::cin >> n_heaps;
	std::vector<int> heaps(n_heaps);
	int nim_sum = 0;
	for (auto& n : heaps) {
		std::cin >> n;
		nim_sum ^= n;
	}

	if (nim_sum == 0) {
		std::cout << "0 0";
	}
	else {
		for (auto i = 0; i < heaps.size(); i++) {
			if ((nim_sum ^ heaps[i]) < heaps[i]) {
				std::cout << i + 1 << " " << heaps[i] - (nim_sum ^ heaps[i]);
				break;
			}
		}
	}
}

int main() {
	std::ios_base::sync_with_stdio(0);
	std::cin.tie(nullptr);

	test_case(1);
}