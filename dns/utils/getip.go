package utils

import (
	"net"
	"net/http"
)

// GetClientIP attempts to retrieve the client IP from the CF-Connecting-IP header,
// and falls back to the remote socket address if the header is not present.
func GetClientIP(r *http.Request) string {
	cfIP := r.Header.Get("CF-Connecting-IP")
	if cfIP != "" {
		return cfIP
	}

	// Fallback to the IP from the remote address
	remoteIP, _, err := net.SplitHostPort(r.RemoteAddr)
	if err != nil {
		// In case the remote address does not contain a port
		return r.RemoteAddr
	}
	return remoteIP
}
