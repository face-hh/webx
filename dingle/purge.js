require('dotenv').config()

const { MongoClient, ObjectId } = require('mongodb');
const fs = require('fs');

const dbName = 'mydb';
const collectionName = 'domains';

async function removeDomains() {
    const data = fs.readFileSync('to_remove.json', 'utf8');
    const toRemove = JSON.parse(data);

    const objectIds = toRemove.map(id => new ObjectId(id));

    const client = new MongoClient(process.env.MONGOURI, { useNewUrlParser: true, useUnifiedTopology: true });

    try {
        await client.connect();

        const database = client.db(dbName);
        const collection = database.collection(collectionName);

        const result = await collection.deleteMany({ _id: { $in: objectIds } });

        console.log(`Deleted ${result.deletedCount} documents`);
    } catch (err) {
        console.error(err);
    } finally {
        await client.close();
    }
}

removeDomains();
