# Publishing

You already know most of the stuff you need to publish a website:

* :heavy\_check\_mark: How to write Web X compatible code.
* :heavy\_check\_mark: [The file structure](getting-started.md#file-structure).

However, there are still some things we haven't done yet.

* :x: Hosting our files.
* :x: Getting a domain on the Web X DNS.

## Hosting our files

You have basically two options:

### Self hosting

Set-up your own server _**the way you like**_, and get a static IP address for it. Make sure it's serving your `index.html` in the root of the IP, and that the other files (CSS 3.5, Luau) are also in the root. Images don't need to be in the root, they can also be retrieved from HTTP routes. Note the IP, as later we'll use it to serve our website.

### GitHub

GitHub makes it way easier for the Web X network to function. Create your own repository with whatever name you'd like it to have and upload your files. Copy the full HTTPS URL of the repo (`https://github.com/{username}/{repo}`) and keep it, as we'll use it later. That would be it!

Now that you have your IP / GitHub URL, you just need to register a domain, provide the URL / IP to Bussin's API systems, and your website should be up and running.

## Getting my domain

Domains can be registered via our API (see [next page](api-reference.md#post-domain)). There are different providers, like these ones:

{% hint style="danger" %}
#### Outdated section

This section contains content from contributors and 3rd parties. It has been marked as outdated. This alert will stay until the content below gets updated.
{% endhint %}

<table><thead><tr><th>Provider</th><th width="243">Where to find it</th><th width="149">Administrator</th><th width="156">Recommended</th><th>Notes</th></tr></thead><tbody><tr><td>Bussin Registrar</td><td>Go to <code>buss://register.it</code> on your Web X browser.</td><td><span data-gb-custom-inline data-tag="emoji" data-code="2714">✔️</span> <strong>Official Web X service</strong></td><td>YES</td><td>Open source</td></tr><tr><td>WebX Registry</td><td>Go to <a href="https://kitty.yachts/"><code>https://kitty.yachts/</code></a> on a WWW browser.</td><td>tlochsta</td><td>???</td><td><em>No notes.</em></td></tr></tbody></table>

Of course, we recommend using the official Bussin Registrar, which looks kinda like this:

![Screenshot](../png4.png)

What we care about is the Publish form. Every other registrer should use the same format, asking for an IP, TLD, and website name.

### Domain parameters

Your domain needs three simple things.

{% hint style="info" %}
#### Website name

The name of your website for the URL. It will be here:

buss://**{here}**.tld

Note that there is a max of 24 characters for your domain, and it can't contain offensive terms.
{% endhint %}

{% hint style="info" %}
#### TLD

The Top-Level Domain you'd like your website to have. It will be here:

buss://name.**{here}**

There is a limited amount of options. You can choose from the following:
{% endhint %}

| TLD     | Meaning                                                                                                                                                         |
| ------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| .mf     | English shorthand for "motherf\*cker". Could be used for a personal website.                                                                                    |
| .btw    | English shorthand for "by the way".                                                                                                                             |
| .fr     | English shorthand for "for real".                                                                                                                               |
| .yap    | To talk nonsense for the sake of talking.                                                                                                                       |
| .dev    | Abbreviation for "developer".                                                                                                                                   |
| .scam   | English word "scam". Could be used for joke sites, _**not real scams**_. See [Web X Community Rules](site-publishing-and-domain-registering.md#community-rules) |
| .zip    | The file extension for compressed files.                                                                                                                        |
| .root   | The superuser account in Unix and Linux.                                                                                                                        |
| .web    | English shorthand for "website".                                                                                                                                |
| .rizz   | Slang word defined as "style, charm, or attractiveness; ability to attract a romantic partner".                                                                 |
| .habibi | Arabic for loved / husband.                                                                                                                                     |
| .sigma  | Slang term used for a popular, successful, but highly independent and self-reliant man. Usually used ironically                                                 |
| .now    | English word "now".                                                                                                                                             |
| .it     | English word "it".                                                                                                                                              |
| .soy    | An effeminate or unmasculine man, popular in "wojak" memes.                                                                                                     |
| .lol    | Slang and english shorthand for "laughing out loud". Could be used for joke, funny websites.                                                                    |
| .uwu    | Emoticon representating a cute face.                                                                                                                            |
| .ohio   | One of the 50 states of the United States of America. Hell yeah.                                                                                                |

{% hint style="info" %}
In a nutshell, choose from "mf", "btw", "fr", "yap", "dev", "scam", "zip", "root", "web", "rizz", "habibi", "sigma", "now", "it", "soy", "lol", "uwu", "ohio"
{% endhint %}

{% hint style="info" %}
#### IP

The IP / GitHub URL I told you about before. Give it to the system so it can access it and serve your website.
{% endhint %}

### Community Rules

Web X is supervised by the team at Bussin, who administrates the Web X API. Any website that doesn't follow these rules will be removed.

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
### BUSSIN WEB X COMMUNITY RULES AGREEMENT

By publishing content to this platform ("Bussin Napture"/"Bussin WebX"), _**you agree to comply with all rules and regulations set forth by the administrators**_. **The administrators reserve the right to interpret and enforce these rules at their discretion.**
{% endhint %}

{% hint style="warning" %}
#### Reporting websites that don't follow the rules

To report websites that are not following the listed rules, please use [the Discord server](https://discord.gg/cNwWvdWj42) or contact `FaceDev` on [Twitter](https://twitter.com/facedevstuff) or Discord.
{% endhint %}

If you submit a website that is not against the rules, you should start seeing it on the Dingle search  results (after \~12 hours at max) and in your `buss://` URL. Plus, your domain provider should give you a "Secret key".

{% hint style="danger" %}
VERY IMPORTANT THING ABOUT YOUR SECRET KEY

#### DO NOT LOSE IT AND DO NOT SHARE IT WITH ANYONE.

Do not lose it because it is required if you want to edit your IP or shut down your domain, and do not share it with anyone because it can compromise your website.
{% endhint %}

{% hint style="info" %}
It doesn't work!

#### 1. Code not matching the Web X standard.

If your site doesn't work, check if your code is correct. Maybe it's that.

#### 2. The rate limit.

There is a global cooldown on how many domains can be registered. Current rate is of _**100 domains per hour**_, globally.

#### 3. The API being down.

Web X is a recently setup project and it's uptime is not of a 100%. It could be down for repairing.

For these reasons we recommend joining [the Discord server](https://discord.gg/cNwWvdWj42), where the development team keeps everyone updated on issues with the network.
{% endhint %}

Congratulations! You already know how to code for Web X and you got a website up and running! Well done, captain. :saluting\_face:
