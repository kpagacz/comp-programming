// link to the problem https://adventofcode.com/2021/day/20
#include<string>
#include<sstream>
#include<iostream>
#include<vector>
#include<cassert>
#include<array>
#include<algorithm>

using grid = std::vector<std::string>;
const int GRID_SIZE = 1000;
const int ENHANCEMENT_ROUNDS = 2;

char enhance_pixels(const std::string& algorithm, std::string pixels) {
  std::replace(pixels.begin(), pixels.end(), '.', '0');
  std::replace(pixels.begin(), pixels.end(), '#', '1');
  return algorithm[std::stoi(pixels, 0, 2)];
}

grid enhance_image(const grid& image, const std::string algorithm) {
  grid answer(image);
  for (int i = 1; i < GRID_SIZE - 1; i++) {
    for (int j = 1; j < GRID_SIZE - 1; j++) {
      std::string pixels;
      for (auto k = i - 1; k <= i  +1; k++)
        for (auto l = j - 1; l <= j + 1; l++) {
          pixels += image[k][l];
        }
      answer[i][j] = enhance_pixels(algorithm, pixels);
    }
  }

  return answer;
}

int count_lit(const grid& g) {
  int counter = 0;
  for(const auto& i : g) for(const auto& c : i) if (c == '#')
        counter++;
  return counter;
}

void print(const grid& g) {
  for(const auto& i : g) {
    for(const auto& c : i)
      std::cout << c << " ";
    std::cout << '\n';
  }
}

int main(int argc, char** argv) {
  // read input
  std::string algorithm;
  std::cin >> algorithm;

  grid g(0.4 * GRID_SIZE, std::string(GRID_SIZE, '.'));

  grid lines;
  std::string line;
  while(std::cin >> line) {
    line.insert(line.begin(), 0.4 * GRID_SIZE, '.');
    line.insert(line.end(), GRID_SIZE - line.size(), '.');
    assert(line.size() == GRID_SIZE);
    lines.push_back(line);
  }

  g.insert(g.end(), lines.begin(), lines.end());
  g.insert(g.end(), GRID_SIZE - g.size(), std::string(GRID_SIZE, '.'));
  assert(g.size() == GRID_SIZE);

  // apply enhancement
  for (int i = 0; i < ENHANCEMENT_ROUNDS; i++) {
    std::cout << "Number of lit pixels: " << count_lit(g) << '\n';
    print(g);
    g = enhance_image(g, algorithm);
  }

  print(g);
  std::cout << "Number of lit pixels: " << count_lit(g) << '\n';
}
