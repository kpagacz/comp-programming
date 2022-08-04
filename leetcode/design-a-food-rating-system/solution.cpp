// link to the problem: https://leetcode.com/problems/design-a-food-rating-system/
#include <algorithm>
#include <array>
#include <cassert>
#include <functional>
#include <iostream>
#include <iterator>
#include <numeric>
#include <queue>
#include <set>
#include <sstream>
#include <stack>
#include <stdexcept>
#include <string>
#include <unordered_map>
#include <unordered_set>
#include <vector>

struct FoodComparator {
  bool operator()(const std::pair<int, std::string>& lhs, const std::pair<int, std::string>& rhs) const {
    if (lhs.first == rhs.first) return std::less<std::string>()(lhs.second, rhs.second);
    return std::greater<int>()(lhs.first, rhs.first);
  }
};

class FoodRatings {
 public:
  std::vector<std::string> foods, cuisines;
  std::vector<int> ratings;
  std::unordered_map<std::string, int> foodToIndex;
  std::unordered_map<std::string, std::set<std::pair<int, std::string>, FoodComparator>> orderedCuisines;

  FoodRatings(std::vector<std::string>& foods, std::vector<std::string>& cuisines, std::vector<int>& ratings) {
    this->foods = foods;
    this->ratings = ratings;
    this->cuisines = cuisines;
    for (int i = 0; i < this->foods.size(); ++i) {
      foodToIndex[this->foods[i]] = i;
      orderedCuisines[cuisines[i]].insert({this->ratings[i], foods[i]});
    }
  }

  void changeRating(std::string food, int newRating) {
    const auto& index = foodToIndex[food];
    auto& orderedCuisine = orderedCuisines[cuisines[index]];
    auto node = orderedCuisine.extract({ratings[index], food});
    ratings[index] = newRating;
    node.value() = {ratings[index], food};
    orderedCuisine.insert(std::move(node));
  }

  std::string highestRated(std::string cuisine) {
    auto& orderedCuisine = orderedCuisines[cuisine];
    return orderedCuisine.begin()->second;
  }
};

/**
 * Your FoodRatings object will be instantiated and called as such:
 * FoodRatings* obj = new FoodRatings(foods, cuisines, ratings);
 * obj->changeRating(food,newRating);
 * string param_2 = obj->highestRated(cuisine);
 */

int main(int argc, char** argv) {
  std::vector<std::string> foods = {"kimchi", "miso", "sushi", "moussaka", "ramen", "bulgogi"};
  std::vector<std::string> cuisines{"korean", "japanese", "japanese", "greek", "japanese", "korean"};
  std::vector<int> ratings{9, 12, 8, 15, 14, 7};
  FoodRatings* foodRatings = new FoodRatings(foods, cuisines, ratings);
  std::cout << foodRatings->highestRated("korean") << '\n';
  std::cout << foodRatings->highestRated("japanese") << '\n';
  foodRatings->changeRating("sushi", 16);
  std::cout << foodRatings->highestRated("japanese") << '\n';
  foodRatings->changeRating("ramen", 16);
  std::cout << foodRatings->highestRated("japanese") << '\n';
}
