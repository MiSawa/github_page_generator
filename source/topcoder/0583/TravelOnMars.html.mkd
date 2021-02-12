---

problem:
    round: SRM583
    level: [Div1 Easy 250]
    rd: "15503"
    pm: "12608"
    name: TravelOnMars
    url: https://community.topcoder.com/stat?c=problem_statement&pm=12608&rd=15503
date: 2015/11/04
tags: [TopCoder]

---

### 概要

$ N $ 個の街が環状に並んでおり, 街 $ i $ からは, $ i - \mathit{range}_i \bmod N $ から $ i + \mathit{range}_i \bmod N $ までのどこへでも一回で行ける.

$ S $ から $ T $ へ行くのに必要な回数を求めよ.

#### 制約

$ 2 \le N \le 50 $.

### 解法

BFS すればよい.
環状の時によくやる, 何周か持つテクが有効.

### ソースコード

~~~ cpp
// 243.97 pts

int TravelOnMars::minTimes( vector <int> range, int startCity, int endCity ){
    const int n = size(range);
    vector<int> d(n * 4, 1<<20);
    queue<int> q;
    q.emplace(startCity + n); d[startCity + n] = 0;
    while(!q.empty()){
        int u = q.front(); q.pop();
        if(u % n == endCity) return d[u];
        for(int v = u-range[u%n]; v <= u+range[u%n]; ++v) if(0 <= v and v < d.size())
            if(chmin(d[v], d[u] + 1)) q.emplace(v);
    }
    return -1;
}

// vim:set foldmethod=marker commentstring=//%s:
~~~

