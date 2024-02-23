---
id: 727e1e7a-5840-4d7f-a2e4-d31d7db0aa68
title: a_rust_blog_with_axum_htmx_and_cloudflare
date: 2024-2-23 19:13:30.666072 UTC
description: Building a blog site in Rust with Axum and HTMX and hosting it on Fly.io and Cloudflare.
draft: false
---

## [HTMX](https://htmx.org)
I [first learned about HTMX](/bl0g/a_solo_engineer_dreams_an_mvc) when I was writing Python web apps 
and read some article in the Django community about it. Not wanting to learn a JS framework, but also not wanting
to write vanilla JS to perform AJAX requests, HTMX looked like the perfect solution. It was only after using 
that I realized how powerfull it was an SPA tool. I didn't think too much more of it though. It was neat and 
I enjoyed using it, but I didn't see myself using it more in the future. It felt brittle and like something I would never
use professionally.

Then last August I noticed HTMX trending a lot on tech Twitter, especially among 'influential' JS developers. I had moved on from 
Python, Django, and Flask, have been in the trite Node, React SPA job. Seeing HTMX again made me curious to give another go. I essentially
am surrounded by people who only think in SPA, client side everything. What would I think of HTMX now?

... And wow! Has my thinking changed. I read [_Hypermedia Systems_](https://hypermedia.systems) and what HTMX has set out to solve
makes incredibly good sense. Simplify state, embrace REST in its original sense. The fact that HTMX empowers backend developers to create
SPAs is looking at it backwards. It's really about extending the simplicity of a MVC into a modern, and powerfull full stack app.

__HATEOAS (**H**ypermedia **a**s **t**he **E**ngine **o**f **A**pplication **S**tate) is the solution.__

__REST is not JSON APIs.__

If I said either of those things out loud at my current job, any cred I'd have would be gone. 

But it's _FUCKING_ true! There is no reason to use Redux anymore. There's no reason for all the state to be on the client. We are constantly 
creating subpar user experiences because we don't want to deal with all the different possibilities of the client's state. We want to give
them a SPA and make them step through it serially. 

And to do that we have to send all JS to the client, the client has to parse and execute the JS, the JS can then get the fucking state, and render
the page.

The performance you can get out of that model is capped, no matter the JS framework, no matter how much you want to make it appear like that site is performant it never will be. 
Look at [react.dev](https://react.dev) - the brand new React site, built by the React team, presenting static data, and load the page with the network 
restricted. It will take over a second for the search bar to become active. This is a site by the _React_ team. Paid for by _Meta_ and that's the best
they can do? 

Just send the JS the client needs for that page. Send some actual HTML that the browser natively can render and describes itself. Let the state live 
on the server and each request to the server provides the state and what the client needs. 

Don't be afraid to make server requests. Even with the SPA you're already doing. It's just more complicated.

From here on out, any app I make personally or through a contract, I'm going to use something like HTMX.

Enough of the ranting, the point of this acticle is to articulate how I built the site you're currently on, starting with...

## Cloudflare

I really like Cloudflare. It's been an amazing alternative to things like Vercel and Netifly for serverless JS apps,
and their Workers API has been something I've wanted to spend more time with but haven't had a reason to yet.

### Caching and Caching Pitfalls

As stated above, I've been using and loving the idea of HATEOAS. Only send the browser HTML from the server.
Only send what's need. There's only a single source of truth for state, and it's the server.

You get the idea.

When you host your website on Cloudflare infrastructure, you can use their caching for **FREE**! 
Why pay more per request to your origin server when Cloudflare can do the same and faster. I have one origin, 
but I'm able to take advantage of Cloudflare's globally distruted edge network. Caches are faster worldwide,
even when my server is using Rust. It reminds me of my performance mantra: _It's always faster to not do something than do something_.
It's always faster to not go to my origin.

My only complaint with Cloudflare's cache is it doesn't respect `Vary` headers, or any headers at all. 
My server decides whether to return a partial document of just the necessary HTML by the existence of a `HX-Request` request header. I should be
able to cache that response and the full HTML response base on the presence of that `Vary` header, but Cloudflare doesn't 
support that... I initially had a work around where the HTMX requests for partial HTML included a query param (that Cloudflare does respect) and 
I would then trim off the param before it showed in the URL so that refreshes or sent links would still work correctly. And that worked, everything
was cached and it was beautiful. 

But it felt like that wasn't in the vein of this site, where I wanted simplicity and performance. This was adding complexity for performance.

So I turned off Cloudflare's cache and every request goes to the origin. It's still fast, just not as much so.

## Fly

## Axum

