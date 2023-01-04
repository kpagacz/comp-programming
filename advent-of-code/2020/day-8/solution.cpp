#include <cassert>
#include <fstream>
#include <iostream>
#include <stack>
#include <string>
#include <vector>

using NumType = int;
enum Instruction { NOP, JMP, ACC };

class Solution {
 public:
  NumType part1(const std::string& path) {
    auto instructions = parseInput(path);

    NumType accumulator = 0, it = 0;
    std::vector<bool> visited(instructions.size(), false);

    while (visited[it] == false) {
      const auto& [instruction, value] = instructions[it];
      visited[it] = true;
      if (instruction == NOP) it++;
      else if (instruction == JMP) it += value;
      else it++, accumulator += value;
    }
    return accumulator;
  }

  NumType part2(const std::string& path) {
    auto instructions = parseInput(path);

    NumType accumulator = 0;

    std::stack<NumType> stack;
    std::vector<bool> visited(instructions.size(), false);
    stack.push(0);
    NumType lastModified = -1;

    auto traceBackUntilModified = [&](auto& instructionsStack) {  // goes back to the previously modified instruction
      stack.pop();
      while (stack.top() != lastModified) {
        const auto& [instruction, value] = instructions[stack.top()];
        if (instruction == ACC) accumulator -= value;
        visited[stack.top()] = false;
        stack.pop();
        assert(!stack.empty());
      }
    };

    auto traceBackUntilNextNopOrJmp = [&](auto& instructionsStack) {
      stack.pop();
      auto topInstruction = instructions[stack.top()];
      while (topInstruction.first == ACC) {
        accumulator -= topInstruction.second;
        visited[stack.top()] = false;
        stack.pop();
        topInstruction = instructions[stack.top()];
        assert(!stack.empty());
      }
    };

    while (!stack.empty()) {
      auto top = stack.top();
      if (top == instructions.size()) return accumulator;
      assert(top >= 0 && top < instructions.size());

      // no loop
      if (visited[top] == false) {
        const auto& [instruction, value] = instructions[top];
        visited[top] = true;
        if (instruction == NOP) stack.push(top + 1);
        else if (instruction == JMP) stack.push(top + value);
        else stack.push(top + 1), accumulator += value;

        continue;
      }

      if (lastModified == -1) {
        traceBackUntilNextNopOrJmp(stack);
      } else {
        traceBackUntilModified(stack);
        visited[stack.top()] = false;
        traceBackUntilNextNopOrJmp(stack);
      }

      lastModified = stack.top();
      const auto& [instruction, value] = instructions[stack.top()];
      visited[lastModified] = true;
      if (instruction == NOP) stack.push(stack.top() + value);
      else stack.push(stack.top() + 1);
    }

    return accumulator;
  }

 private:
  std::vector<std::pair<Instruction, NumType>> parseInput(const std::string& path) {
    std::fstream input(path, std::ios_base::in);
    std::vector<std::pair<Instruction, NumType>> instructions;

    std::string instruction, value;
    while (input >> instruction) {
      input >> value;
      if (instruction == "nop") instructions.push_back({NOP, 0});
      else if (instruction == "jmp") instructions.push_back({JMP, std::stoi(value)});
      else instructions.push_back({ACC, std::stoi(value)});
    }

    return instructions;
  }
};

int main() {
  Solution s;
  std::cout << "Part 1: " << s.part1("input") << '\n';
  std::cout << "Part 2: " << s.part2("input") << '\n';
  return 0;
}
