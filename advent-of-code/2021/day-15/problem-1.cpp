#include<iostream>
#include<string>
#include<vector>
#include<numeric>
#include<algorithm>
#include<set>

constexpr int INF = 10000000;
using adjacency = std::vector<std::vector<int>>;

int main() {
  // read input
  std::vector<int> weights;
  std::string line;
  while(std::cin >> line) {
    for(const auto& v : line)
      weights.push_back(v - '0');
  }

  const int width = line.size();
  const int height = weights.size() / line.size();

  // build adjacency matrix
  adjacency m(weights.size(), std::vector<int>(weights.size(), INF));

  for (auto i = 0U; i < weights.size(); i++) {
    if (i % width != 0)
      m[i][i - 1] = weights[i - 1];
    if (i % width != width - 1)
      m[i][i + 1] = weights[i + 1];
    if (i / width != 0)
      m[i][i - width] = weights[i - width];
    if (i / width != height - 1)
      m[i][i + width] = weights[i + width];
  }

  // djikstra on a set
  std::vector<int> shortest(weights.size() + 1, INF);

  std::set<int> queue;
  for (int i = 0; i < weights.size(); i ++)
    queue.insert(i);

  shortest[0] = 0;

  while(!queue.empty()) {
    int current_min_dist = weights.size();
    for(const auto& e : queue) if (shortest[e] < shortest[current_min_dist])
        current_min_dist = e;

    queue.erase(current_min_dist);

    for (auto i = 0; i < weights.size(); i++) {
      if (m[current_min_dist][i] != INF && m[current_min_dist][i] + shortest[current_min_dist] < shortest[i]) {
        shortest[i] = m[current_min_dist][i] + shortest[current_min_dist];
      }
    }
  }

  std::cout << "Cheapest path cost: " << shortest[weights.size() - 1];
}
