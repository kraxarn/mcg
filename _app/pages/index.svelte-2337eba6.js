import{S as J,i as Q,s as W,e as _,J as L,j as N,t as $,c as m,a as i,K as U,d as o,l as j,g as D,b as t,L as V,f as q,H as e,M as z,h as F,I as K,N as X,O as Y,P as Z,v as ee,Q as te,w as se,R as ae,x as re,p as le,n as oe,A as ne}from"../chunks/vendor-f77cd1bd.js";function ce(f){let s,a,l,c,u,d,v,n,y=Math.floor(f[1]+1)+"",T,O,E,I=Math.floor(f[1])+"",w,R,p,g,k,b,B;return{c(){s=_("div"),a=_("button"),l=L("svg"),c=L("path"),u=N(),d=_("div"),v=_("div"),n=_("strong"),T=$(y),O=N(),E=_("strong"),w=$(I),R=N(),p=_("button"),g=L("svg"),k=L("path"),this.h()},l(r){s=m(r,"DIV",{class:!0});var h=i(s);a=m(h,"BUTTON",{"aria-label":!0,class:!0});var M=i(a);l=U(M,"svg",{"aria-hidden":!0,viewBox:!0,class:!0});var x=i(l);c=U(x,"path",{d:!0,class:!0}),i(c).forEach(o),x.forEach(o),M.forEach(o),u=j(h),d=m(h,"DIV",{class:!0});var G=i(d);v=m(G,"DIV",{class:!0,style:!0});var S=i(v);n=m(S,"STRONG",{style:!0,"aria-hidden":!0,class:!0});var C=i(n);T=D(C,y),C.forEach(o),O=j(S),E=m(S,"STRONG",{class:!0});var H=i(E);w=D(H,I),H.forEach(o),S.forEach(o),G.forEach(o),R=j(h),p=m(h,"BUTTON",{"aria-label":!0,class:!0});var P=i(p);g=U(P,"svg",{"aria-hidden":!0,viewBox:!0,class:!0});var A=i(g);k=U(A,"path",{d:!0,class:!0}),i(k).forEach(o),A.forEach(o),P.forEach(o),h.forEach(o),this.h()},h(){t(c,"d","M0,0.5 L1,0.5"),t(c,"class","svelte-139m1ow"),t(l,"aria-hidden","true"),t(l,"viewBox","0 0 1 1"),t(l,"class","svelte-139m1ow"),t(a,"aria-label","Decrease the counter by one"),t(a,"class","svelte-139m1ow"),V(n,"top","-100%"),t(n,"aria-hidden","true"),t(n,"class","svelte-139m1ow"),t(E,"class","svelte-139m1ow"),t(v,"class","counter-digits svelte-139m1ow"),V(v,"transform","translate(0, "+100*f[2]+"%)"),t(d,"class","counter-viewport svelte-139m1ow"),t(k,"d","M0,0.5 L1,0.5 M0.5,0 L0.5,1"),t(k,"class","svelte-139m1ow"),t(g,"aria-hidden","true"),t(g,"viewBox","0 0 1 1"),t(g,"class","svelte-139m1ow"),t(p,"aria-label","Increase the counter by one"),t(p,"class","svelte-139m1ow"),t(s,"class","counter svelte-139m1ow")},m(r,h){q(r,s,h),e(s,a),e(a,l),e(l,c),e(s,u),e(s,d),e(d,v),e(v,n),e(n,T),e(v,O),e(v,E),e(E,w),e(s,R),e(s,p),e(p,g),e(g,k),b||(B=[z(a,"click",f[4]),z(p,"click",f[5])],b=!0)},p(r,[h]){h&2&&y!==(y=Math.floor(r[1]+1)+"")&&F(T,y),h&2&&I!==(I=Math.floor(r[1])+"")&&F(w,I),h&4&&V(v,"transform","translate(0, "+100*r[2]+"%)")},i:K,o:K,d(r){r&&o(s),b=!1,X(B)}}}function ie(f,s){return(f%s+s)%s}function ue(f,s,a){let l,c,u=0;const d=Y();Z(f,d,y=>a(1,c=y));const v=()=>a(0,u-=1),n=()=>a(0,u+=1);return f.$$.update=()=>{f.$$.dirty&1&&d.set(u),f.$$.dirty&2&&a(2,l=ie(c,1))},[u,c,l,d,v,n]}class de extends J{constructor(s){super();Q(this,s,ue,ce,W,{})}}function ve(f){let s,a,l,c,u,d,v,n,y,T,O,E,I,w,R,p,g,k,b,B;return b=new de({}),{c(){s=N(),a=_("section"),l=_("h1"),c=_("div"),u=_("picture"),d=_("source"),v=N(),n=_("img"),T=$(`

		to your new`),O=_("br"),E=$("SvelteKit app"),I=N(),w=_("h2"),R=$("try editing "),p=_("strong"),g=$("src/routes/index.svelte"),k=N(),ee(b.$$.fragment),this.h()},l(r){te('[data-svelte="svelte-1anpopb"]',document.head).forEach(o),s=j(r),a=m(r,"SECTION",{class:!0});var M=i(a);l=m(M,"H1",{class:!0});var x=i(l);c=m(x,"DIV",{class:!0});var G=i(c);u=m(G,"PICTURE",{});var S=i(u);d=m(S,"SOURCE",{srcset:!0,type:!0}),v=j(S),n=m(S,"IMG",{src:!0,alt:!0,class:!0}),S.forEach(o),G.forEach(o),T=D(x,`

		to your new`),O=m(x,"BR",{}),E=D(x,"SvelteKit app"),x.forEach(o),I=j(M),w=m(M,"H2",{});var C=i(w);R=D(C,"try editing "),p=m(C,"STRONG",{});var H=i(p);g=D(H,"src/routes/index.svelte"),H.forEach(o),C.forEach(o),k=j(M),se(b.$$.fragment,M),M.forEach(o),this.h()},h(){document.title="Home",t(d,"srcset","svelte-welcome.webp"),t(d,"type","image/webp"),ae(n.src,y="svelte-welcome.png")||t(n,"src",y),t(n,"alt","Welcome"),t(n,"class","svelte-mjk9ig"),t(c,"class","welcome svelte-mjk9ig"),t(l,"class","svelte-mjk9ig"),t(a,"class","svelte-mjk9ig")},m(r,h){q(r,s,h),q(r,a,h),e(a,l),e(l,c),e(c,u),e(u,d),e(u,v),e(u,n),e(l,T),e(l,O),e(l,E),e(a,I),e(a,w),e(w,R),e(w,p),e(p,g),e(a,k),re(b,a,null),B=!0},p:K,i(r){B||(le(b.$$.fragment,r),B=!0)},o(r){oe(b.$$.fragment,r),B=!1},d(r){r&&o(s),r&&o(a),ne(b)}}}const _e=!0;class me extends J{constructor(s){super();Q(this,s,null,ve,W,{})}}export{me as default,_e as prerender};
