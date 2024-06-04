package lookup

import (
	"context"
	"dns/database"
	"encoding/json"
	"net/http"

	"github.com/gorilla/mux"
	"go.mongodb.org/mongo-driver/bson"
)

func Specific(w http.ResponseWriter, r *http.Request) {
	vars := mux.Vars(r)
	name := vars["name"]
	tld := vars["tld"]

	if name == "" || tld == "" {
		http.Error(w, "Invalid domain details", http.StatusBadRequest)
		return
	}

	result := database.Db.FindOne(context.Background(), bson.M{"name": name, "tld": tld})
	var domain bson.M
	if err := result.Decode(&domain); err != nil {
		http.Error(w, "Domain not found", http.StatusNotFound)
		return
	}

	response := map[string]interface{}{
		"name": domain["name"],
		"tld":  domain["tld"],
		"ip":   domain["ip"],
	}

	w.Header().Set("Content-Type", "application/json")
	json.NewEncoder(w).Encode(response)
}
