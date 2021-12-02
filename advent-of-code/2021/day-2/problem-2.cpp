#include<iostream>
#include<string>

int main() {
  std::string direction;
  int unit;

  int total_depth = 0, total_forward = 0, aim = 0;

  while(std::cin >> direction) {
    std::cin >> unit;

    if (direction == "forward") {
      total_forward += unit;
      total_depth += aim * unit;
    } else if (direction == "down") {
      aim += unit;
    } else {
      aim -= unit;
    }
  }

  std::cout << "depth: " << total_depth << " forward: " << total_forward << "\n";
  std::cout << total_depth * total_forward;
}
