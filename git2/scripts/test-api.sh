#!/bin/bash

# Test script for Fraud Detection API

API_URL="http://localhost:8080"

echo "🧪 Testing Fraud Detection Engine API"
echo "======================================"

# Test 1: Health Check
echo -e "\n1️⃣ Health Check"
curl -s "$API_URL/health" | jq .

# Test 2: Normal Transaction (should be approved)
echo -e "\n2️⃣ Normal Transaction"
curl -s -X POST "$API_URL/api/v1/transaction" \
  -H "Content-Type: application/json" \
  -d '{
    "user_id": "user123",
    "amount": 50.00,
    "merchant": "Amazon",
    "location": "BR",
    "device_id": "device456"
  }' | jq .

sleep 1

# Test 3: High Amount Transaction (should trigger review)
echo -e "\n3️⃣ High Amount Transaction"
curl -s -X POST "$API_URL/api/v1/transaction" \
  -H "Content-Type: application/json" \
  -d '{
    "user_id": "user123",
    "amount": 6000.00,
    "merchant": "Luxury Store",
    "location": "BR",
    "device_id": "device456"
  }' | jq .

sleep 1

# Test 4: Velocity Attack (multiple rapid transactions)
echo -e "\n4️⃣ Velocity Attack (4 transactions in quick succession)"
for i in {1..4}; do
  echo "Transaction $i:"
  curl -s -X POST "$API_URL/api/v1/transaction" \
    -H "Content-Type: application/json" \
    -d '{
      "user_id": "user999",
      "amount": 100.00,
      "merchant": "Store",
      "location": "BR",
      "device_id": "device789"
    }' | jq '.status, .risk_score, .rules_triggered'
  sleep 0.2
done

# Test 5: Unknown Device
echo -e "\n5️⃣ Unknown Device (after establishing pattern)"
# First, establish a pattern
for i in {1..3}; do
  curl -s -X POST "$API_URL/api/v1/transaction" \
    -H "Content-Type: application/json" \
    -d '{
      "user_id": "user555",
      "amount": 50.00,
      "merchant": "Store",
      "location": "BR",
      "device_id": "known_device"
    }' > /dev/null
  sleep 1
done

# Now try from unknown device
echo "Transaction from unknown device:"
curl -s -X POST "$API_URL/api/v1/transaction" \
  -H "Content-Type: application/json" \
  -d '{
    "user_id": "user555",
    "amount": 50.00,
    "merchant": "Store",
    "location": "BR",
    "device_id": "unknown_device_xyz"
  }' | jq .

echo -e "\n✅ Tests completed!"
