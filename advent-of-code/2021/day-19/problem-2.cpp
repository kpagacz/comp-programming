// link to the problem https://adventofcode.com/2021/day/19
#include <array>
#include <iostream>
#include <queue>
#include <set>
#include <sstream>
#include <string>
#include <unordered_map>
#include <vector>
#include<cassert>

using point = std::array<int, 3>;
using scanner = std::vector<point>;

const int ALIGN_THRESHOLD = 12;

namespace std {
template <>
struct hash<point> {
  size_t operator()(const point& p) const {
    size_t h = 14331;
    for (const auto& i : p) h += 31 * i;
    return h;
  }
};
}  // namespace std

std::vector<scanner> read_input() {
  std::string line;
  std::vector<scanner> scanners;
  while (std::getline(std::cin, line)) {
    if (line[0] == '-' && line[1] == '-') {
      scanners.push_back(scanner());
      continue;
    }
    if (line == "") {
      continue;
    } else {
      std::stringstream in(line);
      std::vector<std::string> coords;
      std::string token;
      while (std::getline(in, token, ',')) {
        coords.push_back(token);
      }

      int x = std::stoi(coords[0], 0, 10);
      int y = std::stoi(coords[1], 0, 10);
      int z = std::stoi(coords[2], 0, 10);
      scanners.back().push_back(point{x, y, z});
    }
  }
  return scanners;
}

point add(const point& a, const point& b) {
  return point{a[0] + b[0], a[1] + b[1], a[2] + b[2]};
}

point difference(const point& a, const point& b) {
  return point{a[0] - b[0], a[1] - b[1], a[2] - b[2]};
}

bool same_probes(const point& a, const point& b, const point& diff) {
  return add(a, diff) == b;
}

void X_rotate_scanner(scanner& s) {
  for (auto& p : s) {
    int temp_y = p[1];
    p[1] = -p[2];
    p[2] = temp_y;
  }
}

void Y_rotate_scanner(scanner& s) {
  for (auto& p : s) {
    int temp = p[0];
    p[0] = p[2];
    p[2] = -temp;
  }
}

void Z_rotate_scanner(scanner& s) {
  for (auto& p : s) {
    int temp = p[0];
    p[0] = -p[1];
    p[1] = temp;
  }
}

std::pair<point, int> count_aligned(const scanner& ref, const scanner& s) {
  std::unordered_map<point, int> diffs;
  for (const auto& k : ref)
    for (const auto& j : s) {
      auto added = diffs.insert({difference(k, j), 1});
      if (!added.second) added.first->second++;
    }

  return *std::max_element(diffs.begin(), diffs.end(),
                           [](auto a, auto b) { return a.second < b.second; });
}

std::pair<point, int> are_aligned(const scanner& ref, const scanner& s) {
  return count_aligned(ref, s);
}

void print(const scanner& s) {
  for (const auto& i : s)
    std::cout << i[0] << " " << i[1] << " " << i[2] << "\n";
  std::cout << '\n';
}

std::pair<point, int> orient_scanner(const scanner& ref, scanner& s) {
  // all 24 possible orientations
  for (int j = 0; j < 4; j++) {
    for (int i = 0; i < 4; i++) {
    auto translation_vector = count_aligned(ref, s);
    if (translation_vector.second >= ALIGN_THRESHOLD) {
      std::for_each(s.begin(), s.end(),
                    [&](auto& p) { p = add(p, translation_vector.first); });
      return translation_vector;
    }
      Z_rotate_scanner(s);
    }
    X_rotate_scanner(s);
  }

  Y_rotate_scanner(s);
  for (int i = 0; i < 4; i++) {
    auto translation_vector = count_aligned(ref, s);
    if (translation_vector.second >= ALIGN_THRESHOLD) {
      std::for_each(s.begin(), s.end(),
                    [&](auto& p) { p = add(p, translation_vector.first); });
      return translation_vector;
    }
    Z_rotate_scanner(s);
  }

  Y_rotate_scanner(s);
  Y_rotate_scanner(s);
  for (int i = 0; i < 4; i++) {
    auto translation_vector = count_aligned(ref, s);
    if (translation_vector.second >= ALIGN_THRESHOLD) {
      std::for_each(s.begin(), s.end(),
                    [&](auto& p) { p = add(p, translation_vector.first); });
      return translation_vector;
    }
    Z_rotate_scanner(s);
  }

  return {{0, 0, 0}, {0}};
}

int main() {
  std::vector<scanner> ss = read_input();

  std::queue<int> oriented;
  oriented.push(0);
  std::vector<bool> to_orient(ss.size(), true);
  to_orient[0] = false;

  scanner scanner_locations;
  scanner_locations.push_back({0, 0, 0});

  while (std::any_of(to_orient.begin(), to_orient.end(),
                     [](auto p) { return p; })) {
    assert(!oriented.empty());
    auto ref = oriented.front();
    oriented.pop();
    for (int i = 1; i < to_orient.size(); i++) {
      if (to_orient[i]) {
        auto result = orient_scanner(ss[ref], ss[i]);
        if (result.second >= ALIGN_THRESHOLD) {
          to_orient[i] = false;
          oriented.push(i);
          scanner_locations.push_back(result.first);
        }
      }
    }
  }

  int largest_manhattan = 0;
  for (int i = 0; i < scanner_locations.size(); i++)
    for (int j = i; j < scanner_locations.size(); j++) {
      int manhattan = 0;
      for (auto k = 0; k < 3; k++) {
        manhattan += std::abs(scanner_locations[i][k] - scanner_locations[j][k]);
      }
      if (manhattan > largest_manhattan) largest_manhattan = manhattan;
    }

  std::cout << "Largest Manhattan: " << largest_manhattan << '\n';
}
