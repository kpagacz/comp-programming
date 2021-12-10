#include<iostream>
#include<string>
#include<unordered_map>
#include<stack>
#include<set>

const std::unordered_map<char, int> points{{')', 3}, {']', 57}, {'}', 1197}, {'>', 25137}};
const std::unordered_map<char, char> closers{{'(', ')'}, {'[', ']'}, {'{', '}'}, {'<', '>'}};
const std::set<char> openings{'(', '{', '[', '<'};

int main() {
  int total_error = 0;

  std::string line;
  while(std::cin >> line) {
    std::stack<char> line_stack;
    for(const auto& c : line) {
      if (line_stack.empty()) {
        line_stack.push(c);
        continue;
      }
      if (openings.find(c) != openings.end()) {
        line_stack.push(c);
        continue;
      } else {
        if (c != closers.at(line_stack.top())) {
          total_error += points.at(c);
          break;
        } else {
          line_stack.pop();
        }
      }
    }
  }

  std::cout << "Total error: " << total_error << '\n';
}
