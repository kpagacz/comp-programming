// link to the problem https://adventofcode.com/2021/day/22
#include <stdio.h>

#include <algorithm>
#include <array>
#include <iostream>
#include <numeric>
#include <set>
#include <string>
#include <unordered_map>
#include <vector>

using point = std::array<int, 3>;

namespace std {
template <>
struct hash<point>{
  std::size_t operator()(const point& k) const {
    std::size_t answer = (int)1e9 + 5;
    std::size_t h1 = std::hash<int>{}(k[0]), h2 = std::hash<int>{}(k[1]),
                h3 = std::hash<int>{}(k[2]);
    for (const auto& v : {h1, h2, h3}) answer += 101 * answer + v;

    return answer;
  }
};
}

int main(int argc, char** argv) {
  std::unordered_map<point, bool> cubes;

  std::string type;
  int x1, x2, y1, y2, z1, z2;
  while (std::cin >> type) {
    std::scanf(" x=%d..%d,y=%d..%d,z=%d..%d", &x1, &x2, &y1, &y2, &z1, &z2);
    bool stop = false;
    for (const auto& i : {x1, x2, y1, y2, z1, z2})
      stop = stop || std::abs(i) > 50;
    if (stop) continue;
    if (type == "on") {
      for (auto i = x1; i <= x2; i++)
        for (auto j = y1; j <= y2; j++)
          for (auto k = z1; k <= z2; k++) cubes.insert({point{i, j, k}, true});
    } else {
      for (auto i = x1; i <= x2; i++)
        for (auto j = y1; j <= y2; j++)
          for (auto k = z1; k <= z2; k++) cubes.erase(point{i, j, k});
    }
    std::cout << "On: " << cubes.size() << '\n';
  }

  std::cout << "Number of turned on cubes: " << cubes.size() << '\n';
}
