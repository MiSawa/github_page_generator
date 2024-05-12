---

problem:
    round: SRM592
    level: [Div1 Med 500]
    rd: "15704"
    pm: "12735"
    name: LittleElephantAndPermutationDiv1
    url: http://community.topcoder.com/stat?c=problem_statement&pm=12735&rd=15704
date: 2015/10/02
tags: [TopCoder, 順列, DP]

---

### 概要

$ (1, \dots, n) $ の($ 1 $-origin の)順列のペア $ \sigma, \pi $ に対し,
`$$
f(\sigma, \pi) = \sum_{i = 1}^{n} \max(\sigma(i), \pi(i))
$$`
とする.

$ f(\sigma, \pi) \ge K $ となる順列のペアの個数を答えよ.


#### 制約

$ 1 \le n \le 50,\ 1 \le K \le 2500 $.

### 解法

解らなくて editorial 見た.

$ n $ から順に, 同時に $ \sigma, \pi $ 内の場所を決めていく.

既に相方に置いてある場所は, ($ n $ から順に置いているので), $ f $ は何も得られない.

最初は

|  n  |     |     |     |
|:---:|:---:|:---:|:---:|
|     |  n  |     |     |

($ 2n $ 得られる)とか

|  n  |     |     |     |
|:---:|:---:|:---:|:---:|
|  n  |     |     |     |

($ n $ 得られる)みたいなのがある.

その次は,

|  ?  | n-1 |     |     |
|:---:|:---:|:---:|:---:|
|     |  ?  | n-1 |     |

とか, いくつか置き方がある.

とにもかくにも, DP を ($i$ を置く, ペアで埋まっている個数, 残り必要な f の値) ですればいい.

### ソースコード

~~~ cpp
// 189.75 pts
constexpr int mod = 1E9 + 7;

// i を入れる.
// 両方空いている + 片方ずつ空いている = i.
// 両方空いてるのが c マス.

array<array<array<int, 2600>, 60>, 60> dp;
array<int, 60> factorial_double;
int dfs(int i, long long c, int need){
    if(need <= 0) return factorial_double[i+1];
    if(i == 0) return need <= 0;
    if(dp[i][c][need] != -1) return dp[i][c][need];
    const long long x = i+1 - c;
    long long res = 0;
    // 両方 "片方空いている" に入れる
    if(x) (res += x * x * dfs(i-1, c, need)) %= mod;
    // 一方だけ "片方空いている" に入れる
    if(x and c) (res += x * c * 2 * dfs(i-1, c-1, need - i)) %= mod;
    // 両方 "両方空いている" に入れる
    if(c)      (res += c * dfs(i-1, c-1, need - i)) %= mod;
    if(c >= 2) (res += c * (c - 1) * dfs(i-1, c-2, need - i - i)) %= mod;
    return dp[i][c][need] = res;
}
int LittleElephantAndPermutationDiv1::getNumber( int n, int K ){
    K -= n;
    factorial_double[0] = 1;
    rep(i, 55) factorial_double[i+1] = (long long) factorial_double[i] * (i+1) * (i+1) % mod;
    rep(i, 60) rep(j, 60) rep(k, 2600) dp[i][j][k] = -1;
    return dfs(n-1, n, K);
}
~~~

