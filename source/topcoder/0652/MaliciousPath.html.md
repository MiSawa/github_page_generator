---

problem:
    round: SRM652
    level: [Div1 Medium 500]
    rd: 16316
    pm: 13596
    name: MaliciousPath
    url: http://community.topcoder.com/stat?c=problem_statement&pm=13596&rd=16316
date: 2015/03/10
tags: [TopCoder, グラフ, 最短路, DP]

---

### 概要

$n$ 頂点の重み付き有向グラフ $G$ が与えられる.
ここで, $G$ は多重辺や自己ループは許す, 全ての頂点の出次数が正なグラフである.
Alice は頂点 $0$ から頂点 $n-1$ に行きたい.
Bob はそれを邪魔したい.
Alice は 1 ターンに一つ, 今居る場所から辺を辿って移動しようとする.
Bob は $k$ 回までその移動を邪魔し, 指定した辺を辿らせる事が出来る.

Bob の目標は, Alice に $n-1$ まで辿り着かせない事, それが不可能ならば, Alice の辿る辺のコストの和(同じ辺でも辿る度に加算)が最も大きくなるようにする事である.
一方, Alice の目標は, $n-1$ まで辿り着く事, それが可能ならコストの和が最も小さくなるようにする事である.

双方が最善を尽くした時のコストを求めよ.
辿りつけなければ $-1$.

#### 制約

$n = |V(G)| \le 10^3$, $|E(G)| \le 2.5 * 10^3$,
$c(e) := \mathrm{cost}(e) \le 10^6$.

### 解法
``long long`` に注意.

$H$ を $G$ の逆グラフとし,
Alice が頂点 $u$ に居て, Bob が残り $k$ 回邪魔出来る時の結果を $\mathrm{dp}[k][u]$ とする.

$\mathrm{dp}[0]$ は, $n-1$ からの $H$ での最短路長の配列である.

$\mathrm{dp}[k-1]$ が定まっている時,
- $\mathrm{dp}[k][u]$ は $\max_{e = (u, v)} \mathrm{dp}[k-1][v] + c(e)$ 以上.
- $\mathrm{dp}[k][u]$ は, $\min_{e = (u, v)} \mathrm{dp}[k][v] + c(e)$ 以上.
の二つを満たすよな最小のものを, $\mathrm{dp}[k][u]$ とすればよい.

後者が難しいが, 「後者だけで dijkstra 法をしつつ, 今確定する頂点が前者を満たさなければ, 前者を満たすように置き換える」という戦略でよい.
なぜならば, dijkstra 法で必要なのは, 「今確定する頂点が本当にこれ以降変わらない」という事で, これは満たされるから.

### ソースコード

~~~ cpp
struct E{
    int t, c;
    E(){}
    E(int t, int c) : t(t), c(c){}
};

typedef vector<vector<E>> G;
static const ll INF = 1LL<<50;

long long MaliciousPath::minPath( int n, int K, vector <int> from, vector <int> to, vector <int> cost ){
    G g(n), h(n);
    repsz(i, from) if(from[i] != n-1){
        g[from[i]].eb(to[i], cost[i]);
        h[to[i]].eb(from[i], cost[i]);
    }
    vector<ll> dp(n, INF), qb(n);
    for(int k = 0; k <= K; ++k){
        qb.assign(n, 0);
        rep(u, n) for(auto &e : g[u]) chmax(qb[u], dp[e.t] + e.c);

        vector<ll> d(n, INF);
        priority_queue<tuple<ll, int>, vector<tuple<ll, int>>, greater<tuple<ll, int>>> pq;
        pq.emplace(d[n-1] = 0, n-1);
        while(!pq.empty()){
            ll c; int u;
            tie(c, u) = pq.top();
            pq.pop();
            if(d[u] < c) continue;
            if(k) chmax(c, qb[u]);
            qb[u] = c;
            for(auto &e : h[u]) if(chmin(d[e.t], c + e.c)) pq.emplace(d[e.t], e.t);
        }
        swap(dp, qb);
    }
    if(dp[0] >= INF) dp[0] = -1;
    return dp[0];
}
~~~

