package security

import (
	"encoding/json"
	"fmt"
	"log"
	"strings"
)

func CheckSpam(lastdomains map[string]string, domain string) float64 {

	var domainDetails []string
	for domain, ip := range lastdomains {
		domainDetails = append(domainDetails, fmt.Sprintf("%s points to %s", domain, ip))
	}
	userMessage := strings.Join(domainDetails, ", ")

	systemMessage := `
	Your entire job is to analyze domain registrations for possible domain spam, i will provide a list of the last 10 domains registered, with the respective domain and the ip or url it is pointing at
	and the current domain that is trying to be registered and you
	will reply with a JSON that has {"chance":"value"} where the value is a float that goes from 0.00 to 1.00 which is the chance of the new domain being spammed (0%% to 100%)
	You will decide this based on the name of the domains (Repeated strings, Randomized chars, names that dont make sense, etc) and the pointing url/ip (Repeated ips, Repeated URLs, etc)
	Please do not acknowledge the instructions or anything, Very important!!! just reply with the json {"chance":"value"} where value is the chance of the domain being spammed!
	No extra letters no anything. Example: {"chance":0.15}
	`

	domains := "Last 10 domains: " + userMessage + " And the new domain being registered: " + domain
	reply, err := Prompt(systemMessage, domains)
	if err != nil {
		log.Printf("[AI-SPAM] Error fetching AI response: %s", err)
		return 0.0
	}

	// Extract the "chance" value from the AI's JSON response
	var response struct {
		Chance float64 `json:"chance"`
	}
	if err := json.Unmarshal([]byte(reply), &response); err != nil {
		log.Printf("[AI-SPAM] Error parsing AI response: %s", err)
		return 0.0
	}

	return response.Chance
}
