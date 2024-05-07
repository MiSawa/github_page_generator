---

problem:
    round: SRM589
    level: [Div1 Med 450]
    rd: "15701"
    pm: "12729"
    name: GearsDiv1
    url: http://community.topcoder.com/stat?c=problem_statement&pm=12729&rd=15701
date: 2015/10/06
tags: [TopCoder]

---

### 概要

'R', 'G', 'B' の色のついた歯車と, 歯車同士の隣接関係が与えられる.
但し, 同じ色同士は隣接していない.

隣接した歯車は逆向きに回らなければならない.
また, 同じ色の歯車は同じ向きに回らなければならない.

これらの条件を満たすように, いくつか歯車を取り除く.
取り除く歯車の個数の最小値を求めよ.


#### 制約

$ 1 \le n \le 50 $.

### 解法

歯車が回る条件は, 隣接関係のグラフが二部グラフであること.

同じ色が同じ向きになるというのは, 二部グラフで同じ色は全て片側に固まっているということ.

どの色がどちら側になるかを固定する(1:2 に分ける三パターン)と, 同じ側の辺をなくせばよくて, それは最大独立集合と呼ばれているのでした.

普通に MIS を呼んでいたけれど, "但し, 同じ色同士は隣接していない." があるので,
1:2 の 2 側だけ二部グラフの最大独立集合をすればよい.

### ソースコード

~~~ cpp
// 375.27 pts

typedef long long ll;
typedef vector<ll> G;
namespace MIS{//{{{
    inline int tz(const ll &x){ return __builtin_ctzll(x); }
    inline int popcnt(const ll &x){ return __builtin_popcountll(x); }
    int n;
    ll g[70];
    ll res;
    void mis(ll choosed, ll rem){
        if(popcnt(choosed) > popcnt(res)) res = choosed;
        int k = -1;
        for(ll A = rem; A; A &= A-1){
            int u = tz(A), c = popcnt(g[u] & rem);
            if(c <= 1 || k == -1 || c > popcnt(g[k]&rem)) k = u;
            if(c <= 1) break;
        }
        if(k == -1) return;
        if(popcnt(g[k] & rem) >= 2) mis(choosed, rem & ~bit(k));
        mis(choosed | bit(k), rem & ~(bit(k)|g[k]));
    }
    ll solve(const G &_g){
        n = size(_g);
        rep(i, n) g[i] = _g[i];
        res = 0;
        mis(0, bit(n)-1);
        return res;
    }
};//}}}

int GearsDiv1::getmin( string color_, vector <string> graph ){
    const int n = size(color_);
    vector<int> color(n);
    rep(i, n){
        if(color_[i] == 'R') color[i] = 0;
        if(color_[i] == 'G') color[i] = 1;
        if(color_[i] == 'B') color[i] = 2;
    }

    int res = 0;
    rep(A, 4){
        vector<vector<int>> vs(2);
        rep(i, n) vs[A>>color[i]&1].emplace_back(i);
        vector<G> g(2);
        rep(t, 2){
            g[t].resize(size(vs[t]));
            repsz(i, vs[t]) rep(j, i) if(graph[vs[t][i]][vs[t][j]] == 'Y'){
                g[t][i] |= bit(j);
                g[t][j] |= bit(i);
            }
        }
        chmax(res, __builtin_popcountll(MIS::solve(g[0])) +
                __builtin_popcountll(MIS::solve(g[1])));
    }
    return n - res;
}
~~~

