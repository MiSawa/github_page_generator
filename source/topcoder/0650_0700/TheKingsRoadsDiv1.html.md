---

problem:
    round: SRM650
    level: [Div1Medium]
    rd: 16314
    pm: 13271
    name: TaroFillingAStringDiv1
    url: http://community.topcoder.com/stat?c=problem_statement&pm=13271&rd=16314
date: 2015/03/03
tags: [TopCoder, グラフ, 木]

---

### 概要

$h$ と, $2^h-1$ 頂点, $2^h+1$ 辺のグラフが与えられる.
このグラフを, 高さ $h$ の完全二分木に辺を $3$ 本加えて作れるか判定せよ.

#### 制約

$2 \le h \le 10$.

### 解法

- 多重辺, 自己ループを除去.
- 次数 $4$ の頂点を見て, 全ての頂点が次数 $3$ 以下になるような, $3$ 本以下の辺の取り除き方を全パターン試す.
- 各パターンに対し, 根を一つ固定する.
- 根の次数が $1$ なら弾き, $3$ なら一本除くパターンを全て試す.

ここまでやると, 答えが YES で実際の根が固定したやつと一致していたならば, 根から距離 $h-2$ までの頂点が高さ $h-1$ の完全二分木で,
葉同士を辺が結んでいる可能性のあるグラフになるハズ.
あとは, 実際にそうなっているか試すだけ.


### ソースコード

~~~ cpp
int dfs(vector<vector<int>> &g, int r, int p, int h, vector<int> &used){
    if(used[r]) return false;
    used[r] = true;
    if(h == 0) return true;
    if(size(g[r]) <= 2) return false;
    for(auto &u : g[r]) if(u != p) if(!dfs(g, u, r, h-1, used)) return false;
    return true;
}

bool check(vector<vector<int>> &g, int r, int h){
    int n = size(g);
    vector<int> used(n, 0);
    rep(A, 1<<size(g[r])) if(__builtin_popcount(A) == 2){
        fill(all(used), 0);
        used[r] = true;
        bool ok = true;
        rep(i, size(g[r])) if(A>>i&1) ok &= dfs(g, g[r][i], r, h-1, used);
        rep(u, n) ok &= used[r];
        if(ok) return true;
    }
    return false;
}

string TheKingsRoadsDiv1::getAnswer( int h, vector <int> a, vector <int> b ){
    for(aur x : a) --x;
    for(aur x : b) --x;
    rep(i, size(a)) if(a[i] > b[i]) swap(a[i], b[i]);

    int n = (1<<h) - 1;
    vector<vector<int>> g(n);
    set<pair<int, int>> used;
    rep(i, size(a)) if(a[i] != b[i]){
        if(used.count(make_pair(a[i], b[i]))) continue;
        used.emplace(a[i], b[i]);
        g[a[i]].emplace_back(b[i]);
        g[b[i]].emplace_back(a[i]);
    }
    vector<pair<int, int>> ng;
    rep(u, n) if(size(g[u]) > 3) for(aur v : g[u]) if(size(g[v]) > 1){
        ng.emplace_back(min(u, v), max(u, v));
    }
    sort(all(ng));
    ng.erase(unique(all(ng)), end(ng));
    rep(A, 1<<size(ng)) if(__builtin_popcount(A) <= 3){
        auto g_bck = g;
        auto rm = [&](int &u, int &v){
            rep(i, size(g[u])) if(g[u][i] == v){
                swap(g[u][i], g[u].back());
                g[u].pop_back();
            }
        };
        rep(i, size(ng)) if(A>>i&1){
            rm(ng[i].fst, ng[i].snd);
            rm(ng[i].snd, ng[i].fst);
        }
        bool ok = true;
        rep(u, n) if(size(g[u]) > 3) ok = false;
        if(ok) rep(u, n) if(check(g, u, h-1))
            return "Correct";
        g = g_bck;
    }
    return "Incorrect";
}
~~~

