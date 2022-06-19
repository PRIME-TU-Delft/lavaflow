import{S as J,i as K,s as Q,w as y,x as T,y as S,E as z,q as g,o as h,B as P,J as H,e as N,t as I,k as R,c as V,a as W,h as D,d,m as q,b as M,g as $,K as C,P as se,Q as ae,L as oe,l as j,n as U,p as X,f as fe,F as me,G as de,H as _e,I as ge,a4 as he,j as pe}from"../../chunks/index-e50d45aa.js";import{B as Z,a as be,N as le,e as $e,f as ve}from"../../chunks/NavigationButton-d591ec72.js";import{P as ke}from"../../chunks/P5-bd5cfe29.js";import{t as G,c as ie,a as we,D as te}from"../../chunks/contourLineStore-67ef1863.js";import{d as ue}from"../../chunks/difficultyStore-0791473c.js";import"../../chunks/@vite-plugin-wasm-pack@wasm-9c455baf.js";import{P as ye}from"../../chunks/Page-c4546c49.js";import{M as Te}from"../../chunks/Modal-15ebe271.js";import"../../chunks/preload-helper-60cab3ee.js";import"../../chunks/index-ca69d11e.js";function Se(a){let t,n;return t=new ke({props:{sketch:a[0]}}),{c(){y(t.$$.fragment)},l(e){T(t.$$.fragment,e)},m(e,r){S(t,e,r),n=!0},p:z,i(e){n||(g(t.$$.fragment,e),n=!0)},o(e){h(t.$$.fragment,e),n=!1},d(e){P(t,e)}}}function Pe(a,t,n){let e,r,s;H(a,G,l=>n(5,e=l)),H(a,ue,l=>n(6,r=l)),H(a,ie,l=>n(7,s=l));let{foregroundWidth:f}=t,{foregroundHeight:u}=t,{targetSelected:o}=t,{curves:i}=t;function c(l){l.setup=()=>{l.createCanvas(f,u)},l.draw=()=>{l.background(230),l.stroke(30),i.forEach(k=>{l.strokeWeight(4),l.noFill(),l.beginShape();for(let[E,x]of k)l.vertex(E,x);l.endShape(l.CLOSE)});const v=s!=null?s:[];for(let k of s){l.strokeWeight(1),l.stroke("#e15f55f0");const[E,x]=k;l.fill("#e15f55f0"),l.ellipse(E,x,50,50),l.noFill(),l.strokeWeight(.5),l.ellipse(E,x,r.min_crater_distance*2,r.min_crater_distance*2)}let p=20;for(let k=0;k<e.length;k++){let E=e[k];E.update(l,v,r.min_crater_distance,e,r.min_steam_turbine_separation,k),E.drawCircle(l,p,k+1)}let b="";if(e.length!=r.min_steam_turbines&&r.min_steam_turbines==r.max_steam_turbines?b="You must place exactly "+r.min_steam_turbines+" steam turbines":e.length<r.min_steam_turbines?b="You must place at least "+r.min_steam_turbines+" steam turbines":e.length==r.max_steam_turbines&&(b="You have reached the maximal amount of "+r.max_steam_turbines+" steam turbines"),e.length<r.min_steam_turbines||e.length>=r.max_steam_turbines){l.noStroke(),l.fill(51),l.textSize(15),l.textAlign(l.CENTER);let k=l.textWidth(b);l.rectMode(l.CENTER),l.rect(l.width/2,60,k+10,30),l.strokeWeight(1),l.fill(255),l.text(b,l.width/2,65)}},l.mousePressed=()=>{if(!(e!=null&&e.length))return;let v=!1;for(let p=0;p<e.length;p++){let b=!1;e[p].deselect(),v||(b=e[p].pressed(l,!0),v=b),b&&n(1,o=p)}},l.mouseReleased=()=>{if(!!(e!=null&&e.length)){for(let v of e)v.released();G.set(e)}}}return a.$$set=l=>{"foregroundWidth"in l&&n(2,f=l.foregroundWidth),"foregroundHeight"in l&&n(3,u=l.foregroundHeight),"targetSelected"in l&&n(1,o=l.targetSelected),"curves"in l&&n(4,i=l.curves)},[c,o,f,u,i]}class Ee extends J{constructor(t){super(),K(this,t,Pe,Se,Q,{foregroundWidth:2,foregroundHeight:3,targetSelected:1,curves:4})}}function Ce(a){let t,n,e,r,s,f,u,o,i;return{c(){t=N("div"),n=N("h2"),e=I("Instructions"),r=R(),s=N("div"),f=I(`Add targets where you think the lava will flow. You can't place targets too close to 
		the craters or too close to other targets. How close you can place targets to craters
		and other targets depends on the difficulty mode you selected. There is also a minimum
		amount of targets you need to place, which also depends on your difficulty mode. You
		can never place more than 10 targets.`),u=R(),o=N("div"),i=I("Points are based on how close the steam turbine is placed to the lava stream, divided by the number of turbines. The points represent how much energy the average steam turbine generates, measured in megawatts."),this.h()},l(c){t=V(c,"DIV",{class:!0});var l=W(t);n=V(l,"H2",{});var v=W(n);e=D(v,"Instructions"),v.forEach(d),r=q(l),s=V(l,"DIV",{class:!0});var p=W(s);f=D(p,`Add targets where you think the lava will flow. You can't place targets too close to 
		the craters or too close to other targets. How close you can place targets to craters
		and other targets depends on the difficulty mode you selected. There is also a minimum
		amount of targets you need to place, which also depends on your difficulty mode. You
		can never place more than 10 targets.`),p.forEach(d),u=q(l),o=V(l,"DIV",{class:!0});var b=W(o);i=D(b,"Points are based on how close the steam turbine is placed to the lava stream, divided by the number of turbines. The points represent how much energy the average steam turbine generates, measured in megawatts."),b.forEach(d),l.forEach(d),this.h()},h(){M(s,"class","instruction"),M(o,"class","instruction"),M(t,"class","instructions svelte-tryer5")},m(c,l){$(c,t,l),C(t,n),C(n,e),C(t,r),C(t,s),C(s,f),C(t,u),C(t,o),C(o,i)},p:z,i:z,o:z,d(c){c&&d(t)}}}class Ie extends J{constructor(t){super(),K(this,t,null,Ce,Q,{})}}function De(a){let t,n;return t=new Ie({}),{c(){y(t.$$.fragment)},l(e){T(t.$$.fragment,e)},m(e,r){S(t,e,r),n=!0},i(e){n||(g(t.$$.fragment,e),n=!0)},o(e){h(t.$$.fragment,e),n=!1},d(e){P(t,e)}}}function ne(a){let t,n,e,r;function s(u){a[11](u)}let f={foregroundHeight:a[16],foregroundWidth:a[17],curves:a[4].curves};return a[0]!==void 0&&(f.targetSelected=a[0]),n=new Ee({props:f}),se.push(()=>ae(n,"targetSelected",s)),{c(){t=N("div"),y(n.$$.fragment),this.h()},l(u){t=V(u,"DIV",{class:!0});var o=W(t);T(n.$$.fragment,o),o.forEach(d),this.h()},h(){M(t,"class","sketch svelte-1invhxf")},m(u,o){$(u,t,o),S(n,t,null),r=!0},p(u,o){const i={};o&65536&&(i.foregroundHeight=u[16]),o&131072&&(i.foregroundWidth=u[17]),o&16&&(i.curves=u[4].curves),!e&&o&1&&(e=!0,i.targetSelected=u[0],oe(()=>e=!1)),n.$set(i)},i(u){r||(g(n.$$.fragment,u),r=!0)},o(u){h(n.$$.fragment,u),r=!1},d(u){u&&d(t),P(n)}}}function Be(a){var r;let t,n,e=((r=a[4])==null?void 0:r.curves)&&a[2]&&a[3]&&ne(a);return{c(){e&&e.c(),t=j()},l(s){e&&e.l(s),t=j()},m(s,f){e&&e.m(s,f),$(s,t,f),n=!0},p(s,f){var u;((u=s[4])==null?void 0:u.curves)&&s[2]&&s[3]?e?(e.p(s,f),f&28&&g(e,1)):(e=ne(s),e.c(),g(e,1),e.m(t.parentNode,t)):e&&(U(),h(e,1,1,()=>{e=null}),X())},i(s){n||(g(e),n=!0)},o(s){h(e),n=!1},d(s){e&&e.d(s),s&&d(t)}}}function Ae(a){let t;return{c(){t=N("div"),this.h()},l(n){t=V(n,"DIV",{slot:!0,style:!0}),W(t).forEach(d),this.h()},h(){M(t,"slot","background"),fe(t,"background","#aaa")},m(n,e){$(n,t,e)},p:z,d(n){n&&d(t)}}}function He(a){let t,n;const e=a[7].default,r=me(e,a,a[12],null);return{c(){t=N("div"),r&&r.c(),this.h()},l(s){t=V(s,"DIV",{slot:!0});var f=W(t);r&&r.l(f),f.forEach(d),this.h()},h(){M(t,"slot","headerButton")},m(s,f){$(s,t,f),r&&r.m(t,null),n=!0},p(s,f){r&&r.p&&(!n||f&4096)&&de(r,e,s,s[12],n?ge(e,s[12],f,null):_e(s[12]),null)},i(s){n||(g(r,s),n=!0)},o(s){h(r,s),n=!1},d(s){s&&d(t),r&&r.d(s)}}}function Ne(a){let t;return{c(){t=I("Place turbine | Instructions")},l(n){t=D(n,"Place turbine | Instructions")},m(n,e){$(n,t,e)},d(n){n&&d(t)}}}function Ve(a){let t,n;return t=new le({props:{back:!0,to:"/scan/mapscanning",$$slots:{default:[Ye]},$$scope:{ctx:a}}}),{c(){y(t.$$.fragment)},l(e){T(t.$$.fragment,e)},m(e,r){S(t,e,r),n=!0},p(e,r){const s={};r&4096&&(s.$$scope={dirty:r,ctx:e}),t.$set(s)},i(e){n||(g(t.$$.fragment,e),n=!0)},o(e){h(t.$$.fragment,e),n=!1},d(e){P(t,e)}}}function We(a){let t,n,e,r,s=a[3].length>0&&re(a);function f(){return a[10](a[17],a[16])}return e=new Z({props:{icon:$e,$$slots:{default:[xe]},$$scope:{ctx:a}}}),e.$on("click",f),{c(){t=N("div"),s&&s.c(),n=R(),y(e.$$.fragment),this.h()},l(u){t=V(u,"DIV",{class:!0});var o=W(t);s&&s.l(o),n=q(o),T(e.$$.fragment,o),o.forEach(d),this.h()},h(){M(t,"class","editTurbines svelte-1invhxf")},m(u,o){$(u,t,o),s&&s.m(t,null),C(t,n),S(e,t,null),r=!0},p(u,o){a=u,a[3].length>0?s?(s.p(a,o),o&8&&g(s,1)):(s=re(a),s.c(),g(s,1),s.m(t,n)):s&&(U(),h(s,1,1,()=>{s=null}),X());const i={};o&4096&&(i.$$scope={dirty:o,ctx:a}),e.$set(i)},i(u){r||(g(s),g(e.$$.fragment,u),r=!0)},o(u){h(s),h(e.$$.fragment,u),r=!1},d(u){u&&d(t),s&&s.d(),P(e)}}}function Ye(a){let t;return{c(){t=I("No image found. Go to map scanning")},l(n){t=D(n,"No image found. Go to map scanning")},m(n,e){$(n,t,e)},d(n){n&&d(t)}}}function re(a){let t,n;return t=new Z({props:{secondary:!0,icon:ve,$$slots:{default:[Me]},$$scope:{ctx:a}}}),t.$on("click",a[5]),{c(){y(t.$$.fragment)},l(e){T(t.$$.fragment,e)},m(e,r){S(t,e,r),n=!0},p(e,r){const s={};r&4096&&(s.$$scope={dirty:r,ctx:e}),t.$set(s)},i(e){n||(g(t.$$.fragment,e),n=!0)},o(e){h(t.$$.fragment,e),n=!1},d(e){P(t,e)}}}function Me(a){let t;return{c(){t=I("Clear all")},l(n){t=D(n,"Clear all")},m(n,e){$(n,t,e)},d(n){n&&d(t)}}}function xe(a){let t;return{c(){t=I("Add steam turbine")},l(n){t=D(n,"Add steam turbine")},m(n,e){$(n,t,e)},d(n){n&&d(t)}}}function Fe(a){let t,n,e,r,s,f;t=new Z({props:{secondary:!0,icon:be,$$slots:{default:[Ne]},$$scope:{ctx:a}}}),t.$on("click",a[9]);const u=[We,Ve],o=[];function i(c,l){return c[4]?0:1}return e=i(a),r=o[e]=u[e](a),{c(){y(t.$$.fragment),n=R(),r.c(),s=j()},l(c){T(t.$$.fragment,c),n=q(c),r.l(c),s=j()},m(c,l){S(t,c,l),$(c,n,l),o[e].m(c,l),$(c,s,l),f=!0},p(c,l){const v={};l&4096&&(v.$$scope={dirty:l,ctx:c}),t.$set(v);let p=e;e=i(c),e===p?o[e].p(c,l):(U(),h(o[p],1,1,()=>{o[p]=null}),X(),r=o[e],r?r.p(c,l):(r=o[e]=u[e](c),r.c()),g(r,1),r.m(s.parentNode,s))},i(c){f||(g(t.$$.fragment,c),g(r),f=!0)},o(c){h(t.$$.fragment,c),h(r),f=!1},d(c){P(t,c),c&&d(n),o[e].d(c),c&&d(s)}}}function Le(a){let t,n,e,r,s;function f(o){a[8](o)}let u={title:"transformation instructions",closeButtons:"top",$$slots:{default:[De]},$$scope:{ctx:a}};return a[1]!==void 0&&(u.visible=a[1]),t=new Te({props:u}),se.push(()=>ae(t,"visible",f)),r=new ye({props:{$$slots:{footer:[Fe,({foregroundHeight:o,foregroundWidth:i})=>({16:o,17:i}),({foregroundHeight:o,foregroundWidth:i})=>(o?65536:0)|(i?131072:0)],headerButton:[He,({foregroundHeight:o,foregroundWidth:i})=>({16:o,17:i}),({foregroundHeight:o,foregroundWidth:i})=>(o?65536:0)|(i?131072:0)],background:[Ae,({foregroundHeight:o,foregroundWidth:i})=>({16:o,17:i}),({foregroundHeight:o,foregroundWidth:i})=>(o?65536:0)|(i?131072:0)],default:[Be,({foregroundHeight:o,foregroundWidth:i})=>({16:o,17:i}),({foregroundHeight:o,foregroundWidth:i})=>(o?65536:0)|(i?131072:0)]},$$scope:{ctx:a}}}),{c(){y(t.$$.fragment),e=R(),y(r.$$.fragment)},l(o){T(t.$$.fragment,o),e=q(o),T(r.$$.fragment,o)},m(o,i){S(t,o,i),$(o,e,i),S(r,o,i),s=!0},p(o,[i]){const c={};i&4096&&(c.$$scope={dirty:i,ctx:o}),!n&&i&2&&(n=!0,c.visible=o[1],oe(()=>n=!1)),t.$set(c);const l={};i&200735&&(l.$$scope={dirty:i,ctx:o}),r.$set(l)},i(o){s||(g(t.$$.fragment,o),g(r.$$.fragment,o),s=!0)},o(o){h(t.$$.fragment,o),h(r.$$.fragment,o),s=!1},d(o){P(t,o),o&&d(e),P(r,o)}}}function ze(a,t,n){let e,r,s,f;H(a,ue,m=>n(2,e=m)),H(a,G,m=>n(3,r=m)),H(a,ie,m=>n(13,s=m)),H(a,we,m=>n(4,f=m));let{$$slots:u={},$$scope:o}=t,i=-1,c=!1;function l(m,B,F,ee){if(r.length>=e.max_steam_turbines)return;const _=new te(m,B,20);_.enableSelection();for(let A of r)for(;_.isTooCloseTo(A.x,A.y,e.min_steam_turbine_separation);){let Y=_.x-A.x,w=_.y-A.y;Y==0&&(Y=.001),w==0&&(w=.001),_.x+=Y,_.y+=w}let L=!1,O=_;for(;;){L=!1,(_.x<=0||_.x>=F||_.y<=0||_.y>=ee)&&(_.x=20+Math.random()*(F-40),_.y=20+Math.random()*(ee-40));for(let w of r)_.isTooCloseTo(w.x,w.y,e.min_steam_turbine_separation)&&(L=!0,O=w);for(let w of s)_.isTooCloseTo(w[0],w[1],e.min_crater_distance)&&(L==!1&&(O=new te(w[0],w[1],20)),L=!0);if(!L)break;let A=_.x-O.x,Y=_.y-O.y;A==0&&(A=.001),Y==0&&(Y=.001),_.x+=A,_.y+=Y}G.add(_)}function v(){i!=-1&&(G.remove(i),n(0,i=-1))}function p(){for(let m=r.length-1;m>=0;m--)n(0,i=m),v()}function b(m,B){let F=!1;for(;r.length<e.min_steam_turbines;)l(m/8,B/8,m,B),F=!0;F||l(m/8,B/8,m,B-100)}function k(m){c=m,n(1,c)}const E=()=>n(1,c=!c),x=(m,B)=>b(m,B-100);function ce(m){i=m,n(0,i)}return a.$$set=m=>{"$$scope"in m&&n(12,o=m.$$scope)},[i,c,e,r,f,p,b,u,k,E,x,ce,o]}class Ge extends J{constructor(t){super(),K(this,t,ze,Le,Q,{})}}const Re=()=>{const a=he("__svelte__");return{page:{subscribe:a.page.subscribe},navigating:{subscribe:a.navigating.subscribe},get preloading(){return console.error("stores.preloading is deprecated; use stores.navigating instead"),{subscribe:a.navigating.subscribe}},session:a.session,updated:a.updated}},qe={subscribe(a){return Re().page.subscribe(a)}};function Oe(a){let t,n=a[0].params.slug+"",e;return{c(){t=I("Apply and go to "),e=I(n)},l(r){t=D(r,"Apply and go to "),e=D(r,n)},m(r,s){$(r,t,s),$(r,e,s)},p(r,s){s&1&&n!==(n=r[0].params.slug+"")&&pe(e,n)},d(r){r&&d(t),r&&d(e)}}}function je(a){let t,n;return t=new le({props:{back:!0,to:"/visualise/"+a[0].params.slug,$$slots:{default:[Oe]},$$scope:{ctx:a}}}),{c(){y(t.$$.fragment)},l(e){T(t.$$.fragment,e)},m(e,r){S(t,e,r),n=!0},p(e,r){const s={};r&1&&(s.to="/visualise/"+e[0].params.slug),r&3&&(s.$$scope={dirty:r,ctx:e}),t.$set(s)},i(e){n||(g(t.$$.fragment,e),n=!0)},o(e){h(t.$$.fragment,e),n=!1},d(e){P(t,e)}}}function Je(a){let t,n;return t=new Ge({props:{$$slots:{default:[je]},$$scope:{ctx:a}}}),{c(){y(t.$$.fragment)},l(e){T(t.$$.fragment,e)},m(e,r){S(t,e,r),n=!0},p(e,[r]){const s={};r&3&&(s.$$scope={dirty:r,ctx:e}),t.$set(s)},i(e){n||(g(t.$$.fragment,e),n=!0)},o(e){h(t.$$.fragment,e),n=!1},d(e){P(t,e)}}}const ot=!0;function Ke(a,t,n){let e;return H(a,qe,r=>n(0,e=r)),[e]}class lt extends J{constructor(t){super(),K(this,t,Ke,Je,Q,{})}}export{lt as default,ot as prerender};
