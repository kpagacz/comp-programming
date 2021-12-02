#include<iostream>


int main() {
  int previous_sum = 0, current_sum = 0, number;
  int numbers[3];
  int increases = 0;
  for(int i = 0; i < 3 ; i++) {
    std::cin >> numbers[i];
    previous_sum += numbers[i];
  }
  current_sum = previous_sum;

  while (std::cin >> number) {
    current_sum = previous_sum - numbers[0] + number;
    if (current_sum > previous_sum) increases++;
    numbers[0] = numbers[1];
    numbers[1] = numbers[2];
    numbers[2] = number;
  }

  std::cout << increases << std::endl;
}
