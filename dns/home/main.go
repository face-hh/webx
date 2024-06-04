package home

import "net/http"

func Handler(w http.ResponseWriter, r *http.Request) {
	w.Write([]byte(`
Hello, world! The available endpoints are:

 ○ GET /domains - Displays all domains that are registered and their IPs

 ○ GET /domain/{name}/{tld} - Displays information about a specific domain

 ○ POST /domain - Registers a domain

 ○ POST /domainapi/{apikey} -  Registers a domain using a third party registrar key (Bypasses ratelimit & Spam check)

 ○ PUT /domain/{secretkey} - Updates domain information

 ○ DELETE /domain/{secretkey} - Deletes a domain

 ○ GET /tlds - Displays avaliable TLDs

	


JSON For registering domain using /domain or /domainapi/{apikey}:
{
    "tld": {tld},
    "ip": {ip},
    "name": {name}
}
	
`))
}
