// link to the problem: https://leetcode.com/problems/implement-stack-using-queues/
#include <algorithm>
#include <array>
#include <cassert>
#include <iostream>
#include <iterator>
#include <numeric>
#include <queue>
#include <set>
#include <sstream>
#include <stack>
#include <stdexcept>
#include <string>
#include <unordered_map>
#include <unordered_set>
#include <vector>

class MyStack {
 public:
  MyStack() {}

  void push(int x) { main.push(x); }

  int pop() {
    const int top = this->top();
    while (main.size() > 1) {
      secondary.push(main.front());
      main.pop();
    }
    main = secondary;
    secondary = {};
    return top;
  }

  int top() { return main.back(); }

  bool empty() { return main.empty(); }

 private:
  std::queue<int> main, secondary;
};

int main(int argc, char** argv) {}
