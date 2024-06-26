---

problem:
    contest: ARC038
    id: A
    name: カードと兄妹
    url: http://arc038.contest.atcoder.jp/tasks/arc038_a
date: 2015/05/03
tags: [AtCoder]

---

### 概要

二人で交互にターンが回ってくるゲームをする.

まず, それぞれ整数 `$A_1, \dots, A_n$` が書いてあるカードが与えられる.

各ターンに, 一つの数を選び, それを場から取り去る事が出来る.

先手も後手も, 自分が取り去ったカードに書いてあった数の合計を最大化したい.

両方最適にプレイした時, 先手の取れる合計を求めよ.

#### 制約

`$1 \le n \le 10^3$`, `$1 \le A_i \le 10^3$`.


### 解法

でかい方から取るの一択.
先手が手に入れるのは, でかい順でソートした時の偶数番目 (`$0$`-origin).


### ソースコード

~~~ cpp
bool solve(){
    int n; cin >> n;
    vector<int> a(n); for(auto &x : a) cin >> x;
    sort(rall(a));
    int res = 0;
    rep(i, n) if(i%2 == 0) res += a[i];
    cout << res << endl;
    return true;
}
~~~

