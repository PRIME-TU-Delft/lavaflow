import{S as d,i as b,s as h,e as m,b as y,v as S,d as i,H as k,f as P,g as c,h as w,I as C,y as $,z,A as E,B as N}from"./index.525666b8.mjs";import{P as W}from"./P5.c5dcb965.mjs";import{s as q}from"./sizeStore.02e4eb31.mjs";function _(a){let s,n;return s=new W({props:{sketch:a[1]}}),{c(){$(s.$$.fragment)},l(r){z(s.$$.fragment,r)},m(r,t){E(s,r,t),n=!0},p:k,i(r){n||(c(s.$$.fragment,r),n=!0)},o(r){i(s.$$.fragment,r),n=!1},d(r){N(s,r)}}}function x(a){let s=a[0],n,r,t=_(a);return{c(){t.c(),n=m()},l(o){t.l(o),n=m()},m(o,e){t.m(o,e),y(o,n,e),r=!0},p(o,[e]){e&1&&h(s,s=o[0])?(S(),i(t,1,1,k),P(),t=_(o),t.c(),c(t,1),t.m(n.parentNode,n)):t.p(o,e)},i(o){r||(c(t),r=!0)},o(o){i(t),r=!1},d(o){o&&w(n),t.d(o)}}}function A(a,s,n){let r;C(a,q,e=>n(0,r=e));let{curves:t}=s;const o=e=>{e.setup=()=>{const[l,u]=[r?.width,r?.height];if(!t||t.length===0||!l||!u){console.log("No curves or size available. Can't draw.");return}e.createCanvas(l,u),t.forEach(p=>{e.strokeWeight(1),e.stroke(30),e.fill(30),e.strokeWeight(4),e.noFill(),e.beginShape();for(let f of p){let v=f[0],g=f[1];e.vertex(v,g)}e.endShape(e.CLOSE)})}};return a.$$set=e=>{"curves"in e&&n(2,t=e.curves)},[r,o,t]}class I extends d{constructor(s){super(),b(this,s,A,x,h,{curves:2})}}export{I as P};