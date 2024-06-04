package security

import (
	"bytes"
	"dns/config"
	"encoding/json"
	"io/ioutil"
	"net/http"
)

const APIURL = "https://api.groq.com/openai/v1/chat/completions"

type Message struct {
	Role    string `json:"role"`
	Content string `json:"content"`
}

type Response struct {
	Choices []Choice `json:"choices"`
}

type Choice struct {
	Message Message `json:"message"`
}

var conf = config.GetConfig()
var apiKey = conf.AIKey

// Prompt sends a system and user prompt to the AI and gets a response
func Prompt(systemMessage, userMessage string) (string, error) {
	request := struct {
		Messages []Message `json:"messages"`
		Model    string    `json:"model"`
	}{
		Messages: []Message{
			{Role: "system", Content: systemMessage},
			{Role: "user", Content: userMessage},
		},
		Model: "mixtral-8x7b-32768", // Use the specified model
	}

	jsonData, err := json.Marshal(request)
	if err != nil {
		return "", err
	}

	client := &http.Client{}
	req, err := http.NewRequest("POST", APIURL, bytes.NewBuffer(jsonData))
	if err != nil {
		return "", err
	}

	req.Header.Set("Authorization", "Bearer "+apiKey)
	req.Header.Set("Content-Type", "application/json")

	resp, err := client.Do(req)
	if err != nil {
		return "", err
	}
	defer resp.Body.Close()

	responseBody, err := ioutil.ReadAll(resp.Body)
	if err != nil {
		return "", err
	}

	var response Response
	err = json.Unmarshal(responseBody, &response)
	if err != nil {
		return "", err
	}

	if len(response.Choices) > 0 && response.Choices[0].Message.Role == "assistant" {
		return response.Choices[0].Message.Content, nil
	}

	return "", nil // No replies or not the expected format
}
