package domain

import (
	"context"
	"dns/database"
	"encoding/json"
	"net/http"

	"github.com/gorilla/mux"
	"go.mongodb.org/mongo-driver/bson"
)

func Update(w http.ResponseWriter, r *http.Request) {
	vars := mux.Vars(r)
	key := vars["key"]

	var updateData struct {
		IP string `json:"ip"`
	}
	if err := json.NewDecoder(r.Body).Decode(&updateData); err != nil {
		http.Error(w, "Invalid request", http.StatusBadRequest)
		return
	}

	result, err := database.Db.UpdateOne(context.Background(),
		bson.M{"secret_key": key},
		bson.M{"$set": bson.M{"ip": updateData.IP}})

	if err != nil || result.ModifiedCount == 0 {
		http.Error(w, "Failed to update domain or domain not found", http.StatusNotFound)
		return
	}

	w.Header().Set("Content-Type", "application/json")
	json.NewEncoder(w).Encode(map[string]string{"ip": updateData.IP})
}
