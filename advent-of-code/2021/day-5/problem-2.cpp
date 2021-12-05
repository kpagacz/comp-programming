// link to the problems https://adventofcode.com/2021/day/5
#include<iostream>
#include<cstdio>
#include<array>
#include<unordered_map>
#include<algorithm>
#include<cassert>

typedef std::array<int, 2U> point;

namespace std {
  template <>
  struct hash<point> {
    std::size_t operator()(const point& key) const {
      std::size_t hash = 17;
      hash = (hash * 31 + std::hash<int>()(key[0])) % 1000000001;
      hash = (hash * 63 + std::hash<int>()(key[1])) % 1000000001;
      return hash;
    }
  };
};

int isSegmentOrthogonal(point p1, point p2) {
  if (p1[0] == p2[0])
    return 0;
  if (p1[1] == p2[1])
    return 1;
  return -1;
}

void addPoint(std::unordered_map<point,int>& crossings, const point& p) {
  auto elem = crossings.find(p);
  if (elem != crossings.end()) {
    elem->second = elem->second + 1;
  } else {
    crossings.insert({p, 1});
  }
}

int main() {
  // read input
  int x1, x2, y1, y2;
  std::unordered_map<point, int> crossings;

  while((scanf("%d,%d -> %d,%d", &x1, &y1, &x2, &y2)) != -1) {
    point p1 = point({x1, y1});
    point p2 = point({x2, y2});
    int coord = isSegmentOrthogonal(p1, p2);
    if (coord == -1) {
      auto x_increment = x2 > x1 ? 1 : -1;
      auto y_increment = y2 > y1 ? 1 : -1;
      for (auto i = x1, j = y1; i != x2; i += x_increment, j += y_increment) {
        addPoint(crossings, point({i, j}));
      }
    }
    if (coord == 0) {
      assert(x1 == x2);
      auto increment = y2 > y1 ? 1 : -1;
      for (auto i = y1; i != y2; i += increment)
        addPoint(crossings, point({x1, i}));
    }
    if (coord == 1) {
      assert(y1 == y2);
      auto increment = x2 > x1 ? 1 : -1;
      for (auto i = x1; i != x2; i += increment)
        addPoint(crossings, point({i, y1}));
    }
    addPoint(crossings, p2);
  }

  auto answer = std::count_if(crossings.begin(), crossings.end(), [](std::unordered_map<point, int>::value_type entry)
                              { return entry.second > 1; });

  std::cout << "Number of points where the lines cross: " << answer << '\n';
}
