---
outline: deep
prev:
  text: 'Site publishing & Domain registering'
  link: 'dev-publish'
---

# API Reference

How to work around with WebX's API, hosted at https://api.buss.lol.

This is the URI of the WebX API, which holds all the DNS of the network. You got different endpoints to do your stuff.
https://api.buss.lol/.

:::warning
APIs have rate limits. They are provided in the headers.
:::

## `GET` /domains
```
- https://api.buss.lol/domains
- Returns a JSON (application/json; charset=utf-8)
```

This will return a JSON with a lot of entries (all the working domains). Each entry looks like this (*without the comments*):
```jsonc
[
    // this is an entry
    {
        "tld":"it",
        "name":"register",
        "ip":"https://github.com/face-hh/webx-registrar"
    },
    // another one
    {
        "tld":"it",
        "name":"dingle",
        "ip":"https://github.com/face-hh/dingle-frontend"
    }
]
```

Being `tld`, `name`, and `ip` the properties we saw in the [last page](dev-publish.md#domain-parameters).
Useful to get a list of all the domains.

## `GET` /domain/{name}/{tld}
*Being `{name}` the `name` property of the domain and `{tld}` it's TLD.*
```
- https://api.buss.lol/domain/name/tld
- Returns a JSON (application/json; charset=utf-8)
```

This will return a JSON with a single of entries (the specified domain). It looks like this (*without the comment*):
```jsonc
// only entry
{
    "tld":"it",
    "name":"register",
    "ip":"https://github.com/face-hh/webx-registrar"
}
```

Being `tld`, `name`, and `ip` the properties we saw in the [last page](dev-publish.md#domain-parameters).
