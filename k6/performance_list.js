import http from 'k6/http';

export const options = {
  vus: 30,
  duration: '30s',
  thresholds: {
    http_req_failed: ['rate<0.01'], // http errors should be less than 1%
    http_req_duration: ['p(95)<50'] // 95% of requests should be below 50ms
  },
};

export default function () {
  http.get('http://localhost:8080/taxonomy');
};
