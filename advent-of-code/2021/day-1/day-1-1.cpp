#include<iostream>


int main() {
  int previous, current_number;
  int increases = 0;
  std::cin >> previous;
  while (std::cin >> current_number) {
    if (current_number > previous) ++increases;
    previous = current_number;
  }

  std::cout << increases << std::endl;
}
