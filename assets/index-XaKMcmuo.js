(async()=>{(function(){const o=document.createElement("link").relList;if(o&&o.supports&&o.supports("modulepreload"))return;for(const i of document.querySelectorAll('link[rel="modulepreload"]'))e(i);new MutationObserver(i=>{for(const s of i)if(s.type==="childList")for(const a of s.addedNodes)a.tagName==="LINK"&&a.rel==="modulepreload"&&e(a)}).observe(document,{childList:!0,subtree:!0});function n(i){const s={};return i.integrity&&(s.integrity=i.integrity),i.referrerPolicy&&(s.referrerPolicy=i.referrerPolicy),i.crossOrigin==="use-credentials"?s.credentials="include":i.crossOrigin==="anonymous"?s.credentials="omit":s.credentials="same-origin",s}function e(i){if(i.ep)return;i.ep=!0;const s=n(i);fetch(i.href,s)}})();const M=""+new URL("ewts_bg-BBtl-8ew.wasm",import.meta.url).href,P=async(o={},n)=>{let e;if(n.startsWith("data:")){const i=n.replace(/^data:.*?base64,/,"");let s;if(typeof Buffer=="function"&&typeof Buffer.from=="function")s=Buffer.from(i,"base64");else if(typeof atob=="function"){const a=atob(i);s=new Uint8Array(a.length);for(let t=0;t<a.length;t++)s[t]=a.charCodeAt(t)}else throw new Error("Cannot decode base64-encoded data URL");e=await WebAssembly.instantiate(s,o)}else{const i=await fetch(n),s=i.headers.get("Content-Type")||"";if("instantiateStreaming"in WebAssembly&&s.startsWith("application/wasm"))e=await WebAssembly.instantiateStreaming(i,o);else{const a=await i.arrayBuffer();e=await WebAssembly.instantiate(a,o)}}return e.instance.exports};let p;function R(o){p=o}const S=typeof TextDecoder>"u"?(0,module.require)("util").TextDecoder:TextDecoder;let T=new S("utf-8",{ignoreBOM:!0,fatal:!0});T.decode();let _=null;function b(){return(_===null||_.byteLength===0)&&(_=new Uint8Array(p.memory.buffer)),_}function x(o,n){return o=o>>>0,T.decode(b().subarray(o,o+n))}let v=0;const U=typeof TextEncoder>"u"?(0,module.require)("util").TextEncoder:TextEncoder;let y=new U("utf-8");const B=typeof y.encodeInto=="function"?function(o,n){return y.encodeInto(o,n)}:function(o,n){const e=y.encode(o);return n.set(e),{read:o.length,written:e.length}};function F(o,n,e){if(e===void 0){const r=y.encode(o),d=n(r.length,1)>>>0;return b().subarray(d,d+r.length).set(r),v=r.length,d}let i=o.length,s=n(i,1)>>>0;const a=b();let t=0;for(;t<i;t++){const r=o.charCodeAt(t);if(r>127)break;a[s+t]=r}if(t!==i){t!==0&&(o=o.slice(t)),s=e(s,i,i=t+o.length*3,1)>>>0;const r=b().subarray(s+t,s+i),d=B(o,r);t+=d.written,s=e(s,i,t,1)>>>0}return v=t,s}const L=typeof FinalizationRegistry>"u"?{register:()=>{},unregister:()=>{}}:new FinalizationRegistry(o=>p.__wbg_ewtsconverter_free(o>>>0,1));class j{__destroy_into_raw(){const n=this.__wbg_ptr;return this.__wbg_ptr=0,L.unregister(this),n}free(){const n=this.__destroy_into_raw();p.__wbg_ewtsconverter_free(n,0)}constructor(){const n=p.ewtsconverter_new();return this.__wbg_ptr=n>>>0,L.register(this,this.__wbg_ptr,this),this}ewtsToUnicode(n){let e,i;try{const s=F(n,p.__wbindgen_malloc,p.__wbindgen_realloc),a=v,t=p.ewtsconverter_ewtsToUnicode(this.__wbg_ptr,s,a);return e=t[0],i=t[1],x(t[0],t[1])}finally{p.__wbindgen_free(e,i,1)}}}function q(o,n){throw new Error(x(o,n))}function I(){const o=p.__wbindgen_export_0,n=o.grow(4);o.set(0,void 0),o.set(n+0,void 0),o.set(n+1,null),o.set(n+2,!0),o.set(n+3,!1)}URL=globalThis.URL;const f=await P({"./ewts_bg.js":{__wbindgen_throw:q,__wbindgen_init_externref_table:I}},M),V=f.memory,z=f.__wbg_ewtsconverter_free,D=f.ewtsconverter_new,H=f.ewtsconverter_ewtsToUnicode,K=f.__wbindgen_export_0,G=f.__wbindgen_malloc,J=f.__wbindgen_realloc,Q=f.__wbindgen_free,C=f.__wbindgen_start,X=Object.freeze(Object.defineProperty({__proto__:null,__wbg_ewtsconverter_free:z,__wbindgen_export_0:K,__wbindgen_free:Q,__wbindgen_malloc:G,__wbindgen_realloc:J,__wbindgen_start:C,ewtsconverter_ewtsToUnicode:H,ewtsconverter_new:D,memory:V},Symbol.toStringTag,{value:"Module"}));R(X),C();function Y(o){return o&&o.__esModule&&Object.prototype.hasOwnProperty.call(o,"default")?o.default:o}var E={exports:{}},Z=E.exports,N;function $(){return N||(N=1,function(o){(function(n,e){o.exports?o.exports=e():n.Toastify=e()})(Z,function(n){var e=function(t){return new e.lib.init(t)},i="1.12.0";e.defaults={oldestFirst:!0,text:"Toastify is awesome!",node:void 0,duration:3e3,selector:void 0,callback:function(){},destination:void 0,newWindow:!1,close:!1,gravity:"toastify-top",positionLeft:!1,position:"",backgroundColor:"",avatar:"",className:"",stopOnFocus:!0,onClick:function(){},offset:{x:0,y:0},escapeMarkup:!0,ariaLive:"polite",style:{background:""}},e.lib=e.prototype={toastify:i,constructor:e,init:function(t){return t||(t={}),this.options={},this.toastElement=null,this.options.text=t.text||e.defaults.text,this.options.node=t.node||e.defaults.node,this.options.duration=t.duration===0?0:t.duration||e.defaults.duration,this.options.selector=t.selector||e.defaults.selector,this.options.callback=t.callback||e.defaults.callback,this.options.destination=t.destination||e.defaults.destination,this.options.newWindow=t.newWindow||e.defaults.newWindow,this.options.close=t.close||e.defaults.close,this.options.gravity=t.gravity==="bottom"?"toastify-bottom":e.defaults.gravity,this.options.positionLeft=t.positionLeft||e.defaults.positionLeft,this.options.position=t.position||e.defaults.position,this.options.backgroundColor=t.backgroundColor||e.defaults.backgroundColor,this.options.avatar=t.avatar||e.defaults.avatar,this.options.className=t.className||e.defaults.className,this.options.stopOnFocus=t.stopOnFocus===void 0?e.defaults.stopOnFocus:t.stopOnFocus,this.options.onClick=t.onClick||e.defaults.onClick,this.options.offset=t.offset||e.defaults.offset,this.options.escapeMarkup=t.escapeMarkup!==void 0?t.escapeMarkup:e.defaults.escapeMarkup,this.options.ariaLive=t.ariaLive||e.defaults.ariaLive,this.options.style=t.style||e.defaults.style,t.backgroundColor&&(this.options.style.background=t.backgroundColor),this},buildToast:function(){if(!this.options)throw"Toastify is not initialized";var t=document.createElement("div");t.className="toastify on "+this.options.className,this.options.position?t.className+=" toastify-"+this.options.position:this.options.positionLeft===!0?(t.className+=" toastify-left",console.warn("Property `positionLeft` will be depreciated in further versions. Please use `position` instead.")):t.className+=" toastify-right",t.className+=" "+this.options.gravity,this.options.backgroundColor&&console.warn('DEPRECATION NOTICE: "backgroundColor" is being deprecated. Please use the "style.background" property.');for(var r in this.options.style)t.style[r]=this.options.style[r];if(this.options.ariaLive&&t.setAttribute("aria-live",this.options.ariaLive),this.options.node&&this.options.node.nodeType===Node.ELEMENT_NODE)t.appendChild(this.options.node);else if(this.options.escapeMarkup?t.innerText=this.options.text:t.innerHTML=this.options.text,this.options.avatar!==""){var d=document.createElement("img");d.src=this.options.avatar,d.className="toastify-avatar",this.options.position=="left"||this.options.positionLeft===!0?t.appendChild(d):t.insertAdjacentElement("afterbegin",d)}if(this.options.close===!0){var c=document.createElement("button");c.type="button",c.setAttribute("aria-label","Close"),c.className="toast-close",c.innerHTML="&#10006;",c.addEventListener("click",(function(m){m.stopPropagation(),this.removeElement(this.toastElement),window.clearTimeout(this.toastElement.timeOutValue)}).bind(this));var l=window.innerWidth>0?window.innerWidth:screen.width;(this.options.position=="left"||this.options.positionLeft===!0)&&l>360?t.insertAdjacentElement("afterbegin",c):t.appendChild(c)}if(this.options.stopOnFocus&&this.options.duration>0){var u=this;t.addEventListener("mouseover",function(m){window.clearTimeout(t.timeOutValue)}),t.addEventListener("mouseleave",function(){t.timeOutValue=window.setTimeout(function(){u.removeElement(t)},u.options.duration)})}if(typeof this.options.destination<"u"&&t.addEventListener("click",(function(m){m.stopPropagation(),this.options.newWindow===!0?window.open(this.options.destination,"_blank"):window.location=this.options.destination}).bind(this)),typeof this.options.onClick=="function"&&typeof this.options.destination>"u"&&t.addEventListener("click",(function(m){m.stopPropagation(),this.options.onClick()}).bind(this)),typeof this.options.offset=="object"){var h=s("x",this.options),w=s("y",this.options),k=this.options.position=="left"?h:"-"+h,it=this.options.gravity=="toastify-top"?w:"-"+w;t.style.transform="translate("+k+","+it+")"}return t},showToast:function(){this.toastElement=this.buildToast();var t;if(typeof this.options.selector=="string"?t=document.getElementById(this.options.selector):this.options.selector instanceof HTMLElement||typeof ShadowRoot<"u"&&this.options.selector instanceof ShadowRoot?t=this.options.selector:t=document.body,!t)throw"Root element is not defined";var r=e.defaults.oldestFirst?t.firstChild:t.lastChild;return t.insertBefore(this.toastElement,r),e.reposition(),this.options.duration>0&&(this.toastElement.timeOutValue=window.setTimeout((function(){this.removeElement(this.toastElement)}).bind(this),this.options.duration)),this},hideToast:function(){this.toastElement.timeOutValue&&clearTimeout(this.toastElement.timeOutValue),this.removeElement(this.toastElement)},removeElement:function(t){t.className=t.className.replace(" on",""),window.setTimeout((function(){this.options.node&&this.options.node.parentNode&&this.options.node.parentNode.removeChild(this.options.node),t.parentNode&&t.parentNode.removeChild(t),this.options.callback.call(t),e.reposition()}).bind(this),400)}},e.reposition=function(){for(var t={top:15,bottom:15},r={top:15,bottom:15},d={top:15,bottom:15},c=document.getElementsByClassName("toastify"),l,u=0;u<c.length;u++){a(c[u],"toastify-top")===!0?l="toastify-top":l="toastify-bottom";var h=c[u].offsetHeight;l=l.substr(9,l.length-1);var w=15,k=window.innerWidth>0?window.innerWidth:screen.width;k<=360?(c[u].style[l]=d[l]+"px",d[l]+=h+w):a(c[u],"toastify-left")===!0?(c[u].style[l]=t[l]+"px",t[l]+=h+w):(c[u].style[l]=r[l]+"px",r[l]+=h+w)}return this};function s(t,r){return r.offset[t]?isNaN(r.offset[t])?r.offset[t]:r.offset[t]+"px":"0px"}function a(t,r){return!t||typeof r!="string"?!1:!!(t.className&&t.className.trim().split(/\s+/gi).indexOf(r)>-1)}return e.lib.init.prototype=e.lib,e})}(E)),E.exports}var tt=$();const et=Y(tt),ot=new j,O=document.querySelector(".inputarea"),g=document.querySelector(".convertedResult"),W=document.querySelector(".copyResult");function A(){const o=O.value,n=ot.ewtsToUnicode(o);g.textContent=n;const e=n.length===0;g.classList.toggle("empty",e),W.disabled=e}function nt(){O.addEventListener("input",()=>{A(),g.scrollTo({top:999999,behavior:"smooth"})})}A(),nt(),W.addEventListener("click",()=>{const o=g.textContent;!o||!o.trim()||navigator.clipboard.writeText(o).then(()=>{et({text:"Text copied",duration:2e3,newWindow:!0,gravity:"top",position:"right",style:{background:"linear-gradient(to right, var(--color_orange), var(--color_pink))",fontSize:"1.2em"}}).showToast()})})})();
