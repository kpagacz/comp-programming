// link to the problem https://adventofcode.com/2021/day/22
#include <stdio.h>

#include <algorithm>
#include <array>
#include <iostream>
#include <numeric>
#include <queue>
#include <string>
#include <vector>

using segment = std::array<int, 2>;    // x1, x2
using rectangle = std::array<int, 4>;  // x1, x2, y1, y2
using cube = std::array<int, 6>;       // x1, x2, y1, y2, z1, z2

void print(const std::vector<rectangle>& arr) {
  for (const auto& i : arr)
    std::cout << i[0] << " " << i[1] << " " << i[2] << " " << i[3] << " "
              << i[4] << "\n";
}

template <class T>
void print(const T arr) {
  for (const auto& i : arr) std::cout << i << " ";
  std::cout << '\n';
}

int segment_length(const std::vector<segment>& arr) {
  using event = std::array<int, 2>;  // x, begin(0) or end(1)
  std::vector<event> underlying;
  for (auto i = 0; i < arr.size(); i++) {
    auto e = arr[i];
    underlying.push_back({e[0], 0});
    underlying.push_back({e[1], 1});
  }

  auto event_comp = [](const event& a, const event& b) { return a[0] > b[0]; };
  std::priority_queue events{event_comp, underlying};

  int total_length = 0;
  int counter = 0;
  int last_x = 0;
  while (!events.empty()) {
    event top = events.top();
    events.pop();

    if (counter != 0) total_length += top[0] - last_x;
    last_x = top[0];

    if (top[1] == 0) {
      counter++;
    } else {
      counter--;
    }
  }

  return total_length;
}

uint64_t rectangles_area(const std::vector<rectangle>& arr) {
  using event = std::array<int, 5>;  // x, y1, y2, begin/end, owner

  std::vector<event> underlying;
  for (int i = 0; i < arr.size(); i++) {
    const auto& r = arr[i];
    underlying.push_back({r[0], r[2], r[3], 0, i});
    underlying.push_back({r[1], r[2], r[3], 1, i});
  }
  auto event_comp = [](const event& a, const event& b) { return a[0] > b[0]; };
  std::priority_queue events(event_comp, underlying);

  uint64_t answer = 0;
  int last_x = 0;
  uint64_t segments_length = 0;
  std::vector<bool> active_rs(arr.size(), false);
  while (!events.empty()) {
    event top = events.top();
    events.pop();

    answer += ((top[0] - last_x)) * (segments_length);
    last_x = top[0];

    active_rs[top[4]] = top[3] == 0;
    std::vector<segment> segments;
    for (int i = 0; i < arr.size(); i++) {
      if (active_rs[i]) {
        segment s{arr[i][2], arr[i][3]};
        segments.push_back(s);
      }
    }
    segments_length = segment_length(segments);
  }
  return answer;
}

uint64_t cubes_volume(const std::vector<cube>& arr) {
  using event = std::array<int, 7>;  // x, y1, y2, z1, z2, begin/end, owner

  std::vector<event> underlying;
  for (int i = 0; i < arr.size(); i++) {
    const auto& c = arr[i];
    underlying.push_back({c[0], c[2], c[3], c[4], c[5], 0, i});
    underlying.push_back({c[1], c[2], c[3], c[4], c[5], 1, i});
  }

  auto event_comp = [](const event& a, const event& b) { return a[0] > b[0]; };
  std::priority_queue events(event_comp, underlying);

  uint64_t answer = 0;
  int last_x = 0;
  uint64_t active_area = 0;
  std::vector<bool> active_cs(arr.size(), false);
  while (!events.empty()) {
    event top = events.top();
    events.pop();

    answer += (top[0] - last_x) * active_area;
    last_x = top[0];

    active_cs[top[6]] = top[5] == 0;
    std::vector<rectangle> rects;
    for (int i = 0; i < arr.size(); i++) {
      if (active_cs[i]) {
        const auto& c = arr[i];
        rectangle r{c[2], c[3], c[4], c[5]};
        rects.push_back(r);
      }
    }

    active_area = rectangles_area(rects);
  }

  return answer;
}

bool cubes_overlap(const cube& c1, const cube& c2) {
  return c1[1] > c2[0] && c1[0] < c2[1] && c1[3] > c2[2] && c1[2] < c2[3] &&
         c1[5] > c2[4] && c1[4] < c2[5];
}

std::vector<cube> split_cube(const cube& c, const cube& splitter) {
  std::vector<cube> answer;
  if (!cubes_overlap(c, splitter)) {
    answer.push_back(c);
    return answer;
  }
  // There will be 6 possible cubes
  int x1, x2, y1, y2, z1, z2;
  // 1
  z1 = splitter[5];
  z2 = c[5];
  x1 = std::max(splitter[0], c[0]);
  x2 = std::min(splitter[1], c[1]);
  y1 = std::max(splitter[2], c[2]);
  y2 = std::min(splitter[3], c[3]);
  if (z2 > z1) answer.push_back({x1, x2, y1, y2, z1, z2});
  // 2
  z1 = c[4];
  z2 = splitter[4];
  if (z2 > z1) answer.push_back({x1, x2, y1, y2, z1, z2});
  // 3
  x1 = std::max(splitter[0], c[0]);
  x2 = std::min(splitter[1], c[1]);
  z1 = c[4];
  z2 = c[5];
  y1 = splitter[3];
  y2 = c[3];
  if (y2 > y1) answer.push_back({x1, x2, y1, y2, z1, z2});
  // 4
  y1 = c[2];
  y2 = splitter[2];
  if (y2 > y1) answer.push_back({x1, x2, y1, y2, z1, z2});
  // 5
  x1 = c[0];
  x2 = splitter[0];
  y1 = c[2];
  y2 = c[3];
  z1 = c[4];
  z2 = c[5];
  if (x2 > x1) answer.push_back({x1, x2, y1, y2, z1, z2});
  // 6
  x1 = splitter[1];
  x2 = c[1];
  if (x2 > x1) answer.push_back({x1, x2, y1, y2, z1, z2});

  return answer;
}

int main(int argc, char** argv) {
  // read input
  std::string type;
  int x1, x2, y1, y2, z1, z2;
  std::vector<cube> cubes;

  while (std::cin >> type) {
    std::scanf(" x=%d..%d,y=%d..%d,z=%d..%d", &x1, &x2, &y1, &y2, &z1, &z2);
    if (type == "on") {
      cubes.push_back({x1, x2 + 1, y1, y2 + 1, z1, z2 + 1});
    } else {
      std::vector<cube> new_cubes;
      std::for_each(cubes.begin(), cubes.end(), [&](auto c) {
        auto to_insert = split_cube(c, {{x1, x2 + 1, y1, y2 + 1, z1, z2 + 1}});
        new_cubes.insert(new_cubes.end(), to_insert.begin(), to_insert.end());
      });
      cubes = new_cubes;
    }
  }

  std::cout << cubes_volume(cubes) << '\n';
}
