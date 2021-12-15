#include<iostream>
#include<string>
#include<vector>
#include<numeric>
#include<algorithm>
#include<set>

constexpr int INF = 100000000;
using adjacency = std::vector<std::vector<int>>;

int main() {
  // read input
  std::vector<int> original_weights;
  std::string line;
  while(std::cin >> line) {
    for(const auto& v : line)
      original_weights.push_back(v - '0');
  }

  const int width = line.size();
  const int height = original_weights.size() / width;

  // enlarge input
  std::vector<int> weights(25 * original_weights.size(), 0);
  for(auto i = 0; i < height; i++)
    for (auto j = 0; j < width; j++) {
      weights[i * 5 * width + j] = original_weights[i * width + j];
    }

  for (auto i = 0; i < height; i++) {
    for (auto j = width; j < 5 * width; j++) {
      weights[i * 5 * width + j] = weights[i * 5 * width + j - width] + 1;
      if (weights[i * 5 * width + j] == 10)
        weights[i * 5 * width + j] = 1;
    }
  }

  for (int i = height; i < 5 * height; i ++) {
    for (int j = 0; j < 5 * width; j++) {
      weights[i * 5 * width + j] = weights[(i - height) * 5 * width + j] + 1;
      if (weights[i * 5 * width + j] == 10)
        weights[i * 5 * width + j] = 1;
    }
  }

  // djikstra (sort of)
  std::vector<int> shortest(weights.size() + 1, INF);

  std::set<int> queue;
  for (int i = 0; i < weights.size(); i++)
    queue.insert(i);

  shortest[0] = 0;

  while(!queue.empty()) {
    int current_min_dist = weights.size();
    for(const auto& e : queue) if (shortest[e] < shortest[current_min_dist])
        current_min_dist = e;

    queue.erase(current_min_dist);

    if (current_min_dist == weights.size() - 1)
      break;

    if (queue.size() % 100 == 0) std::cout << "Iteration: " << weights.size() - queue.size() << '\n';
    // if (current_min_dist == 0) {
    //   std::cout << (bool)(current_min_dist / (5 * width) != 0 && shortest[current_min_dist] + weights[current_min_dist - 5 * width] < shortest[current_min_dist - 5 * width]) << " "
    //             << (bool)(current_min_dist / (5 * width) != 5 * height - 1 && shortest[current_min_dist] + weights[current_min_dist + 5 * width] < shortest[current_min_dist + 5 * width]) << " "
    //             << (bool)(current_min_dist % (5 * width) != 0 && shortest[current_min_dist] + weights[current_min_dist - 1] < shortest[current_min_dist - 1]) << " "
    //             << (bool)(current_min_dist % (5 * width) != 5 * width - 1 && shortest[current_min_dist] + weights[current_min_dist + 1] < shortest[current_min_dist + 1]);
    // }

    if (current_min_dist / (5 * width) != 0 && shortest[current_min_dist] + weights[current_min_dist - 5 * width] < shortest[current_min_dist - 5 * width])
      shortest[current_min_dist - 5 * width] = shortest[current_min_dist] + weights[current_min_dist - 5 * width];

    if (current_min_dist / (5 * width) != 5 * height - 1 && shortest[current_min_dist] + weights[current_min_dist + 5 * width] < shortest[current_min_dist + 5 * width])
      shortest[current_min_dist + 5 * width] = shortest[current_min_dist] + weights[current_min_dist + 5 * width];

    if (current_min_dist % (5 * width) != 0 && shortest[current_min_dist] + weights[current_min_dist - 1] < shortest[current_min_dist - 1])
      shortest[current_min_dist - 1] = shortest[current_min_dist] + weights[current_min_dist - 1];

    if (current_min_dist % (5 * width) != 5 * width - 1 && shortest[current_min_dist] + weights[current_min_dist + 1] < shortest[current_min_dist + 1])
      shortest[current_min_dist + 1] = shortest[current_min_dist] + weights[current_min_dist + 1];
  }


  std::cout << "Cheapest path cost: " << shortest[weights.size() - 1];
}
