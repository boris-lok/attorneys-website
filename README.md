# attorneys-website

This is a simple website for showcasing attorneys.

[Ref]

- https://www.muramatsu-law-office.com/
- https://olympia-law.com/member
- https://horilaw.net/lawyers/
- https://pixabay.com/images/search/lawyer 
- https://github.com/woollysammoth/sveltekit-docker-nginx/blob/main/nginx/dev/default.conf
- http://www.cwl.com.tw/shiang-wen.html

# Library
- [Markdown](https://magidoc.js.org/svelte-plugins/marked)

# Use Local Machine to build
1. docker buildx build --platform linux/amd64,linux/arm64 --push --tag backend:latest .
2. docker save backend:latest -o backend.tar 
3. scp backend.tar user@remote_host:/path/to/destination
4. ssh user@remote_host 'docker load -i /path/to/destination/backend.tar'
5. rename the images
6. docker compose up -d
