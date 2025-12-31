# Load Testing com k6

## Instalação

```bash
# macOS
brew install k6

# Windows
choco install k6

# Linux
sudo gpg -k
sudo gpg --no-default-keyring --keyring /usr/share/keyrings/k6-archive-keyring.gpg --keyserver hkp://keyserver.ubuntu.com:80 --recv-keys C5AD17C747E3415A3642D57D77C6C491D6AC1D69
echo "deb [signed-by=/usr/share/keyrings/k6-archive-keyring.gpg] https://dl.k6.io/deb stable main" | sudo tee /etc/apt/sources.list.d/k6.list
sudo apt-get update
sudo apt-get install k6
```

## Executar Teste

```bash
# Teste básico
k6 run load-test.js

# Com mais detalhes
k6 run --out json=results.json load-test.js

# Teste de stress (mais agressivo)
k6 run --vus 2000 --duration 5m load-test.js
```

## Métricas Esperadas

- **Throughput**: > 50,000 req/s
- **Latência p95**: < 200ms
- **Taxa de erro**: < 1%

## Interpretação dos Resultados

```
✓ checks.........................: 99.9% ✓ 49950 ✗ 50
✓ http_req_duration..............: avg=45ms p(95)=87ms
✓ http_reqs......................: 52341 (52341/s)
✓ errors.........................: 0.1%
```

Isso significa:
- Sistema processou 52,341 requisições por segundo
- 95% das requisições completaram em menos de 87ms
- Taxa de erro de apenas 0.1%
