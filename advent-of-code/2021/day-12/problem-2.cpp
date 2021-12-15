#include<iostream>
#include<stdlib.h>
#include<string>
#include<vector>
#include<unordered_map>
#include<sstream>
#include<algorithm>
#include<queue>

using mapping = std::unordered_map<std::string, int>;
using grid = std::vector<std::vector<int>>;

void print_grid(const grid& g) {
  for (int i = 0; i < g.size(); i++) {
    for (int j = 0; j < g.size(); j++)
      std::cout << g[i][j] << " ";
    std::cout << "\n";
  }
}

template<class T, class U>
void print_map(const std::unordered_map<T, U>& m) {
  for (const auto& entry : m) {
    std::cout << "Key: " << entry.first << " Value: " << entry.second << '\n';
  }
}

void print_path(std::vector<int> p) {
  for(auto i : p)
    std::cout << i << " ";
  std::cout << '\n';
}

std::pair<std::string, std::string> parse_line(std::string line) {
  std::string from, to;
  std::stringstream in(line);
  std::getline(in, from, '-');
  std::getline(in, to, '-');

  return std::make_pair(from, to);
}

bool visited_small_cave_twice(const std::vector<int>& path, const std::vector<bool>& small_caves) {
  std::unordered_map<int, int> occurences;
  std::for_each(path.begin(), path.end(), [&](auto i)
                { auto result = occurences.insert({i, 1});
                  if (!result.second)
                    ++(result.first->second); });
  return std::any_of(occurences.begin(), occurences.end(), [&](auto p)
                     { return small_caves[p.first] == true && (p.second > 1); });
}

int find_paths(const int& source, const int& destination, const grid& g, const std::vector<bool>& small_caves) {
  int answer = 0;

  std::queue<std::vector<int>> paths;
  std::vector<int> path;
  path.push_back(source);
  paths.push(path);

  while(!paths.empty()) {
    path = paths.front();
    paths.pop();

    if (path.back() == destination) {
      answer++;
      continue;
    }

    auto visited_small_twice = visited_small_cave_twice(path, small_caves);
    for (auto i = 0U; i < g.size(); i++) {
      bool connected{g[path.back()][i] == 1};
      bool is_start{i == source };
      bool is_small_cave_on_path{small_caves[i] == true && std::find(path.begin(), path.end(), i) != path.end()};
      if (connected && !is_start && (!is_small_cave_on_path || !visited_small_twice)) {
        std::vector<int> new_path{path};
        new_path.push_back(i);
        paths.push(new_path);
      }
    }
  }

  return answer;
}


int main() {
  // read input
  grid adjacency;
  mapping map;
  std::string from, to;
  std::string line;
  while (std::getline(std::cin, line)) {
    auto vs = parse_line(line);
    auto from = vs.first, to = vs.second;
    if(map.find(from) == map.end()) {
      map.insert({from, map.size()});
    }
    if(map.find(to) == map.end()) {
      map.insert({to, map.size()});
    }

    adjacency.resize(map.size(), std::vector<int>(map.size(), 0));
    for(auto& v : adjacency)
      v.resize(map.size(), 0);

    adjacency[map[from]][map[to]] = 1;
    adjacency[map[to]][map[from]] = 1;

  }
  print_map<std::string, int>(map);
  print_grid(adjacency);
  std::cout << '\n';

  // mark small caves
  std::vector<bool> small_caves(map.size(), false);
  for(const auto& e : map) {
    if (std::all_of(e.first.begin(), e.first.end(), &islower))
      small_caves[e.second] = true;
  }

  // find routes
  int number_of_routes = 0;
  auto paths = find_paths(map["start"], map["end"], adjacency, small_caves);

  std::cout << "Total paths: " << paths << '\n';
}
