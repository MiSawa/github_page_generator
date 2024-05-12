---

problem:
    round: SRM586
    level: [Div1 Easy 250]
    rd: "15698"
    pm: "12691"
    name: PiecewiseLinearFunction
    url: http://community.topcoder.com/stat?c=problem_statement&pm=12691&rd=15698
date: 2015/10/07
tags: [TopCoder]

---

### 概要

$ [0, w] $ 上定義された, $ f(i) = y[i] $ を線型補完した関数が与えられる.

$ f(x) = t $ の解 $ x $ が最も多くなるような $ t $ を求め, その個数を返せ.
無限個あるときは $ -1 $.


#### 制約

$ 1 \le w \le 50,\ -10^9 \le y[i] \le 10^9 $

### 解法

$2$ 回も resubmit してしまった...
候補点を列挙してごにょる.

imos 法で $ o(n^2) $ でも行けそう.

### ソースコード

~~~ cpp
// 147.09 pts

int PiecewiseLinearFunction::maximumSolutions( vector <int> y ){
    int res = 0;
    const int n = size(y);
    rep(i, n-1) if(y[i] == y[i+1]) return -1;
    vector<long double> candidates(all(y));
    sort(all(candidates));
    rep(i, n-1) candidates.emplace_back((candidates[i]+candidates[i+1])/2);
    for(auto t : candidates){
        int now = 0;
        rep(i, n) now += y[i] == t;
        rep(i, n-1) if(y[i] != t and y[i+1] != t)
            now += (y[i] < t) xor (y[i+1] < t);
        chmax(res, now);
    }

    return res;
}
~~~

