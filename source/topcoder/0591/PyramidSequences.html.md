---

problem:
    round: SRM591
    level: [Div1 Med 500]
    rd: "15703"
    pm: "12619"
    name: PyramidSequences
    url: http://community.topcoder.com/stat?c=problem_statement&pm=12619&rd=15703
date: 2015/10/02
tags: [TopCoder, 整数]

---

### 概要

$ X $ に対し, 高さ $ X $ のピラミッド列とは,

$ (1, 2, \dots, X-1, X, X-1, \dots, 2) $

の $ 2X - 2 $ 個の無限繰り返しを指す.

高さ $ N $ と $ M $ のピラミッド列 $ A $ と $ B $ が, (左端の $ 1 $ が並ぶようにして) おいてある.

$ (A_i, B_i) $ のペアとしてありうる組合せを数え上げよ.


#### 制約

$ 1 \le N, M \le 10^9$.

### 解法

ここより [editorial](http://apps.topcoder.com/wiki/display/tc/SRM+591) を見たほうがよい.
まだ読んでないけど, ここよりずっと綺麗っぽい.


$ 1, \dots, N, \dots, 1 $ じゃなくて $ 0, \dots, N-1, \dots, 0 $ にしとく.

$ \gcd(2N-2, 2M-2) $ まで取ると, 左右対称なので,
とりあえず, $ A $ が増加している部分だけ考えればよい.

それぞれの周期を $ a = 2N - 2,\ b = 2 M - 2 $ とする.

$ t $ が $ A $ の増加部分で現れるのは, $ t, t + a, t + 2a, \dots $ 番目.

$ t $ が現れる index を $ \bmod b $ した値は,
`$$
t + ka \equiv i \mod b \\
\Leftrightarrow
i - t \equiv k a \mod b
$$`
だから, $ g = \gcd(a, b) $ として, $ i - t \equiv 0 \mod g $ になる.

逆にこのときは, $ (x - t) / g \equiv k (a / g) \mod (b / g) $ で, $ a/g $ の $ \bmod (b/g) $ の逆元を両辺にかけて $ k $ を復元出来るから, 実際にそこにもってこれる.


一方, $ B_i $ は, $ i \bmod b < b/2 $ か否かに従って $ i \bmod b $ か $ -i \bmod b $.

よって, $ (t, x) $ が現れうるのは, $ t \equiv \pm x \mod g $ のとき.

よって,

~~~ cpp
rep(i, N) rep(j, M) if((i-j)%g == 0 or (i+j)%g == 0)
    ++res;
~~~

というのは正しい答えを返す.

あとは, $ i $ を固定した時に $ i \equiv j \mod g $ の解と $ i \equiv -j \mod g $ の解が一致するか否かで場合分けしつつ, ループを一重に落とせる.

~~~ cpp
rep(i, N){
    if((i*2) % g == 0){
        res += (M-1-(i%g))/g+1;
    }else{
        res += (M - 1 - (i%g)) / g + 1 + (M - 1 - ((g-i%g)%g)) / g + 1;
    }
}
~~~

これは明らかに $ \bmod g $ で周期的な挙動をするから, $ g $ が小さい時は周期でスキップすればよい.

一方, $ g $ が大きいときは, `(M - 1 - (i%g)) / g` などの変わり目がすぐにわかることを利用して, `+= g` のタイプのスキップをすると楽.

(端っこが面倒だからこういうスキップをしているだけで, 算数をすると, いちおう $ O(1) $ には出来る)

あと, なんか $N = M$ のときコーナーケースになっていた.
どこの議論か追う気力がない.


### ソースコード

~~~ cpp
/*
   A 側の増加部分だけ考えればいい. (逆から見ても同じペアになるはずだから).
   a = 2N-2, b = 2M-2 とする.
   0 1 2 ... N-1 N-2 ... 0 と思う.

   t が現れるのは,
      t, t+a, ..., t + ab
   番目.
   i 番目の B の値は,
      i % b < b/2 なら  i % b
      そうでないなら  (-i) % b

    (t + ka) == x mod b
   <=>
    x - t == ka mod b
   だから,
     x - t : gcd(a, b) の倍数.
   g = gcd(a, b) とすると,
   要するに, (t, x) となりうる x は,
     g | (x - t)
      or
     g | (x + t)
   なやつら.

   g | (x-t) かつ g | (x+t) となるには, g | 2t が必要で,
   この時は
    x == t == -t mod g
   よって, x == (t%g) な 0 <= x < M の個数 = (M - 1 - (t%g))/g + 1

   そうでないとき,
     x == t mod g
     or
     x == -t mod g
    (M - 1 - (t%g)) / g + 1
    +
    (M - 1 - ((g-t%g)%g)) / g + 1
*/

// 150.0 pts

long long PyramidSequences::distinctPairs( int N_, int M_ ){//{{{
    long long N = N_, M = M_;
    if(M == N) return M;
    int a = 2 * N - 2, b = 2 * M - 2;
    int g = __gcd(a, b);
    long long res = 0;
    // rep(i, N) rep(j, M) if((i-j)%g == 0 or (i+j)%g == 0)//{{{
    //     ++res;
    // rep(i, N){
    //     int cnt = 0;
    //     rep(j, M) if((i-j)%g == 0 or (i+j)%g == 0)
    //         ++cnt;
    //     cout << i << ": " << cnt << endl;
    //     if((i*2)%g == 0){
    //         cout << (M-1-(i%g))/g+1 << endl;
    //     }else{
    //         cout << (M - 1 - (i%g)) / g + 1 + (M - 1 - ((g-i%g)%g)) / g + 1 << endl;
    //     }
    //     // cout << (i*2 % g == 0 ? (a-1)/g : (a-1)/g*2) << endl;
    // }//}}}
    if(g > 100000){
        res -= ((N - 1) / g + 1) * ((M - 1) / g + 1);
        res -= ((N - 1 - g/2) / g + 1) * ((M - 1 - g/2) / g + 1);

        // M-1-(i%g) == 0 <=> i == M-1 mod g
        // [0, (M-1)%g+1) は (M-1) / g + 1
        // [(M-1)%g+1, g) は (M-1) / g.
        // rep(i, N){
        //     res += (M - 1 - (i%g)) / g + 1;
        // }
        for(int i = 0; i < N; i += g){
            int l = i, r = min<int>(N, i + (M-1)%g + 1);
            res += (r - l) * ((M-1) / g + 1);
            l = r; r = min<int>(N, i + g);
            res += (r - l) * ((M-1) / g);
        }

        // M - 1 + i == 0 <=> i == 1-M mod g
        // [0, ((1-M)%g+g)%g+1) は (M-1)/g+1
        // [((1-M)%g+g)%g+1, g) は (M-1)/g
        // rep(i, N){
        //     res += (M - 1 - ((g-i%g)%g)) / g + 1;
        // }
        for(int i = 0; i < N; i += g){
            int l = i, r = min<int>(N, i + ((1-M)%g+g)%g+1);
            res += (r - l) * ((M-1) / g+1);
            l = r; r = min<int>(N, i + g);
            res += (r - l) * ((M-1) / g);
        }

        // rep(i, N){
        //     if((i*2) % g == 0){
        //         res += (M-1-(i%g))/g+1;
        //     }else{
        //         res += (M - 1 - (i%g)) / g + 1;
        //         res += (M - 1 - ((g-i%g)%g)) / g + 1;
        //     }
        // }
    }else{
        rep(i, N){
            if(i == g*2){
                long long cnt = (N - i) / (g*2);
                while(cnt * g * 2 + i >= N) --cnt;
                i += cnt * g * 2;
                res += cnt * res;
            }
            if((i*2) % g == 0){
                res += (M-1-(i%g))/g+1;
            }else{
                res += (M - 1 - (i%g)) / g + 1 + (M - 1 - ((g-i%g)%g)) / g + 1;
            }
        }
    }
    return res;
}//}}}
~~~

