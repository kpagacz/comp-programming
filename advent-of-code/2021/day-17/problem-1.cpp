#include<math.h>
#include<iostream>
#include<stdlib.h>

int main(int argc, char** argv) {
  int x1, x2, y1, y2;
  std::scanf("target area: x=%d..%d , y=%d..%d", &x1, &x2, &y1, &y2);
  int abs_y = std::abs(y1) - 1;
  std::cout << abs_y * (abs_y + 1) / 2;
}
