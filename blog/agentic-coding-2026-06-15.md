---
title: "My Approach to Agentic Coding (2026-06-15)"
description: TODO
timestamp: 2026-06-15T10:51:45
tags:
  - ai-agents
  - coding
canonical_url: https://zanca.dev/blog/agentic-coding-2026-06-15
publish: false
---

Quick disclaimer: I make use of agentic coding in my projects so if you're not a fan of that, you're free to click away from this article.

My goal for the Squeal project was to solve my problem and to not waste too much time on it. I took the approach of agentic coding rather than handwriting trad coding, as I've been doing for the past few weeks. So while developing this project I definitely learned less than from my CodeCrafters project but I had a lot of fun nonetheless as I got to focus on the product design. I already knew that AI was fairly good at writing Rust code, It's very good at writing go and so I figured Rust was a step up. Rust has a lot more usage than Go, There's plenty of rewrite-in-Rust projects on GitHub so AI has plenty of things to study from.

I have a lot of friends who don't like AI in general. Any time I share something cool with them, they shoot back with, "Was this AI generated?" "Oh now that I know that it's AI-generated, it doesn't matter how cool I thought it was. It's now bad". The common criticism being that anyone could have made this app. And that's definitely true but there's a lot of requirements for this app to be made without AI. First you need to know Rust then you need to know Linux and command line intuition And UX design. And this latter half is quite substantial. While building this app I had a lot of back and forth changing how to interact with the app and kept trying to find something that felt better to use. If a Joe Schmo made a prompt based on just looking at/using my project, the resulting app would probably not be the same. Could another seasoned developer make something like this? Yes absolutely. But it probably would have taken them at least a couple of days instead of a couple of hours.

Another big part of how I use AI is that I do still care about the architecture. Even if I will never write a line of code, I will still go in and try to refactor things (with AI). As I know that while the app is still in its early stages it's easier to change and I know that certain things Become a massive maintenance burden later on and so while they're still fresh in my head, I can change them now. AI is not infallible. If you put slop in, you're going to get slop out, So keeping quality up is important.

Something that I still remain a little skeptical about AI coding is AI-generated test suites. It's very easy to generate a bunch of tests and then whenever you change code, to just update all the tests that fail, regardless of if they're relevant or not. And a lot of developers do this too, either because of laziness or just because they don't know better. We looked at a lot of places that just didn't have test coverage or Bad testing policies meant we would just update the tests alongside the code and not really care about the tests. The standard once a metric becomes a metric, it's very easy to game the metric.

But I think it did an okay job this time around as when I went to refactor some of the internals of the database drivers, None of my tests ended up failing meaning that my refactors were atomic. But it's something I still remain somewhat skeptical about as it's very easy to overlook giant test files with a lot of changes.
