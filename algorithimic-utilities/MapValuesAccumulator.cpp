struct MapValuesAccumulator {
  template<class Value, class Pair>
  Value operator()(Value value, const Pair& pair) const {
    return value + pair.second;
  }
};
