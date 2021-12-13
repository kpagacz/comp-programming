#include <iostream>
#include <string>
#include <algorithm>
#include <vector>
#include <array>
#include <deque>
#include<cassert>

constexpr int DAYS = 100;
constexpr int BORDER = DAYS * -1000000;

using point = std::array<int, 2>;
using grid = std::vector<std::vector<int>>;

const std::vector<point> cardinal_directions = {
    {-1, -1}, {-1, 0}, {-1, 1}, {0, -1}, {0, 1}, {1, -1}, {1, 0}, {1, 1}};

constexpr point add_points(const point p1, const point p2)
{
  return {p1[0] + p2[0], p1[1] + p2[1]};
}

int get_value(const point& p, const grid& g)
{
  return g[p[0]][p[1]];
}

std::vector<point> get_flashing(grid g)
{
  std::vector<point> answer;
  for (auto i = 1; i < g.size() - 1; i++)
  {
    for (auto j = 1; j < g.size() - 1; j++)
    {
      if (g[i][j] == 10)
        answer.push_back({i, j});
    }
  }
  return answer;
}

void print_octopuses(grid octopuses) {
  for (int i = 1; i < octopuses.size() - 1; i++) {
    for (int j = 1; j < octopuses.size() - 1; j++)
      std::cout << octopuses[i][j];
    std::cout << '\n';
  }
  std::cout << '\n';
}

std::vector<point> flash_one(point origin, grid& g) {
  std::vector<point> answer;
  for(const auto& c : cardinal_directions) {
    point to_increment = add_points(origin, c);
    ++g[to_increment[0]][to_increment[1]];
    if (get_value(to_increment, g) == 10)
      answer.push_back({to_increment[0], to_increment[1]});
  }

  return answer;
}

bool are_synced(const grid& g) {
  for (auto i = 1; i < g.size() - 1; i++) {
    for (auto j = 1; j < g.size() - 1; j++) {
      if (g[i][j] != 0)
        return false;
    }
  }
  return true;
}


int main()
{
  // read input
  grid octopuses;
  octopuses.push_back(std::vector<int>(12, BORDER));
  std::string line;
  while (std::cin >> line)
  {
    std::vector<int> row;
    row.push_back(BORDER); // border
    for (auto &c : line)
      row.push_back(c - '0');
    row.push_back(BORDER);
    octopuses.push_back(row);
  }
  octopuses.push_back(std::vector<int>(12, BORDER));
  assert(octopuses.size() == octopuses[0].size());

  // days
  int flashed;
  for (auto i = 0; true; i++)
  {
    // increment all by one
    for (auto i = 1; i < octopuses.size() - 1; i++)
      for (auto j = 1; j < octopuses.size() - 1; j++)
        octopuses[i][j]++;

    // find flashing octopuses
    auto flashing = get_flashing(octopuses);
    std::deque<point> to_flash;
    for(const auto& f : flashing)
      to_flash.push_back(f);

    while(!to_flash.empty()) {
      point f = to_flash.front();
      to_flash.pop_front();

      auto more = flash_one(f, octopuses);
      for(auto m : more)
        to_flash.push_back(m);
    }

    // zero flashed octopuses
    for (auto i = 1; i < octopuses.size() - 1; i++)
      for (auto j = 1; j < octopuses.size() - 1; j++)
        if (octopuses[i][j] > 9) octopuses[i][j]=0;

    std::cout << "end of day " << i + 1 << " octopuses:\n";
    print_octopuses(octopuses);

    if (are_synced(octopuses)) {
      flashed = i + 1;
      break;
    }
  }


  std::cout << "Day all flashed: " << flashed << '\n';
}
