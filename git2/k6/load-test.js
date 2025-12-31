import http from 'k6/http';
import { check, sleep } from 'k6';
import { Rate } from 'k6/metrics';

// Custom metrics
const errorRate = new Rate('errors');

// Test configuration
export const options = {
  stages: [
    { duration: '30s', target: 100 },   // Ramp up to 100 users
    { duration: '1m', target: 500 },    // Ramp up to 500 users
    { duration: '2m', target: 1000 },   // Ramp up to 1000 users
    { duration: '2m', target: 1000 },   // Stay at 1000 users
    { duration: '30s', target: 0 },     // Ramp down
  ],
  thresholds: {
    http_req_duration: ['p(95)<200'],   // 95% of requests must complete below 200ms
    http_req_failed: ['rate<0.01'],     // Error rate must be below 1%
    errors: ['rate<0.01'],
  },
};

// Test data generators
const users = Array.from({ length: 1000 }, (_, i) => `user${i}`);
const merchants = ['Amazon', 'Netflix', 'Uber', 'iFood', 'Mercado Livre', 'Magazine Luiza'];
const locations = ['BR', 'US', 'UK', 'AR', 'MX'];
const devices = Array.from({ length: 100 }, (_, i) => `device${i}`);

function randomElement(arr) {
  return arr[Math.floor(Math.random() * arr.length)];
}

function generateTransaction() {
  return {
    user_id: randomElement(users),
    amount: Math.random() * 1000 + 10,
    merchant: randomElement(merchants),
    location: randomElement(locations),
    device_id: randomElement(devices),
  };
}

export default function () {
  const url = 'http://localhost:8080/api/v1/transaction';
  const payload = JSON.stringify(generateTransaction());

  const params = {
    headers: {
      'Content-Type': 'application/json',
    },
  };

  const res = http.post(url, payload, params);

  // Check response
  const success = check(res, {
    'status is 200': (r) => r.status === 200,
    'response time < 200ms': (r) => r.timings.duration < 200,
    'has transaction_id': (r) => JSON.parse(r.body).transaction_id !== undefined,
  });

  errorRate.add(!success);

  // Small sleep to simulate realistic traffic
  sleep(0.1);
}

// Summary handler
export function handleSummary(data) {
  return {
    'summary.json': JSON.stringify(data),
    stdout: textSummary(data, { indent: ' ', enableColors: true }),
  };
}

function textSummary(data, options) {
  const indent = options.indent || '';
  const enableColors = options.enableColors || false;

  let summary = '\n';
  summary += `${indent}✓ checks.........................: ${data.metrics.checks.values.passes} / ${data.metrics.checks.values.fails}\n`;
  summary += `${indent}✓ http_req_duration..............: avg=${data.metrics.http_req_duration.values.avg.toFixed(2)}ms p(95)=${data.metrics.http_req_duration.values['p(95)'].toFixed(2)}ms\n`;
  summary += `${indent}✓ http_reqs......................: ${data.metrics.http_reqs.values.count} (${data.metrics.http_reqs.values.rate.toFixed(2)}/s)\n`;
  summary += `${indent}✓ errors.........................: ${(data.metrics.errors.values.rate * 100).toFixed(2)}%\n`;

  return summary;
}
