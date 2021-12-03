// link https://adventofcode.com/2021/day/3
#include<iostream>
#include<string>
#include<cstring>
#include<cmath>

int main() {
  std::string line;

  std::cin >> line;
  int *most_common_bits = new int[line.size()];
  std::memset(most_common_bits, 0, line.size() * sizeof(int));

  do {
    for(int i = 0; i < line.size(); i++) {
      if (line[i] == '1') {
        most_common_bits[i]++;
      } else {
        most_common_bits[i]--;
      }
    }
  } while (std::cin >> line);

  std::string gamma_rate_bits = "";
  for (int i = 0; i < line.size(); i++) {
    if (most_common_bits[i] > 0) {
      gamma_rate_bits += "1";
    } else {
      gamma_rate_bits += "0";
    }
  }

  int gamma_rate = std::stoi(gamma_rate_bits, 0, 2);
  std::cout << "gamma rate: " << gamma_rate << "\n";
  int epsilon_rate = (int)(std::pow(2, line.size()) - gamma_rate - 1);
  std::cout << "epsilon rate: " << epsilon_rate << '\n';
  std::cout << "gamma rate * epsilon rate: " << epsilon_rate * gamma_rate << '\n';

  delete[] most_common_bits;
}
