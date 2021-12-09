  #include<set>
  #include<array>
  #include<iostream>

using point = std::array<int, 2>;

struct Basin {
  std::set<point*, PointComparator> locations;
  Basin() = default;
  int size() { return locations.size(); }
};

int main() {
  Basin *basin = new Basin();
  basin->locations.insert(new point({1, 2}));
  // point *pp = new point({1, 2});

  for(auto& p : basin->locations) {
    std::cout << p->at(0) << " " << p->at(1) << '\n';
  }
  auto p = basin->locations.find(new point({1, 2}));

  std::cout << (*p)->at(0) << '\n';
}
