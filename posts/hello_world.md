---
title: hello_world
date: 2023-08-24 01:59:02.067798 UTC
description: A new hello, but also a hello again.
---
## Hello!

And welcome back to my blog. Excited to be doing this in Rust this time!! :)

Like just about every software engineer, a hobby of mine is rewritting my personal site. About two years ago
[I explored the JAMstack](https://github.com/Austionian?tab=repositories&q=blog.r00ks&type=&language=typescript&sort=) and tried out [Gatsby](https://www.gatsbyjs.com/) and [NextJS](https://nextjs.org).
Those iterations followed [my original static](https://github.com/Austionian/RooksBooksBlog), and _tbh_ most
pratical implementaion, [Hexo](https://hexo.io/index.html) site.

## A New Stack
Well it's not really a stack, more of a simple MVC in Axum, with Tera as the templating tool. The interesting bits
being the mirco JS frameworks -- HTMX and AlpineJS -- I'm using to make the website more dynamic than a simple
SSG.

### Pros of old ways
- Free hosting. It's easy to host a JAMstack site for free and have a really simple CI. Same with Hexo or some other
SSG, which can just be put on Github pages.

### Cons of the old ways
- The JAMstack is JS. Who really wants more JS in their lives?
