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

void print(const std::vector<std::array<int, 5>>& arr) {
  for (const auto& i : arr)
    std::cout << i[0] << " " << i[1] << " " << i[2] << " " << i[3] << " "
              << i[4] << "\n";
}

using event1d = std::array<int, 3>; // x1, x2,add(1) or del(0)
using event2d = std::array<int, 5>;  // x1, x2, y1, y2,add(1) or del(0)
using cube = std::array<int, 7>;     // x1,x2,y1,y2,z1,z2,add(1) or del(0)

int rectangles_area(std::vector<event2d> events) {
  int answer = 0;
  std::sort(events.begin(), events.end(),
            [](auto a, auto b) { return a[0] < b[0]; });

  print(events);

  return answer;
}

int main(int argc, char** argv) {
  // read input
  std::string type;
  int x1, x2, y1, y2, z1, z2;
  std::vector<cube> cube_events;

  // while (std::cin >> type) {
  //   std::scanf(" x=%d..%d,y=%d..%d,z=%d..%d", &x1, &x2, &y1, &y2, &z1, &z2);
  //   cube_events.push_back({x1, x2, y1, y2, z1, z2, type == "on"});
  // }

  std::vector<event2d> events{{1, 2, 3, 4, 0}, {0, 4, 3, 4, 1}};
  rectangles_area(events);
}
