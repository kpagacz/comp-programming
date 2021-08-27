// link to the https://pl.spoj.com/problems/FR_06_05/
#include<iostream>
#include<iomanip>
#include<algorithm>
#include<numeric>
#include<vector>
#include<math.h>

struct Point {
	double x, y;
	Point(int _x, int _y): x(_x), y(_y) {}
	Point() = default;
};

void test_case(int no) {
	int n_points;
	std::cin >> n_points;
	std::vector<Point> points(n_points);
	for (Point& p : points) {
		std::cin >> p.x >> p.y;
	}

	Point previous_point = points.front();
	double distance = 0.0;
	for (int i = 1; i < n_points; ++i) {
		distance += std::sqrt((previous_point.x - points[i].x) * (previous_point.x - points[i].x) + (previous_point.y - points[i].y) * (previous_point.y - points[i].y));
		previous_point = points[i];
	}

	std::cout << std::fixed << std::setprecision(2) << distance / 1000 << "\n";
}

int main() {
	std::ios_base::sync_with_stdio(0);
	std::cin.tie(nullptr);
	int tests;
	std::cin >> tests;
	while (tests) {
		test_case(tests);
		--tests;
	}
}