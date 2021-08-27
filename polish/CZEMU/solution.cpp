// link to the problem https://pl.spoj.com/problems/CZEMU/
#include<iostream>
#include<algorithm>
#include<string>
#include<vector>

void test_case() {
	std::cin.ignore(std::numeric_limits<std::streamsize>::max(), '\n');
	std::vector<std::string> rose;
	std::vector<std::string> mentor;
	std::string problem;
	std::cin >> problem;
	while (problem != "List") {
		rose.push_back(problem);
		std::cin >> problem;
	}
	std::cin.ignore(std::numeric_limits<std::streamsize>::max(), '\n');
	std::cin >> problem;
	while (!std::cin.eof()) {
		mentor.push_back(problem);
		std::cin >> problem;
	}

	std::vector<std::string> difference;
	std::sort(rose.begin(), rose.end());
	std::sort(mentor.begin(), mentor.end());
	std::set_difference(rose.begin(), rose.end(), mentor.begin(), mentor.end(), std::back_inserter(difference));

	for (std::string& p : difference) {
		std::cout << p << "\n";
	}
}

int main() {
	std::ios_base::sync_with_stdio(0);
	std::cin.tie(nullptr);
	test_case();
}