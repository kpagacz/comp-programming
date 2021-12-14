#include<iostream>
#include<algorithm>
#include<unordered_map>
#include<vector>
#include<string>

using rules = std::unordered_map<std::string, std::string>;
using polymer = std::unordered_map<std::string, int>;

int main() {
  std::string start;
  std::cin >> start;

  polymer p;

  for(int i = 1; i < start.size(); i++) {
    p.insert({start.substr(i - 1, i), 1});
  }

  for(const auto& pair : p)
    std::cout << pair.first << " " << pair.second << "\n";
}
