---

problem:
    round: SRM580
    level: [Div1 Med 600]
    rd: "15500"
    pm: "12576"
    name: ShoutterDiv1
    url: https://community.topcoder.com/stat?c=problem_statement&pm=12576&rd=15500
date: 2015/11/05
tags: [TopCoder, 区間スケジューリング, DP]

---

### 概要

$ n $ 人の人間がいる.

$ i $ 人目は区間 $ [s_i, t_i] $ に部室におり, 同時に部室に居た瞬間のあるペアは SNS で友達になっている.
また, 同時に部屋に居た瞬間のないペアは, その SNS で友達でない.

$ n $ 人がそれぞれ自己紹介を SNS に書き込んだ.
書き込んだ人の友達にそのポストが見える.

また, "リポスト" という, "自分が見えるポストのうちの一つを, 自分の友達全員に見えるように投稿しなおす" 行為が出来る.
全ての自己紹介が全ての人に見えるようになるために必要なリポストの数の最小値を求めよ.

#### 制約

$ 1 \le N \le 2500 $.

### 解法

区間グラフ的なやつであることは重要.

人 $ i $ の自己紹介を全員に広めるためには, "最初に部室を出た人間" からも "最後に部室に入った人間" からも見えるようにすればよい.

基本的には, 区間スケジューリングのように, "今見えている人の中で最後に部室を出る人間" がリポストするようにする.
但し,

- ポストした本人の区間はタダでゲット出来る.
- 最初の人をちゃんと選ぶ.
- 終了条件をちゃんとする.

に注意.


変な入力形式で $ n \le 50^2 $ だったが, $ 50^2 = 250 $ とアホな勘違いをして, 計算量がひどいのを提出し, 採点後に再提出した.


### ソースコード

~~~ cpp
// 180.00 pts

int ShoutterDiv1::count( vector <string> s1000, vector <string> s100, vector <string> s10, vector <string> s1, vector <string> t1000, vector <string> t100, vector <string> t10, vector <string> t1 ){
    vector<int> s = mk(s1000, s100, s10, s1);
    vector<int> t = mk(t1000, t100, t10, t1);
    const int n = size(s);

    vector<int> succ(11000, -1);
    repsz(time, succ) rep(u, n) if(s[u] <= time) chmax(succ[time], t[u]);

    int min_t = 1<<20, max_t = -1;
    int min_idx = -1, max_idx = -1;
    {//{{{
        rep(i, n) chmin(min_t, t[i]);
        rep(i, n) if(s[i] <= min_t) if(min_idx == -1 or t[min_idx] < t[i]) min_idx = i;
        rep(i, n) chmax(max_t, s[i]);
        rep(i, n) if(t[i] >= max_t) if(max_idx == -1 or s[max_idx] > s[i]) max_idx = i;
    }//}}}

    int res = 0;
    rep(u, n){
        int now = 1, time = t[min_idx];
        if(s[u] <= min_t){
            now = 0; time = t[u];
        }
        while(true){
            if(s[u] <= time) chmax(time, t[u]);
            if(max_t <= time) break;
            ++now;
            if(succ[time] <= time) return -1;
            time = succ[time];
        }
        res += now;
    }
    return res;
}

// vim:set foldmethod=marker commentstring=//%s:
~~~

