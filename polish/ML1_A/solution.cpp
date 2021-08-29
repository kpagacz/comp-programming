// link to the problem https://pl.spoj.com/problems/ML1_A/
#include<iostream>
#include<map>
#include<string>
#include<algorithm>

void test_case() {
	int n_day;
	std::cin >> n_day;
	std::map<std::string, int> days = { {"pon", 0}, {"wt", 0}, {"sr", 0}, {"czw", 0}, {"pt", 0}, {"sob", 0}, {"niedz", 0} };
	std::string day;
	for(int i = 0; i < n_day; ++i) {
		std::cin >> day;
		days[day]++;
	}

	auto comparator = [](const std::pair<std::string, int>& first, const std::pair<std::string, int>& second) {return first.second < second.second; };
	std::map<std::string, int>::const_iterator max_days = std::max_element(days.begin(), days.end(), comparator);
	std::cout << max_days->second << " " << n_day;
}

int main() {
	std::ios_base::sync_with_stdio(0);
	std::cin.tie(nullptr);
	test_case();
}