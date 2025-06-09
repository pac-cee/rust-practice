package main

import (
	"context"
	"encoding/json"
	"fmt"
	"log"
	"net/http"
	"os"
	"strconv"

	"github.com/gin-gonic/gin"
	"github.com/go-redis/redis/v8"
	"github.com/joho/godotenv"
)

type Order struct {
	ID       int     `json:"id"`
	UserID   string  `json:"user_id"`
	Type     string  `json:"type"` // "buy" or "sell"
	Price    float64 `json:"price"`
	Amount   float64 `json:"amount"`
	Remaining float64 `json:"remaining"`
	Status   string  `json:"status"`
}

var (
	orderID     = 1
	redisClient *redis.Client
	ctx         = context.Background()
	orderIDMutex sync.Mutex // concurrency-safe order ID
)

func loadEnv() {
	err := godotenv.Load("../.env")
	if err != nil {
		log.Println("No .env file found, using environment variables")
	}
}

func setupRedis() {
	redisURL := os.Getenv("REDIS_URL")
	opt, err := redis.ParseURL(redisURL)
	if err != nil {
		log.Fatalf("Failed to parse REDIS_URL: %v", err)
	}
	redisClient = redis.NewClient(opt)
}

func main() {
	loadEnv()
