
fetch(
  "https://public.api.bsky.app/xrpc/app.bsky.feed.getAuthorFeed?actor=ollielynas.com&limit=100"
)
  .then((res) => res.json())
  .then((out) => {
    window.bluesky_response = out;

    update_image();

    let i = 0;
    while (i < out["feed"].length) {
        let row = document.createElement("div");
        console.log(i);
        for (let n = 0; n < 4; n += 1) {
            console.log(n);
            try {
            let link = out["feed"][i]["post"]["embed"]["images"][0]["thumb"];
            let link_elm = document.createElement("a");
            let img = document.createElement("img");
            img.src = link;
            link_elm.href="#"+i;
            console.log(link);
            img.className = "bluesky_img";
            link_elm.appendChild(img);
            row.appendChild(link_elm);
            i+= 1;
        } catch (e) {
            
        }
        }

        document.body.appendChild(row);
    }
  })
  .catch((err) => {
    console.log(err)
    fetch("./bluesky_backup.json")
      .then((res) => res.json())
      .then((out) => {
        console.log("used backup");
        window.bluesky_response = out;
        update_image(0);
      })
      .catch((err) => {});
  });

function update_image(index) {
  let out = window.bluesky_response;
  console.log(out);
}

const preloadImageBluesky = (src) =>
  new Promise((resolve, reject) => {
    const image = new Image();
    image.onload = resolve;
    image.onerror = reject;
    image.src = src;
  });
