import{S as O,i as Q,s as Z,k as g,a as j,y as R,l as v,m as w,c as y,z as M,h as f,b as E,G as p,A as P,g as $,v as ne,d as D,f as ie,B as S,K as De,C as oe,n as d,D as fe,E as ue,F as ce,L as Ie,q as G,r as H,M as N,H as W,u as he,o as ke}from"../chunks/index.525666b8.mjs";import{A as Ee}from"../chunks/ActionButton.b6436ad5.mjs";import{B as je,I as $e,m as ye,a as be}from"../chunks/Preview.svelte_svelte_type_style_lang.069820c8.mjs";const Ve=n=>({}),pe=n=>({});function _e(n){let e,l;const r=n[5].label,s=oe(r,n,n[7],pe);return{c(){e=g("div"),s&&s.c(),this.h()},l(t){e=v(t,"DIV",{class:!0});var a=w(e);s&&s.l(a),a.forEach(f),this.h()},h(){d(e,"class","my-2 text-sm text-zinc-800")},m(t,a){E(t,e,a),s&&s.m(e,null),l=!0},p(t,a){s&&s.p&&(!l||a&128)&&fe(s,r,t,t[7],l?ce(r,t[7],a,Ve):ue(t[7]),pe)},i(t){l||($(s,t),l=!0)},o(t){D(s,t),l=!1},d(t){t&&f(e),s&&s.d(t)}}}function ge(n){let e,l;return e=new $e({props:{path:n[0]}}),{c(){R(e.$$.fragment)},l(r){M(e.$$.fragment,r)},m(r,s){P(e,r,s),l=!0},p(r,s){const t={};s&1&&(t.path=r[0]),e.$set(t)},i(r){l||($(e.$$.fragment,r),l=!0)},o(r){D(e.$$.fragment,r),l=!1},d(r){S(e,r)}}}function ve(n){let e,l;return e=new $e({props:{path:n[1]}}),{c(){R(e.$$.fragment)},l(r){M(e.$$.fragment,r)},m(r,s){P(e,r,s),l=!0},p(r,s){const t={};s&2&&(t.path=r[1]),e.$set(t)},i(r){l||($(e.$$.fragment,r),l=!0)},o(r){D(e.$$.fragment,r),l=!1},d(r){S(e,r)}}}function Ae(n){let e,l,r,s,t=n[0]&&ge(n);const a=n[5].default,i=oe(a,n,n[7],null);let o=n[1]&&ve(n);return{c(){e=g("p"),t&&t.c(),l=j(),i&&i.c(),r=j(),o&&o.c(),this.h()},l(u){e=v(u,"P",{class:!0});var c=w(e);t&&t.l(c),l=y(c),i&&i.l(c),r=y(c),o&&o.l(c),c.forEach(f),this.h()},h(){d(e,"class","flex items-center gap-2")},m(u,c){E(u,e,c),t&&t.m(e,null),p(e,l),i&&i.m(e,null),p(e,r),o&&o.m(e,null),s=!0},p(u,c){u[0]?t?(t.p(u,c),c&1&&$(t,1)):(t=ge(u),t.c(),$(t,1),t.m(e,l)):t&&(ne(),D(t,1,1,()=>{t=null}),ie()),i&&i.p&&(!s||c&128)&&fe(i,a,u,u[7],s?ce(a,u[7],c,null):ue(u[7]),null),u[1]?o?(o.p(u,c),c&2&&$(o,1)):(o=ve(u),o.c(),$(o,1),o.m(e,null)):o&&(ne(),D(o,1,1,()=>{o=null}),ie())},i(u){s||($(t),$(i,u),$(o),s=!0)},o(u){D(t),D(i,u),D(o),s=!1},d(u){u&&f(e),t&&t.d(),i&&i.d(u),o&&o.d()}}}function Re(n){let e,l,r,s,t=n[4].label&&_e(n);return r=new je({props:{href:n[2],outline:n[3],color:"red",$$slots:{default:[Ae]},$$scope:{ctx:n}}}),r.$on("click",n[6]),{c(){e=g("div"),t&&t.c(),l=j(),R(r.$$.fragment)},l(a){e=v(a,"DIV",{});var i=w(e);t&&t.l(i),l=y(i),M(r.$$.fragment,i),i.forEach(f)},m(a,i){E(a,e,i),t&&t.m(e,null),p(e,l),P(r,e,null),s=!0},p(a,[i]){a[4].label?t?(t.p(a,i),i&16&&$(t,1)):(t=_e(a),t.c(),$(t,1),t.m(e,l)):t&&(ne(),D(t,1,1,()=>{t=null}),ie());const o={};i&4&&(o.href=a[2]),i&8&&(o.outline=a[3]),i&131&&(o.$$scope={dirty:i,ctx:a}),r.$set(o)},i(a){s||($(t),$(r.$$.fragment,a),s=!0)},o(a){D(t),D(r.$$.fragment,a),s=!1},d(a){a&&f(e),t&&t.d(),S(r)}}}function Me(n,e,l){let{$$slots:r={},$$scope:s}=e;const t=De(r);let{iconPrefix:a=""}=e,{icon:i=""}=e,{href:o=""}=e,{outline:u=!1}=e;function c(_){Ie.call(this,n,_)}return n.$$set=_=>{"iconPrefix"in _&&l(0,a=_.iconPrefix),"icon"in _&&l(1,i=_.icon),"href"in _&&l(2,o=_.href),"outline"in _&&l(3,u=_.outline),"$$scope"in _&&l(7,s=_.$$scope)},[a,i,o,u,t,r,c,s]}class we extends O{constructor(e){super(),Q(this,e,Me,Re,Z,{iconPrefix:0,icon:1,href:2,outline:3})}}function Pe(n){let e,l;const r=n[1].default,s=oe(r,n,n[0],null);return{c(){e=g("div"),s&&s.c(),this.h()},l(t){e=v(t,"DIV",{class:!0});var a=w(e);s&&s.l(a),a.forEach(f),this.h()},h(){d(e,"class","flex flex-col gap-4")},m(t,a){E(t,e,a),s&&s.m(e,null),l=!0},p(t,[a]){s&&s.p&&(!l||a&1)&&fe(s,r,t,t[0],l?ce(r,t[0],a,null):ue(t[0]),null)},i(t){l||($(s,t),l=!0)},o(t){D(s,t),l=!1},d(t){t&&f(e),s&&s.d(t)}}}function Se(n,e,l){let{$$slots:r={},$$scope:s}=e;return n.$$set=t=>{"$$scope"in t&&l(0,s=t.$$scope)},[s,r]}class Te extends O{constructor(e){super(),Q(this,e,Se,Pe,Z,{})}}function Ce(n){let e,l,r,s,t,a;return{c(){e=g("div"),l=g("img"),s=j(),t=g("div"),a=G("AR: Lava Flow"),this.h()},l(i){e=v(i,"DIV",{class:!0});var o=w(e);l=v(o,"IMG",{class:!0,src:!0,alt:!0}),s=y(o),t=v(o,"DIV",{class:!0});var u=w(t);a=H(u,"AR: Lava Flow"),u.forEach(f),o.forEach(f),this.h()},h(){d(l,"class","h-full max-h-96 w-full object-cover saturate-50 lg:rounded-bl-3xl lg:rounded-br-3xl"),N(l.src,r="volcano.jpg")||d(l,"src",r),d(l,"alt","title artwork"),d(t,"class","title absolute top-1/2 w-full text-center text-5xl font-bold text-white svelte-1mourln"),d(e,"class","relative mx-auto max-h-96 max-w-3xl")},m(i,o){E(i,e,o),p(e,l),p(e,s),p(e,t),p(t,a)},p:W,i:W,o:W,d(i){i&&f(e)}}}class Ge extends O{constructor(e){super(),Q(this,e,null,Ce,Z,{})}}function He(n){let e,l,r,s,t,a,i,o,u,c,_;return{c(){e=g("div"),l=g("img"),t=j(),a=g("div"),i=g("h4"),o=G(n[0]),u=j(),c=g("p"),_=G(n[2]),this.h()},l(b){e=v(b,"DIV",{class:!0});var k=w(e);l=v(k,"IMG",{class:!0,src:!0,alt:!0}),t=y(k),a=v(k,"DIV",{});var m=w(a);i=v(m,"H4",{});var I=w(i);o=H(I,n[0]),I.forEach(f),u=y(m),c=v(m,"P",{});var T=w(c);_=H(T,n[2]),T.forEach(f),m.forEach(f),k.forEach(f),this.h()},h(){d(l,"class","h-16 w-16 rounded-full object-cover"),N(l.src,r="/profile-images/"+n[1])||d(l,"src",r),d(l,"alt",s="profile for "+n[0]),d(e,"class","flex gap-4 ")},m(b,k){E(b,e,k),p(e,l),p(e,t),p(e,a),p(a,i),p(i,o),p(a,u),p(a,c),p(c,_)},p(b,[k]){k&2&&!N(l.src,r="/profile-images/"+b[1])&&d(l,"src",r),k&1&&s!==(s="profile for "+b[0])&&d(l,"alt",s),k&1&&he(o,b[0]),k&4&&he(_,b[2])},i:W,o:W,d(b){b&&f(e)}}}function qe(n,e,l){let{name:r}=e,{image:s="default.jpeg"}=e,{title:t}=e;return n.$$set=a=>{"name"in a&&l(0,r=a.name),"image"in a&&l(1,s=a.image),"title"in a&&l(2,t=a.title)},[r,s,t]}class Y extends O{constructor(e){super(),Q(this,e,qe,He,Z,{name:0,image:1,title:2})}}function Be(n){let e;return{c(){e=G("Start lava app")},l(l){e=H(l,"Start lava app")},m(l,r){E(l,e,r)},d(l){l&&f(e)}}}function Le(n){let e;return{c(){e=G("Download template")},l(l){e=H(l,"Download template")},m(l,r){E(l,e,r)},d(l){l&&f(e)}}}function ze(n){let e,l;return{c(){e=g("div"),l=G(`Alongside this applet, a printable template was designed on which you can draw your
					level-curves.`),this.h()},l(r){e=v(r,"DIV",{slot:!0});var s=w(e);l=H(s,`Alongside this applet, a printable template was designed on which you can draw your
					level-curves.`),s.forEach(f),this.h()},h(){d(e,"slot","label")},m(r,s){E(r,e,s),p(e,l)},p:W,d(r){r&&f(e)}}}function Ue(n){let e,l;return e=new we({props:{href:"https://apps.apple.com/nl/app/webxr-viewer/id1295998056",outline:!0,icon:be,$$slots:{label:[Fe],default:[Xe]},$$scope:{ctx:n}}}),{c(){R(e.$$.fragment)},l(r){M(e.$$.fragment,r)},m(r,s){P(e,r,s),l=!0},p(r,s){const t={};s&1&&(t.$$scope={dirty:s,ctx:r}),e.$set(t)},i(r){l||($(e.$$.fragment,r),l=!0)},o(r){D(e.$$.fragment,r),l=!1},d(r){S(e,r)}}}function Xe(n){let e;return{c(){e=G("Download webXR browser")},l(l){e=H(l,"Download webXR browser")},m(l,r){E(l,e,r)},d(l){l&&f(e)}}}function Fe(n){let e,l;return{c(){e=g("div"),l=G(`We have detected that your device does not support webXR natively. The app is usable,
						however we would recommend you to download a special AR browser.`),this.h()},l(r){e=v(r,"DIV",{class:!0,slot:!0});var s=w(e);l=H(s,`We have detected that your device does not support webXR natively. The app is usable,
						however we would recommend you to download a special AR browser.`),s.forEach(f),this.h()},h(){d(e,"class","mt-4"),d(e,"slot","label")},m(r,s){E(r,e,s),p(e,l)},p:W,d(r){r&&f(e)}}}function We(n){let e,l,r,s,t,a,i,o,u,c,_,b,k,m,I,T,J,A,B,le,L,re,z,se,U,ae,X,x;s=new Ee({props:{twClass:"no-underline",href:"/capture/instructions",icon:ye,$$slots:{default:[Be]},$$scope:{ctx:n}}}),_=new we({props:{href:"/Template.pdf",outline:!0,icon:be,$$slots:{label:[ze],default:[Le]},$$scope:{ctx:n}}});let C=Ue(n);return B=new Y({props:{name:"Abel de Bruijn",image:"adebruijn.jpeg",title:"Developer: User Interface and Augmented Reality"}}),L=new Y({props:{name:"Rens Dur",image:"rdur.jpeg",title:"Developer: Model Construction and Image Recognition"}}),z=new Y({props:{name:"Pauline Hengst",title:"Developer: Model Construction and Model Graphics"}}),U=new Y({props:{name:"Julia van der Kris",image:"jvanderkris.jpg",title:"Developer: WebAssembly/deployment and Model Generation"}}),X=new Y({props:{name:"Jonas van Marrewijk",image:"jvanmarrewijk.jpeg",title:"Developer emeritus: Image Recognition"}}),{c(){e=g("div"),l=G(`The climate crisis is upon us! Lava tends to very precisely follow the steepest downwards
		direction, when it flows down the hills of a volcano. Its intense heat makes for a great
		opportunity to generate electricity for nearby cities. Your job is to predict where the lava
		will flow and place steam turbines on its paths. The steam turbines generate higher amounts of
		electricity as the lava reaches closer. Save the world by using this amazing sustainable
		energy-source!

		`),r=g("div"),R(s.$$.fragment),t=j(),a=g("div"),i=g("h2"),o=G("Instructions:"),u=j(),c=g("div"),R(_.$$.fragment),b=j(),C&&C.c(),k=j(),m=g("div"),I=g("h2"),T=G("Credits"),J=j(),A=g("div"),R(B.$$.fragment),le=j(),R(L.$$.fragment),re=j(),R(z.$$.fragment),se=j(),R(U.$$.fragment),ae=j(),R(X.$$.fragment),this.h()},l(h){e=v(h,"DIV",{class:!0});var V=w(e);l=H(V,`The climate crisis is upon us! Lava tends to very precisely follow the steepest downwards
		direction, when it flows down the hills of a volcano. Its intense heat makes for a great
		opportunity to generate electricity for nearby cities. Your job is to predict where the lava
		will flow and place steam turbines on its paths. The steam turbines generate higher amounts of
		electricity as the lava reaches closer. Save the world by using this amazing sustainable
		energy-source!

		`),r=v(V,"DIV",{class:!0});var K=w(r);M(s.$$.fragment,K),K.forEach(f),V.forEach(f),t=y(h),a=v(h,"DIV",{class:!0});var F=w(a);i=v(F,"H2",{});var me=w(i);o=H(me,"Instructions:"),me.forEach(f),u=y(F),c=v(F,"DIV",{class:!0});var ee=w(c);M(_.$$.fragment,ee),b=y(ee),C&&C.l(ee),ee.forEach(f),F.forEach(f),k=y(h),m=v(h,"DIV",{class:!0});var te=w(m);I=v(te,"H2",{});var de=w(I);T=H(de,"Credits"),de.forEach(f),J=y(te),A=v(te,"DIV",{class:!0});var q=w(A);M(B.$$.fragment,q),le=y(q),M(L.$$.fragment,q),re=y(q),M(z.$$.fragment,q),se=y(q),M(U.$$.fragment,q),ae=y(q),M(X.$$.fragment,q),q.forEach(f),te.forEach(f),this.h()},h(){d(r,"class","mt-4 flex justify-end"),d(e,"class","prose mx-auto mt-12 p-8"),d(c,"class","not-prose rounded-xl bg-red-100 p-4"),d(a,"class","prose mx-auto mt-8 px-4"),d(A,"class",""),d(m,"class","prose mx-auto mt-12 mb-12 rounded-xl border-2 border-red-200 p-4")},m(h,V){E(h,e,V),p(e,l),p(e,r),P(s,r,null),E(h,t,V),E(h,a,V),p(a,i),p(i,o),p(a,u),p(a,c),P(_,c,null),p(c,b),C&&C.m(c,null),E(h,k,V),E(h,m,V),p(m,I),p(I,T),p(m,J),p(m,A),P(B,A,null),p(A,le),P(L,A,null),p(A,re),P(z,A,null),p(A,se),P(U,A,null),p(A,ae),P(X,A,null),x=!0},p(h,V){const K={};V&1&&(K.$$scope={dirty:V,ctx:h}),s.$set(K);const F={};V&1&&(F.$$scope={dirty:V,ctx:h}),_.$set(F),C.p(h,V)},i(h){x||($(s.$$.fragment,h),$(_.$$.fragment,h),$(C),$(B.$$.fragment,h),$(L.$$.fragment,h),$(z.$$.fragment,h),$(U.$$.fragment,h),$(X.$$.fragment,h),x=!0)},o(h){D(s.$$.fragment,h),D(_.$$.fragment,h),D(C),D(B.$$.fragment,h),D(L.$$.fragment,h),D(z.$$.fragment,h),D(U.$$.fragment,h),D(X.$$.fragment,h),x=!1},d(h){h&&f(e),S(s),h&&f(t),h&&f(a),S(_),C&&C.d(),h&&f(k),h&&f(m),S(B),S(L),S(z),S(U),S(X)}}}function Je(n){let e,l,r,s,t,a,i,o,u,c,_,b,k;return c=new Ge({}),b=new Te({props:{$$slots:{default:[We]},$$scope:{ctx:n}}}),{c(){e=g("div"),l=g("a"),r=g("img"),t=j(),a=g("a"),i=g("img"),u=j(),R(c.$$.fragment),_=j(),R(b.$$.fragment),this.h()},l(m){e=v(m,"DIV",{class:!0});var I=w(e);l=v(I,"A",{href:!0,rel:!0,target:!0,class:!0});var T=w(l);r=v(T,"IMG",{class:!0,src:!0,alt:!0}),T.forEach(f),t=y(I),a=v(I,"A",{href:!0,rel:!0,target:!0,class:!0});var J=w(a);i=v(J,"IMG",{class:!0,src:!0,alt:!0}),J.forEach(f),I.forEach(f),u=y(m),M(c.$$.fragment,m),_=y(m),M(b.$$.fragment,m),this.h()},h(){d(r,"class","logo-left svelte-ojuhql"),N(r.src,s="PRIME logo tekst met symbolen web.png")||d(r,"src",s),d(r,"alt","prime logo"),d(l,"href","https://www.tudelft.nl/ewi/over-de-faculteit/afdelingen/applied-mathematics/studeren/prime"),d(l,"rel","noopener noreferrer"),d(l,"target","_blank"),d(l,"class","link left-4"),d(i,"class","logo-right svelte-ojuhql"),N(i.src,o="TU Delft logo uitgelijnd web.png")||d(i,"src",o),d(i,"alt","TU Delft logo"),d(a,"href","https://www.tudelft.nl"),d(a,"rel","noopener noreferrer"),d(a,"target","_blank"),d(a,"class","link right-4"),d(e,"class","logo-container svelte-ojuhql")},m(m,I){E(m,e,I),p(e,l),p(l,r),p(e,t),p(e,a),p(a,i),E(m,u,I),P(c,m,I),E(m,_,I),P(b,m,I),k=!0},p(m,[I]){const T={};I&1&&(T.$$scope={dirty:I,ctx:m}),b.$set(T)},i(m){k||($(c.$$.fragment,m),$(b.$$.fragment,m),k=!0)},o(m){D(c.$$.fragment,m),D(b.$$.fragment,m),k=!1},d(m){m&&f(e),m&&f(u),S(c,m),m&&f(_),S(b,m)}}}function Ke(n){return ke(()=>{localStorage&&localStorage.clear()}),[]}class Qe extends O{constructor(e){super(),Q(this,e,Ke,Je,Z,{})}}export{Qe as default};