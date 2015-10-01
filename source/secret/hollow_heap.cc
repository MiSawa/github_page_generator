#include <bits/stdc++.h>
#define all(x) begin(x),end(x)
#define rall(x) (x).rbegin(),(x).rend()
#define REP(i,b,n) for(int i=(int)(b);i<(int)(n);++i)
#define rep(i,n) REP(i,0,n)
#define repsz(i,v) rep(i,(v).size())
#define aur auto&
#define bit(n) (1LL<<(n))
#define eb emplace_back
#define mt make_tuple
#define fst first
#define snd second
using namespace std;
typedef long long ll;
//#define int long long
template<class C>int size(const C &c){ return c.size(); }
template<class T>bool chmin(T&a,const T&b){if(a<=b)return false;a=b;return true;}
template<class T>bool chmax(T&a,const T&b){if(a>=b)return false;a=b;return true;}

template<typename Priority>
struct HollowHeap{//{{{
    static constexpr int max_rank = 40;

    struct Node{//{{{
        int id, rank; // rank = 0 if this node is hollow node.
        vector<Node *> ch;
        Node(const int &id) :id(id), rank(1){}
    };//}}}
    using Ref = Node *;

    Ref root;
    vector<Ref> nodes;
    vector<Priority> priority;

    HollowHeap(const int &n) : root(nullptr), nodes(n, nullptr), priority(n) {}

    bool empty() const { return root == nullptr; }
    int top() const { return root->id; }

    void chmin(const int &id, const Priority &p){//{{{
        if(nodes[id] == nullptr){
            priority[id] = p;
            root = meld(root, nodes[id] = alloc_node(id));
        }else if(p < priority[id]){
            priority[id] = p;
            nodes[id] = decrease_key(nodes[id]);
        }
    }//}}}
    void pop(){//{{{
        int id = root->id;
        erase(root);
        nodes[id] = nullptr;
    }//}}}
    void pop(const int &id){//{{{
        erase(nodes[id]);
        nodes[id] = nullptr;
    }//}}}

    private:
    Ref alloc_node(const int &id){//{{{
        Ref res = new Node(id);
        return res;
    }//}}}
    void delete_node(Ref u){ delete u; }

    Ref meld(const Ref &u, const Ref &v){//{{{
        if(u == nullptr or v == nullptr) return u ? u : v;
        return link(u, v);
    }//}}}
    Ref link(Ref u, Ref v){//{{{
        if(priority[u->id] < priority[v->id]) swap(u, v);
        v->ch.emplace_back(u);
        return v;
    }//}}}
    Ref ranked_link(const Ref &u, const Ref &v){//{{{
        Ref res = link(u, v);
        ++res->rank;
        return res;
    }//}}}
    void erase(Ref u){//{{{
        if(u != root) return;
        root = nullptr;
        static array<vector<Ref>, max_rank> roots;
        roots[0].emplace_back(u);
        for(int i = 0; i < max_rank; ++i){
            while(!roots[i].empty()){
                Ref u = roots[i].back(); roots[i].pop_back();
                if(i == 0){
                    for(auto &v : u->ch) roots[v->rank].emplace_back(v);
                }else if(roots[i].empty()){
                    root = meld(root, u);
                }else{
                    roots[i+1].emplace_back(ranked_link(u, roots[i].back()));
                    roots[i].pop_back();
                }
            }
        }
    }//}}}
    Ref decrease_key(Ref u){//{{{
        if(u == root) return u;
        Ref v = alloc_node(u->id);
        v->rank = std::max(u->rank - 2, 1); u->rank = 0;
        if(link(v, root) == root)   v->ch.emplace_back(u);
        else                        root = v;
        return v;
    }//}}}
};//}}}


bool solve(){
    
    return true;
}
signed main(){
    cin.tie(nullptr);
    ios_base::sync_with_stdio(false);
    cout << std::fixed << std::setprecision(10);
    solve();
    return 0;
}
// vim:set foldmethod=marker commentstring=//%s:
