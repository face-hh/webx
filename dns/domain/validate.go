package domain

import (
	"net"
	"net/url"
	"regexp"
)

func contains(slice []string, val string) bool {
	for _, item := range slice {
		if item == val {
			return true
		}
	}
	return false
}

func validateDomain(name, tld string) bool {
	if len(name) > 24 {
		return false
	}
	nameRegex := regexp.MustCompile(`^[a-zA-Z\-]+$`)
	if !nameRegex.MatchString(name) {
		return false
	}

	return contains(tlds, tld)
}

func ValidateIP(input string) bool {

	if len(input) > 64 {
		return false
	}

	if ip := net.ParseIP(input); ip != nil {
		return true
	}

	// Check for URL validity
	if u, err := url.ParseRequestURI(input); err == nil {
		if u.Scheme != "" && u.Host != "" {
			return true
		}
	}

	return false
}
