//link to the problem https://pl.spoj.com/problems/AL_04_03/
#include<iostream>
#include<cstring>

int area(bool* neighbours, int vertex, int vertices, int& volume) {
	int children_area = 0;
	int children = 0;
	for (int i = vertex; i < vertices; ++i) {
		if (neighbours[vertex * vertices + i] == true) {
			children_area += area(neighbours, i, vertices, volume);
			++children;
		}
	}

	volume += children_area + children + 1;
	return children_area + children + 1;
}

void test_case() {
	int vertices;
	std::cin >> vertices;
	bool* neighbours = new bool[vertices * vertices];
	std::memset(neighbours, 0, vertices * vertices);

	int from, to;
	for (int i = 0; i < vertices - 1; i++) {
		std::cin >> from >> to;
		neighbours[from * vertices + to] = true;
		neighbours[to * vertices + from] = true;
	}

	int volume = 0;
	area(neighbours, 0, vertices, volume);
	std::cout << volume << "\n";
}


int main() {
	int tests;
	std::cin >> tests;
	while (tests) {
		test_case();
		--tests;
	}
}