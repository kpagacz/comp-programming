#include<iostream>
#include<string>

int main() {
  std::string direction;
  int unit;

  int total_depth = 0, total_forward = 0;

  while(std::cin >> direction) {
    std::cin >> unit;

    if (direction == "forward") {
      total_forward += unit;
    } else if (direction == "down") {
      total_depth += unit;
    } else {
      total_depth -= unit;
    }
  }

  std::cout << total_depth * total_forward;
}
