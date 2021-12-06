#include<iostream>
#include<string>
#include<vector>
#include<sstream>
#include<algorithm>
#include<cmath>

constexpr int LIMIT = 255;

int main() {
  // input
  std::string line;
  std::getline(std::cin, line);
  std::stringstream line_stream = std::stringstream(line);

  std::vector<uint64_t> original_fish;
  std::string days;
  while(std::getline(line_stream, days, ',')) {
    original_fish.push_back(std::stoi(days, 0, 10));
  }

  uint64_t *normal_fish = new uint64_t[LIMIT + 1];
  uint64_t *new_fish = new uint64_t[LIMIT + 1];
  std::fill_n(normal_fish, 7, 1);
  std::fill_n(new_fish, 9, 1);
  normal_fish[7] = normal_fish[8] = 2;
  for (int i = 9; i < LIMIT; i++) {
    normal_fish[i] = normal_fish[i - 7] + new_fish[i - 7];
    new_fish[i] = normal_fish[i - 9] + new_fish[i - 9];
  }

  uint64_t result = 0;
  std::for_each(original_fish.begin(), original_fish.end(), [&](uint64_t f)
                { result += normal_fish[LIMIT - f] + new_fish[LIMIT - f]; });

  std::cout << "fish number: " << result << '\n';
 }
