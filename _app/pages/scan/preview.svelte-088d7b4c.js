import{S as w,i as B,s as I,e as _,c as d,K as b,b as c,g as l,E as h,d as o,w as u,x as f,y as m,q as $,o as g,B as p,a0 as E,a as v,t as k,h as x,J as P}from"../../chunks/index-cd071420.js";import{B as q}from"../../chunks/mdi-ff5aee52.js";import{P as N,g as S,p as y}from"../../chunks/imageStore-78473fa9.js";import{N as D}from"../../chunks/NavigationButton-1ab31783.js";import"../../chunks/singletons-5be3e935.js";function R(s){let t,r;return{c(){t=_("img"),this.h()},l(e){t=d(e,"IMG",{src:!0,alt:!0,class:!0}),this.h()},h(){b(t.src,r=s[0])||c(t,"src",r),c(t,"alt",s[1]),c(t,"class","svelte-1mad6ai")},m(e,a){l(e,t,a)},p(e,[a]){a&1&&!b(t.src,r=e[0])&&c(t,"src",r),a&2&&c(t,"alt",e[1])},i:h,o:h,d(e){e&&o(t)}}}function A(s,t,r){let{src:e}=t,{alt:a}=t;return s.$$set=n=>{"src"in n&&r(0,e=n.src),"alt"in n&&r(1,a=n.alt)},[e,a]}class V extends w{constructor(t){super(),B(this,t,A,R,I,{src:0,alt:1})}}function C(s){let t,r;return t=new V({props:{src:s[0],alt:"foreground"}}),{c(){u(t.$$.fragment)},l(e){f(t.$$.fragment,e)},m(e,a){m(t,e,a),r=!0},p(e,a){const n={};a&1&&(n.src=e[0]),t.$set(n)},i(e){r||($(t.$$.fragment,e),r=!0)},o(e){g(t.$$.fragment,e),r=!1},d(e){p(t,e)}}}function G(s){let t;return{c(){t=k("Redraw borders")},l(r){t=x(r,"Redraw borders")},m(r,e){l(r,t,e)},d(r){r&&o(t)}}}function J(s){let t,r;return t=new D({props:{slot:"headerButton",to:"/scan/maptransform",back:!0,$$slots:{default:[G]},$$scope:{ctx:s}}}),{c(){u(t.$$.fragment)},l(e){f(t.$$.fragment,e)},m(e,a){m(t,e,a),r=!0},p(e,a){const n={};a&4&&(n.$$scope={dirty:a,ctx:e}),t.$set(n)},i(e){r||($(t.$$.fragment,e),r=!0)},o(e){g(t.$$.fragment,e),r=!1},d(e){p(t,e)}}}function K(s){let t,r,e;return r=new V({props:{src:s[0],alt:"background"}}),{c(){t=_("div"),u(r.$$.fragment),this.h()},l(a){t=d(a,"DIV",{slot:!0});var n=v(t);f(r.$$.fragment,n),n.forEach(o),this.h()},h(){c(t,"slot","background")},m(a,n){l(a,t,n),m(r,t,null),e=!0},p(a,n){const i={};n&1&&(i.src=a[0]),r.$set(i)},i(a){e||($(r.$$.fragment,a),e=!0)},o(a){g(r.$$.fragment,a),e=!1},d(a){a&&o(t),p(r)}}}function M(s){let t,r;return{c(){t=_("span"),r=k("Visualise")},l(e){t=d(e,"SPAN",{});var a=v(t);r=x(a,"Visualise"),a.forEach(o)},m(e,a){l(e,t,a),P(t,r)},p:h,d(e){e&&o(t)}}}function j(s){let t,r,e;return r=new q({props:{$$slots:{default:[M]},$$scope:{ctx:s}}}),r.$on("click",s[1]),{c(){t=_("div"),u(r.$$.fragment),this.h()},l(a){t=d(a,"DIV",{slot:!0});var n=v(t);f(r.$$.fragment,n),n.forEach(o),this.h()},h(){c(t,"slot","footer")},m(a,n){l(a,t,n),m(r,t,null),e=!0},p(a,n){const i={};n&4&&(i.$$scope={dirty:n,ctx:a}),r.$set(i)},i(a){e||($(r.$$.fragment,a),e=!0)},o(a){g(r.$$.fragment,a),e=!1},d(a){a&&o(t),p(r)}}}function z(s){let t,r;return t=new N({props:{title:"image transformation",$$slots:{footer:[j],background:[K],headerButton:[J],default:[C]},$$scope:{ctx:s}}}),{c(){u(t.$$.fragment)},l(e){f(t.$$.fragment,e)},m(e,a){m(t,e,a),r=!0},p(e,[a]){const n={};a&5&&(n.$$scope={dirty:a,ctx:e}),t.$set(n)},i(e){r||($(t.$$.fragment,e),r=!0)},o(e){g(t.$$.fragment,e),r=!1},d(e){p(t,e)}}}function F(s,t,r){let e;E(s,y,n=>r(0,e=n));function a(){S("/demo")}return[e,a]}class U extends w{constructor(t){super(),B(this,t,F,z,I,{})}}export{U as default};
