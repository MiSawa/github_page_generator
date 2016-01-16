---

problem:
    round: SRM591
    level: [Div1 Easy 275]
    rd: "15703"
    pm: "12746"
    name: TheTree
    url: http://community.topcoder.com/stat?c=problem_statement&pm=12746&rd=15703
date: 2015/10/02
tags: [TopCoder]

---

### 概要

木の情報として, 根から深さ $ i $ の頂点が $ c_i $ 個あるという情報が与えられる.

その情報と整合性が取れる木のうち, もっとも直径が長いものの直径を返せ.


#### 制約

$ 1 \le n \le 50,\ 1 \le c_i \le 1000 $.

### 解法

基本的には二つの path を伸ばしたいが, $ c_i = 1 $ の所で途切れる.

与えられる $ c $ は根が入ってないので, 根を追加すると書くのが楽?


### ソースコード

~~~ cpp
// 246.06 pts

int TheTree::maximumDiameter( vector <int> cnt ){
    reverse(all(cnt));
    cnt.emplace_back(1);
    reverse(all(cnt));

    const int n = size(cnt);
    int res = 0;
    rep(i, n) if(cnt[i] == 1){
        int j = i+1;
        while(j < n and cnt[j] > 1) ++j;
        chmax(res, (j-i) + (n-i) - 2);
    }
    return res;
}
~~~

