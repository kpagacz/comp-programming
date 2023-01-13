#include <algorithm>
#include <cassert>
#include <fstream>
#include <iostream>
#include <numeric>
#include <string>
#include <tuple>
#include <vector>

using Num = int64_t;
class Solution {
 public:
  Num part1(const std::string& path) {
    auto expressions = parseInput(path);
    return std::transform_reduce(expressions.begin(), expressions.end(), (Num)0, std::plus<Num>(),
                                 [&](const auto& expression) {
                                   auto [value, _] = evaluate(0, expression);
                                   return value;
                                 });
  }
  Num part2(const std::string& path) {}

  void test() {
    auto path = "test";
    auto expressions = parseInput(path);
    for (int i = 0; i < expressions.size(); i++) {
      std::cout << expressions[i] << " = ";
      auto [value, it] = evaluate(0, expressions[i]);
      std::cout << value << '\n';
    }
  }

 private:
  std::pair<Num, std::size_t> evaluate(std::size_t it, const std::string& expression) {
    Num value, token1Value, token2Value;
    std::string op, token1, token2;
    std::size_t nextTokenStart, tokenEnd;

    // first token
    tokenEnd = expression.find_first_not_of("0123456789(", it);
    token1 = expression.substr(it, tokenEnd - it);
    std::cout << "token1:" << token1 << '\n';
    if (token1.starts_with("(")) std::tie(token1Value, it) = evaluate(it + 1, expression);
    else it = tokenEnd, token1Value = std::stoll(token1);
    if (it >= expression.size()) return {token1Value, expression.npos};
    else if (expression.at(it) == ')') return {token1Value, it + 1};

    // loop for an op and second token
    value = token1Value;
    std::cout << "token1Value:" << value << " it:" << it << '\n';
    while ((nextTokenStart = expression.find_first_not_of(' ', it)) != expression.npos) {
      // std::cout << "nextTokenStart:" << nextTokenStart << '\n';
      op = expression.at(nextTokenStart);
      assert(op == "+" || op == "*");
      it = nextTokenStart + 2;
      assert(expression.at(it) != ' ');
      tokenEnd = expression.find_first_of(") ", it);
      token2 = expression.substr(it, tokenEnd - it);
      std::cout << "token2:" << token2 << '\n';
      if (token2.starts_with("(")) std::tie(token2Value, it) = evaluate(it + 1, expression);
      else it = tokenEnd, token2Value = std::stoll(token2);

      if (op == "+") value += token2Value;
      else value *= token2Value;

      // stop if the tokenEnd is )
      if (it >= expression.size() || expression.at(it) == ')') {
        std::cout << "found ) returning:" << value << " " << tokenEnd + 1 << '\n';
        return {value, it + 1};
      }
    }

    return {value, it};
  }
  std::vector<std::string> parseInput(const std::string& path) {
    std::vector<std::string> expressions;
    std::fstream input(path, std::ios_base::in);
    std::string expression;
    while (std::getline(input, expression)) expressions.push_back(expression);
    return expressions;
  }
};
int main() {
  Solution s;
  std::cout << "Part 1: " << s.part1("input") << '\n';
  // s.test();
  return 0;
}
