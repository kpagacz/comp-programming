#include <iostream>
#include <string>
#include <vector>
#include <algorithm>
#include <iterator>

constexpr int LIMIT =  1e9;
using grid = std::vector<std::string>;

void print(const std::string &line) {
  std::cout << line << '\n';
}

int main(int argc, char **argv) {
  std::string line;
  grid trench;

  while (std::cin >> line) {
    trench.push_back(line);
  }

  std::copy(trench.begin(), trench.end(), std::ostream_iterator<std::string>(std::cout, "\n"));

  int step = 0;
  while (step < LIMIT) {
    bool moved = false;

    for (auto i = 0; i < trench.size(); i++) {
      std::vector<int> to_move;
      for (auto j = 0; j < trench[i].size(); j++)
        if (trench[i][j] == '>' && trench[i][(j + 1) % trench[i].size()] == '.')
            to_move.push_back(j);

      for (const auto &pos : to_move)
        std::swap(trench[i][pos], trench[i][(pos + 1) % trench[0].size()]);
      moved = moved || to_move.size() > 0;
    }

    for (auto i = 0; i < trench[0].size(); i++) {
      std::vector<int> to_move;
      for (auto j = 0; j < trench.size(); j++)
        if (trench[j][i] == 'v' && trench[(j + 1) % trench.size()][i] == '.')
          to_move.push_back(j);

      for(const auto& pos : to_move)
        std::swap(trench[pos][i], trench[(pos + 1) % trench.size()][i]);
      moved = moved || to_move.size() > 0;
    }

    step++;
    if (!moved)
      break;
  }

  std::copy(trench.begin(), trench.end(), std::ostream_iterator<std::string>(std::cout, "\n"));
  std::cout << "First non-moving step: " << step << '\n';
}
