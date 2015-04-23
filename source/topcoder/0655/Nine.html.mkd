---

problem:
    round: SRM655
    level: [Div1 Medium 500]
    rd: 16415
    pm: 13712
    name: Nine
    url: http://community.topcoder.com/stat?c=problem_statement&pm=13712&rd=16415
date: 2015/03/10
tags: [TopCoder, DP, 桁]

---

### 概要

`d.size()` 桁の数(leading zero を許す)がある.

数を十進数列と思ったときの部分列を, $N$ 回持ってきて,
もう一度十進の数と見たとき, 毎回 $\equiv 0 \pmod 9$ だった.

各回で何桁目を選んだかが与えられるので, 元の数としてありうるもののパターン数を数えよ.


#### 制約

$1 \le N \le 5$, $1 \le d{\rm .size()} \le 5000$.

### 解法

基本的には, `dp[i番目][0回目の和 mod 9][1回目の和 mod 9]...` を更新したいが,
これだと $d{\rm .size()} * 9^n$ とか状態数があって死ぬ.

$N \le 5$ に注目すると, 各回で選んだか選ばなかったかの種類は $2^N \le 32$ だから, こいつで状態数をまとめてやる.

すると,

`dp[A : 選び方の種類の2進エンコード][ v : 各回の mod 9] = sum{ dp[A-1][v のうち A で選んでるやつに d を足したの] * (A の個数で和が mod 9 になるような選び方) | d}`

的なので更新出来る.

但し, 最後の, `(A の個数で和が mod 9 になるような選び方)` は別に DP して計算する.

### ソースコード

~~~ cpp
constexpr ll mod = 1000 * 1000 * 1000 + 7;

vector<vector<int>> memo2;
ll pattern(int cnt, int m){//{{{
    if(cnt == 0) return m == 0;
    int &res = memo2[cnt][m];
    if(res != -1) return res;
    res = 0;
    rep(i, 10) (res += pattern(cnt-1, (m + 9 - i) % 9)) %= mod;
    //tr << "pattern " << cnt << ", " << m << ": " << res << endl;
    return res;
}//}}}

inline ll powMod(ll b, ll e, ll m){//{{{
    ll res = 1;
    for(; e; e >>= 1, b = b * b % m) if(e&1) res = res * b % m;
    return res;
}//}}}

unordered_map<int, int> memo;
vector<int> cnt;
int n;
int dfs(int A, int &mods){
    if(A == 0){
        if(mods != 0) return 0;
        return powMod(10, cnt[A], mod);
    }
    if(cnt[A] == 0) return dfs(A-1, mods);
    if(memo.count(mods * 32 + A)){ return memo[mods * 32 + A]; }
    int &res = memo[mods * 32 + A];
    rep(t, 9){
        res += (dfs(A-1, mods) * pattern(cnt[A], t)) % mod;
        res %= mod;
        int p = 1;
        rep(i, n){
            if(A>>i&1){
                mods += p;
                if((mods / p) % 10 == 9) mods -= p * 9;
            }
            p *= 10;
        }
    }
    return res;
}

int Nine::count( int N, vector <int> _d ){
    cnt.assign(1<<N, 0);
    memo.clear();
    memo2.assign(_d.size() + 1, vector<int>(9, -1));
    n = N;
    for(auto &x : _d) ++cnt[x];
    int mods = 0;
    return dfs((1<<N)-1, mods);
}
~~~

