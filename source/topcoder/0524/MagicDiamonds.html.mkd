---

problem:
    round: SRM524
    level: [Div1 Easy 250]
    rd: "14549"
    pm: "11607"
    name: MagicDiamonds
    url: http://community.topcoder.com/stat?c=problem_statement&pm=11607&rd=14549
date: 2015/10/27
tags: [TopCoder]

---

### 概要

$ n $ を素数でない正整数の和として表したい.
必要な数の個数の最小値を答えよ.

#### 制約

$ 1 \le n \le 10^{12} $.

### 解法

$ n $ が素数でないなら $ 1 $.
素数なら, $ n - 1 $ と $ 1 $ で分けたくなる.

$ n $ と $ n - 1 $ が両方素数である, $ 3 $ がコーナーケース.

### ソースコード

~~~ cpp
// 245.71 pts

long long MagicDiamonds::minimalTransfer( long long n ){
    if(!isPrime(n)) return 1;
    if(n == 3) return 3;
    return 2;
}

// vim:set foldmethod=marker commentstring=//%s:
~~~

