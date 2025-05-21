

fetch("https://public.api.bsky.app/xrpc/app.bsky.feed.getAuthorFeed?actor=ollielynas.com&limit=50")
.then(res => res.json())
.then(out =>
  {
      window.bluesky_response = out;
    update_image();

    setInterval(() => {
        document.getElementById("bluesky_img").style.opacity = 0;
        setTimeout(() => {
            update_image();
            
        }, 500)
    }, 10000)

  })
.catch(err => console.log(err));


function update_image() {
    let out = window.bluesky_response;
    let l = out["feed"].length;
    let i = Math.floor(Math.min((Math.random() * (l - 1)), (Math.random() * (l - 1))));



   let link = out["feed"][i]["post"]["embed"]["images"][0]["fullsize"];




   if (link !== undefined) {
    preloadImage(link).then(
        a => {
            document.getElementById("bluesky_img").src = link;
            document.getElementById("bluesky_img").style.opacity = 1;
        }
    )
   }
}



const preloadImage = src => 
  new Promise((resolve, reject) => {
    const image = new Image()
    image.onload = resolve
    image.onerror = reject
    image.src = src
  })
