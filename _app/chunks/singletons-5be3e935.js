import{s as p,E as f}from"./index-cd071420.js";const n=[];function h(s,u=f){let o;const i=new Set;function r(e){if(p(s,e)&&(s=e,o)){const c=!n.length;for(const t of i)t[1](),n.push(t,s);if(c){for(let t=0;t<n.length;t+=2)n[t][0](n[t+1]);n.length=0}}}function b(e){r(e(s))}function l(e,c=f){const t=[e,c];return i.add(t),i.size===1&&(o=u(r)||f),e(s),()=>{i.delete(t),i.size===0&&(o(),o=null)}}return{set:r,update:b,subscribe:l}}let a;function g(s){a=s.client}export{a as c,g as i,h as w};
