// link to the problem https://pl.spoj.com/problems/DDZ_02_04/
#include<array>
#include<iostream>
#include<cstring>
#include<algorithm>
#include<string>
#include<numeric>

bool* erastothenes_sieve() {
	bool* sieve = new bool[1000000];
	std::memset(sieve, true, 1000000);

	sieve[0] = sieve[1] = false;
	for (int i = 2; i <= 1000; ++i) {
		if (sieve[i])
			for (int j = i + i; j < 1000000; j += i) sieve[j] = false;
	}

	return sieve;
}


void count_primes(bool* sieve, const std::array<int, 5>& digits, int& counter) {
	auto concatenator = [&](std::string number, int digit) {
		return std::move(number) + std::to_string(digit);
	};
	std::string string_number = std::accumulate(std::next(std::begin(digits)), std::end(digits), std::to_string(digits[0]), concatenator);
	int number = std::stoi(string_number);
	if (sieve[number]) ++counter;
}

int main() {
	bool* sieve = erastothenes_sieve();
	std::array<int, 5> digits;
	std::cin >> digits[0] >> digits[1] >> digits[2] >> digits[3] >> digits[4];
	std::sort(std::begin(digits), std::end(digits));

	int prime_counter = 0;
	do {
		count_primes(sieve, digits, prime_counter);
	} while (std::next_permutation(std::begin(digits), std::end(digits)));

	std::cout << prime_counter << "\n";
	delete[]sieve;
}