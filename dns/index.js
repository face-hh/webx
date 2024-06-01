require("dotenv").config();

const express = require('express');
const bodyParser = require('body-parser')
const {
    OpenAI
} = require('openai');

const {
    MongoClient
} = require('mongodb');
const {
    generateApiKey
} = require('./utils');
const {
    rateLimit
} = require('express-rate-limit');

const openai = new OpenAI();

const app = express();
const port = 8000;

app.use(bodyParser.urlencoded({
    extended: false
}))
app.use(bodyParser.json())

const TLD = [
    "mf", "btw", "fr", "yap", "dev", "scam", "zip", "root", "web", "rizz", "habibi", "sigma",
    "now", "it", "soy", "lol", "uwu"
];


/**
 * The MongoDB collection for storing domain information
 * @type {import('mongodb').Collection}
 */
let db;

const limiter = rateLimit({
    windowMs: 60 * 60 * 1000,
    standardHeaders: 'draft-7',
    legacyHeaders: false,
    limit: 1,
    skip: (_, res) => res.statusCode != 200,
    keyGenerator: function(req, _) {
        return req.ip;
    }
})

app.set('trust proxy', 1);
app.post("/domain", limiter)

async function connectToMongo() {
    const client = new MongoClient(process.env.MONGOURI);
    await client.connect();
    db = client.db('mydb').collection('domains');
}

async function isOffensive(query) {
    const response = await openai.chat.completions.create({
        model: "gpt-4o",
        messages: [{
                "role": "user",
                "content": [{
                    "type": "text",
                    "text": "I need you to reply with \"yes\" and \"no\" only.\n\nYour task is to tell me if this domain name is offensive (i.e. it's vulgar, racist, dicriminating).\n\nDomain: " + query + "\n"
                }]
            },
        ],
        temperature: 1,
        max_tokens: 10,
        top_p: 1,
        frequency_penalty: 0,
        presence_penalty: 0,
    });

    //use trim and tolowercase to ensure a Yes is also a yes
    return response?.choices?.[0]?.message.trim().toLowerCase() == "yes"
}

connectToMongo().catch(console.error);

app.get('/', (_, res) => {
    res.send('Hello, world! The available endpoints are:\nGET /domains,\nGET /domain/:name/:tld,\nPOST /domain,\nPUT /domain/:key,\nDELETE /domain/:key,\nGET /tlds.\nRatelimits provided in headers.\n');
});

app.post('/domain', async (req, res) => {
    const secretKey = generateApiKey(24);

    const newDomain = req.body;

    if (!newDomain.tld || !newDomain.ip || !newDomain.name) {
        return res.status(400).send();
    }

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

        const is_offensive = await isOffensive(newDomain.name + '.' + newDomain.tld);

        if (is_offensive) {
            return res.status(400).send("The given domain is offensive.")
        }
    
        await db.insertOne(data);
        delete data._id;

        res.status(200).json(data);
    } catch (err) {
        res.status(409).send();
    }
});

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

app.get('/domain/:name/:domain', async (req, res) => {
    const {
        name,
        tld
    } = req.params;
    if (!name || !tld) {
        return res.status(400).send();
    }

    try {
        const result = await db.getDomainByDomain(name, tld);
        if (result) {
            res.json({
                id: null,
                domain: result.domain,
                name: result.name,
                ip: result.ip
            });
        } else {
            res.status(404).send();
        }
    } catch (err) {
        res.status(404).send();
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
