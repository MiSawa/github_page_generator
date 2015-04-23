---

problem:
    round: SRM652
    level: [Div1 Easy 250]
    rd: 16316
    pm: 13229
    name: ThePermutationGame
    url: http://community.topcoder.com/stat?c=problem_statement&pm=13229&rd=16316
date: 2015/03/10
tags: [TopCoder, 順列, 置換, 数論]

---

### 概要

$n$ が与えられる.
$\{1, \dots, n\}$ の任意の置換 $f$ に対し,
$f^k(1) = 1$ となるような $k \ge 1$ のうち, 最小のものを求め, $\mathrm{mod}\ 1000000007$ で答えよ.

#### 制約

$N \le 10^5$.

### 解法

要するに, $\mathfrak{S}_n$ の exponent を求めろという問題.
$\operatorname{LCM}(1, \dots, n)$
を $\mathrm{mod} 1000000007$ で求めればよいが,
各素数 $p \le n$ について, $p$ の $1, \dots, n$ に含まれる冪の最大値を求めればよい.

### ソースコード

~~~ cpp
vector<int> sieve(int mx){ //{{{
    vector<int> f(mx+1);
    rep(i, f.size()) f[i] = i;
    vector<int> res;
    for(int i = 2; i <= mx; ++i){
        if(f[i] == i){
            res.eb(i);
            for(int j = i + i; j <= mx; j += i){
                f[j] = 0;
            }
        }
    }
    return res;
} //}}}

const ll mod = 1000 * 1000 * 1000 + 7;
int ThePermutationGame::findMin( int N ){
    ll res = 1;
    for(auto &p : sieve(N+10)){
        ll q = 1;
        while(p * q <= N) q *= p;
        res = res * q % mod;
    }
    return res;
}
~~~

