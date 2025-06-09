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
