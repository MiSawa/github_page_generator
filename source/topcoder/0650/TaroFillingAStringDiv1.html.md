---

problem:
    round: SRM650
    level: [Div1 Easy]
    rd: 16314
    pm: 13669
    name: TaroFillingAStringDiv1
    url: http://community.topcoder.com/stat?c=problem_statement&pm=13669&rd=16314
date: 2015/03/03
tags: [TopCoder, 数え上げ, 算数]

---

### 概要

$N$ 文字の, 各文字が ``A`` か ``B`` からなる文字列で,
同じ文字が $2$ 文字連続している箇所をなるべく少なくしたい.
``position[i]`` 番目の文字が ``value[i]`` であるという制約の下,
連続している箇所が最も少なくなるような文字列のパターン数を $\mathrm{mod}\ 1000000007$ で答えよ.

#### 制約

$N \le 10^9$, $\mathrm{position.size()} \le 50$.

### 解法

指定を ``position`` でソート.
隣合う指定の差の偶奇と, 指定された文字の一致不一致を見ると,
そこを連続する箇所 $0$ で埋められるか, $1$ 必要かが解る.
$0$ の時は $1$ パターン, そうでない時は, どこで連続させるかで, ``position`` の差くらい.

### ソースコード

~~~ cpp
int TaroFillingAStringDiv1::getNumber( int N, vector <int> position, string value ){
    const long long mod = 1000000007;
    vector<pair<int, char>> in;
    rep(i, size(position)) in.emplace_back(position[i], value[i]);
    sort(begin(in), end(in));
    long long res = 1;
    rep(i, size(in)-1){
        if(((in[i+1].fst-in[i].fst)%2 == 0) ^ (in[i+1].snd == in[i].snd)){
            res *= in[i+1].fst-in[i].fst;
            res %= mod;
        }
    }
    return res;
}
~~~

