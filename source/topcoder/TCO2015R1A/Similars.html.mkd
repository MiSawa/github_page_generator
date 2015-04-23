---

problem:
    round: TCO2015R1A
    level: [Div1 Easy 250]
    rd: "16432"
    pm: "13714"
    name: Similars
    url: http://community.topcoder.com/stat?c=problem_statement&pm=13714&rd=16432
date: 2015/04/12
tags: [TopCoder, TCO, bit]

---

### 概要

自然数 $x, y$ に対し, $S(x, y)$ を, $x$ の 十進表記にも $y$ の 十進表記にも含まれる数字の種類数とする.
$L, R$ が与えられるので, $\max \set{ S(a, b) \setmid L \le a < b \le R }$ を求めよ.

#### 制約

$1 \le L < R \le 10^5$.

### 解法

数字 $\set{0, \dots, 9}$ の各部分集合に対し, それらを桁として持つ $L$ 以上 $R$ 以下の数の個数を数えておく.

あとは, $\set{0, \dots, 9}$ の(異なるとは限らない)部分集合を二つとり, それらを桁として持つのが合計 $2$ つ以上あるなら, 共通する桁の数を候補とすればよい.

### ソースコード

~~~ cpp
template<typename T> string to_s(T t){ //{{{
    stringstream ss;
    ss << t;
    return ss.str();
} //}}}
int Similars::maxsim( int L, int R ){
    vector<ll> v(1<<10);
    for(int i = L; i <= R; ++i){
        string s = to_s(i);
        int mask = 0;
        for(aur c : s) mask |= 1<<(c - '0');
        ++v[mask];
    }
    int res = 0;
    // 本番では書かなかったが.
    //   rep(A, 1<<10) if(v[A] >= 2) chmax(res, __builtin_popcount(A));
    // をちゃんと書いたほうがよかった.
    rep(A, 1<<10) rep(B, A) if(v[A] + v[B] >= 2)
        chmax(res, __builtin_popcount(A & B));
    return res;
}
~~~

