package com.botlogictrade;

import io.github.cdimascio.dotenv.Dotenv;
import org.springframework.beans.factory.annotation.Autowired;
import org.springframework.boot.SpringApplication;
import org.springframework.boot.autoconfigure.SpringBootApplication;
import org.springframework.data.redis.core.RedisTemplate;
import org.springframework.data.redis.core.ZSetOperations;
import org.springframework.web.bind.annotation.*;

import java.util.*;

@SpringBootApplication
@RestController
public class MatchingEngineRedisService {
    private static int orderId = 1;
    private static final Object orderIdLock = new Object(); // concurrency control
    @Autowired
    private RedisTemplate<String, Object> redisTemplate;

    @PostMapping("/order")
    /**
     * Adds an order to the orderbook (Redis-backed, concurrency-safe).
     * Handles memory and cache via Redis. Event publishing stub included.
     */
    public Order addOrder(@RequestBody Order req) {
        int id;
        synchronized (orderIdLock) {
            id = orderId++;
        }
        Order order = new Order(id, req.userId, req.type, req.price, req.amount);
        String orderKey = "order:" + order.id;
        redisTemplate.opsForValue().set(orderKey, order);
        if (order.type.equals("buy")) {
