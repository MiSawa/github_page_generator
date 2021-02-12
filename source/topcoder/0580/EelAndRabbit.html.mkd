---

problem:
    round: SRM580
    level: [Div1 Easy 250]
    rd: "15500"
    pm: "12575"
    name: EelAndRabbit
    url: https://community.topcoder.com/stat?c=problem_statement&pm=12575&rd=15500
date: 2015/11/05
tags: [TopCoder]

---

### 概要

川に $ n $ 匹のうなぎがおり, $ i $ 匹目は原点から上流に $ t_i $ の所に頭があり, 更に $ l_i $ だけ上流に尾がある.

各うなぎは, 単位時間に $ 1 $ だけ下流に動く.

うさぎは原点で漁をしている.
高々二回だけ, 次の動作をすることが出来る.

- 今原点に体(両端を含む)があるうなぎを全て獲る

最大何匹の鰻を獲れるか.

#### 制約

$ 1 \le N \le 50 $.

### 解法

一回どのタイミングで獲るかを決め打ちして, もう一回を全探索.

### ソースコード

~~~ cpp
// 236.25 pts

int EelAndRabbit::getmax( vector <int> l, vector <int> t ){
    const int n = size(l);

    vector<int> candidates;
    rep(i, n) candidates.emplace_back(t[i]);
    rep(i, n) candidates.emplace_back(t[i] + l[i]);
    int res = 0;
    for(auto x : candidates){
        vector<pair<int, int>> event;
        int now = 0;
        rep(i, n){
            if(t[i] <= x and x <= t[i] + l[i]){ ++now; continue; }
            event.emplace_back(t[i], -1);
            event.emplace_back(t[i] + l[i], +1);
        }
        sort(all(event));
        int mx = 0, tmp = 0;
        for(auto e : event){
            tmp -= e.second;
            chmax(mx, tmp);
        }
        chmax(res, mx + now);
    }

    return res;
}
~~~

