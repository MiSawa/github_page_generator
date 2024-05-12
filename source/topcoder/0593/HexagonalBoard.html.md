---

problem:
    round: SRM593
    level: [Div1 Easy 250]
    rd: "15705"
    pm: "12784"
    name: HexagonalBoard
    url: http://community.topcoder.com/stat?c=problem_statement&pm=12784&rd=15705
date: 2015/10/01
tags: [TopCoder, 彩色]

---

### 概要

六角座標のセルが与えられる.
指定されたいくつかのセルに色を付けたい.
但し, 辺を共有する二つのセルが同じ色になってはいけない.

必要な色の最小数を求めよ


#### 制約

$ 1 \le n \le 50 $.

### 解法

六角座標の隣接関係は三角格子で, これは三彩色可能.
よって, 答えは $ 0, 1, 2, 3 $ のいずれか.

- 何も塗らなくて良ければ $ 0 $.
- 塗るものがどれも隣り合っていなければ $ 1 $.
- 塗るものの隣接関係のグラフが二部グラフなら $ 2 $.
- そうでなければ $ 3 $.


### ソースコード

~~~ cpp
// 232.37

int HexagonalBoard::minColors( vector <string> board ){
    const int n = size(board);
    vector<int> dx = {-1, 0, 1, 1, 0, -1};
    vector<int> dy = {0, -1, -1, 0, 1, 1};
    vector<vector<int>> g(n*n);
    rep(i, n) rep(j, n) rep(dir, 6){
        int ii = i + dx[dir], jj = j + dy[dir];
        if(0 > ii or ii >= n or 0 > jj or jj >= n) continue;
        if(board[i][j] == 'X' and board[ii][jj] == 'X')
            g[i*n+j].emplace_back(ii*n+jj);
    }
    vector<int> d(n*n, 1<<25);
    rep(i, n*n) if(d[i] == (1<<25)){
        d[i] = 0;
        queue<int> q; q.emplace(i);
        while(!q.empty()){
            int u = q.front(); q.pop();
            for(auto v : g[u]) if(chmin(d[v], d[u] + 1)) q.emplace(v);
        }
    }
    rep(u, n*n) for(auto &v : g[u]) if(d[u]%2 == d[v]%2) return 3;
    rep(u, n*n) if(!g[u].empty()) return 2;
    rep(i, n) rep(j, n) if(board[i][j] == 'X') return 1;
    return 0;
}
~~~

