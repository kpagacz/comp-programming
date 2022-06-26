#include <iostream>
#include <iterator>
#include <map>
#include <string>
#include <unordered_map>
#include <vector>

std::map<char, std::pair<std::size_t, std::size_t>> bucketBorders(const std::string& sequence);
std::vector<bool> tagSuffixes(const std::string& sequence);
std::pair<std::vector<std::size_t>, std::vector<bool>> tagLmsSuffixes(const std::vector<bool> suffixesTypes);
std::unordered_map<std::size_t, std::string_view> lmsBlocks(const std::string& word,
                                                            const std::vector<std::size_t>& lmsSuffixes);
std::vector<std::size_t> lmsBlocksSortedByFirstChar(const std::vector<bool> lmsSuffixFlags,
                                                    const std::vector<int64_t>& suffixArray);
std::pair<std::unordered_map<std::size_t, std::size_t>, int> encodeLmsBlocks(
    const std::vector<std::size_t> lmsBlocksOrder, const std::unordered_map<std::size_t, std::string_view> lmsBlocks);

std::string stringFromEncodedBlocks(std::vector<std::size_t> lmsSuffixes,
                                    std::unordered_map<std::size_t, std::size_t> encodedBlocks);

std::vector<std::size_t> sortLmsBlocksLexicographically(
    const std::unordered_map<std::size_t, std::size_t>& encodedBlocks);

void resetFreeSlots(std::map<char, std::pair<std::size_t, std::size_t>>& freeSlots,
                    const std::map<char, std::pair<std::size_t, std::size_t>>& borders);

void positionLmsSuffixes(std::map<char, std::pair<std::size_t, std::size_t>>& freeSlots,
                         std::vector<int64_t>& suffixArray, const std::vector<std::size_t>& lmsSuffixes,
                         const std::string& word);

void positionLSuffixes(std::map<char, std::pair<std::size_t, std::size_t>>& freeSlots,
                       std::vector<int64_t>& suffixArray, const std::string& word, const std::vector<bool>& types);

void positionSSuffixes(std::map<char, std::pair<std::size_t, std::size_t>>& freeSlots,
                       std::vector<int64_t>& suffixArray, const std::string& word, const std::vector<bool>& types);

std::vector<std::size_t> decodeSuffixArrayPrim(const std::vector<int64_t>& suffixArrayPrim,
                                               const std::vector<std::size_t>& lmsSuffixes);

// Suffix Array construction using Induction Sort
// Constructs the suffix array of a given word in O(n) time.
std::vector<int64_t> sais(const std::string& sequence) {
  const std::string debugEnvVar = std::getenv("DEBUG") ? std::getenv("DEBUG") : "";
  const bool debug = debugEnvVar == "1";

  const std::string sequence_with_sentinel = sequence + "$";
  std::vector<bool> types = tagSuffixes(sequence_with_sentinel);
  // print types
  if (debug) {
    std::cout << "Types: ";
    std::copy(types.begin(), types.end(), std::ostream_iterator<bool>(std::cout, " "));
    std::cout << '\n';
  }
  const auto [lmsSuffixes, lmsFlags] = tagLmsSuffixes(types);
  std::vector<int64_t> suffixArray(sequence_with_sentinel.size(), -1);
  std::map<char, std::pair<std::size_t, std::size_t>> borders = bucketBorders(sequence_with_sentinel);
  std::map<char, std::pair<std::size_t, std::size_t>> freeSlots(borders);

  // place the LMS suffixes in the first free slots
  positionLmsSuffixes(freeSlots, suffixArray, lmsSuffixes, sequence_with_sentinel);
  if (debug) {
    std::cout << "Suffix array after placing LMS suffixes: \n";
    std::copy(suffixArray.begin(), suffixArray.end(), std::ostream_iterator<int64_t>(std::cout, " "));
    std::cout << '\n';
  }

  // forward pass for L-type suffixes
  positionLSuffixes(freeSlots, suffixArray, sequence_with_sentinel, types);
  if (debug) {
    std::cout << "Suffix array after the forward pass placing L suffixes:\n";
    std::copy(suffixArray.begin(), suffixArray.end(), std::ostream_iterator<int64_t>(std::cout, " "));
    std::cout << '\n';
  }

  // reverse pass for S-type suffixes
  resetFreeSlots(freeSlots, borders);
  positionSSuffixes(freeSlots, suffixArray, sequence_with_sentinel, types);
  if (debug) {
    std::cout << "Suffix array after the reverse pass placing S suffixes: \n";
    std::copy(suffixArray.begin(), suffixArray.end(), std::ostream_iterator<int>(std::cout, " "));
    std::cout << '\n';
  }

  const auto blocks = lmsBlocks(sequence_with_sentinel, lmsSuffixes);
  if (debug) {
    std::cout << "LMS blocks:\n";
    for (auto& [block, block_string] : blocks) {
      std::cout << block << ": " << block_string << '\n';
    }
  }

  if (debug) {
    std::cout << "Print lms flags\n";
    std::copy(lmsFlags.begin(), lmsFlags.end(), std::ostream_iterator<bool>(std::cout, " "));
    std::cout << '\n';
  }
  const auto blocksOrder = lmsBlocksSortedByFirstChar(lmsFlags, suffixArray);
  if (debug) {
    std::cout << "LMS blocks sorted by first char:\n";
    std::copy(blocksOrder.begin(), blocksOrder.end(), std::ostream_iterator<int>(std::cout, " "));
    std::cout << '\n';
  }

  const auto [blocksEncoding, uniqueEncodingsCount] = encodeLmsBlocks(blocksOrder, blocks);
  if (debug) {
    std::cout << "Block encodings:\n";
    std::cout << "Total unique: " << uniqueEncodingsCount << '\n';
    for (const auto& block : blocksOrder) std::cout << block << ": " << blocksEncoding.at(block) << '\n';
  }
  std::vector<std::size_t> sortedLmsSuffixes(lmsSuffixes.size());
  if (uniqueEncodingsCount == lmsSuffixes.size()) {
    // base case of the recursion
    if (debug) std::cout << "Base case of recursion\n";
    sortedLmsSuffixes = sortLmsBlocksLexicographically(blocksEncoding);
  } else {
    // non base case of the recursion
    const auto stringFromBlocks = stringFromEncodedBlocks(lmsSuffixes, blocksEncoding);
    if (debug) std::cout << "String from blocks:\n" << stringFromBlocks << '\n';
    const auto suffixArrayPrim = sais(stringFromBlocks);
    sortedLmsSuffixes = decodeSuffixArrayPrim(suffixArrayPrim, lmsSuffixes);
  }

  // print sortedLmsSuffixes
  if (debug) {
    std::cout << "Sorted LMS suffixes:\n";
    std::copy(sortedLmsSuffixes.begin(), sortedLmsSuffixes.end(), std::ostream_iterator<int>(std::cout, " "));
    std::cout << '\n';
  }

  std::fill(suffixArray.begin(), suffixArray.end(), -1);
  resetFreeSlots(freeSlots, borders);
  positionLmsSuffixes(freeSlots, suffixArray, sortedLmsSuffixes, sequence_with_sentinel);
  positionLSuffixes(freeSlots, suffixArray, sequence_with_sentinel, types);
  resetFreeSlots(freeSlots, borders);
  positionSSuffixes(freeSlots, suffixArray, sequence_with_sentinel, types);

  if (debug) {
    std::cout << "Returned suffixArray:\n";
    std::copy(suffixArray.begin(), suffixArray.end(), std::ostream_iterator<int>(std::cout, " "));
    std::cout << '\n';
  }
  return suffixArray;
}

std::map<char, std::pair<std::size_t, std::size_t>> bucketBorders(const std::string& sequence) {
  std::map<char, std::size_t> counts;
  for (const auto& c : sequence) {
    if (counts.contains(c))
      counts[c]++;
    else
      counts[c] = 1;
  }
  int symbolsCount = 0;
  std::map<char, std::pair<std::size_t, std::size_t>> bucketBorders;
  for (const auto& [c, count] : counts) {
    bucketBorders.insert({c, {symbolsCount, symbolsCount + count}});
    symbolsCount += count;
  }
  return bucketBorders;
}

std::vector<bool> tagSuffixes(const std::string& sequence) {
  // A suffix starting at position k is an S-type suffix if:
  // * Text[k] < Text[k+1], or
  // * Text[k] = Text[k+1] and the suffix at index k+1 is S-type, or
  // * Text[k] = $.
  // A suffix starting at position k is a L-type suffix if:
  // * Text[k] > Text[k+1], or
  // * Text[k] = Text[k+1] and the suffix at position k+1 is L-type.
  // false -> S-type suffix; true -> L-type suffix
  std::vector<bool> types(sequence.size());
  types[sequence.size() - 1] = false;
  types[sequence.size() - 2] = true;
  for (std::size_t i{sequence.size() - 2}; i >= 1; --i) {
    if (sequence[i] > sequence[i - 1]) {
      types[i - 1] = false;
    } else if (sequence[i] < sequence[i - 1]) {
      types[i - 1] = true;
    } else {
      types[i - 1] = types[i];
    }
  }
  return types;
}

std::pair<std::vector<std::size_t>, std::vector<bool>> tagLmsSuffixes(const std::vector<bool> suffixesTypes) {
  std::vector<std::size_t> lmsSuffixes;
  std::vector<bool> lmsSuffixFlags(suffixesTypes.size(), false);
  for (std::size_t i{1}; i < suffixesTypes.size(); ++i)
    if (suffixesTypes[i] == false && suffixesTypes[i - 1]) {
      lmsSuffixes.push_back(i);
      lmsSuffixFlags[i] = true;
    }
  return {lmsSuffixes, lmsSuffixFlags};
}

std::unordered_map<std::size_t, std::string_view> lmsBlocks(const std::string& word,
                                                            const std::vector<std::size_t>& lmsSuffixes) {
  std::unordered_map<std::size_t, std::string_view> lmsBlocks;
  auto wordBegin = word.data();
  for (std::size_t i{0}; i < lmsSuffixes.size() - 1; ++i)
    lmsBlocks.insert({lmsSuffixes[i], {wordBegin + lmsSuffixes[i], lmsSuffixes[i + 1] - lmsSuffixes[i] + 1}});
  lmsBlocks.insert({lmsSuffixes.back(), {wordBegin + lmsSuffixes.back(), 1}});
  return lmsBlocks;
}

std::vector<std::size_t> lmsBlocksSortedByFirstChar(const std::vector<bool> lmsSuffixFlags,
                                                    const std::vector<int64_t>& suffixArray) {
  std::vector<std::size_t> lmsBlocksOrder;
  for (const auto& suffix : suffixArray)
    if (lmsSuffixFlags[suffix]) lmsBlocksOrder.push_back(suffix);
  return lmsBlocksOrder;
}

std::pair<std::unordered_map<std::size_t, std::size_t>, int> encodeLmsBlocks(
    const std::vector<std::size_t> lmsBlocksOrder, const std::unordered_map<std::size_t, std::string_view> lmsBlocks) {
  std::unordered_map<std::size_t, std::size_t> numberedLmsBlocks;
  std::size_t blockNumber{0};
  numberedLmsBlocks.insert({lmsBlocksOrder[0], blockNumber});
  for (std::size_t i{1}; i < lmsBlocksOrder.size(); ++i) {
    const auto& previousBlock = lmsBlocksOrder[i - 1];
    const auto& currentBlock = lmsBlocksOrder[i];
    if (lmsBlocks.at(previousBlock) == lmsBlocks.at(currentBlock)) {
      numberedLmsBlocks.insert({currentBlock, blockNumber});
    } else {
      blockNumber++;
      numberedLmsBlocks.insert({currentBlock, blockNumber});
    }
  }
  return {numberedLmsBlocks, blockNumber + 1};
}

std::string stringFromEncodedBlocks(std::vector<std::size_t> lmsSuffixes,
                                    std::unordered_map<std::size_t, std::size_t> encodedBlocks) {
  std::string result;
  for (const auto& suffix : lmsSuffixes) result += std::to_string(encodedBlocks.at(suffix));
  return result;
}

std::vector<std::size_t> sortLmsBlocksLexicographically(
    const std::unordered_map<std::size_t, std::size_t>& blocksEncoding) {
  std::vector<std::size_t> sortedLmsSuffixes(blocksEncoding.size());
  for (const auto& [suffix, encoding] : blocksEncoding) sortedLmsSuffixes[encoding] = suffix;
  return sortedLmsSuffixes;
}

void resetFreeSlots(std::map<char, std::pair<std::size_t, std::size_t>>& freeSlots,
                    const std::map<char, std::pair<std::size_t, std::size_t>>& borders) {
  for (auto& [c, freeSlots] : freeSlots) freeSlots = borders.at(c);
}

void positionLmsSuffixes(std::map<char, std::pair<std::size_t, std::size_t>>& freeSlots,
                         std::vector<int64_t>& suffixArray, const std::vector<std::size_t>& lmsSuffixes,
                         const std::string& word) {
  for (const auto& lmsSuffix : lmsSuffixes) {
    const char c = word[lmsSuffix];
    const int freeSlot = --(freeSlots[c].second);
    suffixArray[freeSlot] = lmsSuffix;
  }
}

void positionLSuffixes(std::map<char, std::pair<std::size_t, std::size_t>>& freeSlots,
                       std::vector<int64_t>& suffixArray, const std::string& word, const std::vector<bool>& types) {
  for (int i{0}; i < suffixArray.size(); ++i) {
    if (suffixArray[i] != -1 && suffixArray[i] > 0 && types[suffixArray[i] - 1]) {
      const char c = word[suffixArray[i] - 1];
      const int freeSlot = freeSlots[c].first++;
      suffixArray[freeSlot] = suffixArray[i] - 1;
    }
  }
}

void positionSSuffixes(std::map<char, std::pair<std::size_t, std::size_t>>& freeSlots,
                       std::vector<int64_t>& suffixArray, const std::string& word, const std::vector<bool>& types) {
  for (std::size_t i{suffixArray.size()}; i-- != 0;) {
    if (suffixArray[i] != -1 && suffixArray[i] > 0 && !types[suffixArray[i] - 1]) {
      const char c = word[suffixArray[i] - 1];
      const int freeSlot = --(freeSlots.at(c).second);
      suffixArray[freeSlot] = suffixArray[i] - 1;
    }
  }
}

std::vector<std::size_t> decodeSuffixArrayPrim(const std::vector<int64_t>& suffixArrayPrim,
                                               const std::vector<std::size_t>& lmsSuffixes) {
  std::vector<std::size_t> sortedLmsSuffixes(lmsSuffixes.size());
  for (std::size_t i{1}; i < suffixArrayPrim.size(); ++i) sortedLmsSuffixes[i - 1] = lmsSuffixes[suffixArrayPrim[i]];
  return sortedLmsSuffixes;
}

int main() {
  const std::string word = "gtcccgatgtcatgtcagga";
  std::cout << "Word: " << word << '\n';
  std::cout << "Suffix array: ";
  const auto suffixArray = sais(word);
  std::copy(suffixArray.begin(), suffixArray.end(), std::ostream_iterator<int64_t>(std::cout, " "));
}
