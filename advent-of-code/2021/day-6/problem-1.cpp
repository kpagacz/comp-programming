#include<iostream>
#include<string>
#include<vector>
#include<sstream>
#include<algorithm>

struct Fish {
  int initial_days;
  int current_days;
  Fish(int _init_days) {
    this->initial_days = _init_days;
    current_days = _init_days;
  }
};

int main() {
  int limit = 80;
  std::string line;
  std::getline(std::cin, line);
  std::stringstream line_stream = std::stringstream(line);

  std::vector<Fish*> fish;
  std::string days;
  while(std::getline(line_stream, days, ',')) {
    Fish* new_fish = new Fish(6);
    new_fish->current_days = std::stoi(days, 0, 10);
    fish.push_back(new_fish);
  }

  for (int i = 0; i < limit; i++) {
    std::vector<Fish*> new_fish;
    std::for_each(fish.begin(), fish.end(), [&](auto &f) {
      if (f->current_days == 0) {
        Fish *spawn = new Fish(6);
        spawn->current_days += 2;
        new_fish.push_back(spawn);
        f->current_days = f->initial_days;
      } else {
        f->current_days--;
      }
    });
    fish.insert(fish.end(), new_fish.begin(), new_fish.end());
  }

  std::cout << "number of fish: " << fish.size();
}
