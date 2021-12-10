#include<iostream>
#include<string>
#include<unordered_map>
#include<stack>
#include<set>
#include<vector>
#include<algorithm>

const std::unordered_map<char, int> points{{')', 3}, {']', 57}, {'}', 1197}, {'>', 25137}};
const std::unordered_map<char, char> closers{{'(', ')'}, {'[', ']'}, {'{', '}'}, {'<', '>'}};
const std::set<char> openings{'(', '{', '[', '<'};
const std::unordered_map<char, int> scores{{')', 1}, {']', 2}, {'}', 3}, {'>', 4}};

int main() {
  std::vector < std::string> completions;
  std::string line;
  while(std::cin >> line) {
    std::stack<char> line_stack;
    bool corrupted = false;
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
          corrupted = true;
          break;
        } else {
          line_stack.pop();
        }
      }
    }

    if (!line_stack.empty() && !corrupted) {
      std::string completion = "";
      while(!line_stack.empty()) {
        completion += closers.at(line_stack.top());
        line_stack.pop();
      }
      completions.push_back(completion);
    }
  }

  std::vector<uint64_t> lines_scores;
  for(const auto& completion : completions) {
    uint64_t score = 0;
    for(auto c : completion) {
      score *= 5;
      score += scores.at(c);
    }
    lines_scores.push_back(score);
  }

  std::sort(lines_scores.begin(), lines_scores.end());
  std::cout << "Middle score: " << lines_scores[lines_scores.size() / 2] << '\n';
}
