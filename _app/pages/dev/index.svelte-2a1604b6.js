import{S as P,i as D,s as E,v as l,j as k,w as c,l as d,x as m,f as u,p,n as i,A as _,d as o,e as v,t as b,c as B,a as j,g as w,b as h,F as x}from"../../chunks/vendor-6a93e758.js";import{B as A}from"../../chunks/Button-ae341517.js";import{B as N}from"../../chunks/Banner-bf3096d8.js";import{P as S}from"../../chunks/Panel-8ebc6e51.js";function q(f){let t,s;return{c(){t=v("a"),s=b("Nakama"),this.h()},l(a){t=B(a,"A",{"sveltekit:prefetch":!0,href:!0});var n=j(t);s=w(n,"Nakama"),n.forEach(o),this.h()},h(){h(t,"sveltekit:prefetch",""),h(t,"href","/dev/nakama")},m(a,n){u(a,t,n),x(t,s)},d(a){a&&o(t)}}}function C(f){let t,s;return{c(){t=v("a"),s=b("Back"),this.h()},l(a){t=B(a,"A",{"sveltekit:prefetch":!0,href:!0});var n=j(t);s=w(n,"Back"),n.forEach(o),this.h()},h(){h(t,"sveltekit:prefetch",""),h(t,"href","/")},m(a,n){u(a,t,n),x(t,s)},d(a){a&&o(t)}}}function F(f){let t,s,a,n;return t=new A({props:{$$slots:{default:[q]},$$scope:{ctx:f}}}),a=new A({props:{$$slots:{default:[C]},$$scope:{ctx:f}}}),{c(){l(t.$$.fragment),s=k(),l(a.$$.fragment)},l(e){c(t.$$.fragment,e),s=d(e),c(a.$$.fragment,e)},m(e,r){m(t,e,r),u(e,s,r),m(a,e,r),n=!0},p(e,r){const $={};r&1&&($.$$scope={dirty:r,ctx:e}),t.$set($);const g={};r&1&&(g.$$scope={dirty:r,ctx:e}),a.$set(g)},i(e){n||(p(t.$$.fragment,e),p(a.$$.fragment,e),n=!0)},o(e){i(t.$$.fragment,e),i(a.$$.fragment,e),n=!1},d(e){_(t,e),e&&o(s),_(a,e)}}}function z(f){let t,s,a,n;return t=new N({props:{title:"Developer"}}),a=new S({props:{$$slots:{default:[F]},$$scope:{ctx:f}}}),{c(){l(t.$$.fragment),s=k(),l(a.$$.fragment)},l(e){c(t.$$.fragment,e),s=d(e),c(a.$$.fragment,e)},m(e,r){m(t,e,r),u(e,s,r),m(a,e,r),n=!0},p(e,[r]){const $={};r&1&&($.$$scope={dirty:r,ctx:e}),a.$set($)},i(e){n||(p(t.$$.fragment,e),p(a.$$.fragment,e),n=!0)},o(e){i(t.$$.fragment,e),i(a.$$.fragment,e),n=!1},d(e){_(t,e),e&&o(s),_(a,e)}}}class K extends P{constructor(t){super();D(this,t,null,z,E,{})}}export{K as default};