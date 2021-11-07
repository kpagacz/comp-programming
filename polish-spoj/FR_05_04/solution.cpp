// link to the problem https://pl.spoj.com/problems/FR_05_04/
#define _CRT_SECURE_NO_WARNINGS
#include<iostream>
#include<string>
#include<algorithm>
#include<vector>
#include<cstdio>

struct Runner {
	std::string name, last_name;
	int minutes, seconds;
	Runner() = default;
	Runner(std::string _name, std::string _last_name, int _minutes, int _seconds):
		name(_name), last_name(_last_name), minutes(_minutes), seconds(_seconds) {}
};


int main() {
	int n_runners;
	std::cin >> n_runners;
	std::vector<Runner> runners;
	std::string name, last_name;
	int minutes, seconds;
	for (int i = 0; i < n_runners; ++i) {
		std::cin >> name >> last_name;
		scanf("%d:%d", &minutes, &seconds);
		runners.push_back(Runner(name, last_name, minutes, seconds));
	}

	auto comparator = [](const Runner first, const Runner second) {return first.minutes == second.minutes ? first.seconds < second.seconds : first.minutes < second.minutes; };
	std::stable_sort(std::begin(runners), std::end(runners), comparator);
	std::vector<Runner>::iterator it = std::begin(runners);
	std::cout << it->name << " " << it->last_name << "\n";
	++it;
	while (it != std::end(runners) && it->minutes == std::begin(runners)->minutes && it->seconds == std::begin(runners)->seconds) {
		std::cout << it->name << " " << it->last_name << "\n";
		++it;
	}
}