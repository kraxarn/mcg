import{S as E,i as z,s as S,L as j,e as p,v as A,j as F,c as v,a as b,w as I,l as N,d as _,b as f,f as g,x as T,F as y,m as q,n as m,o as C,p as d,A as D,t as G,g as H,h as J,J as B,E as L,G as O,H as P,I as U}from"./vendor-6a93e758.js";function V(c){let t,s;return{c(){t=p("span"),s=G(c[1]),this.h()},l(l){t=v(l,"SPAN",{class:!0});var e=b(t);s=H(e,c[1]),e.forEach(_),this.h()},h(){f(t,"class","svelte-wzwvym")},m(l,e){g(l,t,e),y(t,s)},p(l,e){e&2&&J(s,l[1])},i:B,o:B,d(l){l&&_(t)}}}function K(c){let t,s;const l=c[4].default,e=L(l,c,c[3],null);return{c(){t=p("div"),e&&e.c(),this.h()},l(n){t=v(n,"DIV",{class:!0});var i=b(t);e&&e.l(i),i.forEach(_),this.h()},h(){f(t,"class","svelte-wzwvym")},m(n,i){g(n,t,i),e&&e.m(t,null),s=!0},p(n,i){e&&e.p&&(!s||i&8)&&O(e,l,n,n[3],s?U(l,n[3],i,null):P(n[3]),null)},i(n){s||(d(e,n),s=!0)},o(n){m(e,n),s=!1},d(n){n&&_(t),e&&e.d(n)}}}function M(c){let t,s,l,e,n,i;s=new j({props:{icon:c[2]}});const u=[K,V],o=[];function k(a,r){return a[1]===void 0?0:1}return e=k(c),n=o[e]=u[e](c),{c(){t=p("button"),A(s.$$.fragment),l=F(),n.c(),this.h()},l(a){t=v(a,"BUTTON",{id:!0,class:!0});var r=b(t);I(s.$$.fragment,r),l=N(r),n.l(r),r.forEach(_),this.h()},h(){f(t,"id",c[0]),f(t,"class","svelte-wzwvym")},m(a,r){g(a,t,r),T(s,t,null),y(t,l),o[e].m(t,null),i=!0},p(a,[r]){const w={};r&4&&(w.icon=a[2]),s.$set(w);let h=e;e=k(a),e===h?o[e].p(a,r):(q(),m(o[h],1,1,()=>{o[h]=null}),C(),n=o[e],n?n.p(a,r):(n=o[e]=u[e](a),n.c()),d(n,1),n.m(t,null)),(!i||r&1)&&f(t,"id",a[0])},i(a){i||(d(s.$$.fragment,a),d(n),i=!0)},o(a){m(s.$$.fragment,a),m(n),i=!1},d(a){a&&_(t),D(s),o[e].d()}}}function Q(c,t,s){let{$$slots:l={},$$scope:e}=t,{id:n}=t,{content:i}=t,{icon:u}=t;return c.$$set=o=>{"id"in o&&s(0,n=o.id),"content"in o&&s(1,i=o.content),"icon"in o&&s(2,u=o.icon),"$$scope"in o&&s(3,e=o.$$scope)},[n,i,u,e,l]}class W extends E{constructor(t){super();z(this,t,Q,M,S,{id:0,content:1,icon:2})}}export{W as B};