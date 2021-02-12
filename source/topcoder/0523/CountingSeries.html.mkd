---

problem:
    round: SRM523
    level: [Div1 Easy 250]
    rd: "14548"
    pm: "10957"
    name: CountingSeries
    url: http://community.topcoder.com/stat?c=problem_statement&pm=10957&rd=14548
date: 2015/10/19
tags: [TopCoder]

---

### 概要

等差数列 $ a + b i $ と等比数列 $ c d^i $ がある. ($ i \ge 0 $)

$ 1 $ 以上 $ \mathit{upperBound} $ 以下で, 上の等差数列と等比数列の少なくとも一方に含まれるような整数の数を求めよ.

#### 制約

$ 1 \le a, b, c, \mathit{upperBound} \le 10^{12},\ 1 \le d \le 10^5 $.

### 解法

$ d = 1 $ や $ a > \mathit{upperBound} $ なケースに注意.

基本的には, 等差数列に入るものを $ O(1) $ で求めたあと, 等比数列の方をイテレートし, 等差数列に入らないものを数えればよい.

### ソースコード

~~~ cpp
// 232.06 pts

long long CountingSeries::countThem( long long a, long long b, long long c, long long d, long long upperBound ){
    if(upperBound < a){
        if(d == 1) return c <= upperBound;
        long long t = c;
        long long res = 0;
        while(t <= upperBound){
            ++res;
            t *= d;
        }
        return res;
    }
    long long res = 0;
    res += (upperBound - a + b) / b;

    if(d == 1){
        if(a <= c and (c - a) % b == 0) return res;
        return res + (c <= upperBound);
    }
    long long t = c;
    while(t <= upperBound){
        if(t < a or (t - a) % b != 0) ++res;
        t *= d;
    }

    return res;
}

// vim:set foldmethod=marker commentstring=//%s:
~~~

