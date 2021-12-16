// link to the problem https://adventofcode.com/2021/day/15
#include<iostream>
#include<string>
#include<vector>
#include<numeric>
#include<algorithm>
#include<set>
#include<queue>

constexpr int INF = 100000000;

int main() {
  // read input
  std::vector<int> original_weights;
  std::string line;
  while (std::cin >> line) {
    for (const auto &v : line)
      original_weights.push_back(v - '0');
  }

  const int width = line.size();
  const int height = original_weights.size() / width;

  // enlarge input
  std::vector<int> weights(25 * original_weights.size(), 0);
  for (auto i = 0; i < height; i++)
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

  for (int i = height; i < 5 * height; i++) {
    for (int j = 0; j < 5 * width; j++) {
      weights[i * 5 * width + j] = weights[(i - height) * 5 * width + j] + 1;
      if (weights[i * 5 * width + j] == 10)
        weights[i * 5 * width + j] = 1;
    }
  }

  // djikstra (sort of)
  // optimizations:
  // * min heap of vertices instead of vector for logarithmic find and remove the vertex with the shortest path to it
  //   (instead of linear in case of vector)
  // * early stopping when the destination was achieved (tests say this did not do much)
  // * delayed deletion instead of decreasing the key of the vertex in the heap:
  //   I didn't want to implement my own min heap with the decrease key operation. It would mean I have to implement
  //   my own heap structure which felt like more work work than worth.
  //   Delayed deletion means that instead of decreasing the key of the vertex in the heap,
  //   I push the same vertex to the heap. Then, I keep track of whether the vertex was already processed and
  //   if so, then I just skip updates of the shortest path. This comes with some overhead but in terms of
  //   code complication beats anything else while still maintaining the logarithmic complexity.
  // * add vertices gradually instead of pushing them all at once in the beginning. I think this was the most
  //   impactful change (besides changing the data structure). There are 250000 vertices in this problem, it's a huge
  //   deal if we can limit the number of them in all iterations.
  std::vector<int> shortest(weights.size() + 1, INF);

  auto comparator = [&](const int& a, const int& b) { return shortest[a] > shortest[b];};

  std::vector<int> under{0};
  std::priority_queue queue{comparator, under};

  shortest[0] = 0;

  std::vector<bool> done(weights.size(), false);

  while (!queue.empty()) {
    int current_min_dist = queue.top();
    queue.pop();
    if (done[current_min_dist]) continue;
    done[current_min_dist] = true;

    if (current_min_dist == weights.size() - 1)
      break;

    if (current_min_dist / (5 * width) != 0 &&
        shortest[current_min_dist] + weights[current_min_dist - 5 * width] < shortest[current_min_dist - 5 * width]) {
      shortest[current_min_dist - 5 * width] = shortest[current_min_dist] + weights[current_min_dist - 5 * width];
      queue.push(current_min_dist - 5 * width);
    }

    if (current_min_dist / (5 * width) != 5 * height - 1 &&
        shortest[current_min_dist] + weights[current_min_dist + 5 * width] < shortest[current_min_dist + 5 * width]) {
      shortest[current_min_dist + 5 * width] = shortest[current_min_dist] + weights[current_min_dist + 5 * width];
      queue.push(current_min_dist + 5 * width);
    }

    if (current_min_dist % (5 * width) != 0 &&
        shortest[current_min_dist] + weights[current_min_dist - 1] < shortest[current_min_dist - 1]) {
      shortest[current_min_dist - 1] = shortest[current_min_dist] + weights[current_min_dist - 1];
      queue.push(current_min_dist - 1);
    }

    if (current_min_dist % (5 * width) != 5 * width - 1 &&
        shortest[current_min_dist] + weights[current_min_dist + 1] < shortest[current_min_dist + 1]) {
      shortest[current_min_dist + 1] = shortest[current_min_dist] + weights[current_min_dist + 1];
      queue.push(current_min_dist + 1);
    }
  }

  std::cout << "Cheapest path cost: " << shortest[weights.size() - 1] << '\n';
}
