---

problem:
    round: SRM585
    level: [Div1 Easy 250]
    rd: "15697"
    pm: "11361"
    name: TrafficCongestion
    url: http://community.topcoder.com/stat?c=problem_statement&pm=11361&rd=15697
date: 2015/10/07
tags: [TopCoder]

---

### 概要

高さ $ h $ の完全二分木がある.

この頂点たちを distinct な path で被覆したい.
被覆するために必要な path の数の最小値を求めよ.

#### 制約

$ 0 \le h \le 10^6 $

### 解法

下から順番にへの字型にとっていくとよさげ.

最適性は, たぶん木DPをすることを考えると出るはず.

### ソースコード

~~~ cpp
// 231.16 pts

constexpr int mod = 1000000007;
int TrafficCongestion::theMinCars( int treeHeight ){
    long long res = 1;
    rep(i, treeHeight){
        res *= 2;
        if(i%2) ++res;
        else    res += mod-1;
        res %= mod;
    }
    return res;
}
~~~

