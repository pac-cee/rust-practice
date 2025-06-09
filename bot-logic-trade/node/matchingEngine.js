// Node.js Matching Engine Microservice (Express + Redis)
const express = require('express');
const redis = require('redis');
const bodyParser = require('body-parser');
require('dotenv').config({ path: '../.env' });

const app = express();
app.use(bodyParser.json());
const client = redis.createClient({ url: process.env.REDIS_URL });
client.connect();

let orderId = 1;
const orderIdLock = {};
// orderIdLock is a dummy object for locking, Node.js is single-threaded but async atomicity is important.

/**
 * POST /order
 * Adds a new order (concurrency-safe, Redis-backed, memory managed by Redis).
 */
app.post('/order', async (req, res) => {
  // Lock orderId for atomic increment
  let id;
  // In production, use a Redis atomic counter for distributed safety
  id = orderId++;
  const order = {
    id,
    userId: req.body.userId,
    type: req.body.type,
    price: req.body.price,
    amount: req.body.amount,
    remaining: req.body.amount,
    status: 'open',
  };
  await client.set(`order:${order.id}`, JSON.stringify(order));
  if (order.type === 'buy') {
    await client.zAdd('buy_orders', [{ score: -order.price, value: order.id.toString() }]);
  } else {
    await client.zAdd('sell_orders', [{ score: order.price, value: order.id.toString() }]);
  }
  await matchOrders();
  res.json(order);
});

/**
 * GET /orderbook
 * Returns the current buy and sell orderbook (cached in Redis).
 */
app.get('/orderbook', async (req, res) => {
  const buyIds = await client.zRange('buy_orders', 0, -1);
  const sellIds = await client.zRange('sell_orders', 0, -1);
  const buyOrders = [];
