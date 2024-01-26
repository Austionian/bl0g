---
id: 727e1e7a-5840-4d7f-a2e4-d31d7db0aa68
title: a_rust_blog_with_axum_htmx_and_cloudflare
date: 2023-09-14 19:13:30.666072 UTC
description: Building a blog site in Rust with Axum and HTMX and hosting it on Fly.io and Cloudflare.
draft: true
---

## [HTMX](https://htmx.org)
I [first learned about HTMX](/bl0g/a_solo_engineer_dreams_an_mvc) when I was writing Python web apps 
and read some article in the Django community about it. Not wanting to learn a JS framework, but also not wanting
to write vanilla JS to perform AJAX requests, HTMX looked like the perfect solution. It was only after using 
that I realized how powerfull it was an SPA tool. I didn't think too much more of though to be honest. It was neat and 
I enjoyed using it, but I didn't see myself using it more in the future. It felt brittle and like something I would never
use professionally.

About a month ago and I noticed HTMX trending a lot on tech Twitter, especially among 'influential' JS developers. It made me curious
and I decided I should take another look.

And boy has my thinking changed. I've been reading [_Hypermedia Systems_](https://hypermedia.systems) and what HTMX has set out to solve
makes incredibly good sense. Not only does it empower backend developers to create SPAs. The concept simplifies web dev.
JSON-fatigue is real and unnecessary. HATEOAS is the solution. 

## Cloudflare

I really like Cloudflare. It's been an amazing alternative to things like Vercel and Netifly for serverless JS apps,
and their Workers API has been something I've wanted to spend more time with but haven't had a reason to yet.

### Caching and Caching Pitfalls

I've been using and loving the idea of HATEOAS (**H**ypermedia **a**s **t**he **E**ngine **o**f **A**pplication **S**tate).
Only send the browser HTML from the server. Only send what's need. There's only a single source of truth for state, and it's the server.

You get the idea.

When you host your website on Cloudflare infrastructure, you can use their caching for **FREE**! 
Why pay more per request to your origin server when Cloudflare can do the same and faster. I have one origin, 
but I'm able to take advantage of Cloudflare's globally distruted edge network. Caches are faster worldwide,
even when my server is using Rust. It reminds me of my performance mantra: _It's always faster to not do something than do something_.
It's always faster to not go to my origin.

My only complaint with Cloudflare's cache is it doesn't respect `Vary` headers, or any headers at all. 
My server decides whether to return a partial document of just the necessary HTML by the existence of a `HX-Request` request header.
If the initial request to `/some-blog-post` is from HTMX.
