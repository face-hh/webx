package lookup

import (
	"encoding/json"
	"net/http"
)

var tlds = []string{"mf", "btw", "fr", "yap", "dev", "scam", "zip", "root", "web", "rizz", "habibi", "sigma", "now", "it", "soy", "lol", "uwu", "ohio"}

func Tlds(w http.ResponseWriter, r *http.Request) {
	js, err := json.Marshal(tlds)
	if err != nil {
		http.Error(w, err.Error(), http.StatusInternalServerError)
		return
	}
	w.Header().Set("Content-Type", "application/json")
	w.Write(js)
}
