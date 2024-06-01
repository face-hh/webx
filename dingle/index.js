require('dotenv').config();
const express = require('express');
const { MongoClient } = require('mongodb');
const utils = require('./utils');

const app = express();
const port = process.env.PORT || 3000;

let db;
let miniSearch;

async function connectToMongo() {
    const client = new MongoClient(process.env.MONGOURI);
    await client.connect();

    db = client.db('mydb').collection('domains');

    miniSearch = await utils.start();
    console.log("Loaded up minisearch!");

    miniSearch = await utils.get_records(db);

    setInterval(async () => {
        miniSearch = await utils.get_records(db);
    }, 43_200_000); // 12h
}

connectToMongo().catch(console.error);

function checkGlobalDocs(_, res, next) {
    if (typeof miniSearch === 'undefined') {
        return res.status(500).json({ error: 'Wait for documents to load' });
    }
    next();
}

app.get('/search', checkGlobalDocs, (req, res) => {
    const query = req.query.q;
    if (!query) {
        return res.status(400).json({ error: 'Query parameter "q" is required' });
    }

    const results = miniSearch.search(query);

    for(const result of results) {
        delete result["text"]
    }

    res.json(results);
});

app.get('/random', checkGlobalDocs, (req, res) => {
    let random = utils.get_random();

    delete random["text"]

    res.json(random)
})
app.listen(port, () => {
    console.log(`Server is running on port ${port}`);
});
