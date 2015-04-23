---

problem:
    round: TCO2015R1A
    level: [Div1 Medium 500]
    rd: "16432"
    pm: "13707"
    name: Autogame
    url: http://community.topcoder.com/stat?c=problem_statement&pm=13707&rd=16432
date: 2015/04/12
tags: [TopCoder, TCO, graph, functionalgraph]

---

### 概要

Functional Graph, すなわち, 有向グラフであって, 各頂点の出次数がちょうど $1$ であるようなものが与えられる.

自然数 $k$ を指定した時, 各頂点 $v$ に対し, $v$ を始点とする長さ $k$ の walk の終点 $v^k$ が一意に定まる.
これを $k$ ターン後の頂点と呼ぶ.

頂点の部分集合 $\\{v_i\\}$ で, $0 \le j \le k$ なる任意の $j$ で $v_i^j$ が distinct なものの個数を求めよ.

#### 制約

$1 \le n \le 50,\ 1 \le k \le 10^9$.

### 解法

一回同じ頂点になったらその後もずっと同じ頂点だから, $k$ ターン後の頂点 $v_i^k$ が distinct であればよい.

$n$ ターン以内にループに入るから, $\min\\{n, k\\}$ ターンのシミュレートを行うと, $k$ ターン後に同じ頂点になるような頂点達がわかる.

これら, $k$ ターン後に同じ頂点になるやつらは, そのうち一つ選ぶか, 一つも選ばないかの $k+1$ 通りの選び方がある.
同じにならないやつらとは独立なので, それらを掛けあわせればいい.

本番中は $k$ ターン後の位置の一致が推移的なことに気づかず, $k$ ターン後の位置の一致でグラフを作り, 各連結成分で探索をしてしまった.
当然, 連結成分は完全グラフになるので探索は一瞬で終了し, AC はする.

### ソースコード

~~~ cpp
constexpr ll mod = 1000 * 1000 * 1000 + 7;

struct UnionFind{ //{{{
    vector<int> par;
    int n, cnt;
    UnionFind(const int &x=0){init(x);}
    void init(const int &x){par.assign(cnt = n = x, -1);}
    inline int find(const int &x){ return par[x]<0 ? x : par[x] = find(par[x]); }
    inline bool same(const int &x, const int &y){ return find(x) == find(y); }
    inline bool unite(int x, int y){
        if((x = find(x)) == (y = find(y))) return false;
        --cnt;
        if(par[x] > par[y]) swap(x, y);
        par[x] += par[y];
        par[y] = x;
        return true;
    }
    inline int count() const { return cnt; }
    inline int count(int x){ return -par[find(x)]; }
};
//}}}

ll dfs(ll mask, int u, ll ng, const vector<ll> &g){
    for(; u >= 0; --u) if(mask>>u&1) break;
    if(u == -1) return 1;
    if(ng>>u&1) return dfs(mask, u-1, ng, g);
    ll res = 0;
    res += dfs(mask, u-1, ng, g);
    res += dfs(mask, u-1, ng | g[u], g);
    if(res >= mod) res -= mod;
    return res;
}

int Autogame::wayscnt( vector <int> a, int K ){
    for(aur u : a) --u;

    const int n = sz(a);
    vector<ll> g(n);
    rep(u, n) rep(v, n){
        int uu = u, vv = v;
        for(int i = 0; i < min(K, n * n); ++i){
            uu = a[uu], vv = a[vv];
            if(uu == vv){
                g[u] |= 1LL<<v;
                g[v] |= 1LL<<u;
            }
        }
    }
    UnionFind uf(n);
    rep(u, n) rep(v, n) if(g[u]>>v&1) uf.unite(u, v);
    ll res = 1;
    rep(u, n) if(uf.find(u) == u){
        ll mask = 0;
        rep(v, n) if(uf.find(v) == u) mask |= 1LL<<v;
        res = (res * dfs(mask, n, 0, g)) % mod;
    }
    return res;
}
~~~

