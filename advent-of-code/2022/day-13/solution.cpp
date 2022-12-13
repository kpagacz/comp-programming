#include <fstream>
#include <iostream>
#include <stack>
#include <string>
#include <variant>
#include <vector>

struct Packet {
  std::vector<std::variant<int, Packet>> packet;
  Packet() = default;
  Packet(std::vector<std::variant<int, Packet>> _packet) : packet(_packet) {}
  static Packet fromString(const std::string& input) {
    std::stack<Packet*> openPackets;
    std::stack<std::variant<int, Packet>> elements;
    auto it{0};
    while (it < input.size()) {
      if (input[it] == ',') {
        it++;
        continue;
      }
      if (input[it] == '[') {
        elements.push(Packet());
        openPackets.push(&std::get<Packet>(elements.top()));
        it++;
        continue;
      }
      if (input[it] == ']') {
        while (std::holds_alternative<int>(elements.top()) || &std::get<Packet>(elements.top()) != openPackets.top())
          openPackets.top()->packet.push_back(elements.top()), elements.pop();
        std::reverse(openPackets.top()->packet.begin(), openPackets.top()->packet.end());
        openPackets.pop();
        it++;
        continue;
      }
      assert(std::isdigit(input[it]));
      auto nextNotNumber = input.find_first_of(",[]", it);
      std::string no = input.substr(it, nextNotNumber - it);
      elements.push(std::stoi(no));
      it = nextNotNumber;
    }
    assert(elements.size() == 1);
    assert(std::holds_alternative<Packet>(elements.top()));
    return std::get<Packet>(elements.top());
  }
  void print() const {
    std::cout << "[";
    for (const auto& el : packet)
      if (std::holds_alternative<Packet>(el)) std::get<Packet>(el).print();
      else std::cout << std::get<int>(el) << ",";
    std::cout << "]";
  }
};

int compare(const std::variant<int, Packet>& first, const std::variant<int, Packet>& second);
int compare(const Packet& first, const Packet& second) {
  auto firstIt = first.packet.begin(), secondIt = second.packet.begin();
  while (firstIt != first.packet.end() && secondIt != second.packet.end()) {
    auto comparison = compare(*firstIt, *secondIt);
    if (comparison != 0) return comparison;
    firstIt++, secondIt++;
  }
  return (firstIt == first.packet.end()) - (secondIt == second.packet.end());
}
int compare(const std::variant<int, Packet>& first, const std::variant<int, Packet>& second) {
  if (std::holds_alternative<int>(first) && std::holds_alternative<int>(second)) {
    return (std::get<int>(second) > std::get<int>(first)) - (std::get<int>(first) > std::get<int>(second));
  }
  auto firstPacket = std::holds_alternative<int>(first) ? Packet(std::vector<std::variant<int, Packet>>(1, first))
                                                        : std::get<Packet>(first);
  auto secondPacket = std::holds_alternative<int>(second) ? Packet(std::vector<std::variant<int, Packet>>(1, second))
                                                          : std::get<Packet>(second);
  return compare(firstPacket, secondPacket);
}

class Solution {
 public:
  int part1(const std::string& pathToInput) {
    std::fstream input(pathToInput, std::ios_base::in);
    std::string firstPacket, secondPacket;
    int index = 1;
    int sum = 0;
    while (input >> firstPacket) {
      input >> secondPacket;
      Packet first = Packet::fromString(firstPacket), second = Packet::fromString(secondPacket);
      if (compare(first, second) == 1) sum += index;
      index++;
    }
    return sum;
  }

  int part2(const std::string& pathToInput) {
    std::fstream input(pathToInput, std::ios_base::in);
    auto firstDivider = Packet::fromString("[[2]]"), secondDivider = Packet::fromString("[[6]]");
    std::string newpacket;
    auto firstIndex{0}, secondIndex{0};
    while (input >> newpacket) {
      Packet newPacket = Packet::fromString(newpacket);
      if (compare(newPacket, firstDivider) == 1) firstIndex++;
      if (compare(newPacket, secondDivider) == 1) secondIndex++;
    }

    firstIndex++;
    secondIndex += 2;
    return firstIndex * secondIndex;
  }
};

int main() {
  Solution s;
  Packet first = Packet::fromString("[[1],[2,3,4]]"), second = Packet::fromString("[[1],4]");
  std::cout << "Part 1: " << s.part1("input") << '\n';
  std::cout << "Part 2: " << s.part2("input") << '\n';

  std::cout << "\n\nOther input:\n";
  std::cout << "Part 1: " << s.part1("others-solution-and-input/input") << '\n';
  std::cout << "Part 2: " << s.part2("others-solution-and-input/input") << '\n';
  return 0;
}
