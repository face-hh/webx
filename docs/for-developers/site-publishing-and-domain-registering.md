# Site publishing & Domain registering

You already know most of the stuff you need to publish a website:

- :heavy_check_mark: How to write WebX compatible code.
- :heavy_check_mark: [The file structure](getting-started.md#file-structure).

However, there are still some things we haven't done (yet).

- :x: Hosting our files.
- :x: Getting a domain on the WebX DNS.

Hey, we're already half done.

## Hosting our files.

You have basically two options:

### Self hosting

Set-up your own server ***the way you like***, and get a static IP address for it. Make sure it's serving your `index.html` in the root of the IP, and that the other files (CSS 3.5, Luau) are also in the root. Images don't need to be in the root, they can also be retrieved from HTTP routes. Note the IP, as later we'll use it to serve our website.

### GitHub

GitHub makes it way easier for the WebX network to function. Create your own repository with whatever name you'd like it to have and upload your files. Copy the full HTTPS URL of the repo (`https://github.com/{username}/{repo}`) and keep it, as we'll use it later. That would be it!

Now that you have your IP / GitHub URL, you just need to register a domain, provide the URL / IP to Bussin's API systems, and your website should be up and running.

## Getting my domain

Domains can be registered via our API (see [next page](api-reference.md#post-domain)). There are different providers, like these ones:

{% hint style="danger" %}
### Outdated section
This section contains content from contributors and 3rd parties. It has been marked as outdated. This alert will stay until the content below gets updated.
{% endhint %}

| Provider | Where to find it | Administrator | Recommended | Notes |
| -------- | ---------------- | ------------- | ----------- | ----- |
| Bussin Registrar | Go to `buss://register.it` on your WebX browser. | :heavy_check_mark: **Official WebX service** | YES | Open source |
| WebX Registry | Go to [`https://kitty.yachts/`](https://kitty.yachts/) on a WWW browser. | tlochsta | ??? | *No notes.* |
| tr.operium.org | Go to [`http://tr.operium.org:1139/`](http://tr.operium.org:1139/) on a WWW browser. **Not HTTP secure**. | miaunescu0 | ??? | *No notes.* |

###### Something is wrong with this table? As 3rd party browsers get updated, this table can get outdated over time. [Make a PR to help keep it up to date.](https://github.com/face-hh/webx/blob/main/docs) Thanks in advance!

<!-- FOR DEVELOPERS: this is missing information (idk what info (if i knew i'd put it) but im sure theres something missing)-->
Of course, we recommend using the official Bussin Registrar, which looks kinda like this:
<!-- (***note for contributors: please update the screenshot***) -->

![Screenshot](../png4.jpeg)
<!-- TO ANY DEV REVIEWING THE PR - SORRY MAN MY SCREEN IS JUST TOO SMALL FOR THIS XD - someome go take the actual napture and make a better screenshot, thanks-->

What we care about is the Publish form. Every other registrer should use the same format, asking for an IP, TLD, and website name.

### Domain parameters

Your domain needs three simple things.

{% hint style="info" %}
### Website name
The name of your website for the URI. It will be here:

buss://**{here}**.tld

Note that there is a max of 24 characters for your domain.
{% endhint %}

{% hint style="info" %}
### TLD
The Top-Level Domain you'd like your website to have. It will be here:

buss://name.**{here}**

There is a limited amount of options. You can choose from the following:
| TLD | Meaning |
| --- | ------- |
| .mf | English shorthand for "motherf*cker". Could be used for a personal website. |
| .btw | English shorthand for "by the way". |
| .fr | English shorthand for "for real". |
| .yap | Extension for Yappacino files. Yappacino is a JavaScript superset, developed by Bussin (the providers of WebX). |
| .dev | Abbreviation for "developer". |
| .scam | English word "scam". Could be used for joke sites, ***not real scams***. See [WebX Community Rules](#community-rules) |
| .zip | File format that can compress a lot of info in a small package. Could be used for a site with lots of info and resources. |
| .root | The origin or base of something; in computing, the top-level directory or administrative account. |
| .web | English shorthand for "website". Could be used for a general purpose website. |
| .rizz | Slang word defined as "style, charm, or attractiveness; ability to attract a romantic partner". Could be used for a personal website. |
| .habibi | Arabic for loved / husband. |
| .sigma | Slang term used for a popular, successful, but highly independent and self-reliant man. Could be used for a personal website, or any website you believe to be great :wink: |
| .now | English word "now". |
| .it | English word "it". |
| .soy | Spanish for "I am". Could be used for a personal website. |
| .lol | Slang and english shorthand for "laughing out loud". Could be used for joke, funny websites. |
| .uwu | Emoticon representating a cute face. Mainly used by weebs, otakus, and that kind of people. Could be used for a personal website if you're one of those. |
| .ohio | One of the 50 states of the United States of America. Used in cringe TikTok memes to represent extremely weird and out-of-context stuff. Could be used for a maybe-too-much original website :wink: |

In a nutshell, choose from "mf", "btw", "fr", "yap", "dev", "scam", "zip", "root", "web", "rizz", "habibi", "sigma", "now", "it", "soy", "lol", "uwu", "ohio" 
{% endhint %}

{% hint style="info" %}
### IP
The IP / GitHub URL I told you about before. Give it to the system so it can access it and serve your website.
{% endhint %}

### Community Rules

WebX is supervised by the team at Bussin, who administrates the WebX API. Any website that doesn't follow these rules will be removed.

1. If your website contains Not Safe For Work material of any kind, it will be removed.
2. If your website contains frequent racial slurs, references made in bad faith to tragic events, racism towards other races, or anything of that kind, it will be removed.
3. If your website is dedicated to the publication of private information, it will be removed.
4. If your website is actively engaged in leaking information about incoming traffic (i.e., posting the IPs of users), it will be removed.
5. If your website displays content that violates law or regulations, including but not limited to, piracy, hacking, or illegal activities such as drug usage, will result in a removal.
6. If your website contains or distributes malware, viruses, or any other harmful software, it will be removed.
7. If your website is dedicated to harassment, bullying, or targeted attacks against individuals or groups, it will be removed.
8. If your website infringes upon intellectual property rights of others, it will be removed.
9. If your website is involved in fradulent activities, scams, or deceptive practices, it will be removed.
10. If your website contains content that encourages harmful behavior, including self-harm, suicide, substance abuse, or dangerous challanges, it will be removed.
11. If your website's domain contains words or content that are considered against any of the rules listed before, it will be removed.

{% hint style="danger" %}
## BUSSIN WEBX COMMUNITY RULES AGREEMENT
By publishing content to this platform ("Bussin Napture"/"Bussin WebX"), ***you agree to comply with all rules and regulations set forth by the administrators***. **The administrators reserve the right to interpret and enforce these rules at their discretion.**
{% endhint %}

{% hint style="warning" %}
### Reporting websites that don't follow the rules
To report websites that are not following the listed rules, please use [the Discord server](https://discord.gg/cNwWvdWj42) or contact `FaceDev` on [Twitter](https://twitter.com/facedevstuff) or Discord.
{% endhint %}

If you submit a website that is not against the rules, you should start seeing it on the Dingle search results and in your `buss://` URL. Plus, your domain provider should give you a "Secret key" (or "Secret code", "API Key", or however they call it, but it is always the same key).

{% hint style="danger" %}
VERY IMPORTANT THING ABOUT YOUR SECRET KEY
### DO NOT LOOSE IT AND DO NOT SHARE IT WITH ANYONE.
Do not loose it because it is required if you want to edit your IP or shut down your domain, and do not share it with anyone because it can compromise your website.
{% endhint %}

{% hint style="info" %}
What to do if it doesn't work
There are a few reasons it could be wrong:
### 1. Code not matching the WebX standard.
If your site doesn't work, check if your code is correct. Maybe it's that.
### 2. The rate limit.
There is a global cooldown on how many domains can be registered (*there are reasons we did that, ok?*). Current rate is of ***100 domains per hour***, globally.
### 2. The API being down.
WebX is a recently setup project and it's uptime is not of a 100%. It could be down for repairing.

For these reasons we recommend joining [the Discord server](https://discord.gg/cNwWvdWj42), where the development team keeps everyone updated on issues with the network.
{% endhint %}

Do you see your website on the Dingle search results? If no, check "*What to do if it doesn't work*". If it does, congratulations! You already know how to code for WebX and you got a website up and running! Well done, man. :saluting_face:
