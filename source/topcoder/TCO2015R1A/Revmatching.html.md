---

problem:
    round: TCO2015R1A
    level: [Div1 Hard 1000]
    rd: "16432"
    pm: "13708"
    name: Autogame
    url: http://community.topcoder.com/stat?c=problem_statement&pm=13708&rd=16432
date: 2015/04/12
tags: [TopCoder, TCO, graph, matching, 二部グラフ, 結婚定理]

---

### 概要

辺にコスト $c(u, v)$ のついた, 左右の頂点数の一致する完全二部グラフ $K _ {n,n} = (L, R, E)$ が与えられる.

いくつかの辺を取り除き, 完全マッチングが存在しないようにしたい.
取り除く辺のコストの和を最小化し, その最小値を答えよ.


#### 制約

$1 \le n \le 20$.

### 解法

Hall の結婚定理から, 二部グラフ $(L, R, E)$ に完全マッチングが存在しないための必要十分条件は,
$$ \exists A \subset L : \abs{N(A)} < \abs{A} $$
である. ここで, $N(A)$ は $A$ に隣接する頂点の集合.

一つ $A \subset L$ を固定し, 上の条件を $A$ で満たす, すなわち $\abs{N(A)} < \abs{A}$ とする為に必要なコストを考える.

$\abs{N(A)}$ から $v \in R$ を取り除くのにかかるコストは, `$\sum_{u \in A} c(u, v)$` だから, これでソートし, コストの高いもの $\abs{A}-1$ 個以外を取り除くことにすればよい.

あとは, これを各 $A \subset L$ で行い, その最小値を取ればよい.

### ソースコード

~~~cpp
int Revmatching::smallest( vector <string> A ){
    const int n = sz(A);
    vector<vector<int>> g(n, vector<int>(n));
    rep(i, n) rep(j, n) g[i][j] = A[i][j] - '0';
    ll res = numeric_limits<int>::max();
    rep(A, 1<<n) if(A){
        vector<int> c(n);
        rep(i, n) if(A>>i&1) rep(j, n) c[j] += g[i][j];
        sort(all(c));
        ll now = 0;
        int k = __builtin_popcount(A);
        rep(i, n-k+1) now += c[i];
        chmin(res, now);
    }
    return res;
}
~~~

