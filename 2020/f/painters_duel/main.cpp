#include <iostream>
#include <vector>
#include <list>
#include <stack>
#include <queue>
#include <unordered_set>
#include <ctgmath>
using namespace std;

#define DEBUG 1

#ifdef DEBUG
#include <cassert>
#endif

enum Color { NONE, ALMA, BERTHA, CONS };
enum Turn { T_ALMA, T_BERTHA };

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
    int bertha_index;
    int score;
    Turn turn;
    vector<Color> room_colors;

    State(Pos a_pos, Pos b_pos) {
        alma_index = toIndex(a_pos);
        bertha_index = toIndex(b_pos);
        score = 0;
        turn = T_ALMA;
        room_colors = vector<Color>(g_building_sides * g_building_sides, NONE);
        room_colors[alma_index] = ALMA;
        room_colors[bertha_index] = BERTHA;
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
            turn = T_BERTHA;
        } else {
            room_colors[i] = BERTHA;
            bertha_index = i;
            score--;
            turn = T_ALMA;
        }
    }

    vector<State> expand_succs() {
        int start;
        if (turn == T_ALMA) {
            start = alma_index;
        } else {
            start = bertha_index;
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
                succ.turn = T_BERTHA;
            } else {
                succ.turn = T_ALMA;
            }
            result.push_back(succ);
        }

        return result;
    }

    bool canMove(Turn player) {
        int start;
        if (player == T_ALMA) {
            start = alma_index;
        } else {
            start = T_BERTHA;
        }
        return paintable_neighbors(start).size() > 0;
    }

    bool gameFinished() {
        return !canMove(T_ALMA) && !canMove(T_BERTHA);
    }
};

#ifdef DEBUG
void testState()
{
    g_building_sides = 4;
    auto alma_pos = Pos(2, 2);
    auto bertha_pos = Pos(4, 2);
    auto state = State(alma_pos, bertha_pos);
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
        assert(succ.turn == T_BERTHA);
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
#endif

void searchStates(State& initial_state) {
    stack<State> q;
    q.push(initial_state);
    while (q.size() > 0) {
        auto state = q.top();
        q.pop();

        if (state.gameFinished()) {
            cout << state.score << endl;
            // TODO: record max
        } else {
            auto succs = state.expand_succs();
            for (auto succ : succs) {
                // TODO: Favor succs with a higher score
                q.push(succ);
            }
        }
    }
}

int main(int argc, char *argv[])
{
    ios_base::sync_with_stdio(false);
    cin.tie(NULL);

    int t;
    cin >> t;
    for (int ti = 0; ti < t; ++ti) {
        Pos alma_pos, bertha_pos;
        int n_cons;
        cin >> g_building_sides >>
            alma_pos.row >> alma_pos.col >>
            bertha_pos.row >> bertha_pos.col >>
            n_cons;

        auto state = State(alma_pos, bertha_pos);
        state.turn = T_ALMA;

        for (int i = 0; i < n_cons; ++i) {
            Pos cons_pos;
            cin >> cons_pos.row >> cons_pos.col;
            state.room_colors[state.toIndex(cons_pos)] = CONS;
        }

        searchStates(state);
        cout << "---" << endl;
    }

    return 0;
}
