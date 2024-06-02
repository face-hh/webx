const cheerio = require('cheerio');
const MiniSearch = require('minisearch');
const fs = require('fs');

let GLOBAL_DOCS = [];
const needle_docs = [];
const existing_ips = [];
const to_remove = [];

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
        //console.clear()
        console.log(`${i}/${domains.length} (${((i / domains.length) * 100).toFixed(2)}%)`)
//        console.log(`Now at ${domain.name}.${domain.tld} (located at ${domain.ip.startsWith("https://") || domain.ip.startsWith("http://")})`);
        if (!(domain.ip.startsWith("https://") || domain.ip.startsWith("http://")) || existing_ips.includes(domain.ip)) {
            to_remove.push(domain._id);
            continue;
        }

        existing_ips.push(domain.ip);

        let content = await fetchWebsite(domain.ip);

        if(!content) {
            to_remove.push(domain._id);
            continue;
        }

        let res = extractContent(content);

        if (res.descriptions == '') {
            to_remove.push(domain._id);
            continue;
        }
        
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
    fs.writeFileSync("to_remove.json", JSON.stringify(to_remove));

    return miniSearch
}

function letsHTMLplusplus(htmlString) {
    const lines = htmlString.split('\n');
    
    const filteredLines = lines.filter(line => !line.includes('<script'));
    
    const result = filteredLines.join('\n');
    
    return result;
}

/**
 * Extracts the title and descriptions (content of <h1>-<h6>, <p>, and <a> tags) from HTML.
 *
 * @param {string} html - The HTML content to extract from.
 * @returns {{ title: string, descriptions: string[] }} - An object containing the title and descriptions.
 */
function extractContent(html) {
    html = letsHTMLplusplus(html)

    const $ = cheerio.load(html);
    const title = $('title').text();

    fs.writeFileSync("FKGEPAKG", html);

    const descriptions = [];
    $('h1, h2, h3, h4, h5, h6, p, a').each((i, element) => {
        console.log($(element).text())
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