### quick-link
service to collect [Open Graph](https://ogp.me/) data for a preview

### examples
#### /opengraph
```bash
curl --silent -G --data-urlencode "link=https://www.hulu.com/watch/9c70c6d5-ba51-4d1a-afcc-fdae43373a9b" localhost:8000/opengraph | jq
{
  "title": "Seinfeld",
  "type": "tv_show",
  "image": "https://img.hulu.com/user/v3/artwork/c3be4f02-5f40-4a59-8cbc-9ae9c178564c?base_image_bucket_name=image_manager&base_image=026ddc9c-6bf8-4332-888e-17e79ce78417&size=1200x630&format=jpeg",
  "url": "https://www.hulu.com/series/seinfeld-c3be4f02-5f40-4a59-8cbc-9ae9c178564c",
  "audio": "",
  "description": "In the Emmy award-winning \"Seinfeld,\" Jerry Seinfeld provides a hysterical look at life as a single adult in the '90s.",
  "determiner": "",
  "locale": "",
  "locale_alternate": "",
  "site_name": "Hulu",
  "video": ""
}
```
```bash
curl --silent -G --data-urlencode "link=https://www.youtube.com/watch?v=dZtHMvFs7GE" localhost:8000/opengraph | jq
{
  "title": "dragonball z dmx party up in here",
  "type": "video.other",
  "image": "https://i.ytimg.com/vi/dZtHMvFs7GE/hqdefault.jpg",
  "url": "https://www.youtube.com/watch?v=dZtHMvFs7GE",
  "audio": "",
  "description": "sick video",
  "determiner": "",
  "locale": "",
  "locale_alternate": "",
  "site_name": "YouTube",
  "video": ""
}
```
#### /badge
```bash
open http://localhost:8000/badge?link=https%3A%2F%2Fwww.hulu.com%2Fwatch%2F9c70c6d5-ba51-4d1a-afcc-fdae43373a9b
```
![Seinfeld](examples/images/badge-seinfeld.png)
```bash
open http://localhost:8000/badge?link=https%3A%2F%2Fwww.youtube.com%2Fwatch%3Fv%3DdZtHMvFs7GE
```
![DMX](examples/images/badge-dmx.png)

### Reference
- https://ogp.me/
- https://www.npmjs.com/package/link-preview-js
- https://docs.rs/headless_chrome/0.9.0/headless_chrome/
- https://andrejgajdos.com/how-to-create-a-link-preview/
- https://www.youtube.com/watch?v=Qhaz36TZG5Y
