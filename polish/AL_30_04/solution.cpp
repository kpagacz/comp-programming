// link to the problem https://pl.spoj.com/problems/AL_30_04/
#include<iostream>
#include<vector>
#include<algorithm>

void test_case(int no) {
	
}

int main() {
	std::ios_base::sync_with_stdio(0);
	std::cin.tie(nullptr);
	int n_heaps;
	std::cin >> n_heaps;
	std::vector<int> heaps(n_heaps);
	int heaps_with_1 = 0;
	for (auto& n : heaps) {
		std::cin >> n;
		if (n == 1) ++heaps_with_1;
	}

	if ((n_heaps - heaps_with_1) % 2 + (heaps_with_1) % 2 == 1) {
		std::cout << "0 0\n";
		return 0;
	}

	std::vector<int>::iterator to_remove;
	if (heaps_with_1 % 2 == 0) {
		to_remove = std::find_if(heaps.begin(), heaps.end(), [](int i) {return i != 1; });
		std::cout << std::distance(heaps.begin(), to_remove) + 1 << " " << *to_remove- 1 << "\n";
	}
	else {
		to_remove = std::find_if(heaps.begin(), heaps.end(), [](int i) {return i != 1; });
		std::cout << std::distance(heaps.begin(), to_remove) + 1 << " " << *to_remove << "\n";
	}
}