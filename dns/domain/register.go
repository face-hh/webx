package domain

import (
	"context"
	"dns/config"
	"dns/database"
	"dns/security"
	"dns/utils"
	"encoding/json"
	"log"
	"net"
	"net/http"
	"strings"
	"sync"
	"time"

	"github.com/gorilla/mux"
	"go.mongodb.org/mongo-driver/bson"
	"golang.org/x/time/rate"
)

var (
	ddoslimiter              = rate.NewLimiter(rate.Every(time.Minute), 10)
	limiter                  = rate.NewLimiter(rate.Every(time.Hour), 100)
	tlds                     = []string{"mf", "btw", "fr", "yap", "dev", "scam", "zip", "root", "web", "rizz", "habibi", "sigma", "now", "it", "soy", "lol", "uwu", "ohio"}
	chanceLimitValue float64 = 1.00
	chanceLimitMutex sync.Mutex
	resetTicker      = time.NewTicker(1 * time.Hour)
)

func CounterInit() {
	go func() {
		for range resetTicker.C {
			chanceLimitMutex.Lock()
			chanceLimitValue = 1.00
			chanceLimitMutex.Unlock()
		}
	}()
}

func Register(cloudflare bool) http.HandlerFunc {
	return func(w http.ResponseWriter, r *http.Request) {

		var ip string
		if cloudflare {
			ip = utils.GetClientIP(r)
		} else {
			var err error
			ip, _, err = net.SplitHostPort(r.RemoteAddr)
			if err != nil {
				// Fallback to the complete remote address if it's not in IP:port format
				ip = r.RemoteAddr
			}
		}

		ipDDoSlimiter := security.RetrieveSecondLimiter(ip, 3)
		if !ipDDoSlimiter.Allow() {
			http.Error(w, "Blocked due to spam", http.StatusTooManyRequests)
			return
		}

		var newDomain struct {
			Name string `json:"name"`
			TLD  string `json:"tld"`
			IP   string `json:"ip"`
		}
		if err := json.NewDecoder(r.Body).Decode(&newDomain); err != nil {
			http.Error(w, "Invalid request", http.StatusBadRequest)
			return
		}

		if !validateDomain(newDomain.Name, newDomain.TLD) {
			http.Error(w, "Invalid domain details", http.StatusBadRequest)
			return
		}

		if !ValidateIP(newDomain.IP) {
			http.Error(w, "Invalid IP Field, please set it to a valid IP or URL... if you do not have a site yet you can use 1.1.1.1", http.StatusBadRequest)
			return
		}

		if !ddoslimiter.Allow() {
			http.Error(w, "API Under DDoS, Use an external registrar!", http.StatusTooManyRequests)
			return
		}

		if !limiter.Allow() {
			http.Error(w, "API is being Botted, Try again in an hour", http.StatusTooManyRequests)
			return
		}

		iplimiter := security.RetrieveLimiter(ip, 1)
		if !iplimiter {
			http.Error(w, "Try again in an hour", http.StatusTooManyRequests)
			return
		}

		domain := newDomain.Name + "." + newDomain.TLD
		chanceLimitMutex.Lock()
		if chanceLimitValue <= 0 {
			chanceLimitMutex.Unlock()
			http.Error(w, "API is being Botted, Try again in an hour", http.StatusTooManyRequests)
			return
		}
		chanceLimitValue -= 0.01
		chanceLimitMutex.Unlock()

		if security.CheckMature(domain) {
			log.Printf("[DNS] Blocked %s due to innapropriate content", domain)
			http.Error(w, "Domain contains innapropriate content", 450)
			return
		}

		if security.CheckSpam(security.Store.Domains, domain) >= chanceLimitValue {
			log.Printf("[DNS] Blocked %s due to Spam", domain)
			http.Error(w, "Blocked due to possible spam", http.StatusNotAcceptable)
			return
		}

		secretKey := utils.GenerateSecretKey(24)
		data := bson.M{
			"name":       strings.ToLower(newDomain.Name),
			"tld":        newDomain.TLD,
			"ip":         newDomain.IP,
			"secret_key": secretKey,
		}

		result := database.Db.FindOne(context.Background(), bson.M{"name": newDomain.Name, "tld": newDomain.TLD})
		var existingDomain bson.M
		if err := result.Decode(&existingDomain); err == nil {
			http.Error(w, "Domain already registered", http.StatusConflict) // 409 Conflict
			return
		}

		if _, err := database.Db.InsertOne(context.Background(), data); err != nil {
			http.Error(w, "Failed to create domain", http.StatusInternalServerError)
			return
		}

		response := map[string]interface{}{
			"name":       newDomain.Name,
			"tld":        newDomain.TLD,
			"ip":         newDomain.IP,
			"secret_key": secretKey,
		}

		security.Store.AddDomain(domain, newDomain.IP)
		log.Printf("[DNS] Registered domain: %s with IP: %s, Key: %s", domain, newDomain.IP, secretKey)

		w.Header().Set("Content-Type", "application/json")
		json.NewEncoder(w).Encode(response)
	}
}
func RegisterAPI(keys map[string]config.APIKeyConfig) http.HandlerFunc {
	return func(w http.ResponseWriter, r *http.Request) {
		vars := mux.Vars(r)
		apiKey := vars["key"]
		if apiKey == "" {
			http.Error(w, "API Key is required", http.StatusUnauthorized)
			return
		}

		// Validate API key and retrieve rate limiter
		apiConfig, ok := keys[apiKey]
		if !ok {
			http.Error(w, "Invalid API Key", http.StatusForbidden)
			return
		}

		ddoslimiter := security.RetrieveSecondLimiter(apiKey, 5)
		if !ddoslimiter.Allow() {
			http.Error(w, "Blocked due to Spam", http.StatusTooManyRequests)
			return
		}

		limiter := security.RetrieveLimiter(apiKey, apiConfig.RateLimit)
		if !limiter {
			http.Error(w, "Rate limit exceeded, try again later", http.StatusTooManyRequests)
			return
		}

		var newDomain struct {
			Name string `json:"name"`
			TLD  string `json:"tld"`
			IP   string `json:"ip"`
		}

		if err := json.NewDecoder(r.Body).Decode(&newDomain); err != nil {
			http.Error(w, "Invalid request", http.StatusBadRequest)
			return
		}

		if !validateDomain(newDomain.Name, newDomain.TLD) {
			http.Error(w, "Invalid domain details", http.StatusBadRequest)
			return
		}

		if !ValidateIP(newDomain.IP) {
			http.Error(w, "Invalid IP Field, please set it to a valid IP or URL... if you do not have a site yet you can use 1.1.1.1", http.StatusBadRequest)
			return
		}
		domain := newDomain.Name + "." + newDomain.TLD
		if security.CheckMature(domain) {
			log.Printf("[REGISTRAR-API] Blocked %s created by %s due to innapropriate content", domain, apiConfig.Owner)
			http.Error(w, "Domain contains innapropriate content", 450)
			return
		}

		secretKey := utils.GenerateSecretKey(24)
		data := bson.M{
			"name":       strings.ToLower(newDomain.Name),
			"tld":        newDomain.TLD,
			"ip":         newDomain.IP,
			"secret_key": secretKey,
		}

		result := database.Db.FindOne(context.Background(), bson.M{"name": newDomain.Name, "tld": newDomain.TLD})
		var existingDomain bson.M
		if err := result.Decode(&existingDomain); err == nil {
			http.Error(w, "Domain already registered", http.StatusConflict) // 409 Conflict
			return
		}

		if _, err := database.Db.InsertOne(context.Background(), data); err != nil {
			http.Error(w, "Failed to create domain", http.StatusInternalServerError)
			return
		}

		response := map[string]interface{}{
			"name":       newDomain.Name,
			"tld":        newDomain.TLD,
			"ip":         newDomain.IP,
			"secret_key": secretKey,
		}

		log.Printf("[REGISTRAR-API] %s Has registered domain: %s with IP: %s, Key: %s", apiConfig.Owner, domain, newDomain.IP, secretKey)

		w.Header().Set("Content-Type", "application/json")
		json.NewEncoder(w).Encode(response)
	}
}
