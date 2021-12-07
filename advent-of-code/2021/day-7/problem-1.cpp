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
  int min_answer = positions[positions.size() / 2];
  int min_sum = std::accumulate(positions.begin(), positions.end(), 0, [&](int a, int b)
                                { return a + std::abs(b - min_answer); });
  std::cout << "min answer: " << min_answer << '\n'
            << "min sum: " << min_sum << '\n';
}
