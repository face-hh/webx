require("dotenv").config();

const express = require('express');
const bodyParser = require('body-parser')
const path = require('path');

const { MongoClient } = require('mongodb');
const { generateSecretKey } = require('./utils');
const Captcha = require("captcha-generator-alphanumeric").default;
const fs = require('fs');

let captchas = {};

const app = express();
const port = 8000;
const apiKeysEnabled = false

app.use(bodyParser.urlencoded({
    extended: false
}))
app.use(bodyParser.json())

const TLD = [
    "mf", "btw", "fr", "yap", "dev", "scam", "zip", "root", "web", "rizz", "habibi", "sigma",
    "now", "it", "soy", "lol", "uwu", "ohio"
];


/**
 * The MongoDB collection for storing domain information
 * @type {import('mongodb').Collection}
 */
let db;

const FastRateLimit = require("fast-ratelimit").FastRateLimit;

const limiter = new FastRateLimit({
  threshold : 1, // available tokens over timespan
  ttl       : 60 * 60  // time-to-live value of token bucket (in seconds)
});

const apilimiter = new FastRateLimit({
  threshold : 100, // available tokens over timespan
  ttl       : 60 * 60  // time-to-live value of token bucket (in seconds)
});

app.set('trust proxy', 1);

async function connectToMongo() {
    const client = new MongoClient(process.env.MONGOURI);
    await client.connect();
    db = client.db('mydb').collection('domains');
}

connectToMongo().catch(console.error);

app.get('/', (_, res) => {
    res.send('Hello, world! The available endpoints are:\nGET /domains,\nGET /domain/:name/:tld,\nPOST /domain,\nPUT /domain/:key,\nDELETE /domain/:key,\nGET /tlds.\nRatelimits provided in headers.\n');
});

app.post('/domain', async (req, res) => {
    if (!limiter.hasTokenSync(req.ip)) {
        return res.status(429).send("Try again in an hour")
    }

    const secretKey = generateSecretKey(24);

    const newDomain = req.body;

    if (!newDomain.tld || !newDomain.ip || !newDomain.name) {
        return res.status(400).send();
    }

    return do_the_register_shit(newDomain, res, secretKey, req) 
});

app.post('/domainapi/:apiKey', async (req, res) => {
    if (!apilimiter.hasTokenSync(req.params.apiKey)) {
        return res.status(429).send("The hourly limit for your API key has been reached")
    }

    if (!apiKeysEnabled) {
        return res.status(403).send("API Keys are not enabled on this server")
    }

    const secretKey = generateSecretKey(24);

    const newDomain = req.body;

    if (!newDomain.tld || !newDomain.ip || !newDomain.name) {
        return res.status(400).send();
    }

    return do_the_register_shit(newDomain, res, secretKey, req) 
});

async function do_the_register_shit(newDomain, res, secretKey, req){
    if (
        !newDomain.name.match(/^[a-zA-Z\-]+$/) ||
        !TLD.includes(newDomain.tld) ||
        newDomain.name.length > 24
    ) {
        return res.status(400).send("Invalid name, non-existant TLD, or name too long (24 chars).");
    }

    newDomain.name = newDomain.name.toLowerCase();

    const data = {
        tld: newDomain.tld,
        ip: newDomain.ip,
        name: newDomain.name,
        secret_key: secretKey
    };

    try {
        const existingDomain = await db.findOne({
            name: newDomain.name,
            tld: newDomain.tld
        });

        if (existingDomain) {
            return res.status(409).send();
        }

        const offensiveWords = ["nigg", "sex", "porn"];

        if (offensiveWords.some(word => newDomain.name.includes(word))) {
            return res.status(400).send("The given domain is offensive.");
        }

        await db.insertOne(data);
        delete data._id;

        limiter.consumeSync(req.ip)
        res.status(200).json(data);
    } catch (err) {
        res.status(409).send();
    }
}

app.get('/domain/:name/:tld', async (req, res) => {
    const {
        name,
        tld
    } = req.params;
    if (!name || !tld) {
        return res.status(400).send();
    }

    try {
        const result = await db.findOne({
            name,
            tld
        });
        if (result) {
            res.json({
                tld: result.tld,
                name: result.name,
                ip: result.ip
            });
        } else {
            res.status(404).send();
        }
    } catch (err) {
        res.status(500).send();
    }
});

app.put('/domain/:key', async (req, res) => {
    const key = req.params.key;
    if (!key) {
        return res.status(400).send();
    }

    const {
        ip
    } = req.body;
    if (!ip) {
        return res.status(400).send();
    }

    const data = {
        $set: {
            ip
        }
    };

    try {
        const result = await db.updateOne({
            secret_key: key
        }, data);
        if (result.matchedCount === 1) {
            res.json({
                ip
            });
        } else {
            res.status(404).send();
        }
    } catch (err) {
        res.status(500).send();
    }
});


app.delete('/domain/:id', async (req, res) => {
    const id = req.params.id;
    if (!id) {
        return res.status(400).send();
    }

    const result = await db.deleteOne({
        secret_key: id
    });
    if (result.deletedCount === 1) {
        res.status(200).send();
    } else {
        res.status(404).send();
    }
})

app.get('/domains', async (_, res) => {
    try {
        const domains = await db.find({}).toArray();

        const convertedDomains = domains.map(domain => {
            return {
                tld: domain.tld,
                name: domain.name,
                ip: domain.ip
            };
        });

        res.json(convertedDomains);
    } catch (err) {
        res.status(500).send();
    }
});

app.get('/tlds', (_, res) => {
    res.json(TLD);
});


app.listen(port, () => {
    console.log(`Server running at http://localhost:${port}`);
});
