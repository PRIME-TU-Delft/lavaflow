import{S as ne,i as re,s as ae,C as V,k as $,a as w,l as b,m as k,h as v,c as C,n as H,T as Q,J as le,b as x,H as m,K as fe,D as P,E as q,F as A,f as W,t as z,w as ee,x as te,y as oe,z as se,q as ie,r as _e,I as ue}from"./index-5adb68ea.js";import{B as de,I as ce,a as ge}from"./Button-52912701.js";const he=e=>({foregroundWidth:e&2,foregroundHeight:e&4}),U=e=>({foregroundWidth:e[1],foregroundHeight:e[2]}),me=e=>({foregroundWidth:e&2,foregroundHeight:e&4}),X=e=>({foregroundWidth:e[1],foregroundHeight:e[2]}),pe=e=>({foregroundWidth:e&2,foregroundHeight:e&4}),Y=e=>({foregroundWidth:e[1],foregroundHeight:e[2]}),ve=e=>({foregroundWidth:e&2,foregroundHeight:e&4}),Z=e=>({foregroundWidth:e[1],foregroundHeight:e[2]}),$e=e=>({foregroundWidth:e&2,foregroundHeight:e&4}),y=e=>({foregroundWidth:e[1],foregroundHeight:e[2]});function be(e){let o,r,n;return o=new ce({props:{path:ge,color:"var(--text-color)"}}),{c(){ee(o.$$.fragment),r=ie(`
					Back`)},l(s){te(o.$$.fragment,s),r=_e(s,`
					Back`)},m(s,g){oe(o,s,g),x(s,r,g),n=!0},p:ue,i(s){n||(W(o.$$.fragment,s),n=!0)},o(s){z(o.$$.fragment,s),n=!1},d(s){se(o,s),s&&v(r)}}}function ke(e){let o,r;return o=new de({props:{$$slots:{default:[be]},$$scope:{ctx:e}}}),o.$on("click",e[4]),{c(){ee(o.$$.fragment)},l(n){te(o.$$.fragment,n)},m(n,s){oe(o,n,s),r=!0},p(n,s){const g={};s&64&&(g.$$scope={dirty:s,ctx:n}),o.$set(g)},i(n){r||(W(o.$$.fragment,n),r=!0)},o(n){z(o.$$.fragment,n),r=!1},d(n){se(o,n)}}}function He(e){let o,r,n,s,g,l,h,I,D,i,R,E,M,f;const S=e[3].background,_=V(S,e,e[6],y),T=e[3].headerButton,F=V(T,e,e[6],Z),p=F||ke(e),J=e[3].options,u=V(J,e,e[6],Y),K=e[3].default,d=V(K,e,e[6],X),L=e[3].footer,c=V(L,e,e[6],U);return{c(){o=$("div"),r=$("div"),n=w(),s=$("div"),_&&_.c(),g=w(),l=$("div"),h=$("header"),p&&p.c(),I=w(),u&&u.c(),D=w(),i=$("main"),d&&d.c(),R=w(),E=$("footer"),c&&c.c(),this.h()},l(t){o=b(t,"DIV",{class:!0});var a=k(o);r=b(a,"DIV",{class:!0}),k(r).forEach(v),n=C(a),s=b(a,"DIV",{class:!0});var N=k(s);_&&_.l(N),N.forEach(v),g=C(a),l=b(a,"DIV",{class:!0});var B=k(l);h=b(B,"HEADER",{class:!0});var O=k(h);p&&p.l(O),I=C(O),u&&u.l(O),O.forEach(v),D=C(B),i=b(B,"MAIN",{class:!0});var j=k(i);d&&d.l(j),j.forEach(v),R=C(B),E=b(B,"FOOTER",{class:!0});var G=k(E);c&&c.l(G),G.forEach(v),B.forEach(v),a.forEach(v),this.h()},h(){H(r,"class","backdrop svelte-4z33ga"),H(s,"class","background svelte-4z33ga"),H(h,"class","svelte-4z33ga"),H(i,"class","svelte-4z33ga"),Q(i,"fullscreen",e[0]),H(E,"class","svelte-4z33ga"),H(l,"class","foreground svelte-4z33ga"),le(()=>e[5].call(l)),H(o,"class","page svelte-4z33ga")},m(t,a){x(t,o,a),m(o,r),m(o,n),m(o,s),_&&_.m(s,null),m(o,g),m(o,l),m(l,h),p&&p.m(h,null),m(h,I),u&&u.m(h,null),m(l,D),m(l,i),d&&d.m(i,null),m(l,R),m(l,E),c&&c.m(E,null),M=fe(l,e[5].bind(l)),f=!0},p(t,[a]){_&&_.p&&(!f||a&70)&&P(_,S,t,t[6],f?A(S,t[6],a,$e):q(t[6]),y),F&&F.p&&(!f||a&70)&&P(F,T,t,t[6],f?A(T,t[6],a,ve):q(t[6]),Z),u&&u.p&&(!f||a&70)&&P(u,J,t,t[6],f?A(J,t[6],a,pe):q(t[6]),Y),d&&d.p&&(!f||a&70)&&P(d,K,t,t[6],f?A(K,t[6],a,me):q(t[6]),X),(!f||a&1)&&Q(i,"fullscreen",t[0]),c&&c.p&&(!f||a&70)&&P(c,L,t,t[6],f?A(L,t[6],a,he):q(t[6]),U)},i(t){f||(W(_,t),W(p,t),W(u,t),W(d,t),W(c,t),f=!0)},o(t){z(_,t),z(p,t),z(u,t),z(d,t),z(c,t),f=!1},d(t){t&&v(o),_&&_.d(t),p&&p.d(t),u&&u.d(t),d&&d.d(t),c&&c.d(t),M()}}}function We(e,o,r){let{$$slots:n={},$$scope:s}=o,{fullscreen:g=!1}=o,l,h;const I=()=>history.back();function D(){l=this.clientWidth,h=this.clientHeight,r(1,l),r(2,h)}return e.$$set=i=>{"fullscreen"in i&&r(0,g=i.fullscreen),"$$scope"in i&&r(6,s=i.$$scope)},[g,l,h,n,I,D,s]}class Be extends ne{constructor(o){super(),re(this,o,We,He,ae,{fullscreen:0})}}export{Be as P};