---
id: bfd4c3db-cfeb-4679-b205-cddfe832bbed
title: hello_world
date: 2023-08-24 01:59:02.067798 UTC
description: A new hello, but also a hello again.
draft: false
---
## Hello!

And welcome back to my blog. Excited to be doing this in Rust this time!! :)

Like just about every software engineer, a hobby of mine is rewritting my personal site. About two years ago
[I explored the JAMstack](https://github.com/Austionian?tab=repositories&q=blog.r00ks&type=&language=typescript&sort=) and tried out [Gatsby](https://www.gatsbyjs.com/) and [NextJS](https://nextjs.org).
Those iterations followed [my original static](https://github.com/Austionian/RooksBooksBlog), and _tbh_ most
pratical implementation, [Hexo](https://hexo.io/index.html) site.

## A New Stack
Well it's not really a stack, more of a simple MVC in Axum, with Tera as the templating tool. The interesting bits
being the mirco JS frameworks -- HTMX and AlpineJS -- I'm using to make the website more dynamic than a simple
SSG.

### Pros of old ways
- Free hosting. It's easy to host a JAMstack site for free and have a really simple CI. Same with Hexo or some other
SSG, which can just be put on Github pages. While my hosting now isn't free, [fly.io](https://fly.io) mostly is and the free 
tier of [cloudflare](https://cloudflare.com) is incredible. I'd be caching everything on their edge network if only they [supported
caching that respected `vary` headers](https://hypermedia.systems/hypermedia-components/#_caching_http_responses) for HTMX.

### Cons of the old ways
- The JAMstack is JS. Who really wants more JS in their lives?
- NextJS was a complete pain in the ass to try to keep updated. As I wasn't really interested in using that framework after building
my blog with it, it was difficult to want to get to the latest major, or even minor version, as both would break something.
- The site turned out well, but was complicated, and while fast, not as fast as this current implementation. As with any of the JAMstack sites,
you're pretty limited to React's startup and evaluation as your baseline speed. _Have you tried opening [react.dev](https://react.dev) with 
your network throttled? It looks fast, but takes seconds for the site to become interactive._
