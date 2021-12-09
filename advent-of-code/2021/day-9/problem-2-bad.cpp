#include <iostream>
#include <string>
#include <vector>
#include <algorithm>
#include <numeric>
#include <array>
#include <set>
#include <cassert>

#define DEBUG 0
typedef std::vector<std::vector<int> *> smokemap;
using point = std::array<int, 2>;

void print_point(point *p) { std::cout << p->at(0) << " " << p->at(1) << '\n'; }

struct BasinPointComparator
{
  bool operator()(point *a, point *b) const
  {
    // return a->at(0) < b->at(0);
    if (a->at(0) < b->at(0))
      return true;
    else
      return a->at(1) < b->at(1);
  }
};

struct Basin
{
  Basin() = default;
  std::set<point *, BasinPointComparator> locations;
  void print_points()
  {
    std::cout << "Basin points " << locations.size() << " START:\n";
    for (auto p : locations)
    {
      print_point(p);
    }
    std::cout << "Basin points END\n";
  }
};

std::vector<point *> get_neighbours(point *origin, smokemap map)
{
  std::vector<point *> neighbours;
  int x = origin->at(0), y = origin->at(1);
  if (x > 0)
    neighbours.push_back(new point({x - 1, y}));
  if (x < map.size() - 1)
    neighbours.push_back(new point({x + 1, y}));
  if (y > 0)
    neighbours.push_back(new point({x, y - 1}));
  if (y < map[0]->size() - 1)
    neighbours.push_back(new point({x, y + 1}));
  return neighbours;
}

int point_hazard(point *p, smokemap map)
{
  return map[p->at(0)]->at(p->at(1));
}

bool does_belong_to_basin(point *p, std::vector<point *> neighbours, Basin *basin, smokemap map)
{
  if(point_hazard(p, map) == 9)
    return false;
  auto PointComparator = [&](point *a, point *b)
  { return point_hazard(a, map) < point_hazard(b, map); };
  for (auto n : neighbours)
  {
    if (!PointComparator(p, n) && std::find_if(basin->locations.begin(), basin->locations.end(), [&](auto loc)
                                               { return loc->at(0) == n->at(0) && loc->at(1) == n->at(1); }) == basin->locations.end())
    {
      bool temp = basin->locations.find(n) == basin->locations.end();
      return false;
    }
  }
  return true;
}

void expand_basin(Basin *basin, smokemap map)
{
  std::vector<point *> potential_locations;
  auto neighbours = get_neighbours(*(basin->locations.begin()), map);
  potential_locations.insert(potential_locations.end(), neighbours.begin(), neighbours.end());
  auto PointComparator = [&](point *a, point *b)
  { return point_hazard(a, map) > point_hazard(b, map); };
  std::make_heap(potential_locations.begin(), potential_locations.end(), PointComparator);
  while (potential_locations.size() > 0)
  {
    std::pop_heap(potential_locations.begin(), potential_locations.end(), PointComparator);
    point *min_point = potential_locations.back();
    potential_locations.pop_back();
    auto neighbours = get_neighbours(min_point, map);
    if (does_belong_to_basin(min_point, neighbours, basin, map))
    {
      basin->locations.insert(min_point);
      neighbours.erase(std::remove_if(neighbours.begin(), neighbours.end(), [&](point *n)
                                      { return std::find_if(basin->locations.begin(), basin->locations.end(), [&](auto loc)
                                                            { return loc->at(0) == n->at(0) && loc->at(1) == n->at(1); }) != basin->locations.end(); }),
                       neighbours.end());
      neighbours.erase(std::remove_if(neighbours.begin(), neighbours.end(), [&](point *n)
                                      { return std::find_if(potential_locations.begin(), potential_locations.end(), [&](auto loc)
                                                            { return n->at(0) == loc->at(0) && n->at(1) == loc->at(1); }) != potential_locations.end(); }),
                       neighbours.end());
      for (auto n : neighbours)
      {
        potential_locations.push_back(n);
        std::push_heap(potential_locations.begin(), potential_locations.end(), PointComparator);
      }
    }
  }
}

bool is_low_point(smokemap map, int x, int y)
{
  std::vector<int> x_perturb;
  if (x > 0)
    x_perturb.push_back(-1);
  if (x < map.size() - 1)
    x_perturb.push_back(1);
  for (const auto &i : x_perturb)
  {
    if (!(map[x + i]->at(y) > map[x]->at(y)))
      return false;
  }
  std::vector<int> y_perturb;
  if (y > 0)
    y_perturb.push_back(-1);
  if (y < map[0]->size() - 1)
    y_perturb.push_back(1);
  for (const auto &i : y_perturb)
  {
    if (!(map[x]->at(y + i) > map[x]->at(y)))
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
    std::vector<int> *row_heights = new std::vector<int>();
    for (const auto &c : line)
      row_heights->push_back(c - '0');
    heightmap.push_back(row_heights);
  }

  std::vector<Basin *> basins;
  for (int i = 0; i < heightmap.size(); i++)
  {
    for (int j = 0; j < heightmap[0]->size(); j++)
    {
      if (is_low_point(heightmap, i, j))
      {
        if (DEBUG)
          std::cout << "Low point: " << i << " " << j << " value: " << heightmap[i]->at(j) << '\n';
        Basin *new_basin = new Basin();
        new_basin->locations.insert(new point({i, j}));
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
