import http from 'k6/http';

export const options = {
  vus: 20,
  duration: '30s',
  thresholds: {
    http_req_failed: ['rate<0.01'], // http errors should be less than 1%
    http_req_duration: ['p(95)<100'] // 95% of requests should be below 1000ms
  },
};

export default function () {
  http.get('http://localhost:8080/taxonomy/164712');
};
