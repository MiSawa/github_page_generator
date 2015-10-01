---

problem:
    round: SRM592
    level: [Div1 Easy 300]
    rd: "15704"
    pm: "12758"
    name: LittleElephantAndBalls
    url: http://community.topcoder.com/stat?c=problem_statement&pm=12758&rd=15704
date: 2015/10/02
tags: [TopCoder]

---

### 概要

'R', 'G', 'B' からなる列が与えられる.
この順に, その色のボールをテーブルに出していく.

テーブルには一列に置くが, 置いた時に,

- その場所より左に置いたボールの色の種類数
- その場所より右に置いたボールの色の種類数

の和を点数として貰える.

点数を最大化せよ.


#### 制約

$ 1 \le n \le 50 $.

### 解法

最初, 出す順番は任意でいいと誤読して, 時間を浪費してしまった...

出すときは, 常に "真ん中" に近い位置に置くことにして,
右よりか左よりかを決める.

すると, 左の種類を増やせるなら左の方, そうでないなら右の方に置けばいい.


### ソースコード

~~~ cpp
// 198.33 pts
int LittleElephantAndBalls::getNumber( string S ){
    set<char> left, right;

    int res = 0;
    for(auto c : S){
        res += size(left) + size(right);
        if(left.count(c)) right.emplace(c);
        else left.emplace(c);
    }
    return res;
}
~~~

