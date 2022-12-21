#include <functional>
#include <type_traits>

// Requires c++20

namespace utils {
template <typename T>
struct remove_const_and_reference;
template <typename T>
struct remove_const_and_reference<const T&> {
  typedef std::remove_cv_t<std::remove_reference_t<T>> type;
};
template <typename T>
using remove_const_and_reference_t = typename remove_const_and_reference<T>::type;

template <typename... Ts>
struct TupleHash {
  std::size_t operator()(const std::tuple<Ts...>& tuple) const {
    return std::apply(Combine(), tuple);
  }

 private:
  struct Combine {
    std::size_t operator()(const Ts&... args) const {
      const int multiplier = 0b1110110;
      std::size_t hash = 0x123456;
      (
          [&]<typename T>(T&& arg) {
            hash = multiplier * (hash ^ std::hash<remove_const_and_reference_t<decltype(args)>>()(args));
          }(args),
          ...);
      return hash;
    }  // (h(2)^((h(1) ^ init) * m)) * m
  };
};
}  // namespace utils

// int main() {
// TupleHash<int, int> pairHash;
// std::cout << pairHash(std::pair<int, int>{0, 1e10}) << '\n';
// std::cout << pairHash(std::pair<int, int>{1, 0}) << '\n';
// std::cout << pairHash(std::pair<int, int>{0, 0}) << '\n';

// TupleHash<double, int, int64_t> tupleHash;
// std::cout << tupleHash(std::make_tuple(1.1, 0, -7)) << '\n';
// return 0;
// }
