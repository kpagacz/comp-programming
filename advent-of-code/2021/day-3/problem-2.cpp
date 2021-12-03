// link https://adventofcode.com/2021/day/3
#include<iostream>
#include<string>
#include<vector>
#include<cstring>
#include<algorithm>

int main() {
  std::string line;
  std::vector<std::string> oxygen_lines;
  std::vector<std::string> co2_lines;

  std::cin >> line;
  int *most_common_bits = new int[line.size()];
  std::memset(most_common_bits, 0, line.size() * sizeof(int));

  do {
    oxygen_lines.push_back(line);
    co2_lines.push_back(line);
  } while (std::cin >> line);

  int current_position = 0;
  while(oxygen_lines.size() > 1) {
    int most_common_bit = 0;
    for(const auto& line : oxygen_lines)
      line[current_position] == '1' ? most_common_bit++ : most_common_bit--;

    auto has_most_common_bit = [&](std::string line)
    {
      if (most_common_bit >= 0)
      {
        return line[current_position] == '1';
      }
      else
      {
        return line[current_position] == '0';
      }
    };

    oxygen_lines.erase(std::remove_if(oxygen_lines.begin(), oxygen_lines.end(), [&](std::string line)
                                      { return !has_most_common_bit(line); }),
                       oxygen_lines.end());

    current_position++;
  }
  int oxygen_rating = std::stoi(oxygen_lines[0], 0, 2);

  current_position = 0;
  while(co2_lines.size() > 1) {
    int most_common_bit = 0;
    for(const auto& line : co2_lines)
      line[current_position] == '1' ? most_common_bit++ : most_common_bit--;

    auto has_most_common_bit = [&](std::string line)
    {
      if (most_common_bit >= 0)
      {
        return line[current_position] == '1';
      }
      else
      {
        return line[current_position] == '0';
      }
    };

    co2_lines.erase(std::remove_if(co2_lines.begin(), co2_lines.end(), has_most_common_bit),
                       co2_lines.end());

    current_position++;
  }
  int co2_rating = std::stoi(co2_lines[0], 0, 2);
  std::cout << "oxygen rating: " << oxygen_rating << '\n';
  std::cout << "co2 rating: " << co2_rating << '\n';
  std::cout << "oxygen * co2: " << oxygen_rating * co2_rating << '\n';

  delete[] most_common_bits;
}
