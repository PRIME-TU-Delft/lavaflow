import{S as j,i as H,s as J,e as P,b as p,g as _,v as K,d as b,f as O,h as m,y as h,z as w,A as d,B as v,C as M,D as B,E as C,F as R,L as X,a as A,k as U,c as S,l as I,m as V,n as D,I as q,q as k,r as Y,u as Z}from"./index.525666b8.mjs";import{A as E}from"./ActionButton.b6436ad5.mjs";import{A as y}from"./ActionMenu.61217329.mjs";import{M as x,B as ee}from"./Menubar.9b380d9b.mjs";import{d as te}from"./difficultyStore.73951052.mjs";import{g as oe}from"./gltfStore.4f83be3c.mjs";import{t as ne}from"./locationStore.c13cff09.mjs";import{d as Q,B as N}from"./Preview.svelte_svelte_type_style_lang.069820c8.mjs";import{E as re}from"./ErrorMessage.e4cefdaa.mjs";import{L as z}from"./LavaError.c6477686.mjs";function F(s){let t,o;return t=new re({props:{hasActions:!0,error:new z(`You scored ${s[0]}/1000 points`,s[1]),$$slots:{default:[se]},$$scope:{ctx:s}}}),t.$on("dismiss",s[3]),{c(){h(t.$$.fragment)},l(e){w(t.$$.fragment,e)},m(e,r){d(t,e,r),o=!0},p(e,r){const n={};r&3&&(n.error=new z(`You scored ${e[0]}/1000 points`,e[1])),r&16&&(n.$$scope={dirty:r,ctx:e}),t.$set(n)},i(e){o||(_(t.$$.fragment,e),o=!0)},o(e){b(t.$$.fragment,e),o=!1},d(e){v(t,e)}}}function se(s){let t;const o=s[2].default,e=M(o,s,s[4],null);return{c(){e&&e.c()},l(r){e&&e.l(r)},m(r,n){e&&e.m(r,n),t=!0},p(r,n){e&&e.p&&(!t||n&16)&&B(e,o,r,r[4],t?R(o,r[4],n,null):C(r[4]),null)},i(r){t||(_(e,r),t=!0)},o(r){b(e,r),t=!1},d(r){e&&e.d(r)}}}function le(s){let t,o,e=s[0]!==null&&F(s);return{c(){e&&e.c(),t=P()},l(r){e&&e.l(r),t=P()},m(r,n){e&&e.m(r,n),p(r,t,n),o=!0},p(r,[n]){r[0]!==null?e?(e.p(r,n),n&1&&_(e,1)):(e=F(r),e.c(),_(e,1),e.m(t.parentNode,t)):e&&(K(),b(e,1,1,()=>{e=null}),O())},i(r){o||(_(e),o=!0)},o(r){b(e),o=!1},d(r){e&&e.d(r),r&&m(t)}}}function ae(s,t,o){let{$$slots:e={},$$scope:r}=t,{score:n=null}=t,l="You are a volcano god!";function u(c){X.call(this,s,c)}return s.$$set=c=>{"score"in c&&o(0,n=c.score),"$$scope"in c&&o(4,r=c.$$scope)},s.$$.update=()=>{s.$$.dirty&1&&(n===null||n>900?o(1,l="You are a volcano god!, you can bend the lava to your will"):n>800?o(1,l="You are a volcano master!, you have the power of the volcano"):n>700?o(1,l="You are a volcano expert!, who should we call when you are gone?"):n>600?o(1,l="You are a volcano pro!, you know where people are safe and where they are not"):n>500?o(1,l="You are a volcano enthusiast! You can do better by placing the tubines closer to the lava"):n>400?o(1,l="You are a volcano beginner! You can do better by placing the tubines closer to the lava"):n>300?o(1,l="You are a volcano novice! You can do better by placing the tubines closer to the lava"):o(1,l="You are a volcano rookie! You can do better by placing the tubines closer to the lava"))},[n,l,e,u,r]}class fe extends j{constructor(t){super(),H(this,t,ae,le,J,{score:0})}}const ue=s=>({showLava:s&1}),G=s=>({showLava:s[0]}),ce=s=>({showLava:s&1}),T=s=>({showLava:s[0]});function ie(s){let t,o;return t=new ee({props:{class:"mt-4 w-full",$$slots:{default:[ge]},$$scope:{ctx:s}}}),{c(){h(t.$$.fragment)},l(e){w(t.$$.fragment,e)},m(e,r){d(t,e,r),o=!0},p(e,r){const n={};r&2050&&(n.$$scope={dirty:r,ctx:e}),t.$set(n)},i(e){o||(_(t.$$.fragment,e),o=!0)},o(e){b(t.$$.fragment,e),o=!1},d(e){v(t,e)}}}function me(s){let t,o;return t=new fe({props:{score:s[3],$$slots:{default:[de]},$$scope:{ctx:s}}}),t.$on("dismiss",s[10]),{c(){h(t.$$.fragment)},l(e){w(t.$$.fragment,e)},m(e,r){d(t,e,r),o=!0},p(e,r){const n={};r&8&&(n.score=e[3]),r&2050&&(n.$$scope={dirty:r,ctx:e}),t.$set(n)},i(e){o||(_(t.$$.fragment,e),o=!0)},o(e){b(t.$$.fragment,e),o=!1},d(e){v(t,e)}}}function $e(s){let t,o,e,r;return t=new E({props:{secondary:!0,href:s[1]+"/turbine-placement",icon:Q,$$slots:{default:[ve]},$$scope:{ctx:s}}}),e=new E({props:{$$slots:{default:[ke]},$$scope:{ctx:s}}}),e.$on("click",s[6]),{c(){h(t.$$.fragment),o=A(),h(e.$$.fragment)},l(n){w(t.$$.fragment,n),o=S(n),w(e.$$.fragment,n)},m(n,l){d(t,n,l),p(n,o,l),d(e,n,l),r=!0},p(n,l){const u={};l&2&&(u.href=n[1]+"/turbine-placement"),l&2048&&(u.$$scope={dirty:l,ctx:n}),t.$set(u);const c={};l&2048&&(c.$$scope={dirty:l,ctx:n}),e.$set(c)},i(n){r||(_(t.$$.fragment,n),_(e.$$.fragment,n),r=!0)},o(n){b(t.$$.fragment,n),b(e.$$.fragment,n),r=!1},d(n){v(t,n),n&&m(o),v(e,n)}}}function _e(s){let t,o;return t=new E({props:{href:s[1]+"/turbine-placement",icon:Q,$$slots:{default:[Ye]},$$scope:{ctx:s}}}),{c(){h(t.$$.fragment)},l(e){w(t.$$.fragment,e)},m(e,r){d(t,e,r),o=!0},p(e,r){const n={};r&2&&(n.href=e[1]+"/turbine-placement"),r&2080&&(n.$$scope={dirty:r,ctx:e}),t.$set(n)},i(e){o||(_(t.$$.fragment,e),o=!0)},o(e){b(t.$$.fragment,e),o=!1},d(e){v(t,e)}}}function pe(s){let t;return{c(){t=k("Rescan volcano")},l(o){t=Y(o,"Rescan volcano")},m(o,e){p(o,t,e)},d(o){o&&m(t)}}}function be(s){let t;return{c(){t=k("Move")},l(o){t=Y(o,"Move")},m(o,e){p(o,t,e)},d(o){o&&m(t)}}}function ge(s){let t,o,e,r;return t=new E({props:{twClass:"w-full",href:"/capure",$$slots:{default:[pe]},$$scope:{ctx:s}}}),e=new E({props:{twClass:"w-full",secondary:!0,href:s[1]+"/turbine-placement",$$slots:{default:[be]},$$scope:{ctx:s}}}),{c(){h(t.$$.fragment),o=A(),h(e.$$.fragment)},l(n){w(t.$$.fragment,n),o=S(n),w(e.$$.fragment,n)},m(n,l){d(t,n,l),p(n,o,l),d(e,n,l),r=!0},p(n,l){const u={};l&2048&&(u.$$scope={dirty:l,ctx:n}),t.$set(u);const c={};l&2&&(c.href=n[1]+"/turbine-placement"),l&2048&&(c.$$scope={dirty:l,ctx:n}),e.$set(c)},i(n){r||(_(t.$$.fragment,n),_(e.$$.fragment,n),r=!0)},o(n){b(t.$$.fragment,n),b(e.$$.fragment,n),r=!1},d(n){v(t,n),n&&m(o),v(e,n)}}}function he(s){let t;return{c(){t=k("Rescan volcano")},l(o){t=Y(o,"Rescan volcano")},m(o,e){p(o,t,e)},d(o){o&&m(t)}}}function we(s){let t;return{c(){t=k("Place turbines")},l(o){t=Y(o,"Place turbines")},m(o,e){p(o,t,e)},d(o){o&&m(t)}}}function de(s){let t,o,e,r;return t=new N({props:{color:"red",href:"/capture",outline:!0,$$slots:{default:[he]},$$scope:{ctx:s}}}),e=new N({props:{color:"red",href:s[1]+"/turbine-placement",outline:!0,$$slots:{default:[we]},$$scope:{ctx:s}}}),{c(){h(t.$$.fragment),o=A(),h(e.$$.fragment)},l(n){w(t.$$.fragment,n),o=S(n),w(e.$$.fragment,n)},m(n,l){d(t,n,l),p(n,o,l),d(e,n,l),r=!0},p(n,l){const u={};l&2048&&(u.$$scope={dirty:l,ctx:n}),t.$set(u);const c={};l&2&&(c.href=n[1]+"/turbine-placement"),l&2048&&(c.$$scope={dirty:l,ctx:n}),e.$set(c)},i(n){r||(_(t.$$.fragment,n),_(e.$$.fragment,n),r=!0)},o(n){b(t.$$.fragment,n),b(e.$$.fragment,n),r=!1},d(n){v(t,n),n&&m(o),v(e,n)}}}function ve(s){let t;return{c(){t=k("Place turbines")},l(o){t=Y(o,"Place turbines")},m(o,e){p(o,t,e)},d(o){o&&m(t)}}}function ke(s){let t;return{c(){t=k("Start eruption")},l(o){t=Y(o,"Start eruption")},m(o,e){p(o,t,e)},d(o){o&&m(t)}}}function Ye(s){let t,o,e;return{c(){t=k("Place "),o=k(s[5]),e=k(" more turbines")},l(r){t=Y(r,"Place "),o=Y(r,s[5]),e=Y(r," more turbines")},m(r,n){p(r,t,n),p(r,o,n),p(r,e,n)},p(r,n){n&32&&Z(o,r[5])},d(r){r&&m(t),r&&m(o),r&&m(e)}}}function Le(s){let t,o,e,r,n;const l=s[9].arActions,u=M(l,s,s[11],T),c=[_e,$e,me,ie],i=[];function f(a,$){return a[5]>0?0:a[3]?a[4]?3:2:1}return o=f(s),e=i[o]=c[o](s),{c(){u&&u.c(),t=A(),e.c(),r=P()},l(a){u&&u.l(a),t=S(a),e.l(a),r=P()},m(a,$){u&&u.m(a,$),p(a,t,$),i[o].m(a,$),p(a,r,$),n=!0},p(a,$){u&&u.p&&(!n||$&2049)&&B(u,l,a,a[11],n?R(l,a[11],$,ce):C(a[11]),T);let L=o;o=f(a),o===L?i[o].p(a,$):(K(),b(i[L],1,1,()=>{i[L]=null}),O(),e=i[o],e?e.p(a,$):(e=i[o]=c[o](a),e.c()),_(e,1),e.m(r.parentNode,r))},i(a){n||(_(u,a),_(e),n=!0)},o(a){b(u,a),b(e),n=!1},d(a){u&&u.d(a),a&&m(t),i[o].d(a),a&&m(r)}}}function Ae(s){let t,o,e,r,n,l,u;t=new x({props:{title:s[2],back:"/preview"}}),r=new y({props:{$$slots:{default:[Le]},$$scope:{ctx:s}}});const c=s[9].default,i=M(c,s,s[11],G);return{c(){h(t.$$.fragment),o=A(),e=U("div"),h(r.$$.fragment),n=A(),l=U("div"),i&&i.c(),this.h()},l(f){w(t.$$.fragment,f),o=S(f),e=I(f,"DIV",{id:!0});var a=V(e);w(r.$$.fragment,a),a.forEach(m),n=S(f),l=I(f,"DIV",{class:!0});var $=V(l);i&&i.l($),$.forEach(m),this.h()},h(){D(e,"id","overlay"),D(l,"class","h-full w-full")},m(f,a){d(t,f,a),p(f,o,a),p(f,e,a),d(r,e,null),p(f,n,a),p(f,l,a),i&&i.m(l,null),u=!0},p(f,[a]){const $={};a&4&&($.title=f[2]),t.$set($);const L={};a&2107&&(L.$$scope={dirty:a,ctx:f}),r.$set(L),i&&i.p&&(!u||a&2049)&&B(i,c,f,f[11],u?R(c,f[11],a,ue):C(f[11]),G)},i(f){u||(_(t.$$.fragment,f),_(r.$$.fragment,f),_(i,f),u=!0)},o(f){b(t.$$.fragment,f),b(r.$$.fragment,f),b(i,f),u=!1},d(f){v(t,f),f&&m(o),f&&m(e),v(r),f&&m(n),f&&m(l),i&&i.d(f)}}}function Se(s,t,o){let e,r,n;q(s,ne,g=>o(7,r=g)),q(s,te,g=>o(8,n=g));let{$$slots:l={},$$scope:u}=t,{baseUrl:c}=t,{title:i}=t,f=null,a=!1,{showLava:$=!1}=t;function L(){o(0,$=!0),o(4,a=!1),o(3,f=oe.computePlayerPoints(1e3))}const W=()=>o(4,a=!0);return s.$$set=g=>{"baseUrl"in g&&o(1,c=g.baseUrl),"title"in g&&o(2,i=g.title),"showLava"in g&&o(0,$=g.showLava),"$$scope"in g&&o(11,u=g.$$scope)},s.$$.update=()=>{s.$$.dirty&384&&o(5,e=n.min_steam_turbines-r.length)},[$,c,i,f,a,e,L,r,n,l,W,u]}class qe extends j{constructor(t){super(),H(this,t,Se,Ae,J,{baseUrl:1,title:2,showLava:0})}}export{qe as V};