#include <algorithm>
#include <array>
#include <cstdint>
#include <cstdlib>
#include <iostream>
#include <limits>
#include <numeric>
#include <string_view>
#include <vector>

namespace {

constexpr int kPoints = 23;
constexpr int kBlack = 3;
constexpr int kColor = 4;
constexpr std::uint32_t kBlackPatterns = 1771;  // C(23, 3)
constexpr std::uint32_t kColorPatterns = 4845;  // C(20, 4)
constexpr std::uint32_t kStates = kBlackPatterns * kColorPatterns;
constexpr std::uint8_t kUnvisited = std::numeric_limits<std::uint8_t>::max();

using Permutation = std::array<std::uint8_t, kPoints>;
using Positions3 = std::array<int, kBlack>;
using Positions4 = std::array<int, kColor>;

std::array<std::array<std::uint32_t, 5>, kPoints + 1> binomial{};

void init_binomial() {
  for (int n = 0; n <= kPoints; ++n) {
    binomial[n][0] = 1;
    for (int k = 1; k <= 4; ++k) {
      binomial[n][k] = n == 0 ? 0 : binomial[n - 1][k - 1] + binomial[n - 1][k];
    }
  }
}

template <std::size_t K>
std::uint32_t rank_combination(const std::array<int, K>& positions) {
  std::uint32_t rank = 0;
  for (std::size_t i = 0; i < K; ++i) {
    rank += binomial[positions[i]][i + 1];
  }
  return rank;
}

template <std::size_t K>
std::array<int, K> unrank_combination(std::uint32_t rank, int n) {
  std::array<int, K> positions{};
  for (int i = static_cast<int>(K); i >= 1; --i) {
    int x = n - 1;
    while (binomial[x][i] > rank) {
      --x;
    }
    positions[i - 1] = x;
    rank -= binomial[x][i];
    n = x;
  }
  return positions;
}

std::uint32_t encode(Positions3 black, Positions4 color) {
  std::sort(black.begin(), black.end());
  std::sort(color.begin(), color.end());

  Positions4 compressed{};
  for (int i = 0; i < kColor; ++i) {
    compressed[i] = color[i] - static_cast<int>(
        std::lower_bound(black.begin(), black.end(), color[i]) - black.begin());
  }
  return rank_combination(black) * kColorPatterns + rank_combination(compressed);
}

void decode(std::uint32_t id, Positions3& black, Positions4& color) {
  black = unrank_combination<kBlack>(id / kColorPatterns, kPoints);
  const auto compressed = unrank_combination<kColor>(id % kColorPatterns,
                                                     kPoints - kBlack);

  int source = 0;
  int target = 0;
  int black_index = 0;
  for (int position = 0; position < kPoints && target < kColor; ++position) {
    if (black_index < kBlack && black[black_index] == position) {
      ++black_index;
      continue;
    }
    if (compressed[target] == source) {
      color[target++] = position;
    }
    ++source;
  }
}

Permutation identity() {
  Permutation permutation{};
  std::iota(permutation.begin(), permutation.end(), 0);
  return permutation;
}

void add_cycle(Permutation& permutation, std::array<int, 5> cycle) {
  for (int i = 0; i < 5; ++i) {
    permutation[cycle[i]] = cycle[(i + 1) % 5];
  }
}

Permutation compose(const Permutation& first, const Permutation& second) {
  Permutation result{};
  for (int i = 0; i < kPoints; ++i) {
    result[i] = second[first[i]];
  }
  return result;
}

std::array<Permutation, 4> generators() {
  std::array<Permutation, 4> result{identity(), identity(), identity(), identity()};
  add_cycle(result[0], {3, 4, 5, 6, 7});
  add_cycle(result[0], {8, 9, 10, 11, 12});
  add_cycle(result[1], {13, 14, 15, 16, 17});
  add_cycle(result[1], {18, 19, 20, 21, 22});
  add_cycle(result[2], {0, 4, 1, 2, 7});
  add_cycle(result[2], {3, 9, 5, 6, 12});
  add_cycle(result[3], {8, 14, 10, 11, 17});
  add_cycle(result[3], {13, 19, 15, 16, 22});
  return result;
}

std::vector<Permutation> moves(bool disc_turn_metric) {
  std::vector<Permutation> result;
  for (const auto& generator : generators()) {
    auto power = generator;
    for (int exponent = 1; exponent <= 4; ++exponent) {
      if (disc_turn_metric || exponent == 1 || exponent == 4) {
        result.push_back(power);
      }
      power = compose(power, generator);
    }
  }
  return result;
}

std::uint32_t apply(std::uint32_t id, const Permutation& permutation) {
  Positions3 black{};
  Positions4 color{};
  decode(id, black, color);
  for (int& position : black) {
    position = permutation[position];
  }
  for (int& position : color) {
    position = permutation[position];
  }
  return encode(black, color);
}

bool self_test() {
  for (std::uint32_t id = 0; id < kStates; ++id) {
    Positions3 black{};
    Positions4 color{};
    decode(id, black, color);
    if (encode(black, color) != id) {
      return false;
    }
  }

  const std::array<std::uint32_t, 8> samples{
      0, 1, 4844, 4845, 123456, 4000000, 8580494, kStates - 1};
  for (const bool disc_turn_metric : {false, true}) {
    const auto metric_moves = moves(disc_turn_metric);
    if (metric_moves.size() != (disc_turn_metric ? 16U : 8U)) {
      return false;
    }
    for (const auto id : samples) {
      for (const auto& move : metric_moves) {
        if (apply(id, move) >= kStates) {
          return false;
        }
      }
      if (!disc_turn_metric) {
        for (std::size_t generator = 0; generator < 4; ++generator) {
          const auto moved = apply(id, metric_moves[2 * generator]);
          if (apply(moved, metric_moves[2 * generator + 1]) != id) {
            return false;
          }
        }
      }
    }
  }
  return true;
}

void print_positions(const Positions3& black, const Positions4& color) {
  std::cout << "  farthest black positions:";
  for (const int position : black) {
    std::cout << ' ' << position + 1;
  }
  std::cout << "\n  farthest tracked-color positions:";
  for (const int position : color) {
    std::cout << ' ' << position + 1;
  }
  std::cout << '\n';
}

void analyze(bool disc_turn_metric) {
  const auto metric_moves = moves(disc_turn_metric);
  std::vector<std::uint8_t> distance(kStates, kUnvisited);
  std::vector<std::uint32_t> queue(kStates);
  std::uint32_t tail = 0;

  const Positions3 solved_black{0, 1, 2};
  for (int column = 0; column < 5; ++column) {
    const Positions4 solved_color{3 + column, 8 + column, 13 + column,
                                  18 + column};
    const auto goal = encode(solved_black, solved_color);
    if (distance[goal] == kUnvisited) {
      distance[goal] = 0;
      queue[tail++] = goal;
    }
  }

  std::array<std::uint64_t, 64> histogram{};
  std::uint32_t head = 0;
  std::uint8_t diameter = 0;
  std::uint32_t farthest = queue[0];
  while (head < tail) {
    const auto current = queue[head++];
    const auto current_distance = distance[current];
    ++histogram[current_distance];
    if (current_distance > diameter) {
      diameter = current_distance;
      farthest = current;
    }
    for (const auto& move : metric_moves) {
      const auto next = apply(current, move);
      if (distance[next] == kUnvisited) {
        distance[next] = current_distance + 1;
        queue[tail++] = next;
      }
    }
  }

  Positions3 black{};
  Positions4 color{};
  decode(farthest, black, color);
  std::cout << (disc_turn_metric ? "Disc-turn" : "Fifth-turn")
            << " metric\n";
  std::cout << "  moves: " << metric_moves.size() << '\n';
  std::cout << "  abstract states reached: " << tail << " / " << kStates
            << '\n';
  std::cout << "  exact black-plus-one-color goal radius: "
            << static_cast<int>(diameter) << '\n';
  std::cout << "  distance histogram:";
  for (int depth = 0; depth <= diameter; ++depth) {
    std::cout << ' ' << depth << ':' << histogram[depth];
  }
  std::cout << '\n';
  print_positions(black, color);
}

}  // namespace

int main(int argc, char** argv) {
  init_binomial();
  if (!self_test()) {
    std::cerr << "self-test failed\n";
    return EXIT_FAILURE;
  }
  if (argc == 2 && std::string_view(argv[1]) == "--self-test") {
    std::cout << "optimal-number self-test passed\n";
    return EXIT_SUCCESS;
  }
  if (argc != 1) {
    std::cerr << "usage: optimal-number [--self-test]\n";
    return EXIT_FAILURE;
  }

  analyze(false);
  analyze(true);
  return EXIT_SUCCESS;
}
