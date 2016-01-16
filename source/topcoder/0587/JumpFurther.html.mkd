---

problem:
    round: SRM587
    level: [Div1 Easy 250]
    rd: "15699"
    pm: "12300"
    name: JumpFurther
    url: http://community.topcoder.com/stat?c=problem_statement&pm=12300&rd=15699
date: 2015/10/07
tags: [TopCoder]

---

### 概要

階段を登る.
$ i $ 秒目には, $ i $ 段(一秒で, 一歩で)上に上がるか, 何もしないことが選べる.

$ \mathit{badStep} $ 段目だけ, 壊れていて, そこを踏むことはできない.

$ N $ 秒で何段上がれるか.

#### 制約

$ 1 \le N \le 2000 $

### 解法

$ \mathit{badStep} $ が三角数でなければ, 何も気にせず全部登れる.

三角数なときは, $ 1 $ 秒目だけやめればいい.

下のコードでは, なんでこんな制約なんだろうなぁと思いながら, 一回やめるのを全部試している.

### ソースコード

~~~ cpp
// 236.93 pts

int JumpFurther::furthest( int N, int badStep ){
    {
        int now = 0, ok = true;
        for(int i = 1; i <= N; ++i){
            now += i;
            if(now == badStep) ok = false;
        }
        if(ok) return now;
    }
    int res = 0;
    for(int skip = 1; skip <= N; ++skip){
        int now = 0, ok = true;
        for(int i = 1; i <= N; ++i) if(i != skip){
            now += i;
            if(now == badStep) ok = false;
        }
        if(ok) chmax(res, now);
    }
    return res;
}
~~~

