package security

import (
	"sync"
	"time"

	"golang.org/x/time/rate"
)

var ipCounts = make(map[string]int)
var mtx2 sync.Mutex
var resetInterval = time.Hour
var Limiters = make(map[string]*rate.Limiter)
var mtx sync.Mutex

func Init() {
	// Initialize a periodic cleanup to reset the counters every hour
	go func() {
		for range time.Tick(resetInterval) {
			resetCounts()
		}
	}()
}

func resetCounts() {
	mtx2.Lock()
	defer mtx2.Unlock()
	ipCounts = make(map[string]int) // Reset all IP counters
}

func RetrieveLimiter(name string, limit int) bool {
	mtx2.Lock()
	defer mtx2.Unlock()

	count, exists := ipCounts[name]
	if !exists || count < limit {
		ipCounts[name] = count + 1
		return true
	}
	return false
}

func RetrieveSecondLimiter(name string, rateLimit int) *rate.Limiter {
	mtx.Lock()
	defer mtx.Unlock()

	limiter, exists := Limiters[name]
	if !exists {
		limiter = rate.NewLimiter(rate.Every(time.Second), rateLimit) // Max n requests per minute
		Limiters[name] = limiter
	}
	return limiter
}
