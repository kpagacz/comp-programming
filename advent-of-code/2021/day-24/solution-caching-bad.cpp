#include <ctype.h>

#include <algorithm>
#include <array>
#include <cassert>
#include <iostream>
#include <numeric>
#include <sstream>
#include <string>
#include <tuple>
#include <unordered_map>
#include <vector>

using key = std::array<int, 5>;  // operation index, w value, x, y, z value
using registers = std::array<int, 4>;
using cache = std::unordered_map<key, std::pair<bool, std::string>>;
template <>
struct std::hash<key> {
  std::size_t operator()(key const& k) const {
    return std::hash<int>()(std::get<0>(k)) ^ std::hash<int>()(std::get<1>(k)) ^
           std::hash<int>()(std::get<2>(k)) ^ std::hash<int>()(k[3]) ^
           std::hash<int>()(k[4]);
  }
};

// Globals
cache mem;

int evaluate(std::optional<std::string> symbol, const registers& regs) {
  if (std::isalpha(symbol.value()[0])) {
    // std::cout << "Output symbolic " << symbol.value()[0] << " index: " <<
    // symbol.value()[0] - 'w' << "\n";
    return regs[symbol.value()[0] - 'w'];
  } else {
    return std::stoi(symbol.value(), 0, 10);
  }
}

struct op {
  std::string op;
  std::string v1;
  std::optional<std::string> v2;
  static void print(struct op const& o) {
    std::cout << "Operation: " << o.op << " register 1: " << o.v1;
    if (o.v2.has_value()) std::cout << " register 2: " << o.v2.value();
    std::cout << '\n';
  }

  bool execute(registers& regs) {
    if (op == "inp") {
      regs[0] = std::stoi(v2.value(), 0, 10);
    }
    if (op == "add") {
      regs[v1[0] - 'w'] += evaluate(v2, regs);
    }
    if (op == "mul") {
      regs[v1[0] - 'w'] *= evaluate(v2, regs);
    }
    if (op == "div") {
      if (evaluate(v2, regs) == 0) return false;
      regs[v1[0] - 'w'] /= evaluate(v2, regs);
    }
    if (op == "mod") {
      if (evaluate(v2.value(), regs) <= 0 || regs[v1[0] - 'w'] < 0)
        return false;
      regs[v1[0] - 'w'] = regs[v1[0] - 'w'] % evaluate(v2, regs);
    }
    if (op == "eql") {
      regs[v1[0] - 'w'] = regs[v1[0] - 'w'] == evaluate(v2, regs);
    }

    return true;
  }
};

void print(std::__1::pair<const key, std::__1::pair<bool, registers>> p) {
  std::cout << "key: ";
  for (const auto& i : p.first) std::cout << i << " ";
  std::cout << " registers: ";
  for (const auto& i : p.second.second) std::cout << i << " ";
  std::cout << '\n';
}

std::pair<bool, std::string> dp(int op_index, registers regs,
                                std::vector<op>& ops) {
  key k{op_index, regs[0], regs[1], regs[2], regs[3]};
  if (mem.contains(k)) return mem[k];

  if (op_index >= ops.size()) {
    std::cout << regs[0] << " " << regs[1] << " " << regs[2] << " " << regs[3]
              << '\n';
    mem.insert_or_assign(k, std::make_pair(regs[3] == 0, ""));
    return {regs[3] == 0, ""};
  }
  if (regs[3] > 1e9) {
    mem.insert_or_assign(k, std::make_pair(false, ""));
    return {false, ""};
  }
  // std::cout << op_index << " "
  //           << " " << ops[op_index].op << " " << regs[0] << " " << regs[1]
  //           << " " << regs[2] << " " << regs[3] << '\n';

  auto o = ops[op_index];

  if (o.op == "inp") {
    // for (int i = 9; i > 0; i--) {
    for (int i = 1; i <= 9; i++) {
      regs[0] = i;
      auto answer = dp(op_index + 1, regs, ops);
      if (answer.first) {
        std::cout << "digit: " << answer.second << " " << regs[0] << " "
                  << regs[1] << " " << regs[2] << " " << regs[3] << '\n';
        mem.insert_or_assign(
            k, std::make_pair(true, std::to_string(i) + answer.second));
        return {true, std::to_string(i) + answer.second};
      }
    }

    mem.insert_or_assign(k, std::make_pair(false, ""));
    return {false, ""};
  }

  auto success = o.execute(regs);
  auto res = dp(op_index + 1, regs, ops);
  mem.insert_or_assign(k, std::make_pair(res.first, res.second));
  return res;
}

int main(int argc, char** argv) {
  registers regs{0, 0, 0, 0};

  std::string line;
  std::vector<op> ops;
  while (std::getline(std::cin, line)) {
    op o;
    std::stringstream in(line);
    in >> o.op >> o.v1;
    if (o.op != "inp") in >> o.v2.emplace();
    ops.push_back(o);
  }

  std::cout << dp(0, regs, ops).second << '\n';
  // std::cout << "Cache size: " << mem.size() << '\n';
  // for(const auto& p : mem) {
  //   std::cout << std::boolalpha << p.first[0] << p.first[1] <<p.first[2]
  //   <<p.first[3] <<p.first[4] << " " << p.second.first << " " <<
  //   p.second.second << '\n';
  // }
}
