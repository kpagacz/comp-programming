#include<iostream>
#include<string>
#include<sstream>
#include<vector>
#include<numeric>
#include<cmath>

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

  // calculate mean
  int answer = (std::accumulate(positions.begin(), positions.end(), 0.0) / positions.size());
  int min_sum = std::accumulate(positions.begin(), positions.end(), 0, [&](int a, int b)
                                { return a + std::abs(b - answer) * (std::abs(b - answer) + 1) / 2; });
  std::cout << "Min sum: " << min_sum << '\n'
            << "Min position: " << answer << '\n';
}
