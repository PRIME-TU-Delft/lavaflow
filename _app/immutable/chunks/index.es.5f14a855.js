import{w as Qt}from"./index.204dd160.js";const ra=Qt();/**
 * GammaCV v0.5.3
 * @license MIT
 * @author Arkadiy Pilguk(apilguk@gmail.com)
 * @author Mihail Zachepilo(mihailzachepilo@gmail.com)
 * Copyright 2018-2023 Peculiar Ventures.
 * All rights reserved.
 *//**
 * @license MIT
 * @author Arkadiy Pilguk(apilguk@gmail.com)
 * @author Mihail Zachepilo(mihailzachepilo@gmail.com)
 * Copyright 2018 Peculiar Ventures and Pentatonica.
 * All rights reserved.
 */class D{static GlobalCountIncrease(){return D.GlobalNodesCount+=1,D.GlobalNodesCount}constructor(t){this.id=D.GlobalCountIncrease(),this.name=`${t}:${this.id}`}}D.GlobalNodesCount=0;/**
 * @license MIT
 * @author Arkadiy Pilguk(apilguk@gmail.com)
 * @author Mihail Zachepilo(mihailzachepilo@gmail.com)
 * Copyright 2018 Peculiar Ventures and Pentatonica.
 * All rights reserved.
 */class te{constructor(t,e,a,n){this.gl=t,this.name=a,this.dtype=n,this.location=t.getUniformLocation(e,this.name)}set(t){const e=this.gl;switch(this.dtype){case"int":e.uniform1i(this.location,t);break;case"float":e.uniform1f(this.location,t);break;case"vec2":e.uniform2fv(this.location,t);break;case"vec3":e.uniform3fv(this.location,t);break;case"vec4":e.uniform4fv(this.location,t);break;case"mat3":e.uniformMatrix3fv(this.location,!1,t);break;case"mat4":e.uniformMatrix4fv(this.location,!1,t);break;default:return!1}return!0}}/**
 * @license MIT
 * @author Arkadiy Pilguk(apilguk@gmail.com)
 * @author Mihail Zachepilo(mihailzachepilo@gmail.com)
 * Copyright 2018 Peculiar Ventures and Pentatonica.
 * All rights reserved.
 */class Lt{constructor(t,e,a,n){this.program=e,this.gl=t,this.name=a,this.dtype=n,this.location=t.getAttribLocation(this.program,this.name),this.ctx=t.createBuffer(),this.empty=new ArrayBuffer(1),n==="float"||n==="int"?this.size=1:(this.size=parseInt(/\d/g.exec(n)[0],10),t.enableVertexAttribArray(this.location))}set(t){const e=this.gl;this.bind(this.ctx),this.dtype==="int"?e.bufferData(e.ELEMENT_ARRAY_BUFFER,new Uint16Array(t),e.STATIC_DRAW):e.bufferData(e.ARRAY_BUFFER,new Float32Array(t),e.STATIC_DRAW)}bind(){const t=this.gl;this.dtype==="int"?t.bindBuffer(t.ELEMENT_ARRAY_BUFFER,this.ctx):(t.bindBuffer(t.ARRAY_BUFFER,this.ctx),t.vertexAttribPointer(this.location,this.size,t.FLOAT,!1,0,0))}unbind(){const t=this.gl;this.dtype==="int"?t.bindBuffer(t.ELEMENT_ARRAY_BUFFER,null):(t.bindBuffer(t.ARRAY_BUFFER,null),t.vertexAttribPointer(this.location,this.size,t.FLOAT,!1,0,0))}disable(){this.gl.disableVertexAttribArray(this.ctx)}enable(){this.gl.enableVertexAttribArray(this.ctx)}delete(){this.gl.deleteBuffer(this.ctx),this.program=null,this.gl=null,this.ctx=null}}var ee="precision highp float;attribute vec3 aVertexPosition;attribute vec2 aTextureCoords;varying vec2 texCoords;void main(void){texCoords=aTextureCoords;gl_Position=vec4(aVertexPosition,1.0);}";/**
 * @license MIT
 * @author Arkadiy Pilguk(apilguk@gmail.com)
 * @author Mihail Zachepilo(mihailzachepilo@gmail.com)
 * Copyright 2018 Peculiar Ventures and Pentatonica.
 * All rights reserved.
 */const Vt="Error: An error occurred compiling the shaders: ";function ae(s){const t=/\d+\|(\s+)/.exec(s);return t?" ".repeat(t[1].length):" ".repeat(2)}function Xt(s){let t=s.split(`
`);const e=(t.length+1).toString().length;return t=t.map((a,n)=>`${(n+1).toString().padStart(e)}|  ${a}`),t}function se(s){let t=0,e=0;for(let a=0;a<s.length;a+=1)/ERROR/.exec(s[a])&&(t+=1),/WARNING/.exec(s[a])&&(e+=1);return{errCount:t,warnCount:e}}function ne(s,t,e=!0){const a=Xt(s),n=(a.length+1).toString().length;let i=t.toString();const r=[],o=[];i.startsWith(Vt)&&(i=i.substr(Vt.length));const l=i.split(`
`),h=se(l);let f=0;for(let g=0;g<l.length;g+=1){const y=l[g],j=/0:(\d+)/.exec(y);if(j){const G=+j[1]+f,Tt=`${" ".repeat(n)}|${ae(a[G-1])}`;r.push(`${y}
${a[G-2]}
${a[G-1]}
${Tt}^
${a[G]}`);const bt=e?"%c":"";a.splice(G,0,`${bt}${Tt}^--${y}${bt}`),e&&(o.push("color: red;"),o.push("color: inherit;")),f+=1}}return{fullText:a.join(`
`),firstError:r[0],errorsStats:h,fullTextStyle:o}}function ie(s,t,e){try{const a=ne(s,e),n=a.errorsStats;console.group(`Error: An error occurred compiling the shader ${t}: ${n.errCount} ERRORS, ${n.warnCount} WARNINGS`),console.log(a.firstError),console.groupCollapsed("Show more"),console.log(a.fullText,...a.fullTextStyle),console.groupEnd(),console.groupEnd()}catch{console.warn("Unable to process GLSG compiling error.")}}/**
 * @license MIT
 * @author Arkadiy Pilguk(apilguk@gmail.com)
 * @author Mihail Zachepilo(mihailzachepilo@gmail.com)
 * Copyright 2018 Peculiar Ventures and Pentatonica.
 * All rights reserved.
 */const re=["pickCurrentValue","pickValue","float"],u=(s,t)=>{if(!s)throw new Error(t)},oe=(s,t)=>{u(s,`GammaCV: DOMFeature not supported, "${t}" is not supported in current environment`)},jt=s=>{oe(document&&document.createElement,s)},le=()=>{u(typeof OffscreenCanvas=="function","OffscreenCanvas")},Ut=s=>Array.isArray(s)&&s.length>0&&!s.some(t=>t%1!==0),lt=s=>s instanceof X,kt=s=>s instanceof Ce,N=s=>s instanceof E,Wt=s=>typeof HTMLVideoElement=="function"?s instanceof HTMLVideoElement:!1,Mt=s=>typeof HTMLCanvasElement=="function"?s instanceof HTMLCanvasElement:"getContext"in s,he=s=>re.includes(s),Ft=s=>/^[A-Za-z](\w+)?$/.test(s);function ce(s,t){console.warn(`GammaCV Deprecation Warning: "${s}" is deprecated${t?`, ${t}`:""}. "${s}" will be removed in next major version.`)}function Ot(){try{return jt("canvas"),document.createElement("canvas")}catch{return le(),new OffscreenCanvas(1,1)}}function ue(){return jt("img"),document.createElement("img")}/**
 * @license MIT
 * @author Arkadiy Pilguk(apilguk@gmail.com)
 * @author Mihail Zachepilo(mihailzachepilo@gmail.com)
 * Copyright 2018 Peculiar Ventures and Pentatonica.
 * All rights reserved.
 */const Yt={};function fe(){let s,t=!1;try{const a=Ot().getContext("webgl");if(!a||!a.getExtension("OES_texture_float"))return!1;const n=a.createFramebuffer(),i=a.createTexture();Yt.MAX_TEXTURE_SIZE=a.getParameter(a.MAX_TEXTURE_SIZE),a.bindTexture(a.TEXTURE_2D,i),a.texImage2D(a.TEXTURE_2D,0,a.RGBA,1,1,0,a.RGBA,a.FLOAT,null),a.bindFramebuffer(a.FRAMEBUFFER,n),a.framebufferTexture2D(a.FRAMEBUFFER,a.COLOR_ATTACHMENT0,a.TEXTURE_2D,i,0),t=a.checkFramebufferStatus(a.FRAMEBUFFER)===a.FRAMEBUFFER_COMPLETE,a.readPixels(0,0,1,1,a.RGBA,a.FLOAT,new Float32Array(4)),s=a.getError()===a.NO_ERROR}catch{s=!1}return t&&s}const de={SUPPORTS_FLOAT_TEXTURES:fe(),DEBUG:!1,MAX_TEXTURE_SIZE:Yt.MAX_TEXTURE_SIZE},P=Object.assign({},de);/**
 * @license MIT
 * @author Arkadiy Pilguk(apilguk@gmail.com)
 * @author Mihail Zachepilo(mihailzachepilo@gmail.com)
 * Copyright 2018 Peculiar Ventures and Pentatonica.
 * All rights reserved.
 */function qt(s){let t=`
void main(void) {
  vec2 coords = gl_FragCoord.xy - 0.5;
  vec4 result = operation(coords.y, coords.x);

  gl_FragColor = result;
}
  `;return!P.SUPPORTS_FLOAT_TEXTURES&&s.dtype==="float32"&&(t=`
    void main(void) {
      vec2 coords = gl_FragCoord.xy;

      highp float ox = floor(coords.x / 4.0);
      float dx = floor(coords.x - ox * 4.0 + 0.5);
    
      vec4 result = operation(coords.y - 0.5, floor((coords.x - 0.5) / 4.0));

      float value;

      if (dx == 1.0) {
        value = result.r;
      } else if (dx == 2.0) {
        value = result.g;
      } else if (dx == 3.0) {
        value = result.b;
      } else if (dx == 4.0) {
        value = result.a;
      }
    
      gl_FragColor = encode_float(value);
    }
    `),t}var pe="precision highp float;highp vec4 encode_float(highp float f){if(f==1./0.){return vec4(0.0,0.0,128.0,127.0)/255.0;}highp vec4 rgba;highp float e=5.0;highp float F=abs(f);highp float sign=step(0.0,-f);highp float exponent=floor(log2(F));highp float mantissa=(exp2(-exponent)*F);exponent=floor(log2(F)+127.0)+floor(log2(mantissa));rgba[0]=128.0*sign+floor(exponent*exp2(-1.0));rgba[1]=128.0*mod(exponent,2.0)+mod(floor(mantissa*64.0*2.0),128.0);rgba[2]=floor(mod(floor(mantissa*exp2(23.0-8.0)),exp2(8.0)));rgba[3]=floor(exp2(23.0)*mod(mantissa,exp2(-15.0)));return rgba.abgr/255.0;}float decode_float(highp vec4 rgba){rgba=rgba.abgr*255.0;highp float sign=1.0-step(128.0,rgba[0])*2.0;highp float exponent=2.0*mod(rgba[0],128.0)+step(128.0,rgba[1])-127.0;exponent=floor(exponent+0.5);highp float mantissa=mod(rgba[1],128.0)*32768.0*2.0+rgba[2]*256.0+rgba[3]+float(0x800000);highp float result=sign*mantissa*exp2(-23.0)*exp2(exponent);return result;}";/**
 * @license MIT
 * @author Arkadiy Pilguk(apilguk@gmail.com)
 * @author Mihail Zachepilo(mihailzachepilo@gmail.com)
 * Copyright 2018 Peculiar Ventures and Pentatonica.
 * All rights reserved.
 */function ge(s){const t=Object.keys(s.input),e=[];for(let a=0;a<t.length;a+=1){const n=t[a];if(!s.input[n].shape)continue;const i=[...s.input[n].shape],r=i[1].toFixed(1),o=i[0].toFixed(1),l=(i[1]*4).toFixed(1);let h=(f,g,y)=>`${f} ${g}_${n}(float y, float x) {
	return texture2D(${n}, vec2((x + 0.5) / ${r}, (y + 0.5) / ${o}))${y};
}`;!P.SUPPORTS_FLOAT_TEXTURES&&s.input[n].dtype==="float32"&&(h=(f,g,y)=>`
        ${f} ${g}_${n}(float y, float x) {
          float r = decode_float(texture2D(${n}, vec2((x * 4.0 + 0.5) / ${l}, y / ${o})));
          float g = decode_float(texture2D(${n}, vec2((x * 4.0 + 1.5) / ${l}, y / ${o})));
          float b = decode_float(texture2D(${n}, vec2((x * 4.0 + 2.5) / ${l}, y / ${o})));
          float a = decode_float(texture2D(${n}, vec2((x * 4.0 + 3.5) / ${l}, y / ${o})));

          return vec4(r, g, b, a)${y};
        }
      `),e.push(h("vec4","pickValue","")),e.push(h("float","pickScalarValue",".x"))}return e.join(`
`)}/**
 * @license MIT
 * @author Arkadiy Pilguk(apilguk@gmail.com)
 * @author Mihail Zachepilo(mihailzachepilo@gmail.com)
 * Copyright 2018 Peculiar Ventures and Pentatonica.
 * All rights reserved.
 */const ye=()=>pe;var $t=Object.freeze({__proto__:null,float:ye,main:qt,pickValue:ge});/**
 * @license MIT
 * @author Arkadiy Pilguk(apilguk@gmail.com)
 * @author Mihail Zachepilo(mihailzachepilo@gmail.com)
 * Copyright 2018 Peculiar Ventures and Pentatonica.
 * All rights reserved.
 */function Kt(s){return["bool","int","uint","float","double","vec2","vec3","vec4","mat2","mat3","mat4","sampler2D"].indexOf(s)>=0}function xe(s){let t=typeof s;s=String(s);const e=/^(vec\d|mat\d)\([^)]+\)$/.exec(s);return e?t=e[1]:/^\d+$/.exec(s)?t="int":/^\d+\.(\d+)?$/.exec(s)?t="float":t==="boolean"&&(t="bool"),t}function me(s){const t=Object.assign({},s.uniform),e=Object.keys(s.input);let a=`precision highp float;
`;for(let r=0;r<e.length;r+=1){const o=e[r];t[o]={dtype:"sampler2D"}}const n=Object.keys(t);for(let r=0;r<n.length;r+=1){const o=n[r];if(!Kt(t[o].dtype))throw new Error(`KernelConstructor: Uniform ${o} has invalid type "${t[o].dtype}"`);a+=`uniform ${t[o].dtype} ${o};
`}a+=`varying vec2 texCoords;
`;const i=Object.keys(s.constant);for(let r=0;r<i.length;r+=1){const o=i[r];let l=s.constant[o];typeof l==="number"&&l%1===0&&(l=l.toFixed(1));const f=xe(l);if(!Kt(f))throw new Error(`KernelConstructor: Constant ${o}, has invalid type "${f}"`);a+=`#define ${o} ${l}
`}return a}function ve(s){const e=[];return P.SUPPORTS_FLOAT_TEXTURES||e.push("float"),e.concat(s.chunks.filter((n,i,r)=>r.indexOf(n)===i)).map(n=>{const i=` Chunk ${n} `,r=35-i.length,o=`${"-".repeat(Math.floor(r/2))}${i}${"-".repeat(Math.ceil(r/2))}`;if(typeof $t[n]=="function")return`/*${o}*/
${$t[n](s)}
/*${"-".repeat(35)}*/`;throw new TypeError(`KernelConstructor: Chunk "${n}" is not a function`)}).join(`
`)}function Ee(s){return!!/void main\(([^)]+)?\)([\s]+)?{/.exec(s)}function Te(s){let t;if(Ee(s.kernel))t=s.kernel;else{const e=me(s),a=ve(s),n=qt(s);t=[e,a,s.kernel,n].join(`

`)}return P.DEBUG&&(console.groupCollapsed(s.name),console.log(Xt(t).join(`
`)),console.groupEnd()),t}/**
 * @license MIT
 * @author Arkadiy Pilguk(apilguk@gmail.com)
 * @author Mihail Zachepilo(mihailzachepilo@gmail.com)
 * Copyright 2018 Peculiar Ventures and Pentatonica.
 * All rights reserved.
 */class X extends D{constructor(t){u(typeof t<"u","Operation: Operation should have a name"),super(t),this.dtype=null,this.input={},this.uniform={},this.constant={},this.chunks=[],this.inputKeys=[],this.isInitialized=!1,this.lastCtx=Math.random(),this.cache=!0}run(t,e,a){u(this.isInitialized,"Operation: Unable to run uninitialized operation.");const n=this.gl,i=t.texture[this.name];if(e===this.lastCtx&&this.cache&&!a)return i.bind(this.program,!1,this.inputKeys.length),this.bindBuffer(),!1;this.lastCtx=e,n.useProgram(this.program);for(let r=0;r<this.inputKeys.length;r+=1){const o=this.inputKeys[r],l=this.input[o],h=l.name,f=t.texture[h];f.bind(this.program,o,r),N(l)&&f.set(l),kt(l)&&f.set(l.media)}return i.bind(this.program,!1,this.inputKeys.length),this.bindBuffer(),P.SUPPORTS_FLOAT_TEXTURES?n.viewport(0,0,this.shape[1],this.shape[0]):n.viewport(0,0,(this.dtype==="float32"?4:1)*this.shape[1],this.shape[0]),n.clearColor(0,0,0,1),n.clear(n.COLOR_BUFFER_BIT|n.DEPTH_BUFFER_BIT),n.drawElements(n.TRIANGLES,6,n.UNSIGNED_SHORT,0),!0}unbindBuffer(){const t=this.gl;t.bindFramebuffer(t.FRAMEBUFFER,null)}bindBuffer(){const t=this.gl;t.bindFramebuffer(t.FRAMEBUFFER,this.framebuffer)}init(t){if(!this.isInitialized){if(this.gl=t,this.program=t.createProgram(),this.framebuffer=t.createFramebuffer(),this.isInitialized)return!1;this.constant.OUT_VIEW=`vec2(${this.shape[1]}, ${this.shape[0]})`,this.kernel=Te(this);try{this.vertexShader=this.getShader("vertex",ee),t.attachShader(this.program,this.vertexShader),this.fragmentShader=this.getShader("fragment",this.kernel),t.attachShader(this.program,this.fragmentShader),t.linkProgram(this.program),t.useProgram(this.program)}catch(a){throw ie(this.kernel,this.name,a),new Error(`Operation: Error during shader compilation.
${a.message}`)}this.attributes={aVertexPosition:new Lt(this.gl,this.program,"aVertexPosition","vec3"),aTextureCoords:new Lt(this.gl,this.program,"aTextureCoords","vec2"),aIndices:new Lt(this.gl,this.program,"aIndices","int")},this.attributes.aVertexPosition.set([1,1,0,-1,1,0,-1,-1,0,1,-1,0]),this.attributes.aTextureCoords.set([1,1,0,1,0,0,1,0]),this.attributes.aIndices.set([0,1,2,0,2,3]);const e=Object.keys(this.uniform);for(let a=0;a<e.length;a+=1){const n=this.uniform[e[a]];this.uniform[e[a]]=new te(this.gl,this.program,n.name,n.dtype),n.defaultValue&&this.uniform[e[a]].set(n.defaultValue)}this.isInitialized=!0}return!0}getShader(t,e){const a=this.gl;let n=null;if(t==="fragment"?n=a.createShader(a.FRAGMENT_SHADER):n=a.createShader(a.VERTEX_SHADER),a.shaderSource(n,e),a.compileShader(n),!a.getShaderParameter(n,a.COMPILE_STATUS))throw new Error(`Operation: An error occurred compiling the shaders.
${a.getShaderInfoLog(n)}`);return n}traverse(t,e){const a=Object.keys(this.input);for(let n=0;n<a.length;n+=1){const i=a[n];lt(this.input[i])?this.input[i].traverse(t,e):t(this.input[i],e)}t(this,e)}getDependencies(){const t=[],e=Object.keys(this.input);for(let a=0;a<e.length;a+=1){const n=e[a];if(lt(this.input[n])){const i=this.input[n].getDependencies();for(let r=0;r<i.length;r+=1)t.indexOf(i[r])===-1&&t.push(i[r]);t.concat(i)}}return t.push(this.name),t}assignInput(t,e){this.input[t]=e,this.inputKeys.indexOf(t)===-1&&this.inputKeys.push(t)}cloneProp(t){const e=Object.keys(this[t]),a={};for(let n=0;n<e.length;n+=1){const i=e[n];a[i]=this[t][i]}return a}destroy(){this.program&&this.gl.deleteProgram(this.program),this.vertexShader&&this.gl.deleteShader(this.vertexShader),this.fragmentShader&&this.gl.deleteShader(this.fragmentShader),this.framebuffer&&this.gl.deleteFramebuffer(this.framebuffer)}clone(){const t=new X(this.name.split(":")[0]);return t.input=this.cloneProp("input"),t.uniform=this.cloneProp("uniform"),t.constant=this.cloneProp("constant"),t.dtype=this.dtype,t.kernel=this.kernel,t.chunks=this.chunks,t}}/**
 * @license MIT
 * @author Arkadiy Pilguk(apilguk@gmail.com)
 * @author Mihail Zachepilo(mihailzachepilo@gmail.com)
 * Copyright 2018 Peculiar Ventures and Pentatonica.
 * All rights reserved.
 */class be{constructor(t,e,a,n){if(t==="float32"||t==="uint8")this.unit=a,this.dtype=t,this.gl=e,this.ctx=e.createTexture(),this.shape=n,e.bindTexture(e.TEXTURE_2D,this.ctx),e.texParameteri(e.TEXTURE_2D,e.TEXTURE_MAG_FILTER,e.NEAREST),e.texParameteri(e.TEXTURE_2D,e.TEXTURE_MIN_FILTER,e.NEAREST),e.texParameteri(e.TEXTURE_2D,e.TEXTURE_WRAP_S,e.CLAMP_TO_EDGE),e.texParameteri(e.TEXTURE_2D,e.TEXTURE_WRAP_T,e.CLAMP_TO_EDGE),this.allocate();else throw new Error(`GPUTexture: Invalid texture type, currently supported is: float32, uint8, but got ${t} `)}allocate(){const t=this.gl;let e=this.shape[1],a=t.UNSIGNED_BYTE;this.dtype==="float32"&&(P.SUPPORTS_FLOAT_TEXTURES?a=t.FLOAT:e*=4),t.texImage2D(t.TEXTURE_2D,0,t.RGBA,e,this.shape[0],0,t.RGBA,a,null)}set(t=null){const e=this.gl;if(Wt(t)||Mt(t))e.texImage2D(e.TEXTURE_2D,0,e.RGBA,e.RGBA,e.UNSIGNED_BYTE,t);else{let a=t.shape[1],n=e.UNSIGNED_BYTE,i=t.data;t.dtype==="float32"&&(P.SUPPORTS_FLOAT_TEXTURES?n=e.FLOAT:(a*=4,i=t.uint8View)),e.texImage2D(e.TEXTURE_2D,0,e.RGBA,a,this.shape[0],0,e.RGBA,n,i)}}bind(t,e,a){const n=this.gl;if(e){const i=n.getUniformLocation(t,e);n.uniform1i(i,a)}n.activeTexture(n.TEXTURE0+a),n.bindTexture(n.TEXTURE_2D,this.ctx),this.unit=a}unbind(){const t=this.gl;t.activeTexture(t.TEXTURE0+this.unit),t.bindTexture(t.TEXTURE_2D,null)}delete(){const t=this.gl;t.deleteTexture(this.ctx),this.gl=null,this.program=null,this.ctx=null,t.bindTexture(t.TEXTURE_2D,null)}}/**
 * @license MIT
 * @author Arkadiy Pilguk(apilguk@gmail.com)
 * @author Mihail Zachepilo(mihailzachepilo@gmail.com)
 * Copyright 2018 Peculiar Ventures and Pentatonica.
 * All rights reserved.
 */class oa{constructor(){this.canvas=Ot(),this.canvas.width=1,this.canvas.height=1,this.initWebGL(this.canvas),this.operation={},this.texture={},this.textureCount=0}initWebGL(t,e){this.canvas=t;const a=this.canvas.getContext("webgl",e);u(!!a,"Session: WebGL not supported.");const n=a.getExtension("OES_texture_float");u(!!n,"Session: Unable to find extension OES_texture_float"),a.clear(a.COLOR_BUFFER_BIT|a.DEPTH_BUFFER_BIT),this.gl=a}init(t){u(!!t,"Session: Unable to initialize undefined operation"),u(lt(t)||N(t),"Session: Unable to initialize operation with invalid input type"),lt(t)&&t.traverse((e,a)=>{a.operation[e.name]=e},this),(N(t)||kt(t))&&(this.operation[t.name]=t),this.update()}update(){const t=this.gl,e=Object.keys(this.operation);for(let a=0;a<e.length;a+=1){const n=this.operation[e[a]];n instanceof X&&n.init(this.gl),this.texture[e[a]]||(this.texture[e[a]]=new be(n.dtype,this.gl,this.textureCount,n.shape),n instanceof X&&(t.bindFramebuffer(t.FRAMEBUFFER,n.framebuffer),t.framebufferTexture2D(t.FRAMEBUFFER,t.COLOR_ATTACHMENT0,t.TEXTURE_2D,this.texture[e[a]].ctx,0),t.bindFramebuffer(t.FRAMEBUFFER,null)),this.textureCount+=1)}}runOp(t,e,a=!1){const n=t.sequence;let i=!1;for(let r=0;r<n.length;r+=1){const o=n[r],l=this.operation[o],h=r===n.length-1;u(!!l,`Session: Unable to run uninitialized operation ${t.name}.`),Mt(a)&&h&&((this.canvas.width!==l.shape[1]||this.canvas.height!==l.shape[0])&&(this.canvas.width=l.shape[1],this.canvas.height=l.shape[0]),l.framebuffer=null),l.run(this,e,i)?i=!0:i=!1,a&&h&&N(a)&&this.readToTensor(a),a&&h&&Mt(a)&&this.readToCanvas(a,l.shape)}}destroy(){const t=this.gl.getExtension("WEBGL_lose_context"),e=Object.keys(this.texture),a=Object.keys(this.operation);t&&t.loseContext();for(let n=0;n<e.length;n+=1)this.texture[e[n]].delete();for(let n=0;n<a.length;n+=1){const i=this.operation[a[n]];i instanceof X&&i.destroy()}this.canvas=null,this.operation={},this.texture={},this.gl=null,this.textureCount=0}readToTensor(t){const e=this.gl,a=t.shape[0];let n=t.shape[1],i=e.UNSIGNED_BYTE,r=t.data;t.dtype==="float32"&&(P.SUPPORTS_FLOAT_TEXTURES?i=e.FLOAT:(n*=4,r=t.uint8View)),e.readPixels(0,0,n,a,e.RGBA,i,r)}readToCanvas(t,e){const a=t.getContext("2d");t.width=e[1],t.height=e[0],a.drawImage(this.canvas,0,0,e[1],e[0],0,0,e[1],e[0])}}/**
 * @license MIT
 * @author Arkadiy Pilguk(apilguk@gmail.com)
 * @author Mihail Zachepilo(mihailzachepilo@gmail.com)
 * Copyright 2018 Peculiar Ventures and Pentatonica.
 * All rights reserved.
 */class T{constructor(t){this.op=new X(t),this.name=t,this.checkShape=e=>{const a=Object.keys(e);return e[a[0]]},this.preCompile=()=>{},this.postCompile=()=>{},this.chunks=[]}GLSLKernel(t){return u(typeof t=="string","RegisterOperation: The kernel should be a string but it is not."),this.op.kernel=t,this}LoadChunk(...t){for(const e of t)u(he(e),`RegisterOperation: There is no available GLSL chunk supported: ${e}`);return this.op.chunks=this.op.chunks.concat(t),this}Input(t,e){return u(Ft(t)),this.op.input[t]={name:t,dtype:e},this}Output(t){return u(this.op.dtype===null,"RegisterOperation: The operation allows a single output."),this.op.dtype=t,this}Constant(t,e){return u(Ft(t)),this.op.constant[t]=e,this}SetShapeFn(t){return u(typeof t=="function","RegisterOperation: SetShapeFn should receive function in first argument"),this.checkShape=t,this}PreCompile(t){return u(typeof t=="function","RegisterOperation: PreCompile should receive function in first argument"),this.preCompile=t,this}PostCompile(t){return u(typeof t=="function","RegisterOperation: PostCompile should receive function in first argument"),this.postCompile=t,this}Uniform(t,e,a){return u(Ft(t)),this.op.uniform[t]={name:t,dtype:e,defaultValue:a},this}Compile(t){const e=this.op.clone(),a={},n=Object.keys(t);this.preCompile(e);for(let i=0;i<n.length;i+=1){const r=n[i],o=t[r];u(!!o,`RegisterOperation:${e.name}.${r}:
         Can't compile operation with undefined input.`),u(N(o)||kt(o)||lt(o),`RegisterOperation:${e.name}.${r}:
         Can't compile operation with invalid input type.
         You can only use Tensor or another Operation to be an input`),a[r]=t[r].shape,e.assignInput(r,t[r])}return e.shape=this.checkShape(a),e.sequence=e.getDependencies(),e}}/**
 * @license MIT
 * @author Arkadiy Pilguk(apilguk@gmail.com)
 * @author Mihail Zachepilo(mihailzachepilo@gmail.com)
 * Copyright 2018 Peculiar Ventures and Pentatonica.
 * All rights reserved.
 */function Se(s){const t=new Array(s);for(let e=0;e<s;e+=1)t[e]=e;return t}function la(s,t=!1){let e=null;return lt(s)&&(e=new E(t||s.dtype,s.shape)),N(s)&&(e=new E(t||s.dtype,s.shape)),kt(s)&&(e=new E(t||s.dtype,s.shape)),e}function Ae(s,t){if(t.data.set)t.data.set(s.data);else for(let e=0;e<t.size;e+=1)t.data[e]=s.data[e]}/**
 * @license MIT
 * @author Arkadiy Pilguk(apilguk@gmail.com)
 * @author Mihail Zachepilo(mihailzachepilo@gmail.com)
 * Copyright 2018 Peculiar Ventures and Pentatonica.
 * All rights reserved.
 */class E extends D{constructor(t,e,a,n,i=0){super("Tensor"),this.dtype=t,this.shape=e||[a.length],u(Ut(this.shape),"Tensor: Shape is not valid"),n&&(u(Ut(n),"Tensor: Stride is not valid"),u(this.shape.length===n.length,"Tensor: Stride length should be equal to shape length")),u(typeof i=="number"&&i%1===0,`Tensor: Offset should be integer, but got ${i}`),this.size=E.GetSize(this.shape),this.stride=n||this._defineStride(this.shape),this.offset=i,this._compileJITMethods(),typeof a>"u"?(this.data=E.Malloc(t,this.size),this.empty=E.Malloc(t,this.size)):this.assign(a),!P.SUPPORTS_FLOAT_TEXTURES&&t==="float32"&&(this.uint8View=new Uint8Array(this.data.buffer))}_compileJITMethods(){const t=Se(this.shape.length),e=t.map(n=>`i${n}`).join(","),a=`${this.offset}+${t.map(n=>`${this.stride[n]}*i${n}`).join("+")}`;this.get=new Function(`return function get(${e}) { return this.data[${a}]; }`)(),this.set=new Function(`return function get(${e}, v) { this.data[${a}] = v; }`)(),this.index=new Function(`return function get(${e}, v) { return ${a}; }`)()}_defineStride(t){const e=t.length,a=new Array(e);for(let n=e-1,i=1;n>=0;n-=1)a[n]=i,i*=this.shape[n];return a}assign(t){const e=E.DefineType(t),a=t.length;return u(e===this.dtype,`Tensor: Different dtypes assigned: 
   expected - ${this.dtype} 
   actual - ${e}`),u(a===this.size+this.offset,`Tensor: Different sizes assigned: 
   expected - ${this.size+this.offset} 
   actual - ${a}`),this.data=t,this}release(){return this.empty?this.data.set(this.empty):this.data=E.Malloc(this.dtype,this.size),this}relese(){return ce("Tensor: relese"),this.release(),this}clone(){const t=new E(this.dtype,this.shape,void 0,this.stride,this.offset);return Ae(this,t),t}static IndexToCoord(t,e){const a=new Array(t.length);let n=e,i=t.reduce((r,o)=>r*o);for(let r=0;r<=t.length-2;r+=1){i/=t[r];const o=~~(n/i);n%=i,a[r]=o}return a[a.length-1]=n%t[t.length-1],a}static CoordToIndex(t,e){let a=1,n=0;for(let i=t.length-1;i>=0;i-=1)n+=a*e[i],a*=t[i];return n}static Malloc(t,e){switch(t){case"uint8":return new Uint8Array(e);case"uint16":return new Uint16Array(e);case"uint32":return new Uint32Array(e);case"int8":return new Int8Array(e);case"int16":return new Int16Array(e);case"int32":return new Int32Array(e);case"float32":return new Float32Array(e);case"float64":return new Float64Array(e);case"uint8c":return new Uint8ClampedArray(e);case"array":return new Array(e);default:throw new Error(`Tensor: Unexpected type: ${t}.`)}}static DefineType(t){const e=Object.prototype.toString.call(t);switch(e){case"[object Uint8Array]":return"uint8";case"[object Uint16Array]":return"uint16";case"[object Uint32Array]":return"uint32";case"[object Int8Array]":return"int8";case"[object Int16Array]":return"int16";case"[object Int32Array]":return"int32";case"[object Float32Array]":return"float32";case"[object Float64Array]":return"float64";case"[object Uint8ClampedArray]":return"uint8c";case"[object Array]":return"array";default:throw new Error(`Tensor: Unknown dtype: ${e}.`)}}static GetTypedArray(t,e){if(t===E.DefineType(e))return e;switch(t){case"uint8":return new Uint8Array(e);case"uint16":return new Uint16Array(e);case"uint32":return new Uint32Array(e);case"int8":return new Int8Array(e);case"int16":return new Int16Array(e);case"int32":return new Int32Array(e);case"float32":return new Float32Array(e);case"float64":return new Float64Array(e);case"uint8c":return new Uint8ClampedArray(e);case"array":return new Array(e);default:throw new Error(`Unknown type: ${t}.`)}}static GetSize(t){return t.reduce((e,a)=>e*a,1)}}class Ce extends D{constructor(t,e){super("MediaInput"),this.dtype="uint8",this.inputKeys=[],this.isInitialized=!1,this.lastCtx=Math.random(),this.cache=!0,this.assignMedia(t,e)}assignMedia(t,e){t&&u(Wt(t)||Mt(t),"MediaInput: Is only Video, Canvas element available as input"),e&&u(Ut(e),"MediaInput: Shape is invalid"),this.media=t,this.shape=e}}function _e(s,t=!1,e=!1){const a=new ImageData(s.shape[1],s.shape[0]),n=s.shape[0]*s.shape[1];if(t&&s.dtype==="uint8")return a.data.set(s.data),a;if(!t){for(let i=0;i<n;i+=1){const r=~~(i/s.shape[0]),o=i-r*s.shape[1],l=s.data[i];let h=0;e?h=(o*s.shape[0]+r)*4:h=(r*s.shape[1]+o)*4,a.data[h+0]=l,a.data[h+1]=l,a.data[h+2]=l,a.data[h+3]=255}return a}if(s.dtype==="float32")for(let i=0;i<s.size;i+=1)a.data[i]=s.data[i]*255;else for(let i=0;i<s.size;i+=1)a.data[i]=s.data[i];return a}/**
 * @license MIT
 * @author Arkadiy Pilguk(apilguk@gmail.com)
 * @author Mihail Zachepilo(mihailzachepilo@gmail.com)
 * Copyright 2018 Peculiar Ventures and Pentatonica.
 * All rights reserved.
 */function ha(s,t,e=!1,a=!1){if(!(t instanceof E))throw Error("tensorToCanvas: Input tensor invalid");t.shape[2]&&t.shape[2]===4&&(e=!0);const n=_e(t,e,a);s.getContext("2d").putImageData(n,0,0)}/**
 * @license MIT
 * @author Arkadiy Pilguk(apilguk@gmail.com)
 * @author Mihail Zachepilo(mihailzachepilo@gmail.com)
 * Copyright 2018 Peculiar Ventures and Pentatonica.
 * All rights reserved.
 */function ca(s,t="uint8",e,a=!1){return new Promise((n,i)=>{const r=ue(),o=Ot(),l=o.getContext("2d");let h,f;a&&(r.crossOrigin="anonymous"),r.onload=()=>{e?(h=e[1],f=e[0]):(h=r.width,f=r.height),o.width=h,o.height=f,l.drawImage(r,0,0,h,f);let g;const y=l.getImageData(0,0,h,f);switch(t){case"uint8":{g=new Uint8Array(y.data.buffer);break}case"float32":{g=new Float32Array(y.data);break}default:g=y.data}const j=new E(t,[f,h,4],g);n(j)},r.onerror=i,r.src=s})}var Ie="const float hWidth=(KERNEL_WIDTH-1.0)/2.0;const float hHeight=(KERNEL_HEIGHT-1.0)/2.0;vec4 operation(float y,float x){vec3 finalColour=vec3(0.0);for(float dy=-hHeight;dy<=hHeight;dy+=1.0){for(float dx=-hWidth;dx<=hWidth;dx+=1.0){vec3 k=pickValue_tKernel(float(dy+hHeight),float(dx+hWidth)).rgb;finalColour+=pickValue_tSrc(y+dy,x+dx).rgb*k;}}return vec4(finalColour*factor+bias,1.0);}";function Re(s=3,t=1){const e=new E("float32",[s,s]),a=(s-1)/2,n=new E("float32",[s,s,4]);let i=0;for(let r=0;r<s;r+=1)for(let o=0;o<s;o+=1){const l=Math.exp(-.5*(((o-a)/t)**2+((r-a)/t)**2))/(2*Math.PI*t*t);e.set(o,r,l),i+=e.get(o,r)}for(let r=0;r<s;r+=1)for(let o=0;o<s;o+=1)n.set(o,r,0,e.get(o,r)/i),n.set(o,r,1,e.get(o,r)/i),n.set(o,r,2,e.get(o,r)/i);return n}/**
 * @license MIT
 * @author Arkadiy Pilguk(apilguk@gmail.com)
 * @author Mihail Zachepilo(mihailzachepilo@gmail.com)
 * Copyright 2018 Peculiar Ventures and Pentatonica.
 * All rights reserved.
 */var we=(s,t,e=1,a=0)=>new T("Convolution2d").Input("tSrc",s.dtype).Input("tKernel","float32").Output(s.dtype).LoadChunk("pickValue").Constant("KERNEL_WIDTH",t.shape[1]).Constant("KERNEL_HEIGHT",t.shape[0]).Uniform("bias","float",a).Uniform("factor","float",e).GLSLKernel(Ie).Compile({tSrc:s,tKernel:t});/**
 * @license MIT
 * @author Arkadiy Pilguk(apilguk@gmail.com)
 * @author Mihail Zachepilo(mihailzachepilo@gmail.com)
 * Copyright 2018 Peculiar Ventures and Pentatonica.
 * All rights reserved.
 */var ua=(s,t=3,e=3)=>(u(t>=3,"Kernel size should be greater equal 3"),u(e>0,"Sigma should be greater then 0"),we(s,Re(t,e)));/**
 * @license MIT
 * @author Arkadiy Pilguk(apilguk@gmail.com)
 * @author Mihail Zachepilo(mihailzachepilo@gmail.com)
 * Copyright 2018 Peculiar Ventures and Pentatonica.
 * All rights reserved.
 */function Dt(s=1,t=[1]){let e=s;for(let a=0;a<t.length;a+=1)e/=t[a];return e===1}function zt(s=[1,1],t=[[1,1]]){return Dt(s[0],t.map(e=>e[0]))&&Dt(s[0],t.map(e=>e[1]))}function Gt(s=1,t=1,e=!0,a=s){const n=s**(1/t);if(s%1!==0)throw new RangeError(`Can't get parallel reduction steps for non-integer, got "${s}"`);if(a<1)throw new RangeError(`Can't get parallel reduction steps for maxLayerSize below less than 1, got "${a}"`);if(n%1===0&&n<a)return new Array(t).fill(n);const i=[];let r=s,o=n;for(let l=0;l<t;l+=1){o=r**(1/(t-l));let h=Math.ceil(o);for(;(r%h!==0||r/h>a)&&r/h!==1;)h+=1;if(h===1&&e)break;r/=h,i.push(h)}return i}function Jt(s=[1,1],t=1,e=!0,a=s){const n=Gt(s[0],t,e,a[0]),i=Gt(s[1],t,e,a[1]),r=[];for(let o=0;o<t&&(n[o]||i[o]);o+=1)r.push([n[o]||1,i[o]||1]);return r}var Me="const int kx=int(KX);const int ky=int(KY);const int w=int(WIDTH);const int h=int(HEIGHT);vec4 operation(float gly,float glx){float size=KY*KX;float mean=0.0;float std=0.0;vec3 color=vec3(0.0,0.0,0.0);for(int y=0;y<ky;y+=1){for(int x=0;x<kx;x+=1){vec3 value=pickValue_tSrc(gly*KY+float(y),glx*KX+float(x)).rgb;color+=value.rgb;}}color/=size;mean=color.r;for(int y=0;y<ky;y+=1){for(int x=0;x<kx;x+=1){vec3 value=pickValue_tSrc(gly*KY+float(y),glx*KX+float(x)).rgb;std+=(value.r-mean)*(value.r-mean);}}std/=size;std=sqrt(std);if(std==0.0){std=1.0;}return vec4(color,255.0);}",ke="const int kx=int(KX);const int ky=int(KY);const int w=int(WIDTH);const int h=int(HEIGHT);vec4 operation(float gly,float glx){float size=KX*KY;vec3 std=vec3(0.0,0.0,0.0);vec3 mean=pickValue_tMean(0.0,0.0).rgb;for(int y=0;y<ky;y+=1){for(int x=0;x<kx;x+=1){vec3 value=pickValue_tSrc(gly*KY+float(y),glx*KX+float(x)).rgb;std+=(value-mean)*(value-mean);}}std/=size;std=sqrt(std);if(std.r==0.0){std.r=255.0;}if(std.g==0.0){std.g=255.0;}if(std.b==0.0){std.b=255.0;}return vec4(std,255.0);}",Le="const int kx=int(KX);const int ky=int(KY);const int w=int(WIDTH);const int h=int(HEIGHT);vec4 operation(float gly,float glx){float size=KX*KY;vec3 std=vec3(0.0,0.0,0.0);for(int y=0;y<ky;y+=1){for(int x=0;x<kx;x+=1){vec3 mstd=pickValue_tStd(gly*KY+float(y),glx*KX+float(x)).rgb;std+=mstd*mstd;}}std/=size;std=sqrt(std);if(std.r==0.0){std.r=255.0;}if(std.g==0.0){std.g=255.0;}if(std.b==0.0){std.b=255.0;}return vec4(std,255.0);}",Fe="vec4 operation(float gly,float glx){if(gly==0.0){return texture2D(tMean,vec2(0,0));}else{return texture2D(tStd,vec2(0,0));}}";/**
 * @license MIT
 * @author Arkadiy Pilguk(apilguk@gmail.com)
 * @author Mihail Zachepilo(mihailzachepilo@gmail.com)
 * Copyright 2018 Peculiar Ventures and Pentatonica.
 * All rights reserved.
 */const Ue=(s,t)=>new T("ImageReduceStd").Input("tStd",s.dtype).Output(s.dtype).Constant("WIDTH",s.shape[1]).Constant("HEIGHT",s.shape[0]).Uniform("uWidth","float",s.shape[1]/t[1]).Uniform("uHeight","float",s.shape[0]/t[0]).Constant("KX",t[1]).Constant("KY",t[0]).LoadChunk("pickValue").SetShapeFn(()=>[~~(s.shape[0]/t[0]),~~(s.shape[1]/t[1]),4]).GLSLKernel(Le).Compile({tStd:s}),Oe=(s,t,e)=>new T("ImageExtractStd").Input("tSrc",s.dtype).Input("tMean",t.dtype).Output(s.dtype).Constant("WIDTH",s.shape[1]).Constant("HEIGHT",s.shape[0]).Uniform("uWidth","float",s.shape[1]/e[1]).Uniform("uHeight","float",s.shape[0]/e[0]).Constant("KX",e[1]).Constant("KY",e[0]).LoadChunk("pickValue").SetShapeFn(()=>[~~(s.shape[0]/e[0]),~~(s.shape[1]/e[1]),4]).GLSLKernel(ke).Compile({tSrc:s,tMean:t}),Bt=(s,t)=>new T("ImageExtractMean").Input("tSrc",s.dtype).Output(s.dtype).Constant("WIDTH",s.shape[1]).Constant("HEIGHT",s.shape[0]).Uniform("uWidth","float",s.shape[1]/t[1]).Uniform("uHeight","float",s.shape[0]/t[0]).Constant("KX",t[1]).Constant("KY",t[0]).LoadChunk("pickValue").SetShapeFn(()=>[~~(s.shape[0]/t[0]),~~(s.shape[1]/t[1]),4]).GLSLKernel(Me).Compile({tSrc:s}),Pe=(s,t)=>new T("ImageJoin").Input("tMean",s.dtype).Input("tStd",t.dtype).Output(s.dtype).SetShapeFn(()=>[2,1,4]).GLSLKernel(Fe).Compile({tMean:s,tStd:t});var Ve=(s,t=1,e)=>{let a=[[s.shape[0],s.shape[1]]];Array.isArray(t)?(u(zt(s.shape,t),"ImageMeanStd: Provided steps doesn't converge in 1 px in ImageExtractMeanStd operation"),a=t):typeof t=="number"&&t>0&&(a=Jt(s.shape,t));let n=Bt(s,a[0]);for(let r=1;r<a.length;r+=1)n=Bt(n,a[r]);if(e)return n;let i=Oe(s,n,a[0]);for(let r=1;r<a.length;r+=1)i=Ue(i,a[r]);return Pe(n,i)},$e="const int kx=int(KX);const int ky=int(KY);const float INF=1.0/0.0;const float h2=OUT_VIEW.y/2.0;vec4 operation(float igly,float glx){float size=KX*KY;vec3 minV=vec3(INF);vec3 maxV=vec3(-INF);float gly=igly;if(gly>=h2){gly-=h2;}for(int y=0;y<ky*2;y+=1){for(int x=0;x<kx;x+=1){vec3 value=pickValue_tSrc(gly*KY+float(y),glx*KX+float(x)).rgb;minV=min(minV,value.rgb);maxV=max(maxV,value.rgb);}}if(igly<h2){return vec4(minV,255.0);}else{return vec4(maxV,255.0);}}",Ke="const int kx=int(KX);const int ky=int(KY);const float INF=1.0/0.0;const float h2=OUT_VIEW.y/2.0;vec4 operation(float gly,float glx){float size=KX*KY;vec3 minV=vec3(INF);vec3 maxV=vec3(-INF);vec3 value;for(int y=0;y<ky;y+=1){for(int x=0;x<kx;x+=1){value=pickValue_tSrc(gly*KY+float(y),glx*KX+float(x)).rgb;minV=min(minV,value);maxV=max(maxV,value);}}if(gly<h2){return vec4(minV,255.0);}return vec4(maxV,255.0);}";/**
 * @license MIT
 * @author Arkadiy Pilguk(apilguk@gmail.com)
 * @author Mihail Zachepilo(mihailzachepilo@gmail.com)
 * Copyright 2018 Peculiar Ventures and Pentatonica.
 * All rights reserved.
 */const De=(s,t)=>new T("ImageExtractMinMax").Input("tSrc",s.dtype).Output(s.dtype).Constant("KX",t[1]).Constant("KY",t[0]).LoadChunk("pickValue").SetShapeFn(()=>[~~(s.shape[0]/t[0])*2,~~(s.shape[1]/t[1]),4]).GLSLKernel($e).Compile({tSrc:s}),Ge=(s,t)=>new T("ImageReduceMinMax").Input("tSrc",s.dtype).Output(s.dtype).Constant("KX",t[1]).Constant("KY",t[0]).LoadChunk("pickValue").SetShapeFn(()=>[~~(s.shape[0]/t[0]),~~(s.shape[1]/t[1]),4]).GLSLKernel(Ke).Compile({tSrc:s});var Be=(s,t=1)=>{let e=[[s.shape[0],s.shape[1]]];Array.isArray(t)?(u(zt(s.shape,t),"ImageMeanStd: Provided steps doesn't converge in 1 px in ImageExtractMeanStd operation"),e=t):typeof t=="number"&&t>0&&(e=Jt(s.shape,t));let a=De(s,e[0]);for(let n=1;n<e.length;n+=1)a=Ge(a,e[n]);return a},He="vec4 operation(float y,float x){vec3 chanels=pickValue_tSrc(y,x).rgb;vec3 mean=pickValue_tStdMean(0.0,0.0).rgb;vec3 std=pickValue_tStdMean(1.0,0.0).rgb;vec3 value=(chanels-mean)/std;return vec4(value,1.0);}",Ne="vec4 operation(float y,float x){vec3 chanels=pickValue_tSrc(y,x).rgb;vec3 minV=pickValue_tMinMax(0.0,0.0).rgb;vec3 maxV=pickValue_tMinMax(1.0,0.0).rgb;vec3 value=(chanels-minV)/(maxV-minV);return vec4(value,1.0);}";/**
 * @license MIT
 * @author Arkadiy Pilguk(apilguk@gmail.com)
 * @author Mihail Zachepilo(mihailzachepilo@gmail.com)
 * Copyright 2018 Peculiar Ventures and Pentatonica.
 * All rights reserved.
 */const Xe=(s,t)=>new T("l2Norm").Input("tSrc","uint8").Input("tStdMean","uint8").Output("uint8").LoadChunk("pickValue").GLSLKernel(He).Compile({tSrc:s,tStdMean:t}),je=(s,t)=>new T("minMaxNorm").Input("tSrc","uint8").Input("tMinMax","uint8").Output("uint8").LoadChunk("pickValue").GLSLKernel(Ne).Compile({tSrc:s,tMinMax:t});var fa=(s,t,e=2)=>{u(t==="l2"||t==="minmax",`Unsupported type of normalization operation.
     Currently availiable max and visualize.`);let a=null;return t==="l2"&&(a=Xe(s,Ve(s,e))),t==="minmax"&&(a=je(s,Be(s,e))),a},We="vec4 getPoint(vec2 p){return pickValue_tSrc(p.y,p.x);}mat3 getTransformMatrix(){vec3 r1=pickValue_tTransform(0.0,0.0).rgb;vec3 r2=pickValue_tTransform(1.0,0.0).rgb;vec3 r3=pickValue_tTransform(3.0,0.0).rgb;return mat3(r1,r2,r3);}vec4 operation(float y,float x){mat3 m=getTransformMatrix();float off=0.0;float ixs=0.0;float iys=0.0;float xs=0.0;float ys=0.0;float xs0=0.0;float ys0=0.0;float ws=0.0;float sc=0.0;float a=0.0;float b=0.0;xs0=m[0][1]*y+m[0][2];ys0=m[1][1]*y+m[1][2];ws=m[2][1]*y+m[2][2];xs0+=m[0][0]*x;ys0+=m[1][0]*x;ws+=m[2][0]*x;sc=1.0/ws;xs=xs0*sc;ys=ys0*sc;ixs=xs;iys=ys;a=max(xs-ixs,0.0);b=max(ys-iys,0.0);vec2 mvec=vec2(ixs,iys);vec2 ox=vec2(1.0,0.0);vec2 oy=vec2(1.0,1.0);vec4 p0=getPoint(mvec)+a*(getPoint(mvec+ox)-getPoint(mvec));vec4 p1=getPoint(mvec+oy)+a*(getPoint(mvec+ox+oy)-getPoint(mvec+oy));vec4 pres=p0+b*(p1-p0);return pres;}";/**
 * @license MIT
 * @author Arkadiy Pilguk(apilguk@gmail.com)
 * @author Mihail Zachepilo(mihailzachepilo@gmail.com)
 * Copyright 2018 Peculiar Ventures and Pentatonica.
 * All rights reserved.
 */var da=(s,t,e=[10,10,4],a=s.dtype)=>new T("PerspectiveProjection").Input("tSrc",s.dtype).Input("tTransform","float32").Output(a).LoadChunk("pickValue").Uniform("uSrcWidth","float",s.shape[1]).Uniform("uSrcHeight","float",s.shape[0]).Uniform("uWidth","float",e[1]).Uniform("uHeight","float",e[0]).SetShapeFn(()=>e).GLSLKernel(We).Compile({tSrc:s,tTransform:t}),Ye="vec4 operation(float y,float x){vec4 pixel=pickValue_tSrc(y,x);if(pixel[int(C)]>uT){return vec4(1.0,1.0,1.0,1.0);}else{return vec4(0.0,0.0,0.0,1.0);}}";/**
 * @license MIT
 * @author Arkadiy Pilguk(apilguk@gmail.com)
 * @author Mihail Zachepilo(mihailzachepilo@gmail.com)
 * Copyright 2018 Peculiar Ventures and Pentatonica.
 * All rights reserved.
 */var pa=(s,t=.5,e=0)=>(u(typeof t=="number","Only number available as a threshold value."),u(e===0||e===1||e===2||e===3,"Only RGBA available: 0, 1, 2, 3"),new T("Threshold").Input("tSrc",s.dtype).Output(s.dtype).Constant("C",e).Uniform("uT","float",t).LoadChunk("pickValue").GLSLKernel(Ye).Compile({tSrc:s})),qe="float HKW=floor(KW/2.0);float HKH=floor(KW/2.0);vec4 operation(float y,float x){float R=10000.0;float G=10000.0;float B=10000.0;y=y+HKH;x=x+HKW;for(float dx=0.0;dx<KW;dx+=1.0){for(float dy=0.0;dy<KH;dy+=1.0){vec4 v=pickValue_tSrc((y-dy),(x-dx));vec4 m=pickValue_tKernel(dy,dx);if(v.r<R&&m.r>0.0){R=v.r;}if(v.g<G&&m.g>0.0){G=v.g;}if(v.b<B&&m.b>0.0){B=v.b;}}}return vec4(R,G,B,1.0);}";/**
 * @license MIT
 * @author Arkadiy Pilguk(apilguk@gmail.com)
 * @author Mihail Zachepilo(mihailzachepilo@gmail.com)
 * Copyright 2018 Peculiar Ventures and Pentatonica.
 * All rights reserved.
 */var ga=(s,t=[2,2],e=!1)=>{if(u(t.length===2,"Erosion: Kernel size should be shape of rank 2"),N(e)&&u(t[0]===e.shape[0]&&t[1]===e.shape[1],"Erosion: Structure element has wrong size"),!e){e=new E("float32",[t[0],t[1],4]);for(let a=0;a<t[0];a+=1)for(let n=0;n<t[1];n+=1)e.set(a,n,0,1),e.set(a,n,1,1),e.set(a,n,2,1),e.set(a,n,3,1)}return new T("Erosion").Input("tSrc",s.dtype).Input("tKernel","float32").Output(s.dtype).Constant("KW",t[0]).Constant("KH",t[1]).LoadChunk("pickValue").GLSLKernel(qe).Compile({tSrc:s,tKernel:e})},ze="vec4 operation(float y,float x){vec4 res=pickValue_tSrc(y,x);for(float I=1.0;I<=SAMPLES_PER_PASS;I+=1.0){float cx=x-ceil(pow(1.0+SAMPLES_PER_PASS,PASSI)*I);if(cx<0.0){break;}res+=pickValue_tSrc(y,cx);}return res;}",Je="vec4 operation(float y,float x){vec4 res=pickValue_tSrc(y,x);res=res*res;vec4 v=vec4(0.0);for(float I=1.0;I<=SAMPLES_PER_PASS;I+=1.0){float cx=x-ceil(pow(1.0+SAMPLES_PER_PASS,PASSI)*I);if(cx<0.0){break;}v=pickValue_tSrc(y,cx);res+=v*v;}return res;}",Ze="vec4 operation(float y,float x){vec4 res=pickValue_tSrc(y,x);for(float I=1.0;I<=SAMPLES_PER_PASS;I+=1.0){float cy=y-ceil(pow(1.0+SAMPLES_PER_PASS,PASSI)*I);if(cy<0.0){break;}res+=pickValue_tSrc(cy,x);}return res;}";/**
 * @license MIT
 * @author Arkadiy Pilguk(apilguk@gmail.com)
 * @author Mihail Zachepilo(mihailzachepilo@gmail.com)
 * Copyright 2018 Peculiar Ventures and Pentatonica.
 * All rights reserved.
 */const Ht=(s,t="x",e=0,a=1)=>new T("SummedAreaTable").Input("tSrc",s.dtype).Output("float32").LoadChunk("pickValue").Constant("PASSI",e).Constant("LAST",!1).Constant("SAMPLES_PER_PASS",a).GLSLKernel(t==="x"?ze:Ze).Compile({tSrc:s}),Qe=(s,t=0,e=1)=>new T("SquaredSummedAreaTable").Input("tSrc",s.dtype).Output("float32").LoadChunk("pickValue").Constant("PASSI",t).Constant("LAST",!1).Constant("SAMPLES_PER_PASS",e).GLSLKernel(Je).Compile({tSrc:s}),ta=(s,t=2,e=!1)=>{const a=Math.ceil(s.shape[1]**(1/t)),n=Math.ceil(s.shape[0]**(1/t));let i=s;const r=Math.log(s.shape[1])/Math.log(Math.max(a+1,2)),o=Math.log(s.shape[0])/Math.log(Math.max(n+1,2));e&&(i=Qe(i,0,Math.min(a,s.shape[1]-1)));for(let l=e?1:0;l<r;l+=1)i=Ht(i,"x",l,Math.min(a,s.shape[1]-1));for(let l=0;l<o;l+=1)i=Ht(i,"y",l,Math.min(n,s.shape[0]-1));return i},ea=(s,t=2)=>ta(s,t,!1);var aa="const int Channel=int(C);float pickValue(float y,float x){if(y<0.0||x<0.0){return 0.0;}return pickValue_tIntegralImage(y,x)[Channel];}vec4 operation(float y,float x){vec4 pixel=pickValue_tSrc(y,x);float huS=uS/2.0;vec2 p1=max(floor(vec2(x,y)-huS),vec2(0.0));vec2 p2=min(floor(vec2(x,y)+huS),OUT_VIEW-1.0);vec2 pd=p2+1.0-p1;float s=pd.x*pd.y;p1-=1.0;float sum=pickValue(p2.y,p2.x)-pickValue(p1.y,p2.x)-pickValue(p2.y,p1.x)+pickValue(p1.y,p1.x);if(pixel[Channel]*s<=sum*(100.0-uT)/100.0){return vec4(0.0,0.0,0.0,1.0);}else{return vec4(1.0,1.0,1.0,1.0);}}";/**
 * @license MIT
 * @author Arkadiy Pilguk(apilguk@gmail.com)
 * @author Mihail Zachepilo(mihailzachepilo@gmail.com)
 * Copyright 2018 Peculiar Ventures and Pentatonica.
 * All rights reserved.
 */var ya=(s,t=5,e=50,a=0,n=ea(s))=>(u(typeof e=="number","Only number available as a threshold value."),u(typeof t=="number","Only number available as a size value."),u(a===0||a===1||a===2||a===3,"Only RGBA available: 0, 1, 2, 3"),new T("Threshold").Input("tSrc",s.dtype).Input("tIntegralImage",n.dtype).Output(s.dtype).Constant("C",a).Uniform("uS","float",t).Uniform("uT","float",e).LoadChunk("pickValue").GLSLKernel(aa).Compile({tSrc:s,tIntegralImage:n}));/**
 * @license MIT
 * @author Arkadiy Pilguk(apilguk@gmail.com)
 * @author Mihail Zachepilo(mihailzachepilo@gmail.com)
 * Copyright 2018 Peculiar Ventures and Pentatonica.
 * All rights reserved.
 */const Nt=1e-7;function K(s,t,e){return s-Nt<=t&&t<=e+Nt}class Et{static Intersection(t,e){const a=t.x1,n=t.y1,i=t.x2,r=t.y2,o=e.x1,l=e.y1,h=e.x2,f=e.y2,g=((a*r-n*i)*(o-h)-(a-i)*(o*f-l*h))/((a-i)*(l-f)-(n-r)*(o-h)),y=((a*r-n*i)*(l-f)-(n-r)*(o*f-l*h))/((a-i)*(l-f)-(n-r)*(o-h));if(isNaN(g)||isNaN(y))return!1;if(a>=i){if(!K(i,g,a))return!1}else if(!K(a,g,i))return!1;if(n>=r){if(!K(r,y,n))return!1}else if(!K(n,y,r))return!1;if(o>=h){if(!K(h,g,o))return!1}else if(!K(o,g,h))return!1;if(l>=f){if(!K(f,y,l))return!1}else if(!K(l,y,f))return!1;return[g,y]}constructor(t,e,a,n,i,r){if(t instanceof ArrayBuffer)this.data=new Float32Array(t,e,8);else if(Array.isArray(t)){if(t.length<8)for(let o=t.length;o<=8;o+=1)t.push(0);this.data=new Float32Array(t)}else t!==void 0&&e!==void 0?this.data=new Float32Array([t,e,a,n,i,r,0,0]):this.data=new Float32Array(8)}set(t,e,a,n,i,r){this.data[0]=t,this.data[1]=e,this.data[2]=a,this.data[3]=n,this.data[4]=i,this.data[5]=r,this.data[6]=0,this.data[7]=0}fromParallelCoords(t,e,a,n,i,r){const l=a;let h,f;t>r?(t-=r,h=i-r*e/t,f=(-1+r/t)*a+h):(t=r-t,h=r*e/t,f=(1-r/t)*a+h),this.set(0,h,l,f,t,e)}get length(){if(this.data[6])return this.data[6];const t=this.data[2]-this.data[0],e=this.data[3]-this.data[1],a=Math.sqrt(t**2+e**2);return this.data[6]=a,a}get angle(){if(this.data[7])return this.data[7];const t=this.data[2]-this.data[0],e=this.data[3]-this.data[1];let a=Math.atan(e/t)/Math.PI*180;return a<0&&(a=180+a),this.data[7]=a,a}get x1(){return this.data[0]}get y1(){return this.data[1]}get x2(){return this.data[2]}get y2(){return this.data[3]}get px(){return this.data[4]}get py(){return this.data[5]}set x1(t){this.data[0]=t}set y1(t){this.data[1]=t}set x2(t){this.data[2]=t}set y2(t){this.data[3]=t}set px(t){this.data[4]=t}set py(t){this.data[5]=t}clear(){this.data[0]=0,this.data[1]=0,this.data[2]=0,this.data[3]=0,this.data[4]=0,this.data[5]=0,this.data[6]=0,this.data[7]=0}fromArray(t){this.data.set(t)}toArray(){return Array.prototype.slice.call(this.data)}}Et.BYTES_PER_ELEMENT=36;/**
 * @license MIT
 * @author Arkadiy Pilguk(apilguk@gmail.com)
 * @author Mihail Zachepilo(mihailzachepilo@gmail.com)
 * Copyright 2018 Peculiar Ventures and Pentatonica.
 * All rights reserved.
 */function sa(s,t){const e=[0,0];let a=null,n=null,i=null,r=null;e[0]+=s[0][0],e[0]+=s[1][0],e[0]+=s[2][0],e[0]+=s[3][0],e[1]+=s[0][1],e[1]+=s[1][1],e[1]+=s[2][1],e[1]+=s[3][1],e[0]/=4,e[1]/=4;for(let o=0;o<s.length;o+=1)s[o][0]>=e[0]&&s[o][1]>=e[1]&&(i=s[o]),s[o][0]<=e[0]&&s[o][1]<=e[1]&&(a=s[o]),s[o][0]>=e[0]&&s[o][1]<=e[1]&&(n=s[o]),s[o][0]<=e[0]&&s[o][1]>=e[1]&&(r=s[o]);return[a,n,i,r]}function Rt(s,t){const e=s[2]-s[0],a=s[3]-s[1],n=t[2]-t[0],i=t[3]-t[1],r=e*n+a*i,o=(e*e+a*a)*(n*n+i*i);return Math.acos(r/Math.sqrt(o))}function wt(s,t,e){const a=e;let n=0,i=0,r=0,o=0,l=0,h=0;return r=a.get(0,1)*t+a.get(0,2),o=a.get(1,1)*t+a.get(1,2),l=a.get(2,1)*t+a.get(2,2),r+=a.get(0,0)*s,o+=a.get(1,0)*s,l+=a.get(2,0)*s,h=1/l,n=r*h,i=o*h,[n,i]}function xa(s,t,e,a=0){return na(e,a,a,s.ax,s.ay,t[1]-a,a,s.bx,s.by,t[1]-a,t[0]-a,s.cx,s.cy,a,t[0]-a,s.dx,s.dy,e.shape.length===3&&e.shape[2]===4),e}function na(s,t,e,a,n,i,r,o,l,h,f,g,y,j,G,Tt,bt,Zt=!1){let b=t,d=h,x=r,V=b*d*x,S=G,_=b*S,$=d*_,p=f,U=b*p,m=i,A=e,c=j,k=A*c,I=k*m,ht=c*m*p,L=c*x,ct=c*p,ut=d*x,ft=S*d,F=S*m,B=p*m,C=1/(L-ct-ut+ft-F+B),dt=b*c,R=A*m,O=x*b,pt=S*O,H=A*d,gt=k*p,w=A*p*m,yt=x*S*d,xt=S*A;const St=-($-V+U*m-m*_-k*d+I-ht+L*d)*C,At=(V-$-dt*x+dt*p+I-d*R+F*d-ht)*C,Ct=b,_t=(-p*_+pt+H*x-k*x+gt-w+F*p-yt)*C,mt=(-pt+O*p-xt*d+gt-w+xt*m+yt-L*p)*C,vt=A,It=(-U+O+H-R+ct-L-ft+F)*C,W=(-_+U+k-H+F-B-L+ut)*C;b=a,d=g,x=l,V=b*d*x,S=bt,_=b*S,$=d*_,p=y,U=b*p,m=o,A=n,c=Tt,k=A*c,I=k*m,ht=c*m*p,L=c*x,ct=c*p,ut=d*x,ft=S*d,F=S*m,B=p*m,C=1/(L-ct-ut+ft-F+B),dt=b*c,R=A*m,O=x*b,pt=S*O,H=A*d,gt=k*p,w=A*p*m,yt=x*S*d,xt=S*A;const Y=-($-V+U*m-m*_-k*d+I-ht+L*d)*C,q=(V-$-dt*x+dt*p+I-d*R+F*d-ht)*C,z=b,J=(-p*_+pt+H*x-k*x+gt-w+F*p-yt)*C,Z=(-pt+O*p-xt*d+gt-w+xt*m+yt-L*p)*C,Q=A,tt=(-U+O+H-R+ct-L-ft+F)*C,et=(-_+U+k-H+F-B-L+ut)*C;d=mt-W*vt,x=St*mt,V=St*vt,_=_t*At,$=Ct*_t,U=At*It;const Pt=Ct*It;c=1/(x-V*W-_+$*W+U*vt-Pt*mt),I=-_t+vt*It;const at=-_t*W+mt*It;B=-At+Ct*W;const st=St-Pt;R=St*W-U,O=-At*vt+Ct*mt;const nt=V-$,it=x-_;w=d*c;const rt=B*c,ot=O*c,v=s.data;Zt?(v[0]=Y*w+q*(I*c)-z*(at*c),v[1]=Y*rt+q*(st*c)-z*(R*c),v[2]=-Y*ot-q*(nt*c)+z*(it*c),v[4]=J*w+Z*(I*c)-Q*(at*c),v[5]=J*rt+Z*(st*c)-Q*(R*c),v[6]=-J*ot-Z*(nt*c)+Q*(it*c),v[8]=tt*w+et*(I*c)-at*c,v[9]=tt*rt+et*(st*c)-R*c,v[10]=-tt*ot-et*(nt*c)+it*c):(v[0]=Y*w+q*(I*c)-z*(at*c),v[1]=Y*rt+q*(st*c)-z*(R*c),v[2]=-Y*ot-q*(nt*c)+z*(it*c),v[3]=J*w+Z*(I*c)-Q*(at*c),v[4]=J*rt+Z*(st*c)-Q*(R*c),v[5]=-J*ot-Z*(nt*c)+Q*(it*c),v[6]=tt*w+et*(I*c)-at*c,v[7]=tt*rt+et*(st*c)-R*c,v[8]=-tt*ot-et*(nt*c)+it*c)}/**
 * @license MIT
 * @author Arkadiy Pilguk(apilguk@gmail.com)
 * @author Mihail Zachepilo(mihailzachepilo@gmail.com)
 * Copyright 2018 Peculiar Ventures and Pentatonica.
 * All rights reserved.
 */class M{static Distance(t,e){let a=0;for(let n=0;n<8;n+=2){const i=Math.sqrt((t.data[n]-e.data[n])**2+(t.data[n+1]-e.data[n+1])**2);a+=i**2}return a=Math.sqrt(a/8),a===1/0?0:a}static TriangleS(t,e,a,n,i,r){return Math.abs(t*(n-r)+a*(r-e)+i*(e-n))/2}constructor(...t){t[0]instanceof ArrayBuffer?this.data=new Float32Array(t[0],t[1],M.NUM_ELEMENTS):Array.isArray(t[0])?this.data=new Float32Array(t[0]):t[0]&&t.length===M.NUM_ELEMENTS?this.data=new Float32Array(t):this.data=new Float32Array(M.NUM_ELEMENTS)}isInRect(t,e){const a=M.TriangleS(t,e,this.ax,this.ay,this.bx,this.by),n=M.TriangleS(t,e,this.cx,this.cy,this.bx,this.by),i=M.TriangleS(this.cx,this.cy,t,e,this.dx,this.dy),r=M.TriangleS(this.dx,this.dy,t,e,this.ax,this.ay);return!(a+n+i+r-this.area>0)}isNotEmpty(){return this.data[0]>0&&this.data[1]>0&&this.data[2]>0&&this.data[3]>0&&this.data[4]>0&&this.data[5]>0&&this.data[6]>0&&this.data[7]>0}clone(){return new M(this.toArray())}set(t,e,a,n,i,r,o,l){this.data[0]=t,this.data[1]=e,this.data[2]=a,this.data[3]=n,this.data[4]=i,this.data[5]=r,this.data[6]=o,this.data[7]=l}assign(t){return this.data.set(t.data),this}scale(t,e){return this.data[0]*=t,this.data[1]*=e,this.data[2]*=t,this.data[3]*=e,this.data[4]*=t,this.data[5]*=e,this.data[6]*=t,this.data[7]*=e,this}fromLines(t,e,a,n){const i=sa([Et.Intersection(t,e),Et.Intersection(e,a),Et.Intersection(a,n),Et.Intersection(n,t)]);return!i[0]||!i[1]||!i[2]||!i[3]?!1:(this.data[0]=i[0][0],this.data[1]=i[0][1],this.data[2]=i[1][0],this.data[3]=i[1][1],this.data[4]=i[2][0],this.data[5]=i[2][1],this.data[6]=i[3][0],this.data[7]=i[3][1],!0)}get ax(){return this.data[0]}get ay(){return this.data[1]}get bx(){return this.data[2]}get by(){return this.data[3]}get cx(){return this.data[4]}get cy(){return this.data[5]}get dx(){return this.data[6]}get dy(){return this.data[7]}set ax(t){this.data[0]=t}set ay(t){this.data[1]=t}set bx(t){this.data[2]=t}set by(t){this.data[3]=t}set cx(t){this.data[4]=t}set cy(t){this.data[5]=t}set dx(t){this.data[6]=t}set dy(t){this.data[7]=t}get distA(){return Math.sqrt((this.data[6]-this.data[0])**2+(this.data[7]-this.data[1])**2)}get distB(){return Math.sqrt((this.data[4]-this.data[2])**2+(this.data[5]-this.data[3])**2)}get distC(){return Math.sqrt((this.data[0]-this.data[2])**2+(this.data[1]-this.data[3])**2)}get distD(){return Math.sqrt((this.data[6]-this.data[4])**2+(this.data[7]-this.data[5])**2)}get distE(){return Math.sqrt((this.data[0]-this.data[4])**2+(this.data[1]-this.data[5])**2)}get distF(){return Math.sqrt((this.data[6]-this.data[2])**2+(this.data[7]-this.data[3])**2)}get angleA(){return Rt([this.data[6],this.data[7],this.data[0],this.data[1]],[this.data[0],this.data[1],this.data[2],this.data[3]])}get angleB(){return Rt([this.data[0],this.data[1],this.data[2],this.data[3]],[this.data[2],this.data[3],this.data[4],this.data[5]])}get angleC(){return Rt([this.data[2],this.data[3],this.data[4],this.data[5]],[this.data[4],this.data[5],this.data[6],this.data[7]])}get angleD(){return Rt([this.data[4],this.data[5],this.data[6],this.data[7]],[this.data[6],this.data[7],this.data[0],this.data[1]])}get area(){const t=this.distA,e=this.distB,a=this.distC,n=this.distD,i=(t+e+a+n)/2;return Math.sqrt((i-t)*(i-e)*(i-a)*(i-n))}get P(){return this.distA+this.distB+this.distC+this.distD}mul(t){return this.data[0]*=t,this.data[1]*=t,this.data[2]*=t,this.data[3]*=t,this.data[4]*=t,this.data[5]*=t,this.data[6]*=t,this.data[7]*=t,this}scaleAt(t){return this.data[0]-=t,this.data[1]-=t,this.data[2]-=t,this.data[3]+=t,this.data[4]+=t,this.data[5]+=t,this.data[6]+=t,this.data[7]-=t,this}clear(){this.data[0]=0,this.data[1]=0,this.data[2]=0,this.data[3]=0,this.data[4]=0,this.data[5]=0,this.data[6]=0,this.data[7]=0}fromDeep(t){return this.data[0]=t[0][0],this.data[1]=t[0][1],this.data[2]=t[1][0],this.data[3]=t[1][1],this.data[4]=t[2][0],this.data[5]=t[2][1],this.data[6]=t[3][0],this.data[7]=t[3][1],this}perspective(t){const e=wt(this.data[0],this.data[1],t),a=wt(this.data[2],this.data[3],t),n=wt(this.data[4],this.data[5],t),i=wt(this.data[6],this.data[7],t);return this.data[0]=e[0],this.data[1]=e[1],this.data[2]=a[0],this.data[3]=a[1],this.data[4]=n[0],this.data[5]=n[1],this.data[6]=i[0],this.data[7]=i[1],this}fromArray(t){return this.data.set(t),this}toArray(){return Array.prototype.slice.call(this.data)}isInside(t){return t.ax>this.ax&&t.ay>this.ay&&t.bx<this.bx&&t.by>this.by&&t.cx<this.cx&&t.cy<this.cy&&t.dx>this.dx&&t.dy<this.dy}toJSON(){return this.toArray()}}M.NUM_ELEMENTS=8;M.BYTES_PER_ELEMENT=M.NUM_ELEMENTS*Float32Array.BYTES_PER_ELEMENT;export{M as R,oa as S,E as T,fa as a,ya as b,ua as c,pa as d,ga as e,ha as f,ra as g,xa as h,ca as i,da as j,la as t};
