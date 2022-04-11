// link to the problem: https://leetcode.com/problems/word-ladder/
// this gets the correct answer but TLEs on leetcode
#include <algorithm>
#include <array>
#include <cassert>
#include <iostream>
#include <iterator>
#include <numeric>
#include <queue>
#include <sstream>
#include <stack>
#include <stdexcept>
#include <string>
#include <unordered_map>
#include <vector>

#include "MapValuesAccumulator.cpp"

class Solution {
 public:
  int ladderLength(std::string beginWord, std::string endWord, std::vector<std::string>& wordList) {
    std::vector<std::vector<int>> adjacency(wordList.size(), std::vector<int>(wordList.size()));

    // edge case - end word not in wordlist
    if (std::find(wordList.begin(), wordList.end(), endWord) == wordList.end()) return 0;
    int end_word_position = std::distance(wordList.begin(), std::find(wordList.begin(), wordList.end(), endWord));

    // create adjacency matrix
    std::vector<std::vector<int>> previous(adjacency);
    for (int i = 0; i < adjacency.size(); i++)
      for (int j = 0; j < adjacency.size(); j++) {
        if (i == j) {
          adjacency[i][j] = 0;
          previous[i][j] = i;
        }
        else {
          int matched = 0;
          for (int k = 0; k < endWord.size(); k++) matched += wordList[i][k] == wordList[j][k];
          if (matched == endWord.size() - 1) {
            adjacency[i][j] = adjacency[j][i] = 1;
            previous[i][j] = j;
            previous[j][i] = i;
          } else {
            adjacency[i][j] = adjacency[j][i] = INT32_MAX;
          }
        }
      }
    // FWM
    for (int i = 0; i < adjacency.size(); i++)
      for (int j = 0; j < adjacency.size(); j++)
        for (int k = 0; k < adjacency.size(); k++) {
          if (adjacency[i][j] != INT32_MAX && adjacency[i][k] != INT32_MAX &&
              adjacency[j][k] > adjacency[i][j] + adjacency[i][k]) {
            adjacency[j][k] = adjacency[i][j] + adjacency[i][k];
            previous[j][k] = previous[j][i];
          }
        }

    // for (int i = 0; i < adjacency.size(); i++) {
    //   std::copy(adjacency[i].begin(), adjacency[i].end(), std::ostream_iterator<int>(std::cout, " "));
    //   std::cout << '\n';
    // }

    int answer = INT32_MAX;

    // edge-case: beginWord in wordList
    auto found = std::find(wordList.begin(), wordList.end(), beginWord);
    if (found != wordList.end()) {
      if (adjacency[std::distance(wordList.begin(), found)][end_word_position] == INT32_MAX)
        return 0;
      else {
        auto source = std::distance(wordList.begin(), found);
        auto target = end_word_position;
        int it = source;
        std::cout << wordList[source] << '\n';
        while(it != target) {
          it = previous[it][target];
          std::cout << wordList[it] << '\n';
        }
        return adjacency[std::distance(wordList.begin(), found)][end_word_position] + 1;
      }
    }

    for (int i = 0; i < wordList.size(); i++) {
      int matched = 0;
      for (int k = 0; k < beginWord.size(); k++) matched += beginWord[k] == wordList[i][k];
      if (matched == beginWord.size() - 1) {
        answer = std::min(answer, adjacency[i][end_word_position]);
      }
    }
    return answer == INT32_MAX ? 0 : answer + 2;
  }
};

int main(int argc, char** argv) {
  Solution s;
  std::vector<std::string> words{
      "kid", "tag", "pup", "ail", "tun", "woo", "erg", "luz", "brr", "gay", "sip", "kay", "per", "val", "mes", "ohs",
      "now", "boa", "cet", "pal", "bar", "die", "war", "hay", "eco", "pub", "lob", "rue", "fry", "lit", "rex", "jan",
      "cot", "bid", "ali", "pay", "col", "gum", "ger", "row", "won", "dan", "rum", "fad", "tut", "sag", "yip", "sui",
      "ark", "has", "zip", "fez", "own", "ump", "dis", "ads", "max", "jaw", "out", "btu", "ana", "gap", "cry", "led",
      "abe", "box", "ore", "pig", "fie", "toy", "fat", "cal", "lie", "noh", "sew", "ono", "tam", "flu", "mgm", "ply",
      "awe", "pry", "tit", "tie", "yet", "too", "tax", "jim", "san", "pan", "map", "ski", "ova", "wed", "non", "wac",
      "nut", "why", "bye", "lye", "oct", "old", "fin", "feb", "chi", "sap", "owl", "log", "tod", "dot", "bow", "fob",
      "for", "joe", "ivy", "fan", "age", "fax", "hip", "jib", "mel", "hus", "sob", "ifs", "tab", "ara", "dab", "jag",
      "jar", "arm", "lot", "tom", "sax", "tex", "yum", "pei", "wen", "wry", "ire", "irk", "far", "mew", "wit", "doe",
      "gas", "rte", "ian", "pot", "ask", "wag", "hag", "amy", "nag", "ron", "soy", "gin", "don", "tug", "fay", "vic",
      "boo", "nam", "ave", "buy", "sop", "but", "orb", "fen", "paw", "his", "sub", "bob", "yea", "oft", "inn", "rod",
      "yam", "pew", "web", "hod", "hun", "gyp", "wei", "wis", "rob", "gad", "pie", "mon", "dog", "bib", "rub", "ere",
      "dig", "era", "cat", "fox", "bee", "mod", "day", "apr", "vie", "nev", "jam", "pam", "new", "aye", "ani", "and",
      "ibm", "yap", "can", "pyx", "tar", "kin", "fog", "hum", "pip", "cup", "dye", "lyx", "jog", "nun", "par", "wan",
      "fey", "bus", "oak", "bad", "ats", "set", "qom", "vat", "eat", "pus", "rev", "axe", "ion", "six", "ila", "lao",
      "mom", "mas", "pro", "few", "opt", "poe", "art", "ash", "oar", "cap", "lop", "may", "shy", "rid", "bat", "sum",
      "rim", "fee", "bmw", "sky", "maj", "hue", "thy", "ava", "rap", "den", "fla", "auk", "cox", "ibo", "hey", "saw",
      "vim", "sec", "ltd", "you", "its", "tat", "dew", "eva", "tog", "ram", "let", "see", "zit", "maw", "nix", "ate",
      "gig", "rep", "owe", "ind", "hog", "eve", "sam", "zoo", "any", "dow", "cod", "bed", "vet", "ham", "sis", "hex",
      "via", "fir", "nod", "mao", "aug", "mum", "hoe", "bah", "hal", "keg", "hew", "zed", "tow", "gog", "ass", "dem",
      "who", "bet", "gos", "son", "ear", "spy", "kit", "boy", "due", "sen", "oaf", "mix", "hep", "fur", "ada", "bin",
      "nil", "mia", "ewe", "hit", "fix", "sad", "rib", "eye", "hop", "haw", "wax", "mid", "tad", "ken", "wad", "rye",
      "pap", "bog", "gut", "ito", "woe", "our", "ado", "sin", "mad", "ray", "hon", "roy", "dip", "hen", "iva", "lug",
      "asp", "hui", "yak", "bay", "poi", "yep", "bun", "try", "lad", "elm", "nat", "wyo", "gym", "dug", "toe", "dee",
      "wig", "sly", "rip", "geo", "cog", "pas", "zen", "odd", "nan", "lay", "pod", "fit", "hem", "joy", "bum", "rio",
      "yon", "dec", "leg", "put", "sue", "dim", "pet", "yaw", "nub", "bit", "bur", "sid", "sun", "oil", "red", "doc",
      "moe", "caw", "eel", "dix", "cub", "end", "gem", "off", "yew", "hug", "pop", "tub", "sgt", "lid", "pun", "ton",
      "sol", "din", "yup", "jab", "pea", "bug", "gag", "mil", "jig", "hub", "low", "did", "tin", "get", "gte", "sox",
      "lei", "mig", "fig", "lon", "use", "ban", "flo", "nov", "jut", "bag", "mir", "sty", "lap", "two", "ins", "con",
      "ant", "net", "tux", "ode", "stu", "mug", "cad", "nap", "gun", "fop", "tot", "sow", "sal", "sic", "ted", "wot",
      "del", "imp", "cob", "way", "ann", "tan", "mci", "job", "wet", "ism", "err", "him", "all", "pad", "hah", "hie",
      "aim", "ike", "jed", "ego", "mac", "baa", "min", "com", "ill", "was", "cab", "ago", "ina", "big", "ilk", "gal",
      "tap", "duh", "ola", "ran", "lab", "top", "gob", "hot", "ora", "tia", "kip", "han", "met", "hut", "she", "sac",
      "fed", "goo", "tee", "ell", "not", "act", "gil", "rut", "ala", "ape", "rig", "cid", "god", "duo", "lin", "aid",
      "gel", "awl", "lag", "elf", "liz", "ref", "aha", "fib", "oho", "tho", "her", "nor", "ace", "adz", "fun", "ned",
      "coo", "win", "tao", "coy", "van", "man", "pit", "guy", "foe", "hid", "mai", "sup", "jay", "hob", "mow", "jot",
      "are", "pol", "arc", "lax", "aft", "alb", "len", "air", "pug", "pox", "vow", "got", "meg", "zoe", "amp", "ale",
      "bud", "gee", "pin", "dun", "pat", "ten", "mob"};
  std::cout << s.ladderLength("cet", "ism", words);
}
