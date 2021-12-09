#include<iostream>
#include<string>
#include<vector>
#include<algorithm>
#include<numeric>

#define DEBUG 0
typedef std::vector<std::vector<int>*> smokemap;

bool is_low_point(smokemap map, int x, int y) {
  std::vector<int> x_perturb;
  if (x > 0)
    x_perturb.push_back(-1);
  if (x < map.size() - 1)
    x_perturb.push_back(1);
  for(const auto& i : x_perturb) {
    if (DEBUG) std::cout << x + 1 << " " << y << "\n";
    if (!(map[x + i]->at(y) > map[x]->at(y)))
      return false;
  }
  std::vector<int> y_perturb;
  if (y > 0)
    y_perturb.push_back(-1);
  if (y < map[0]->size() - 1)
    y_perturb.push_back(1);
  for(const auto& i : y_perturb) {
    if (DEBUG) std::cout << x << " " << y + 1 << "\n";
      if (!(map[x]->at(y + i) > map[x]->at(y)))
      return false;
  }

  return true;
}

int main() {
  std::string line;
  smokemap heightmap;
  while(std::getline(std::cin, line)) {
    std::vector<int>* row_heights = new std::vector<int>();
    for(const auto& c : line)
      row_heights->push_back(c - '0');
    heightmap.push_back(row_heights);
  }

  // for(const auto& v : heightmap) {
  //   for(const auto& i : *v)
  //     std::cout << i << " ";
  //   std::cout << '\n';
  // }

  int total_risk = 0;
  for (int i = 0; i < heightmap.size(); i++) {
    for (int j = 0; j < heightmap[0]->size(); j++) {
      if (is_low_point(heightmap, i, j)) {
        if (DEBUG) std::cout << "Low point: " << i << " " << j << " value: " << heightmap[i]->at(j) << '\n';
        total_risk += heightmap[i]->at(j) + 1;
      }
    }
  }

  std::cout << "Total risk: " << total_risk << '\n';
}
