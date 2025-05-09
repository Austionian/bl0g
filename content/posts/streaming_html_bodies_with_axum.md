---
id: 4ed6b33a-d1b9-4888-87b3-dd7e85500b7b
title: streaming_html_bodies_with_axum
date: 2025-05-09 16:18:51.909780723 UTC
description: Stream HTML to clients. The best rendering pattern known.
draft: false
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
the rest later. The user doesn't see the whole page right away anyway, so why send it
all right away?

This was something that really spoke to me and wanted to give a try on my own.

Some time after that podcast I ran across Airbnb's tech blog about how they've [improved their
performance with HTTP streaming](https://airbnb.tech/infrastructure/improving-performance-with-http-streaming/).
I liked how they talked about flushing chunks of content.
But their implementation streamed to a React client. What's the point? Well I get the
point, but why go through all that trouble if you're going to send that much JS
to the client anyway?

I was inspired to do my own implemetation - one with Rust and minimal JS.

**Introducing [gathering.surf](https://gathering.surf)**

![gathering.surf rendering](/assets/images/gathering_surf.gif)

# How I Built It

Gathering.surf is a Lake Michigan focused surf forecast and realtime site. There's so
much data out there when looking at the current conditions (wave height, wind speed/direction,
air/water temperature) and forecast, but they're spread across many different
sites each with varying degrees of usability and performance. I wanted it all in one
place and consumable in a glance. I wanted what the coastal surfers already have in
surfline and similar sites.

I had an rendering in my head that went something like so:

- Display an instant loading screen
- Fetch and parse the data **in parallel**
- Update the UI as data is sent in to the document

## The Server

First I wanted just to get a basic framework of how to stream HTML from rust to a client.
I eventually found the [from_stream](https://docs.rs/axum/latest/axum/body/struct.Body.html#method.from_stream) method on the `Body` struct. I'm surprised I didn't see more examples or discussion of this.

Anyway, all I needed was a stream that implemented `TryStream`. In all, it meant I could take
a `mpsc` (multiple producer, single consumer) channel and convert it into the response body
from Axum. Something like this:

```rust
// create the channel
let (tx, rx) = mpsc::channel::<Result<String, Infallible>>(1);

// send the initial loading state html to the channel
tx.send(Ok(TEMPLATES.render("index.html", &context)?))
    .await?;

// do some things,
//
//
// send messages to the channel off the main thread

// convert the channel into the body
let stream = tokio_stream::wrappers::ReceiverStream::new(rx);
let body = Body::from_stream(stream);

// build the response
Ok(Response::builder()
    .status(StatusCode::OK)
    .header("Content-Type", "text/html; charset=UTF-8")
    .header("X-Content-Type-Options", "nosniff")
    .header("content-encoding", "none")
    .header("cache-control", "no-transform")
    .body(body)?)
```

And volia I have a channel that streams HTML to the client. Getting those headers
correct took a little toying, the main two being `content-encoding` and `cache-control`.

Once I had the channel it felt like I was off to the races! All I had to do was create an async
thread for each api and clone a reference to my `arc`-ed sender to give to the new thread:

```rust
let tx: Arc<Sender<Result<_, _>>> = Arc::new(tx);

let realtime_tx = tx.clone();
tokio::spawn(async move {
    match Realtime::try_get_string(realtime_spot, realtime_state).await {
        Ok(realtime) => {
            let html = html!(
                script type="application/json" id="realtime-data" {(
                PreEscaped(
                        realtime)
                )}
            )
            .into();

            realtime_tx.send(Ok(html)).await.unwrap();
            drop(realtime_tx);
        }
        Err(e) => {
            realtime_tx
                .send(Ok(html!((error_markup("latest", e))).into()))
                .await
                .unwrap();
        }
    }
});
```

As you can see in the script tag above, I stream the parsed api response as JSON to the client
and using `serde` makes this all extremely ergonomic. It also makes it incredibly simple
to host an api endpoint of my own parsed data - I just make the JSON available directly.

## The Client

This part isn't as fun honestly. I think it mostly has to do with the fact that I'm first showing a loading
state as fast as possible to the user and then updating the individual sections with the provided JSON.
So akin to how Airbinb does it, I have a [MutationObserver](https://developer.mozilla.org/en-US/docs/Web/API/MutationObserver) watching for new nodes to
be created on the document. In the code above you can see the realtime data is a `script` tag with
`id="realtime-data"` filled with the actual realtime JSON data.

My MutationObserver looks something like this:

```javascript
const observerCallback = async (mutationList) => {
    for (const mutation of mutationList) {
        if (mutation.target instanceof HTMLElement) {
            if (mutation.target.id === "realtime-data") {
                parseRealtime(JSON.parse(mutation.target.innerText));
            }}
    }

const observer = new MutationObserver(observerCallback);

// Select the node that will be observed for mutations
const targetNode = nonNull(document.querySelector("body"));

// Options for the observer (which mutations to observe)
const config = { attributes: true, childList: true, subtree: true };

observer.observe(targetNode, config);
```

Once the element with `realtime-data` as the id is loaded into the DOM I'm able to
parse its JSON contents and update the elements with the specific set of data.
Tedious but simple!

## Highlights

- There's nothing in the way of getting the initial loading HTML to the client.

- No matter which api is the slowest, it's not going to slow down the other content from being
  displayed to the user. They are like islands of SSR, but they're not rendered on the server.

- If one api fails, I can easily update just that api's related content with an error message,
  no need bring down the whole page.

# Host it on a Pi and Taking the Performance Up

Taking whole this project to another level, I thought it'd be really fun to host it on my
Raspberry Pi 4. Not that I was paying any money when I was originally hosting it on
Fly.io, it was more I wanted total control - and I wanted a Redis cache. I've found everything is fun
and games with web hosting until you introduce a database. It becomes primarly what you pay
for and, if it's cheap, what slows down your site (because the DB is serverless).

I'll write more about how the details about hosting on the Pi in a separate post - it
includes docker compose, cross compiling on x86 to aarch64, making deploys as fast and seemless
as what Cloudflare and Fly.io provide - but having a always on, free Redis cache next to
the service itself meant I could cache each individual api call as its own key. It's the same for
all users and since the underlying services only update a few times a day, a 5 minute TTL
really didn't seem like a much a risk of showing stale data.

Not only that, since I was hosting everything myself, it didn't cost me really anything but
electricity to keep the cache fresh even when no one's recently visited the site. I set a cronjob
on my Pi to hit the gathering.surf's root every 5 minutes to keep the cache as if
it was visited continusly.

## Tunnels

Now that I have a performant web app with incredible performance ready to serve surfer's
all the knowledge needed about the current and upcoming surf, I needed a way to expose it
to the public.

Enter [Cloudflare Tunnels](https://developers.cloudflare.com/cloudflare-one/connections/connect-networks/).

It essentially connects my Pi to Cloudflare's network, so I don't have to expose any of my local network
directly. Cloudflare handles the initial request, and with that comes all of Cloudflare's other
excellent features, and routes it to my Pi where I have the app running in a Docker container. (It's the same
way I host this very site!)

The only downside to using Cloudflare tunnels is that the streaming doesn't always work as expected, i.e. cloudflare
chunks data in buffers and only streams the response to the client when the buffer is full enough. If
the data is small it won't fill the buffer and cloudflare will wait until part of the next
chunk is created to send it. I'm not exactly sure that's what's happening or that there is no
work around.

I could ditch tunnels and get a static IP for my Pi, but then I'd start paying something
again and even then, I'd rather have a reverse proxy in the cloud and include
some sort of fallback if my personal power/network is down.

# Drawbacks

This whole endeavor hasn't been without its drawbacks:

- There's no way to hydrate the page after the document is completely loaded without
  JavaScript. If a user is browsing with JS disabled, they'll just see the loading state
  thinking the website's borked beyond belief.

- There's race conditions that are awkward if more than one api is needed or can effect how
  something is displayed. E.g. the realtime conditions can be out of date or there can be
  missing data if the bouy's no longer in the water or out of battery. In those instances if I just blindly update the
  frontend as the data comes regardless of how it's parsed I could be showing meaningless realtime
  data when in reality I want to be displaying the forecasted data for the current time.
  This pushes hairy display logic to the JS which makes and there's no way around it if
  I want each api to return its data as soon as it's ready.
