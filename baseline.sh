for path in /health /users /users/1 /users/999 /unknown; do
	curl -sS -o /dev/null -w "$path status=%{http_code} latency=%{time_total}s\n" "http://127.0.0.1:8080$path"
done
