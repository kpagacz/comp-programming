#include<iostream>
#include<string>
#include<sstream>
#include<vector>
#include<algorithm>
#include<numeric>
#include<utility>

int main() {
  // input
  std::string line;
  std::cin >> line;
  std::stringstream line_stream(line);
  std::vector<int> positions;
  std::string number;
  while(std::getline(line_stream, number, ',')) {
    positions.push_back(std::stoi(number, 0, 10));
  }

  // calculate median
  std::sort(positions.begin(), positions.end());
  int min_answer = positions.front();
  int minimal_sum = std::accumulate(positions.begin(), positions.end(), 0, [&](int a, int b)
                                    { return a + std::abs(b - min_answer) * (std::abs(b - min_answer) + 1) / 2; });

  for (auto i = positions.front(); i <= positions.back(); i++) {
    int current_sum = std::accumulate(positions.begin(), positions.end(), 0, [&](int a, int b)
                                      { return a + std::abs(b - i) * (std::abs(b - i) + 1) / 2; });
    if (current_sum < minimal_sum)
    {
      minimal_sum = current_sum;
      min_answer = i;
    }
  }

  std::cout << "Minimal sum: " << minimal_sum << '\n'
            << "answer: " << min_answer << '\n';
}
