---
id: 4ed6b33a-d1b9-4888-87b3-dd7e85500b7b
title: streaming_html_bodies_with_axum
date: 2025-02-28 16:18:51.909780723 UTC
description: Stream HTML to clients. The best rendering pattern known.
draft: true
---

# Streaming HTML

I was listening to a podcast awhile back and the guest was talking about how
amazing it is that browsers do their best to render absolutely anything they're
given-don't send a `<head>` or `<body>`, they'll make due. Don't close the `<p>` tag,
no worries, _they got you_. Give them incomplete HTML and finish later, they'll
do their best with what they have at any given moment and happily fill in the rest
when you're finsihed.

Amazon's store's homepage and Google's initial search result were given as examples
as a companies that want more than anything to paint 'above the fold' and fill
the rest later. The user doesn't see the whole page right away, so why send it
to them right away?

This was something that really spoke to me and wanted to give a try on my own.

Some time later I ran across Airbnb's tech blog about how they've (improved their
performance with HTTP streaming)[https://airbnb.tech/infrastructure/improving-performance-with-http-streaming/].
But their implementation streamed to a React client. What's the point? Well I get the
point, but why go through all that trouble if you're going to send that much JS
to the client anyway?

## Drawbacks

There's no way to hydrate the page after the document is completely loaded without
JavaScript. If a user is browsing with JS disabled, they'll just see the loading state
thinking the website's borked beyond belief.
