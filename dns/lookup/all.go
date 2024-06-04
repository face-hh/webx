package lookup

import (
	"context"
	"dns/database"
	"encoding/json"
	"net/http"

	"go.mongodb.org/mongo-driver/bson"
)

func All(w http.ResponseWriter, r *http.Request) {
	cursor, err := database.Db.Find(context.Background(), bson.M{})
	if err != nil {
		http.Error(w, "Failed to retrieve domains", http.StatusInternalServerError)
		return
	}
	var domains []bson.M
	if err = cursor.All(context.Background(), &domains); err != nil {
		http.Error(w, "Failed to parse domains", http.StatusInternalServerError)
		return
	}

	w.Header().Set("Content-Type", "application/json")
	json.NewEncoder(w).Encode(domains)
}
