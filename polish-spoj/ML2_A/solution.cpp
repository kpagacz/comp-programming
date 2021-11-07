// link to the problem https://pl.spoj.com/problems/ML2_A/
#include<iostream>
#include<vector>
#include<cstring>

constexpr int LIMIT = 401;
bool* erastothenes_sieve() {
	bool* sieve = new bool[LIMIT];
	std::memset(sieve, true, 401);
	sieve[0] = sieve[1] = false;
	for (int i = 2; i <= 20; ++i) {
		if (sieve[i]) {
			for (int j = i + i; j < LIMIT; j += i) sieve[j] = false;
		}
	}

	return sieve;
}

std::pair<int, int> find_primes(const int& n, const std::vector<int>& primes) {
	std::vector<int>::const_iterator end_it = std::end(primes) - 1;
	while (end_it > std::begin(primes) && *end_it > n) {
		--end_it;
	}
	int first = -1;
	while (end_it >= std::begin(primes) && *end_it >= n / 2) {
		for (int p : primes) {
			if (*end_it + p > n) {
				break;
			}
			if (*end_it + p == n) {
				first = p;
				break;
			}
		}
		if (first != -1 || end_it == std::begin(primes)) break;
		--end_it;
	}
	return std::make_pair(first, *end_it);
}

void test_case(const bool* sieve, const std::vector<int>& primes) {
	int n;
	std::cin >> n;

	if (n % 2 == 1) {
		if (n - 2 > 0 && sieve[n - 2]) {
			std::cout << "TAK " << 2 << " " << n - 2 << "\n";
			return;
		}
		else {
			std::cout << "NIE\n";
			return;
		}
	}
	else {
		std::pair<int, int> answer = find_primes(n, primes);
		if (answer.first == -1) {
			std::cout << "NIE\n";
			return;
		}
		else {
			std::cout << "TAK " << answer.first << " " << answer.second << "\n";
		}
	}
}


int main() {
	int tests;
	std::cin >> tests;

	const bool* sieve = erastothenes_sieve();
	std::vector<int> primes;
	for (int i = 0; i < LIMIT; ++i) if (sieve[i]) primes.push_back(i);

	while (tests) {
		test_case(sieve, primes);
		--tests;
	}
}