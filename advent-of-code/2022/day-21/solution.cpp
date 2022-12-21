#include <algorithm>
#include <fstream>
#include <iostream>
#include <limits>
#include <numeric>
#include <optional>
#include <sstream>
#include <string>
#include <unordered_map>
#include <unordered_set>

struct Monkey;
enum Operation { ADD, SUBTRACT, MULTIPLY, DIVIDE };
using MonkeyRiddle = std::unordered_map<std::string, Monkey>;
using NumType = int64_t;

struct Calculate {
  static NumType calculate(const NumType& left, const NumType& right, const Operation& op) {
    switch (op) {
      case ADD:
        return left + right;
      case SUBTRACT:
        if (left < right) throw std::invalid_argument("Would result in negative");
        return left - right;
      case MULTIPLY:
        if (right != 0 && std::numeric_limits<NumType>::max() / right < left)
          throw std::invalid_argument("Multiplying would overflow");
        return left * right;
      case DIVIDE:
        if (right == 0) throw std::invalid_argument("Cannot divide by zero");
        if (left % right != 0) throw std::invalid_argument("Dividing with a fraction...");
        return left / right;
    }
  }
};

struct Monkey {
  std::string name;
  std::optional<NumType> number;
  std::string leftOperand, rightOperand;
  Operation op;

  NumType getNumber(MonkeyRiddle& riddle) {
    if (number.has_value()) return number.value();
    // auto left = riddle[leftOperand].getNumber(riddle);
    // auto right = riddle[rightOperand].getNumber(riddle);
    // auto answer = Calculate::calculate(left, right, op);
    // std::cout << "Name:" << name << " leftOp:" << leftOperand << " " << left << " rightOp:" << rightOperand << " "
    //           << right << " op:" << op << " res:" << answer << '\n';
    // return answer;
    number = Calculate::calculate(riddle[leftOperand].getNumber(riddle), riddle[rightOperand].getNumber(riddle), op);
    return number.value();
  }

  bool needsHuman(MonkeyRiddle& riddle) const {
    if (name == "humn") return true;
    if (number.has_value()) return false;
    return riddle.at(leftOperand).needsHuman(riddle) || riddle.at(rightOperand).needsHuman(riddle);
  }

  void print(std::ostream& out, MonkeyRiddle& riddle) {
    if (name == "humn") {
      out << "x";
      return;
    }
    if (number.has_value()) {
      out << number.value();
      return;
    }
    if (needsHuman(riddle) == false) {
      out << getNumber(riddle);
      return;
    }
    out << "(";
    riddle.at(leftOperand).print(out, riddle);
    switch (op) {
      case ADD:
        out << "+";
        break;
      case SUBTRACT:
        out << "-";
        break;
      case MULTIPLY:
        out << "*";
        break;
      case DIVIDE:
        out << "/";
        break;
    }
    riddle.at(rightOperand).print(out, riddle);
    out << ")";
  }

  Monkey() {
    number = std::nullopt;
  }

  Monkey(NumType _number, std::string _name) : name(_name) {
    number = std::optional<NumType>{_number};
  }

  Monkey(const std::string& _leftOperand, const std::string& _rightOperand, const Operation& _op, std::string _name)
      : leftOperand(_leftOperand), rightOperand(_rightOperand), op(_op), name(_name) {
    number = std::nullopt;
  }
};

class Solution {
 public:
  NumType part1(const std::string& path) {
    auto riddle = parseInput(path);
    return riddle.at("root").getNumber(riddle);
  }

  void part2(const std::string& path) {
    auto riddle = parseInput(path);
    auto rootLeftOperand = riddle["root"].leftOperand, rootRightOperand = riddle["root"].rightOperand;
    assert(riddle.at(rootLeftOperand).needsHuman(riddle) ^ riddle.at(rootRightOperand).needsHuman(riddle));

    auto needsHuman = riddle.at(rootLeftOperand).needsHuman(riddle) ? rootLeftOperand : rootRightOperand;
    auto doesNotNeedHuman = riddle.at(rootLeftOperand).needsHuman(riddle) ? rootRightOperand : rootLeftOperand;
    std::cout << "needs human:" << needsHuman << " doesnt:" << doesNotNeedHuman << '\n';
    std::cout << "Must be:" << riddle.at(doesNotNeedHuman).getNumber(riddle) << '\n';
    riddle.at(needsHuman).print(std::cout, riddle);
    std::cout << '\n';
  }

 private:
  MonkeyRiddle parseInput(const std::string_view& path) {
    std::fstream input(path, std::ios_base::in);
    std::string line;
    MonkeyRiddle monkeys;
    while (std::getline(input, line)) {
      if (line.find_first_of("0123456789") != line.npos) {
        std::string monkeyName;
        NumType monkeyNumber;
        std::stringstream ss(line);
        ss >> monkeyName >> monkeyNumber;
        monkeyName.pop_back();
        // std::cout << "Monkey name:" << monkeyName << " monkeyNumber:" << monkeyNumber << '\n';
        Monkey monkey(monkeyNumber, monkeyName);
        monkeys.insert({monkeyName, monkey});
      } else {
        std::string monkeyName, leftOperand, rightOperand, op;
        std::stringstream ss(line);
        ss >> monkeyName >> leftOperand >> op >> rightOperand;
        monkeyName.pop_back();
        Operation operation;
        switch (op[0]) {
          case '+':
            operation = ADD;
            break;
          case '-':
            operation = SUBTRACT;
            break;
          case '*':
            operation = MULTIPLY;
            break;
          case '/':
            operation = DIVIDE;
            break;
        }
        Monkey monkey(leftOperand, rightOperand, operation, monkeyName);
        // std::cout << "Monkey name:" << monkeyName << " leftOp:" << monkey.leftOperand
        //           << " rightOp:" << monkey.rightOperand << " op:" << op << '\n';
        monkeys.insert({monkeyName, monkey});
      }
    }

    return monkeys;
  }
};

int main() {
  Solution s;
  std::cout << "Part 1: " << s.part1("input") << '\n';
  std::cout << "Part 2: " << '\n';
  s.part2("input");
  return 0;
}
