#include <iostream>
#include <vector>
#include <stack>
#include <queue>
#include <unordered_set>
#include <ctgmath>
using namespace std;

#include <cassert>

enum Color { NONE, ALMA, BERTHE, CONS };
enum Turn { T_ALMA, T_berthe };

int g_building_sides = 0;

struct Pos {
    int row, col;
    Pos() {
        row = 1;
        col = 1;
    }

    Pos(int r, int c) {
        row = r;
        col = c;
    }
};

struct State {
    int alma_index;
    int berthe_index;
    int score;
    Turn turn;
    vector<Color> room_colors;

    State(Pos a_pos, Pos b_pos) {
        alma_index = toIndex(a_pos);
        berthe_index = toIndex(b_pos);
        score = 0;
        turn = T_ALMA;

        room_colors = vector<Color>(g_building_sides * g_building_sides, NONE);
        room_colors[alma_index] = ALMA;
        room_colors[berthe_index] = BERTHE;
    }

    Pos toPos(int i) {
        int row = int(sqrt(i)) + 1;
        int prev_row = row - 1;
        int col = i - (prev_row * prev_row) + 1;
        return Pos(row, col);
    }

    int toIndex(Pos p) {
        int prev_row = p.row - 1;
        return (prev_row * prev_row) + p.col - 1;
    }

    bool pointsUp(int i) {
        int row = toPos(i).row;
        if (row % 2 == 1) {
            return i % 2 == 0;
        } else {
            return i % 2 == 1;
        }
    }

    bool isValid(Pos p) {
        if (p.row > 0 && p.row <= g_building_sides &&
            p.col > 0
        ) {
            int prev_row = p.row - 1;
            int n_indexs_in_row = (p.row * p.row) - (prev_row * prev_row);
            return p.col <= n_indexs_in_row;
        } else {
            return false;
        }
    }

    bool canPaint(Pos p) {
        return isValid(p) && room_colors[toIndex(p)] == NONE;
    }

    vector<int> paintable_neighbors(int i) {
        Pos p_orig = toPos(i);
        vector<int> result;
        Pos p;

        // left
        p = Pos(p_orig.row, p_orig.col - 1);
        if (canPaint(p)) {
            result.push_back(toIndex(p));
        }

        // right
        p = Pos(p_orig.row, p_orig.col + 1);
        if (canPaint(p)) {
            result.push_back(toIndex(p));
        }

        if (pointsUp(i)) {
            // down
            p = Pos(p_orig.row + 1, p_orig.col + 1);
            if (canPaint(p)) {
                result.push_back(toIndex(p));
            }
        } else {
            // up
            p = Pos(p_orig.row - 1, p_orig.col - 1);
            if (canPaint(p)) {
                result.push_back(toIndex(p));
            }
        }

        return result;
    }

    void paint(int i) {
        if (turn == T_ALMA) {
            room_colors[i] = ALMA;
            alma_index = i;
            score++;
            turn = T_berthe;
        } else {
            room_colors[i] = BERTHE;
            berthe_index = i;
            score--;
            turn = T_ALMA;
        }
    }

    vector<State> expand_succs() {
        int start;
        if (turn == T_ALMA) {
            start = alma_index;
        } else {
            start = berthe_index;
        }

        auto neighbors = paintable_neighbors(start);
        vector<State> result;
        if (neighbors.size() > 0) {
            for (auto neighbor : neighbors) {
                auto succ = *this;
                succ.paint(neighbor);
                result.push_back(succ);
            }
        } else {
            // Player cannot move
            auto succ = *this;
            if (turn == T_ALMA) {
                succ.turn = T_berthe;
            } else {
                succ.turn = T_ALMA;
            }
            result.push_back(succ);
        }

        return result;
    }

    int nMoves(Turn player) {
        int start;
        if (player == T_ALMA) {
            start = alma_index;
        } else {
            start = berthe_index;
        }
        return paintable_neighbors(start).size();
    }

    bool canMove(Turn player) {
        return nMoves(player) > 0;
    }

    bool gameFinished() {
        return !canMove(T_ALMA) && !canMove(T_berthe);
    }

    char toChar(Color color) {
        switch (color) {
            case NONE:
                return 'N';
            case ALMA:
                return 'A';
            case BERTHE:
                return 'B';
            case CONS:
                return 'C';
            default:
                return 'X';
        }
    }

    void printRooms() {
        int row = 1;
        int col = 1;
        int n_in_row = 1;
        for (size_t i = 0; i < room_colors.size(); ++i) {
            cout << toChar(room_colors[i]) << " ";

            col++;
            if (col > n_in_row) {
                row++;
                n_in_row += 2;
                col = 1;
                cout << endl;
            }
        }
    }
};

void testState()
{
    g_building_sides = 4;
    auto alma_pos = Pos(2, 2);
    auto berthe_pos = Pos(4, 2);
    auto state = State(alma_pos, berthe_pos);
    state.room_colors[0] = CONS;
    state.room_colors[14] = CONS;
    state.turn = T_ALMA;

    auto succs = state.expand_succs();
    for (auto succ : succs) {
        for (size_t i = 0; i < succ.room_colors.size(); ++i) {
            Color c = succ.room_colors[i];
            Pos p = succ.toPos(i);
            cout << "(" << p.row << ", " << p.col << "): " << c << endl;
        }
        cout << "Alma room index: " << succ.alma_index << endl;;
        assert(succ.turn == T_berthe);
        assert(succ.score == 1);
    }

    cout << "---" << endl;

    {
        int i = 11;
        auto p = state.toPos(i);
        assert(p.row == 4);
        assert(p.col == 3);
        assert(state.toIndex(p) == i);
        assert(state.pointsUp(i) == true);
        assert(state.pointsUp(i - 1) == false);
        assert(state.isValid(p) == true);
        assert(state.canPaint(p));

        auto neighbors = state.paintable_neighbors(i);
        for (int i : neighbors) {
            cout << i << endl;
        }
    }
}

int findMinScore(State& initial_state) {
    // TODO: Find the best score that Alma can guarantee

    bool min_score_set = false;
    int min_score = 0;

    stack<State> q;
    q.push(initial_state);
    while (q.size() > 0) {
        auto state = q.top();
        q.pop();

        if (state.gameFinished()) {
            cout << "temp: " << state.score << endl;
            state.printRooms();
            if (!min_score_set) {
                min_score = state.score;
                min_score_set = true;
            } else {
                min_score = min(min_score, state.score);
            }
        } else {
            auto succs = state.expand_succs();
            for (auto succ : succs) {
                // TODO: Favor succs with a higher score
                q.push(succ);
            }
        }
    }

    assert(min_score_set);
    return min_score;
}

int main(int argc, char *argv[])
{
    ios_base::sync_with_stdio(false);
    cin.tie(NULL);

    int t;
    cin >> t;
    for (int ti = 0; ti < t; ++ti) {
        Pos alma_pos, berthe_pos;
        int n_cons;
        cin >> g_building_sides >>
            alma_pos.row >> alma_pos.col >>
            berthe_pos.row >> berthe_pos.col >>
            n_cons;

        auto state = State(alma_pos, berthe_pos);
        state.turn = T_ALMA;

        for (int i = 0; i < n_cons; ++i) {
            Pos cons_pos;
            cin >> cons_pos.row >> cons_pos.col;
            state.room_colors[state.toIndex(cons_pos)] = CONS;
        }

        int result = findMinScore(state);
        cout << "Case #" << ti + 1 << ": " << result << endl;
    }

    return 0;
}
