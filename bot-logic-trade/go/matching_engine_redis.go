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
