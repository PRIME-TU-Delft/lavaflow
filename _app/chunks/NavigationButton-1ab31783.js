import{S as G,i as H,s as J,e as b,w as p,c as v,a as k,x as B,d,b as g,g as V,y as I,q as u,o as _,B as w,E as A,F as L,k as y,m as C,J as h,n as N,p as q,G as R,H as j,I as z}from"./index-cd071420.js";import{B as K,I as F,m as M,a as O}from"./mdi-ff5aee52.js";function D(i){let e,l;return e=new F({props:{path:M,color:"var(--text-color)"}}),{c(){p(e.$$.fragment)},l(t){B(e.$$.fragment,t)},m(t,s){I(e,t,s),l=!0},p:A,i(t){l||(u(e.$$.fragment,t),l=!0)},o(t){_(e.$$.fragment,t),l=!1},d(t){w(e,t)}}}function S(i){let e,l;return e=new F({props:{path:O,color:"var(--text-color)"}}),{c(){p(e.$$.fragment)},l(t){B(e.$$.fragment,t)},m(t,s){I(e,t,s),l=!0},p:A,i(t){l||(u(e.$$.fragment,t),l=!0)},o(t){_(e.$$.fragment,t),l=!1},d(t){w(e,t)}}}function P(i){let e,l,t,s,f,n=i[4]&&D();const m=i[5].default,r=L(m,i,i[6],null);let o=!i[4]&&S();return{c(){e=b("div"),n&&n.c(),l=y(),t=b("div"),r&&r.c(),s=y(),o&&o.c(),this.h()},l(a){e=v(a,"DIV",{class:!0});var c=k(e);n&&n.l(c),l=C(c),t=v(c,"DIV",{class:!0});var E=k(t);r&&r.l(E),E.forEach(d),s=C(c),o&&o.l(c),c.forEach(d),this.h()},h(){g(t,"class","text svelte-1u9nil5"),g(e,"class","button svelte-1u9nil5")},m(a,c){V(a,e,c),n&&n.m(e,null),h(e,l),h(e,t),r&&r.m(t,null),h(e,s),o&&o.m(e,null),f=!0},p(a,c){a[4]?n?(n.p(a,c),c&16&&u(n,1)):(n=D(),n.c(),u(n,1),n.m(e,l)):n&&(N(),_(n,1,1,()=>{n=null}),q()),r&&r.p&&(!f||c&64)&&R(r,m,a,a[6],f?z(m,a[6],c,null):j(a[6]),null),a[4]?o&&(N(),_(o,1,1,()=>{o=null}),q()):o?(o.p(a,c),c&16&&u(o,1)):(o=S(),o.c(),u(o,1),o.m(e,null))},i(a){f||(u(n),u(r,a),u(o),f=!0)},o(a){_(n),_(r,a),_(o),f=!1},d(a){a&&d(e),n&&n.d(),r&&r.d(a),o&&o.d()}}}function Q(i){let e,l,t;return l=new K({props:{disabled:i[3],large:i[2],secondary:i[1],$$slots:{default:[P]},$$scope:{ctx:i}}}),{c(){e=b("a"),p(l.$$.fragment),this.h()},l(s){e=v(s,"A",{href:!0});var f=k(e);B(l.$$.fragment,f),f.forEach(d),this.h()},h(){g(e,"href",i[0])},m(s,f){V(s,e,f),I(l,e,null),t=!0},p(s,[f]){const n={};f&8&&(n.disabled=s[3]),f&4&&(n.large=s[2]),f&2&&(n.secondary=s[1]),f&80&&(n.$$scope={dirty:f,ctx:s}),l.$set(n),(!t||f&1)&&g(e,"href",s[0])},i(s){t||(u(l.$$.fragment,s),t=!0)},o(s){_(l.$$.fragment,s),t=!1},d(s){s&&d(e),w(l)}}}function T(i,e,l){let{$$slots:t={},$$scope:s}=e,{to:f="/"}=e,{secondary:n=!1}=e,{large:m=!1}=e,{disabled:r=!1}=e,{back:o=!1}=e;return i.$$set=a=>{"to"in a&&l(0,f=a.to),"secondary"in a&&l(1,n=a.secondary),"large"in a&&l(2,m=a.large),"disabled"in a&&l(3,r=a.disabled),"back"in a&&l(4,o=a.back),"$$scope"in a&&l(6,s=a.$$scope)},[f,n,m,r,o,t,s]}class X extends G{constructor(e){super(),H(this,e,T,Q,J,{to:0,secondary:1,large:2,disabled:3,back:4})}}export{X as N};