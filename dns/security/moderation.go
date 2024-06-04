package security

import (
	"log"
	"strings"
)

func CheckMature(domain string) bool {

	systemMessage := `
	Your entire job is to analyze domain registrations for possible inappropriate domains, these could include swear words,
	insults, racism, clasism, 18+ content, etc. I will provide a domain and if you find this inappropriate please just respond with json array with
	'response':'yes' or 'response':'no', nothing else. i repeat, is very important you only reply with a json array that has the 'response:' yes or no, dont overexplain...
	very important: do not acknowledge this or give me any confirmation, just reply the json array with 'response':'yes' or 'response':'no'
	`

	reply, err := Prompt(systemMessage, domain)
	if err != nil {
		log.Printf("[AI-MODERATION] Error fetching AI response for %s: %s", domain, err)
		return false
	}

	return strings.Contains(strings.ToLower(reply), "yes")
}
