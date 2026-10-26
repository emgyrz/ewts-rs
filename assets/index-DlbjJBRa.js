(async () => {
  (function() {
    const i = document.createElement("link").relList;
    if (i && i.supports && i.supports("modulepreload")) return;
    for (const n of document.querySelectorAll('link[rel="modulepreload"]')) a(n);
    new MutationObserver((n) => {
      for (const r of n) if (r.type === "childList") for (const t of r.addedNodes) t.tagName === "LINK" && t.rel === "modulepreload" && a(t);
    }).observe(document, {
      childList: true,
      subtree: true
    });
    function e(n) {
      const r = {};
      return n.integrity && (r.integrity = n.integrity), n.referrerPolicy && (r.referrerPolicy = n.referrerPolicy), n.crossOrigin === "use-credentials" ? r.credentials = "include" : n.crossOrigin === "anonymous" ? r.credentials = "omit" : r.credentials = "same-origin", r;
    }
    function a(n) {
      if (n.ep) return;
      n.ep = true;
      const r = e(n);
      fetch(n.href, r);
    }
  })();
  const R = "" + new URL("ewts_wasm_bg-BEnX4Hc7.wasm", import.meta.url).href, S = async (o = {}, i) => {
    let e;
    if (i.startsWith("data:")) {
      const a = i.replace(/^data:.*?base64,/, "");
      let n;
      if (typeof Buffer == "function" && typeof Buffer.from == "function") n = Buffer.from(a, "base64");
      else if (typeof atob == "function") {
        const r = atob(a);
        n = new Uint8Array(r.length);
        for (let t = 0; t < r.length; t++) n[t] = r.charCodeAt(t);
      } else throw new Error("Cannot decode base64-encoded data URL");
      e = await WebAssembly.instantiate(n, o);
    } else {
      const a = await fetch(i), n = a.headers.get("Content-Type") || "";
      if ("instantiateStreaming" in WebAssembly && n.startsWith("application/wasm")) e = await WebAssembly.instantiateStreaming(a, o);
      else {
        const r = await a.arrayBuffer();
        e = await WebAssembly.instantiate(r, o);
      }
    }
    return e.instance.exports;
  };
  let u;
  function M(o) {
    u = o;
  }
  const F = typeof TextDecoder > "u" ? (0, module.require)("util").TextDecoder : TextDecoder;
  let L = new F("utf-8", {
    ignoreBOM: true,
    fatal: true
  });
  L.decode();
  let m = null;
  function _() {
    return (m === null || m.byteLength === 0) && (m = new Uint8Array(u.memory.buffer)), m;
  }
  function C(o, i) {
    return o = o >>> 0, L.decode(_().subarray(o, o + i));
  }
  let T = 0;
  const U = typeof TextEncoder > "u" ? (0, module.require)("util").TextEncoder : TextEncoder;
  let y = new U("utf-8");
  const P = typeof y.encodeInto == "function" ? function(o, i) {
    return y.encodeInto(o, i);
  } : function(o, i) {
    const e = y.encode(o);
    return i.set(e), {
      read: o.length,
      written: e.length
    };
  };
  function B(o, i, e) {
    if (e === void 0) {
      const s = y.encode(o), d = i(s.length, 1) >>> 0;
      return _().subarray(d, d + s.length).set(s), T = s.length, d;
    }
    let a = o.length, n = i(a, 1) >>> 0;
    const r = _();
    let t = 0;
    for (; t < a; t++) {
      const s = o.charCodeAt(t);
      if (s > 127) break;
      r[n + t] = s;
    }
    if (t !== a) {
      t !== 0 && (o = o.slice(t)), n = e(n, a, a = t + o.length * 3, 1) >>> 0;
      const s = _().subarray(n + t, n + a), d = P(o, s);
      t += d.written, n = e(n, a, t, 1) >>> 0;
    }
    return T = t, n;
  }
  const E = typeof FinalizationRegistry > "u" ? {
    register: () => {
    },
    unregister: () => {
    }
  } : new FinalizationRegistry((o) => u.__wbg_ewtsconverter_free(o >>> 0, 1));
  class z {
    __destroy_into_raw() {
      const i = this.__wbg_ptr;
      return this.__wbg_ptr = 0, E.unregister(this), i;
    }
    free() {
      const i = this.__destroy_into_raw();
      u.__wbg_ewtsconverter_free(i, 0);
    }
    constructor() {
      const i = u.ewtsconverter_new();
      return this.__wbg_ptr = i >>> 0, E.register(this, this.__wbg_ptr, this), this;
    }
    ewtsToUnicode(i) {
      let e, a;
      try {
        const n = B(i, u.__wbindgen_malloc, u.__wbindgen_realloc), r = T, t = u.ewtsconverter_ewtsToUnicode(this.__wbg_ptr, n, r);
        return e = t[0], a = t[1], C(t[0], t[1]);
      } finally {
        u.__wbindgen_free(e, a, 1);
      }
    }
  }
  function j(o, i) {
    throw new Error(C(o, i));
  }
  function q() {
    const o = u.__wbindgen_export_0, i = o.grow(4);
    o.set(0, void 0), o.set(i + 0, void 0), o.set(i + 1, null), o.set(i + 2, true), o.set(i + 3, false);
  }
  URL = globalThis.URL;
  const D = await S({
    "./ewts_wasm_bg.js": {
      __wbindgen_throw: j,
      __wbindgen_init_externref_table: q
    }
  }, R), { memory: I, __wbg_ewtsconverter_free: V, ewtsconverter_new: H, ewtsconverter_ewtsToUnicode: $, __wbindgen_export_0: K, __wbindgen_malloc: X, __wbindgen_realloc: G, __wbindgen_free: J, __wbindgen_start: N } = D, Q = Object.freeze(Object.defineProperty({
    __proto__: null,
    __wbg_ewtsconverter_free: V,
    __wbindgen_export_0: K,
    __wbindgen_free: J,
    __wbindgen_malloc: X,
    __wbindgen_realloc: G,
    __wbindgen_start: N,
    ewtsconverter_ewtsToUnicode: $,
    ewtsconverter_new: H,
    memory: I
  }, Symbol.toStringTag, {
    value: "Module"
  }));
  M(Q);
  N();
  function Y(o) {
    return o && o.__esModule && Object.prototype.hasOwnProperty.call(o, "default") ? o.default : o;
  }
  var b = {
    exports: {}
  };
  var Z = b.exports, x;
  function tt() {
    return x || (x = 1, (function(o) {
      (function(i, e) {
        o.exports ? o.exports = e() : i.Toastify = e();
      })(Z, function(i) {
        var e = function(t) {
          return new e.lib.init(t);
        }, a = "1.12.0";
        e.defaults = {
          oldestFirst: true,
          text: "Toastify is awesome!",
          node: void 0,
          duration: 3e3,
          selector: void 0,
          callback: function() {
          },
          destination: void 0,
          newWindow: false,
          close: false,
          gravity: "toastify-top",
          positionLeft: false,
          position: "",
          backgroundColor: "",
          avatar: "",
          className: "",
          stopOnFocus: true,
          onClick: function() {
          },
          offset: {
            x: 0,
            y: 0
          },
          escapeMarkup: true,
          ariaLive: "polite",
          style: {
            background: ""
          }
        }, e.lib = e.prototype = {
          toastify: a,
          constructor: e,
          init: function(t) {
            return t || (t = {}), this.options = {}, this.toastElement = null, this.options.text = t.text || e.defaults.text, this.options.node = t.node || e.defaults.node, this.options.duration = t.duration === 0 ? 0 : t.duration || e.defaults.duration, this.options.selector = t.selector || e.defaults.selector, this.options.callback = t.callback || e.defaults.callback, this.options.destination = t.destination || e.defaults.destination, this.options.newWindow = t.newWindow || e.defaults.newWindow, this.options.close = t.close || e.defaults.close, this.options.gravity = t.gravity === "bottom" ? "toastify-bottom" : e.defaults.gravity, this.options.positionLeft = t.positionLeft || e.defaults.positionLeft, this.options.position = t.position || e.defaults.position, this.options.backgroundColor = t.backgroundColor || e.defaults.backgroundColor, this.options.avatar = t.avatar || e.defaults.avatar, this.options.className = t.className || e.defaults.className, this.options.stopOnFocus = t.stopOnFocus === void 0 ? e.defaults.stopOnFocus : t.stopOnFocus, this.options.onClick = t.onClick || e.defaults.onClick, this.options.offset = t.offset || e.defaults.offset, this.options.escapeMarkup = t.escapeMarkup !== void 0 ? t.escapeMarkup : e.defaults.escapeMarkup, this.options.ariaLive = t.ariaLive || e.defaults.ariaLive, this.options.style = t.style || e.defaults.style, t.backgroundColor && (this.options.style.background = t.backgroundColor), this;
          },
          buildToast: function() {
            if (!this.options) throw "Toastify is not initialized";
            var t = document.createElement("div");
            t.className = "toastify on " + this.options.className, this.options.position ? t.className += " toastify-" + this.options.position : this.options.positionLeft === true ? (t.className += " toastify-left", console.warn("Property `positionLeft` will be depreciated in further versions. Please use `position` instead.")) : t.className += " toastify-right", t.className += " " + this.options.gravity, this.options.backgroundColor && console.warn('DEPRECATION NOTICE: "backgroundColor" is being deprecated. Please use the "style.background" property.');
            for (var s in this.options.style) t.style[s] = this.options.style[s];
            if (this.options.ariaLive && t.setAttribute("aria-live", this.options.ariaLive), this.options.node && this.options.node.nodeType === Node.ELEMENT_NODE) t.appendChild(this.options.node);
            else if (this.options.escapeMarkup ? t.innerText = this.options.text : t.innerHTML = this.options.text, this.options.avatar !== "") {
              var d = document.createElement("img");
              d.src = this.options.avatar, d.className = "toastify-avatar", this.options.position == "left" || this.options.positionLeft === true ? t.appendChild(d) : t.insertAdjacentElement("afterbegin", d);
            }
            if (this.options.close === true) {
              var c = document.createElement("button");
              c.type = "button", c.setAttribute("aria-label", "Close"), c.className = "toast-close", c.innerHTML = "&#10006;", c.addEventListener("click", (function(w) {
                w.stopPropagation(), this.removeElement(this.toastElement), window.clearTimeout(this.toastElement.timeOutValue);
              }).bind(this));
              var l = window.innerWidth > 0 ? window.innerWidth : screen.width;
              (this.options.position == "left" || this.options.positionLeft === true) && l > 360 ? t.insertAdjacentElement("afterbegin", c) : t.appendChild(c);
            }
            if (this.options.stopOnFocus && this.options.duration > 0) {
              var f = this;
              t.addEventListener("mouseover", function(w) {
                window.clearTimeout(t.timeOutValue);
              }), t.addEventListener("mouseleave", function() {
                t.timeOutValue = window.setTimeout(function() {
                  f.removeElement(t);
                }, f.options.duration);
              });
            }
            if (typeof this.options.destination < "u" && t.addEventListener("click", (function(w) {
              w.stopPropagation(), this.options.newWindow === true ? window.open(this.options.destination, "_blank") : window.location = this.options.destination;
            }).bind(this)), typeof this.options.onClick == "function" && typeof this.options.destination > "u" && t.addEventListener("click", (function(w) {
              w.stopPropagation(), this.options.onClick();
            }).bind(this)), typeof this.options.offset == "object") {
              var p = n("x", this.options), h = n("y", this.options), v = this.options.position == "left" ? p : "-" + p, W = this.options.gravity == "toastify-top" ? h : "-" + h;
              t.style.transform = "translate(" + v + "," + W + ")";
            }
            return t;
          },
          showToast: function() {
            this.toastElement = this.buildToast();
            var t;
            if (typeof this.options.selector == "string" ? t = document.getElementById(this.options.selector) : this.options.selector instanceof HTMLElement || typeof ShadowRoot < "u" && this.options.selector instanceof ShadowRoot ? t = this.options.selector : t = document.body, !t) throw "Root element is not defined";
            var s = e.defaults.oldestFirst ? t.firstChild : t.lastChild;
            return t.insertBefore(this.toastElement, s), e.reposition(), this.options.duration > 0 && (this.toastElement.timeOutValue = window.setTimeout((function() {
              this.removeElement(this.toastElement);
            }).bind(this), this.options.duration)), this;
          },
          hideToast: function() {
            this.toastElement.timeOutValue && clearTimeout(this.toastElement.timeOutValue), this.removeElement(this.toastElement);
          },
          removeElement: function(t) {
            t.className = t.className.replace(" on", ""), window.setTimeout((function() {
              this.options.node && this.options.node.parentNode && this.options.node.parentNode.removeChild(this.options.node), t.parentNode && t.parentNode.removeChild(t), this.options.callback.call(t), e.reposition();
            }).bind(this), 400);
          }
        }, e.reposition = function() {
          for (var t = {
            top: 15,
            bottom: 15
          }, s = {
            top: 15,
            bottom: 15
          }, d = {
            top: 15,
            bottom: 15
          }, c = document.getElementsByClassName("toastify"), l, f = 0; f < c.length; f++) {
            r(c[f], "toastify-top") === true ? l = "toastify-top" : l = "toastify-bottom";
            var p = c[f].offsetHeight;
            l = l.substr(9, l.length - 1);
            var h = 15, v = window.innerWidth > 0 ? window.innerWidth : screen.width;
            v <= 360 ? (c[f].style[l] = d[l] + "px", d[l] += p + h) : r(c[f], "toastify-left") === true ? (c[f].style[l] = t[l] + "px", t[l] += p + h) : (c[f].style[l] = s[l] + "px", s[l] += p + h);
          }
          return this;
        };
        function n(t, s) {
          return s.offset[t] ? isNaN(s.offset[t]) ? s.offset[t] : s.offset[t] + "px" : "0px";
        }
        function r(t, s) {
          return !t || typeof s != "string" ? false : !!(t.className && t.className.trim().split(/\s+/gi).indexOf(s) > -1);
        }
        return e.lib.init.prototype = e.lib, e;
      });
    })(b)), b.exports;
  }
  var et = tt();
  const ot = Y(et), it = new z(), k = document.querySelector(".inputarea"), g = document.querySelector(".convertedResult"), O = document.querySelector(".copyResult");
  function A() {
    const o = k.value, i = it.ewtsToUnicode(o);
    g.textContent = i;
    const e = i.length === 0;
    g.classList.toggle("empty", e), O.disabled = e;
  }
  function nt() {
    k.addEventListener("input", () => {
      A(), g.scrollTo({
        top: 999999,
        behavior: "smooth"
      });
    });
  }
  A();
  nt();
  O.addEventListener("click", () => {
    const o = g.textContent;
    !o || !o.trim() || navigator.clipboard.writeText(o).then(() => {
      ot({
        text: "Text copied",
        duration: 2e3,
        newWindow: true,
        gravity: "top",
        position: "right",
        style: {
          background: "linear-gradient(to right, var(--color_orange), var(--color_pink))",
          fontSize: "1.2em"
        }
      }).showToast();
    });
  });
})();
