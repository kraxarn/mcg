import{S as u,i as _,s as h,e as o,c,a as m,d,J as f,b as r,f as v,H as p,I as l}from"../chunks/vendor-ddf3b873.js";function g(i){let e,t,n;return{c(){e=o("div"),t=o("img"),this.h()},l(s){e=c(s,"DIV",{id:!0,class:!0});var a=m(e);t=c(a,"IMG",{src:!0,alt:!0}),a.forEach(d),this.h()},h(){f(t.src,n="images/logo.webp")||r(t,"src",n),r(t,"alt","logo"),r(e,"id","container"),r(e,"class","svelte-qe5r7j")},m(s,a){v(s,e,a),p(e,t)},p:l,i:l,o:l,d(s){s&&d(e)}}}const y=({session:i})=>{if(!i.user)return{status:302,redirect:"/login"}};class b extends u{constructor(e){super();_(this,e,null,g,h,{})}}export{b as default,y as load};
