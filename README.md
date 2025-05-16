[linkding](https://github.com/sissbruecker/linkding) has built-in RSS feed support, allowing RSS feeds to be constructed from any linkding search query.

However, as of v1.39.1, linkding does not have the ability to exclude certain tags from searches, which limits the capabilities of search, and makes it difficult to construct certain RSS feeds.

This is a simple webserver that acts as an interface between linkding and an RSS feed aggregator, allowing searching with excluded tags.

It's intended to be run on the same server that is running linkding / the RSS feed aggregator, where the aggregator makes request to this webserver instead of linkding.
