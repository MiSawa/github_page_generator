---

problem:
    round: SRM602
    level: [Div1 Easy 250]
    rd: "15820"
    pm: "12924"
    name: TypoCoderDiv1
    url: http://community.topcoder.com/stat?c=problem_statement&pm=12924&rd=15820
date: 2015/09/30
tags: [TopCoder, DP]

---

### 概要

レーティング付きのコンテストがある.

レーティング $ 2200 $ 以上は赤, $ 2200 $ 未満が黄色.

各 $ i $ 回目のコンテストに対して, 今のレーティングが $ X $ の時,  レーティングは $ X+D_i $ と $ \max(0, X - D_i) $ のどちらにするか(本気を出すか出さないか) を選択出来る.

但し, $ 2 $ 回以上連続して赤に留まりたくはない.
最も色の変動が激しくなるようにすると, 何回変動させられるか.


#### 制約

$ 1 \le n \le 50,\ 0 \le D_i \le 10^9 $.

### 解法

$ \mathit{dp}[i, x] $ を, 「$ i-1 $ 回目まででレーティングが $ x $ であるようにしたときの色の変化の最大数. 但し, $ 0 \le x < 2000 $」とする.

赤になった次はかならず $ -D_{i+1} $ を選んで黄色にしなきゃいけないので, $ \mathit{dp}[i] $ から $ \mathit{dp}[i+2] $ を更新したりすれば, なんとかなる.

### ソースコード

~~~ cpp
// 182.18
int TypoCoderDiv1::getmax( vector <int> D_, int X ){
    vector<long long> D(begin(D_), end(D_));
    int res = int{};
    const int line = 2200;
    const int n = size(D_);
    auto dp = vector<vector<int>>(n+1, vector<int>(line, -(1<<25)));
    dp[0][X] = 0;
    rep(i, n){
        rep(r, line){
            chmax(dp[i+1][max<ll>(r - D[i], 0)], dp[i][r]);
            if(r + D[i] < line){
                chmax(dp[i+1][r+D[i]], dp[i][r]);
            }else{
                if(i < n-1){
                    long long nr = max<ll>(0, r + D[i] - D[i+1]);
                    if(nr < line) chmax(dp[i+2][nr], dp[i][r] + 2);
                }else{
                    chmax(res, dp[i][r] + 1);
                }
            }
        }
    }
    rep(r, line) chmax(res, dp[n][r]);
    return res;

}
~~~

