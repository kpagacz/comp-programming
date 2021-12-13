#include<iostream>
#include<sstream>
#include<string>
#include<algorithm>
#include<vector>
#include<array>
#include<stdlib.h>

using paper = std::vector<std::vector<bool>>;
using points = std::vector<std::array<int, 2>>;

void sum_points(const paper& p) {
  int sum = 0;
        for (int i = 0; i < p.size(); i++)
        for (int j = 0; j < p[0].size(); j++)
          if (p[i][j] == true)
            sum++;

    std::cout << "Points: " << sum << '\n';
}

int main() {
  // read input
  paper p;
  points pts;
  std::vector<std::pair<char, int>> folds;
  bool points_input = true;
  std::string line;

  while (std::getline(std::cin, line)) {
    if (line.empty()) {
      points_input = false;
      continue;
    }

    std::stringstream in(line);
    if (points_input) {
      std::string char_x, char_y;
      std::getline(in, char_x, ',');
      std::getline(in, char_y, ',');
      pts.push_back({std::stoi(char_x, 0, 10), std::stoi(char_y, 0, 10)});
    } else {
      in.ignore(11);
      std::string value, direction;
      std::getline(in, direction, '=');
      std::getline(in, value, '=');
      folds.push_back(std::make_pair(direction[0], std::stoi(value, 0, 10)));
    }
  }

  int max_x = 0, max_y = 0;
  for(const auto& p : pts) {
    if (p[0] > max_x)
      max_x = p[0];
    if (p[1] > max_y)
      max_y = p[1];
  }
  p.resize(max_x + 1, std::vector<bool>(max_y + 1, false));

  for (const auto& point : pts) {
    p[point[0]][point[1]] = true;
  }


  for(const auto& f : folds) {
    if (f.first == 'x') {
      for (int i = f.second; i < p.size(); i++)
        for (int j = 0; j < p[0].size(); j++)
          if (p[i][j] == true) {
            p[-i + 2 * f.second][j] = true;
            p[i][j] = false;
          }
    } else {
      for (int i = 0; i < p.size(); i++)
        for (int j = f.second; j < p[0].size(); j++)
          if (p[i][j] == true) {
            p[i][-j + 2 * f.second] = true;
            p[i][j] = false;
          }
    }

    sum_points(p);
  }
}
