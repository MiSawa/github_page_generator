---

problem:
    round: SRM600
    level: [Div1 Easy 250]
    rd: "15712"
    pm: "12888"
    name: ORSolitaire
    url: http://community.topcoder.com/stat?c=problem_statement&pm=12888&rd=15712
date: 2015/09/29
tags: [TopCoder, BitOperation]

---

### 概要

$50$ 個ほどの数の集合 $\mathit{numbers}$ と, 目的の数 $\mathit{goal}$ が与えられる.
$\mathit{numbers}$ からいくつか削除して, どう選んで BitwiseOR をとっても $\mathit{goal}$ に出来ないようにしたい.

削除する個数の最小値を求めよ.

#### 制約

$1 \le \mathit{numbers.size()} \le 50,\ 1 \le \mathit{numbers}_i, \mathit{goal} \le 10^9$.

### 解法

OR を取るとはみ出すやつは, 削除する必要が無いので, 無かったことにする.

すると, "OR を取って $\mathit{goal}$ にならない" というのは, "少なくとも一つの bit が $0$ のまま" ということだから,
各ビットについて, そいつを消すのに必要な手数を求めればよい.


### ソースコード

~~~ cpp
// 236.85 pts
int ORSolitaire::getMinimum( vector <int> numbers, int goal ){
    sort(all(numbers));
    vector<int> candidates;
    for(auto x : numbers) if(((~goal)&x) == 0) candidates.emplace_back(x);
    int res = candidates.size();
    rep(i, 30) if(goal>>i&1){
        int cnt = 0;
        for(auto x : candidates) if(x>>i&1) ++cnt;
        chmin(res, cnt);
    }
    return res;
}
~~~

