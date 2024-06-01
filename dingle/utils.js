const cheerio = require('cheerio');
const MiniSearch = require('minisearch');
const fs = require('fs');

let GLOBAL_DOCS = [];
const needle_docs = [];
const existing_ips = [];

const fields = ['title', 'text', 'description'];
const storeFields = ['description', 'url', 'title']
function get_random() {
    return GLOBAL_DOCS[Math.floor(Math.random() * GLOBAL_DOCS.length)]
}

async function start(){
    if (!fs.existsSync("save.json")) return;

    let miniSearch = new MiniSearch({
        fields,
        storeFields
    });

    GLOBAL_DOCS = JSON.parse(fs.readFileSync("save.json"));

    miniSearch.addAll(GLOBAL_DOCS);

    return miniSearch
}

/**
 * @typedef {import('mongodb').Collection} Collection
 */
/**
 * Retrieves records from a MongoDB collection.
 *
 * @param {Collection} db - The MongoDB collection instance.
 */
async function get_records(db) {
    let miniSearch = new MiniSearch({
        fields,
        storeFields
    });

    console.time("Finding documents")
    let domains = await db.find({}).toArray();
    console.timeEnd("Finding documents")
    
    let index = 0;
    console.time("Scraping")

    for(let i = 0; i < domains.length; i++) {
        let domain = domains[i];
        process.stdout.write(`${i}/${domains.length} (${((i / domains.length) * 100).toFixed(2)}%)`)
        process.stdout.clearLine();
        process.stdout.cursorTo(0);
//        console.log(`Now at ${domain.name}.${domain.tld} (located at ${domain.ip.startsWith("https://") || domain.ip.startsWith("http://")})`);
        if (!(domain.ip.startsWith("https://") || domain.ip.startsWith("http://")) || existing_ips.includes(domain.ip)) {
            continue;
        }

        existing_ips.push(domain.ip);

        let content = await fetchWebsite(domain.ip);

        if(!content) continue;

        let res = extractContent(content);

        if (res == '') continue;
        
        needle_docs.push(
            {
                id: index,
                title: res.title,
                text: res.descriptions,
                description: res.meta,
                url: domain.name + '.' + domain.tld
            },
        )

        index++;
    }
    console.timeEnd("Scraping")

    GLOBAL_DOCS = needle_docs;
    console.log(`Refresh finished at ${Date.now()} with a total of ${GLOBAL_DOCS.length} docs.`)
    miniSearch.addAll(GLOBAL_DOCS);

    fs.writeFileSync("save.json", JSON.stringify(GLOBAL_DOCS));

    return miniSearch
}

/**
 * Extracts the title and descriptions (content of <h1>-<h6>, <p>, and <a> tags) from HTML.
 *
 * @param {string} html - The HTML content to extract from.
 * @returns {{ title: string, descriptions: string[] }} - An object containing the title and descriptions.
 */
function extractContent(html) {
    const $ = cheerio.load(html);
    const title = $('title').text();

    const descriptions = [];
    $('h1, h2, h3, h4, h5, h6, p, a').each((i, element) => {
        descriptions.push(filter($(element).text()));
    });
    const meta = $('meta[name="description"]').attr('content');

    return { title, descriptions: descriptions.join(""), meta };
}

function filter(input) {
    const filteredString = input.split('').filter(char => /[a-zA-Z0-9 ]/.test(char)).join('');
    
    const trimmedString = filteredString.replace(/\s+/g, ' ');
    
    return trimmedString.trim();
}

async function fetchWebsite(url) {
    if (url.startsWith("https://github.com/")) {
        return await fetchFromGitHub(url)
    } else {
        try {
            const response = await fetch(url, {
                signal: AbortSignal.timeout(2000)
            });
            if (!response.ok) {
                return
            }
            const html = await response.text();
            return html;
        } catch (error) {
            return
    }
}
}

async function fetchFromGitHub(url) {
    const parts = url.split('/');
    const owner = parts[3] || '';
    const repo = parts[4] || '';
    
    const fullUrl = `https://raw.githubusercontent.com/${owner}/${repo}/main/index.html`;

    try {
        const response = await fetch(fullUrl, {
            signal: AbortSignal.timeout(2000)
        });
        return response.text();
    } catch (error) {
        console.error(`Error fetching URL ("${fullUrl}"): ${error.message}`);
        return '';
    }
}

module.exports = { start, get_records, get_random }