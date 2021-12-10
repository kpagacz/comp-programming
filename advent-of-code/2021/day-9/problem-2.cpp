#include <iostream>
#include <string>
#include <vector>
#include <algorithm>
#include <numeric>
#include <array>
#include <set>
#include <cassert>
#include<deque>

typedef std::vector<std::vector<int>> smokemap;
using point = std::array<int, 2>;

const std::vector<point> cardinal_directions{{0, 1}, {0, -1}, {-1, 0}, {1, 0}};

void print_point(point p) { std::cout << p[0] << " " << p[1] << '\n'; }

point add_points(const point& p1, const point& p2) {
  return {p1[0] + p2[0], p1[1] + p2[1]};
}

std::vector<point> get_neighbours(point origin, smokemap map)
{
  std::vector<point> neighbours;
  for(const auto& d : cardinal_directions)
    neighbours.push_back(add_points(origin, d));
  return neighbours;
}

int point_hazard(point p, smokemap map)
{
  return map[p[0]][p[1]];
}

bool does_belong_to_basin(point p, smokemap map)
{
  if(point_hazard(p, map) == 9)
    return false;
  return true;
}

struct Basin {
  std::set<point> locations;
  point origin;
  Basin() = default;
  void print() {
    std::cout << "Basin size: " << locations.size() << " points:\n";
    for(const auto& p : locations)
      print_point(p);
  }
};

void expand_basin(Basin *basin, smokemap map)
{
  std::deque<point> possible_locations;
  possible_locations.push_back(basin->origin);
  while(possible_locations.size() > 0) {
    point front = possible_locations.front();
    possible_locations.pop_front();
    if (does_belong_to_basin(front, map) && basin->locations.find(front) == basin->locations.end()) {
      basin->locations.insert(front);
      auto ns = get_neighbours(front, map);
      for(auto n : ns)
        possible_locations.push_back(n);
    }
  }
}


bool is_low_point(smokemap map, point p)
{
  std::vector<point> ns = get_neighbours(p, map);
  for(const auto& n : ns) {
    if (!(point_hazard(p, map) < point_hazard(n, map)))
      return false;
  }

  return true;
}

int main()
{
  std::string line;
  smokemap heightmap;
  while (std::getline(std::cin, line))
  {
    std::vector<int> row_heights({9}); // first column of boundary
    for (const auto &c : line)
      row_heights.push_back(c - '0');
    row_heights.push_back(9); // last column of boundary
    heightmap.push_back(row_heights);
  }
  heightmap.insert(heightmap.begin(), std::vector<int>(heightmap[0].size(), 9)); // first row of boundary
  heightmap.push_back(std::vector<int>(heightmap[0].size(), 9)); // last row of boundary

  std::vector<Basin *> basins;
  for (int i = 0; i < heightmap.size(); i++)
  {
    for (int j = 0; j < heightmap[0].size(); j++)
    {
      if (is_low_point(heightmap, {i, j}))
      {
        Basin *new_basin = new Basin();
        new_basin->origin = point{i, j};
        basins.push_back(new_basin);
      }
    }
  }

  std::for_each(basins.begin(), basins.end(), [&](auto b)
                { expand_basin(b, heightmap); });

  std::sort(basins.begin(), basins.end(), [](auto a, auto b)
            { return a->locations.size() > b->locations.size(); });

  uint64_t product = 1;
  for(auto i : {0,1,2}) {
    product *= basins[i]->locations.size();
  }
  std::cout << "Product of 3 biggest basins: " << product << '\n';
}
