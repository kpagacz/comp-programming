// link to the problem https://pl.spoj.com/problems/AL_31_01/
#include<string>
#include<unordered_map>
#include<vector>
#include<iostream>

struct Match {
	std::string team1, team2, result;
	Match(std::string team1_, std::string team2_, std::string result_) : team1(team1_), team2(team2_), result(result_) {}
	Match() = default;
};

void test_case() {
	size_t n_matches;
	std::cin >> n_matches;
	std::vector<Match> matches;
	std::unordered_map<std::string, int> goals;

	std::string team1, team2, dummy, result;
	for (size_t i = 0; i < n_matches; ++i) {
		std::cin >> team1 >> dummy >> team2 >> result;
		matches.push_back(Match(team1, team2, result));
		goals.insert({ {team1, 0}, {team2, 0} });
	}

	int n_goals;
	std::cin >> n_goals;
	while (n_goals) {
		std::cin >> team1;
		++goals[team1];
		--n_goals;	
	}

	int good_bets = 0;
	for (Match match : matches) {
		for (char c : match.result) {
			if (c == '0' && goals[match.team1] == goals[match.team2]) {
				++good_bets;
				break;
			}
			else if (c == '1' && goals[match.team1] > goals[match.team2]) {
				++good_bets;
				break;
			}
			else if (c == '2' && goals[match.team1] < goals[match.team2]){
				++good_bets;
				break;
			}
		}
	}

	if (good_bets == n_matches) {
		std::cout << "TAK";
	}
	else {
		std::cout << "NIE " << good_bets << "/" << n_matches;
	}
	std::cout << "\n";
}

int main() {
	int test;
	std::cin >> test;
	while (test) {
		test_case();
		--test;
	}
}