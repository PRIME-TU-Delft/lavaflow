function v(){}function Q(t,e){for(const n in e)t[n]=e[n];return t}function R(t){return t&&typeof t=="object"&&typeof t.then=="function"}function W(t){return t()}function L(){return Object.create(null)}function b(t){t.forEach(W)}function B(t){return typeof t=="function"}function yt(t,e){return t!=t?e==e:t!==e||t&&typeof t=="object"||typeof t=="function"}let g;function bt(t,e){return g||(g=document.createElement("a")),g.href=e,t===g.href}function U(t){return Object.keys(t).length===0}function V(t,...e){if(t==null)return v;const n=t.subscribe(...e);return n.unsubscribe?()=>n.unsubscribe():n}function gt(t,e,n){t.$$.on_destroy.push(V(e,n))}function xt(t,e,n,r){if(t){const c=D(t,e,n,r);return t[0](c)}}function D(t,e,n,r){return t[1]&&r?Q(n.ctx.slice(),t[1](r(e))):n.ctx}function $t(t,e,n,r){if(t[2]&&r){const c=t[2](r(n));if(e.dirty===void 0)return c;if(typeof c=="object"){const o=[],s=Math.max(e.dirty.length,c.length);for(let u=0;u<s;u+=1)o[u]=e.dirty[u]|c[u];return o}return e.dirty|c}return e.dirty}function wt(t,e,n,r,c,o){if(c){const s=D(e,n,r,o);t.p(s,c)}}function kt(t){if(t.ctx.length>32){const e=[],n=t.ctx.length/32;for(let r=0;r<n;r++)e[r]=-1;return e}return-1}function vt(t){const e={};for(const n in t)e[n]=!0;return e}function Et(t){return t&&B(t.destroy)?t.destroy:v}let E=!1;function X(){E=!0}function Y(){E=!1}function Z(t,e,n,r){for(;t<e;){const c=t+(e-t>>1);n(c)<=r?t=c+1:e=c}return t}function tt(t){if(t.hydrate_init)return;t.hydrate_init=!0;let e=t.childNodes;if(t.nodeName==="HEAD"){const i=[];for(let l=0;l<e.length;l++){const f=e[l];f.claim_order!==void 0&&i.push(f)}e=i}const n=new Int32Array(e.length+1),r=new Int32Array(e.length);n[0]=-1;let c=0;for(let i=0;i<e.length;i++){const l=e[i].claim_order,f=(c>0&&e[n[c]].claim_order<=l?c+1:Z(1,c,_=>e[n[_]].claim_order,l))-1;r[i]=n[f]+1;const a=f+1;n[a]=i,c=Math.max(a,c)}const o=[],s=[];let u=e.length-1;for(let i=n[c]+1;i!=0;i=r[i-1]){for(o.push(e[i-1]);u>=i;u--)s.push(e[u]);u--}for(;u>=0;u--)s.push(e[u]);o.reverse(),s.sort((i,l)=>i.claim_order-l.claim_order);for(let i=0,l=0;i<s.length;i++){for(;l<o.length&&s[i].claim_order>=o[l].claim_order;)l++;const f=l<o.length?o[l]:null;t.insertBefore(s[i],f)}}function et(t,e){t.appendChild(e)}function nt(t,e){if(E){for(tt(t),(t.actual_end_child===void 0||t.actual_end_child!==null&&t.actual_end_child.parentElement!==t)&&(t.actual_end_child=t.firstChild);t.actual_end_child!==null&&t.actual_end_child.claim_order===void 0;)t.actual_end_child=t.actual_end_child.nextSibling;e!==t.actual_end_child?(e.claim_order!==void 0||e.parentNode!==t)&&t.insertBefore(e,t.actual_end_child):t.actual_end_child=e.nextSibling}else(e.parentNode!==t||e.nextSibling!==null)&&t.appendChild(e)}function At(t,e,n){E&&!n?nt(t,e):(e.parentNode!==t||e.nextSibling!=n)&&t.insertBefore(e,n||null)}function O(t){t.parentNode.removeChild(t)}function Ct(t,e){for(let n=0;n<t.length;n+=1)t[n]&&t[n].d(e)}function I(t){return document.createElement(t)}function rt(t){return document.createElementNS("http://www.w3.org/2000/svg",t)}function N(t){return document.createTextNode(t)}function St(){return N(" ")}function jt(){return N("")}function P(t,e,n,r){return t.addEventListener(e,n,r),()=>t.removeEventListener(e,n,r)}function ct(t,e,n){n==null?t.removeAttribute(e):t.getAttribute(e)!==n&&t.setAttribute(e,n)}function Nt(t,e,n){e in t?t[e]=typeof t[e]=="boolean"&&n===""?!0:n:ct(t,e,n)}function Mt(t){return t===""?null:+t}function it(t){return Array.from(t.childNodes)}function st(t){t.claim_info===void 0&&(t.claim_info={last_index:0,total_claimed:0})}function F(t,e,n,r,c=!1){st(t);const o=(()=>{for(let s=t.claim_info.last_index;s<t.length;s++){const u=t[s];if(e(u)){const i=n(u);return i===void 0?t.splice(s,1):t[s]=i,c||(t.claim_info.last_index=s),u}}for(let s=t.claim_info.last_index-1;s>=0;s--){const u=t[s];if(e(u)){const i=n(u);return i===void 0?t.splice(s,1):t[s]=i,c?i===void 0&&t.claim_info.last_index--:t.claim_info.last_index=s,u}}return r()})();return o.claim_order=t.claim_info.total_claimed,t.claim_info.total_claimed+=1,o}function H(t,e,n,r){return F(t,c=>c.nodeName===e,c=>{const o=[];for(let s=0;s<c.attributes.length;s++){const u=c.attributes[s];n[u.name]||o.push(u.name)}o.forEach(s=>c.removeAttribute(s))},()=>r(e))}function qt(t,e,n){return H(t,e,n,I)}function zt(t,e,n){return H(t,e,n,rt)}function ot(t,e){return F(t,n=>n.nodeType===3,n=>{const r=""+e;if(n.data.startsWith(r)){if(n.data.length!==r.length)return n.splitText(r.length)}else n.data=r},()=>N(e),!0)}function Lt(t){return ot(t," ")}function Pt(t,e){e=""+e,t.wholeText!==e&&(t.data=e)}function Tt(t,e){t.value=e==null?"":e}function Wt(t,e,n,r){n===null?t.style.removeProperty(e):t.style.setProperty(e,n,r?"important":"")}let x;function ut(){if(x===void 0){x=!1;try{typeof window!="undefined"&&window.parent&&window.parent.document}catch{x=!0}}return x}function Bt(t,e){getComputedStyle(t).position==="static"&&(t.style.position="relative");const r=I("iframe");r.setAttribute("style","display: block; position: absolute; top: 0; left: 0; width: 100%; height: 100%; overflow: hidden; border: 0; opacity: 0; pointer-events: none; z-index: -1;"),r.setAttribute("aria-hidden","true"),r.tabIndex=-1;const c=ut();let o;return c?(r.src="data:text/html,<script>onresize=function(){parent.postMessage(0,'*')}<\/script>",o=P(window,"message",s=>{s.source===r.contentWindow&&e()})):(r.src="about:blank",r.onload=()=>{o=P(r.contentWindow,"resize",e)}),et(t,r),()=>{(c||o&&r.contentWindow)&&o(),O(r)}}function Dt(t,e,n){t.classList[n?"add":"remove"](e)}function lt(t,e,{bubbles:n=!1,cancelable:r=!1}={}){const c=document.createEvent("CustomEvent");return c.initCustomEvent(t,n,r,e),c}function Ot(t,e=document.body){return Array.from(e.querySelectorAll(t))}let y;function d(t){y=t}function m(){if(!y)throw new Error("Function called outside component initialization");return y}function It(t){m().$$.on_mount.push(t)}function Ft(t){m().$$.after_update.push(t)}function Ht(t){m().$$.on_destroy.push(t)}function Gt(){const t=m();return(e,n,{cancelable:r=!1}={})=>{const c=t.$$.callbacks[e];if(c){const o=lt(e,n,{cancelable:r});return c.slice().forEach(s=>{s.call(t,o)}),!o.defaultPrevented}return!0}}function Jt(t,e){return m().$$.context.set(t,e),e}function Kt(t,e){const n=t.$$.callbacks[e.type];n&&n.slice().forEach(r=>r.call(this,e))}const p=[],T=[],w=[],C=[],G=Promise.resolve();let S=!1;function J(){S||(S=!0,G.then(M))}function Qt(){return J(),G}function j(t){w.push(t)}function Rt(t){C.push(t)}const A=new Set;let $=0;function M(){const t=y;do{for(;$<p.length;){const e=p[$];$++,d(e),at(e.$$)}for(d(null),p.length=0,$=0;T.length;)T.pop()();for(let e=0;e<w.length;e+=1){const n=w[e];A.has(n)||(A.add(n),n())}w.length=0}while(p.length);for(;C.length;)C.pop()();S=!1,A.clear(),d(t)}function at(t){if(t.fragment!==null){t.update(),b(t.before_update);const e=t.dirty;t.dirty=[-1],t.fragment&&t.fragment.p(t.ctx,e),t.after_update.forEach(j)}}const k=new Set;let h;function ft(){h={r:0,c:[],p:h}}function dt(){h.r||b(h.c),h=h.p}function K(t,e){t&&t.i&&(k.delete(t),t.i(e))}function _t(t,e,n,r){if(t&&t.o){if(k.has(t))return;k.add(t),h.c.push(()=>{k.delete(t),r&&(n&&t.d(1),r())}),t.o(e)}}function Ut(t,e){const n=e.token={};function r(c,o,s,u){if(e.token!==n)return;e.resolved=u;let i=e.ctx;s!==void 0&&(i=i.slice(),i[s]=u);const l=c&&(e.current=c)(i);let f=!1;e.block&&(e.blocks?e.blocks.forEach((a,_)=>{_!==o&&a&&(ft(),_t(a,1,1,()=>{e.blocks[_]===a&&(e.blocks[_]=null)}),dt())}):e.block.d(1),l.c(),K(l,1),l.m(e.mount(),e.anchor),f=!0),e.block=l,e.blocks&&(e.blocks[o]=l),f&&M()}if(R(t)){const c=m();if(t.then(o=>{d(c),r(e.then,1,e.value,o),d(null)},o=>{if(d(c),r(e.catch,2,e.error,o),d(null),!e.hasCatch)throw o}),e.current!==e.pending)return r(e.pending,0),!0}else{if(e.current!==e.then)return r(e.then,1,e.value,t),!0;e.resolved=t}}function Vt(t,e,n){const r=e.slice(),{resolved:c}=t;t.current===t.then&&(r[t.value]=c),t.current===t.catch&&(r[t.error]=c),t.block.p(r,n)}function Xt(t,e){const n={},r={},c={$$scope:1};let o=t.length;for(;o--;){const s=t[o],u=e[o];if(u){for(const i in s)i in u||(r[i]=1);for(const i in u)c[i]||(n[i]=u[i],c[i]=1);t[o]=u}else for(const i in s)c[i]=1}for(const s in r)s in n||(n[s]=void 0);return n}function Yt(t){return typeof t=="object"&&t!==null?t:{}}function Zt(t,e,n){const r=t.$$.props[e];r!==void 0&&(t.$$.bound[r]=n,n(t.$$.ctx[r]))}function te(t){t&&t.c()}function ee(t,e){t&&t.l(e)}function ht(t,e,n,r){const{fragment:c,on_mount:o,on_destroy:s,after_update:u}=t.$$;c&&c.m(e,n),r||j(()=>{const i=o.map(W).filter(B);s?s.push(...i):b(i),t.$$.on_mount=[]}),u.forEach(j)}function mt(t,e){const n=t.$$;n.fragment!==null&&(b(n.on_destroy),n.fragment&&n.fragment.d(e),n.on_destroy=n.fragment=null,n.ctx=[])}function pt(t,e){t.$$.dirty[0]===-1&&(p.push(t),J(),t.$$.dirty.fill(0)),t.$$.dirty[e/31|0]|=1<<e%31}function ne(t,e,n,r,c,o,s,u=[-1]){const i=y;d(t);const l=t.$$={fragment:null,ctx:null,props:o,update:v,not_equal:c,bound:L(),on_mount:[],on_destroy:[],on_disconnect:[],before_update:[],after_update:[],context:new Map(e.context||(i?i.$$.context:[])),callbacks:L(),dirty:u,skip_bound:!1,root:e.target||i.$$.root};s&&s(l.root);let f=!1;if(l.ctx=n?n(t,e.props||{},(a,_,...q)=>{const z=q.length?q[0]:_;return l.ctx&&c(l.ctx[a],l.ctx[a]=z)&&(!l.skip_bound&&l.bound[a]&&l.bound[a](z),f&&pt(t,a)),_}):[],l.update(),f=!0,b(l.before_update),l.fragment=r?r(l.ctx):!1,e.target){if(e.hydrate){X();const a=it(e.target);l.fragment&&l.fragment.l(a),a.forEach(O)}else l.fragment&&l.fragment.c();e.intro&&K(t.$$.fragment),ht(t,e.target,e.anchor,e.customElement),Y(),M()}d(i)}class re{$destroy(){mt(this,1),this.$destroy=v}$on(e,n){const r=this.$$.callbacks[e]||(this.$$.callbacks[e]=[]);return r.push(n),()=>{const c=r.indexOf(n);c!==-1&&r.splice(c,1)}}$set(e){this.$$set&&!U(e)&&(this.$$.skip_bound=!0,this.$$set(e),this.$$.skip_bound=!1)}}export{Et as $,Yt as A,mt as B,Q as C,Qt as D,v as E,xt as F,wt as G,kt as H,$t as I,nt as J,bt as K,P as L,b as M,Nt as N,Ot as O,Ht as P,rt as Q,zt as R,re as S,Dt as T,vt as U,Kt as V,Ct as W,T as X,Zt as Y,Rt as Z,Gt as _,it as a,gt as a0,j as a1,Bt as a2,Tt as a3,Mt as a4,Ut as a5,Vt as a6,ct as b,qt as c,O as d,I as e,Wt as f,At as g,ot as h,ne as i,Pt as j,St as k,jt as l,Lt as m,ft as n,_t as o,dt as p,K as q,Jt as r,yt as s,N as t,Ft as u,It as v,te as w,ee as x,ht as y,Xt as z};
