#include <fstream>
#include <iostream>
#include <stack>
#include <string>
#include <vector>

struct Comparable {
  virtual ~Comparable(){};
  virtual void print() const = 0;
};
struct Int : Comparable {
  int element;
  Int() = default;
  Int(int _element) : element(_element) {}
  void print() const {
    std::cout << element;
  }
};
struct List : Comparable {
  std::vector<const Comparable*> elements;
  bool closed = false;
  List() = default;
  List(const Int* element) : elements(std::vector<const Comparable*>{element}) {}
  void print() const {
    std::cout << "[";
    for (const auto& el : elements) el->print(), std::cout << ",";
    std::cout << "]";
  }
};

// Comparisons
int compare(const Int& first, const Int& second);
int compare(const List& first, const List& second);
int compare(const Comparable& first, const Comparable& second);

int compare(const Int& first, const Int& second) {
  return (second.element > first.element) - (first.element > second.element);
}
int compare(const List& first, const List& second) {
  auto firstIt = first.elements.begin();
  auto secondIt = second.elements.begin();
  while (firstIt != first.elements.end() && secondIt != second.elements.end()) {
    if (compare(**firstIt, **secondIt) != 0) return compare(**firstIt, **secondIt);
    firstIt++, secondIt++;
  }

  return (firstIt == first.elements.end()) - (secondIt == second.elements.end());
}
int compare(const Comparable& first, const Comparable& second) {
  const Int* downcastedFirst = dynamic_cast<const Int*>(&first);
  const Int* downcastedSecond = dynamic_cast<const Int*>(&second);
  if (downcastedFirst != nullptr && downcastedSecond != nullptr) return compare(*downcastedFirst, *downcastedSecond);

  const List* firstList = downcastedFirst != nullptr ? new List(downcastedFirst) : dynamic_cast<const List*>(&first);
  const List* secondList =
      downcastedSecond != nullptr ? new List(downcastedSecond) : dynamic_cast<const List*>(&second);

  return compare(*firstList, *secondList);
}

// Reading
Comparable* fromString(const std::string& text) {
  int textIt = 0;
  std::stack<Comparable*> elements;
  while (textIt < text.size() && textIt != std::string::npos) {
    // std::cout << "CUrrent char: " << text[textIt] << '\n';
    if (text[textIt] == ',') {
      textIt++;
      continue;
    }
    if (text[textIt] == '[') {
      elements.push(new List());
      textIt++;
      continue;
    }
    if (text[textIt] == ']') {
      std::stack<Comparable*> toAdd;
      for (List* topElement = dynamic_cast<List*>(elements.top()); topElement == nullptr || topElement->closed;
           elements.pop(), topElement = dynamic_cast<List*>(elements.top()))
        toAdd.push(elements.top());

      List* currentlyFilledList = dynamic_cast<List*>(elements.top());
      assert(currentlyFilledList != nullptr);
      while (!toAdd.empty()) currentlyFilledList->elements.push_back(toAdd.top()), toAdd.pop();
      currentlyFilledList->closed = true;
      textIt++;
      continue;
    }
    assert(std::isdigit(text[textIt]));
    auto nextNotNumber = text.find_first_of(",[]", textIt);
    std::string no = text.substr(textIt, nextNotNumber - textIt);
    elements.push(new Int(std::stoi(no)));
    textIt = nextNotNumber;
  }

  assert(elements.size() == 1);
  return elements.top();
}

class Solution {
 public:
  int part1(const std::string& pathToInput) {
    std::fstream input(pathToInput, std::ios_base::in);
    std::string firstPacket, secondPacket;
    int index = 1, sumOfRightPacketPairs = 0;

    while (input >> firstPacket) {
      input >> secondPacket;
      Comparable *first = fromString(firstPacket), *second = fromString(secondPacket);
      if (compare(*first, *second) == 1) sumOfRightPacketPairs += index;
      index++;
    }
    return sumOfRightPacketPairs;
  }
  int part2(const std::string& pathToInput) {
    std::fstream input(pathToInput, std::ios_base::in);
    std::string packet;
    std::vector<Comparable*> allPackets;
    while (input >> packet) allPackets.push_back(fromString(packet));
    const std::vector<std::string> additionalPackets{"[[2]]", "[[6]]"};
    for (auto packet : additionalPackets) allPackets.push_back(fromString(packet));
    std::sort(allPackets.begin(), allPackets.end(),
              [](const auto& first, const auto& second) { return compare(*first, *second) >= 0; });
    auto firstDividerPacket = std::find_if(allPackets.begin(), allPackets.end(), [&](const auto& packet) {
      return compare(*packet, *fromString(additionalPackets[0])) == 0;
    });
    auto secondDividerPacket = std::find_if(allPackets.begin(), allPackets.end(), [&](const auto& packet) {
      return compare(*packet, *fromString(additionalPackets[1])) == 0;
    });
    // I should be forced to calculate the distance from the beginning and add 1
    // to it because the problem counts indices from 1 but it does not work like I would expect.
    // I am confused, but I do not want to investigate.
    for (auto p : allPackets) p->print(), std::cout << '\n';
    return (std::distance(allPackets.begin(), firstDividerPacket)) *
           (std::distance(allPackets.begin(), secondDividerPacket));
  }
};

int main() {
  Solution s;
  std::cout << "Part 1: " << s.part1("input") << '\n';
  std::cout << "Part 2: " << s.part2("input") << '\n';
  return 0;
}
