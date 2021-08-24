// link to the problem https://pl.spoj.com/problems/MWP8_1A/
#include<iostream>
#include<math.h>

int gcf(int a, int b) {
	int rest = (b - a) % a;
	if (rest == 0) {
		return std::abs(a);
	}
	else {
		b = a;
		a = rest;
		return gcf(a, b);
	}
}

int main() {
	std::ios_base::sync_with_stdio(false);
	std::cin.tie(NULL);
	
	int n_elements;
	std::cin >> n_elements;
	int* elements = new int[n_elements];
	for (int i = 0; i < n_elements; ++i) std::cin >> elements[i];
	int* gcfs = new int[n_elements * n_elements];
	for (int i = 0; i < n_elements; ++i)
		for (int j = 0; j < n_elements; ++j)
			gcfs[i * n_elements + j] = gcf(elements[i], elements[j]);

	for (int i = 0; i < n_elements; ++i) {
		for (int j = 0; j < n_elements; ++j) {
			std::cout << gcfs[i * n_elements + j] << " ";
		}
		std::cout << "\n";
	}	
}

