### quick-link
service to collect [Open Graph](https://ogp.me/) data for a preview

### example
```bash
curl --silent -G --data-urlencode "link=https://www.hulu.com/watch/9c70c6d5-ba51-4d1a-afcc-fdae43373a9b" localhost:8000/opengraph | jq
```
```bash
open http://localhost:8000/opengraph?link=https%3A%2F%2Fwww.hulu.com%2Fwatch%2F9c70c6d5-ba51-4d1a-afcc-fdae43373a9b
```

### Reference
- https://ogp.me/
- https://www.npmjs.com/package/link-preview-js
- https://docs.rs/headless_chrome/0.9.0/headless_chrome/
- https://andrejgajdos.com/how-to-create-a-link-preview/
