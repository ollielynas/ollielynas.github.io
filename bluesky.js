
fetch("https://public.api.bsky.app/xrpc/app.bsky.feed.getAuthorFeed?actor=ollielynas.com&limit=50")
.then(res => res.json())
.then(out =>
  {
      window.bluesky_response = out;
    update_image();

    setInterval(() => {

            update_image();

    }, 5000)

  })
.catch(err => {
  fetch("./bluesky_backup.json")
.then(res => res.json())
.then(out =>
  {
    console.log("used backup");
      window.bluesky_response = out;
    try{update_image()}catch (e){};

    setInterval(() => {
        try {
            update_image();
          } catch (e) {
            console.log(e);
          }

    }, 3000)

  })
.catch(err => {
});
});


function update_image() {
    let out = window.bluesky_response;
    let l = out["feed"].length;


    let images = [
      document.getElementById("bluesky_img1"),
      document.getElementById("bluesky_img2"),
      document.getElementById("bluesky_img3"),
      document.getElementById("bluesky_img4"),
    ];

        let picked = [];

        let updated_images = false;

       for (let image of images) {
      if (image.src !="") {
        continue;
      }

      updated_images = true;
      
      let link = "";
      let i = 0;
      while (true) {
            i = Math.floor(Math.min((Math.random() * (l - 1)), (Math.random() * (l - 1))));
          link = out["feed"][i]["post"]["embed"]["images"][0]["fullsize"];

        let valid = true;
        for (let image2 of picked)  {
          if (image2 == link) {
            valid = false;
          }
        }
        if (valid) {
          break;
        }
      }
      if (link !== undefined) {
        preloadImageBluesky(link).then((a) => {
          image.src = link;
          image.style.opacity = 1;
                // let bsk_link = "https://bsky.app/profile/ollielynas.com/post/"+out["feed"][i]["post"]["uri"].split(".post/")[1];

          image.parentElement.href = "";
        });
      }
    }


    if (updated_images) {return;}

    
    let image = images[Math.floor(Math.min(Math.random() * 4, 3))];
    let link = "";
      let i = Math.floor(Math.min((Math.random() * (l - 1)), (Math.random() * (l - 1))));

      link = out["feed"][i]["post"]["embed"]["images"][0]["fullsize"];
      console.log(out["feed"][i]["post"]);
      let valid = true;
      for (let image2 of images) {
        if (image2.src == link && link !== undefined) {
         return
        }
      }
      


   if (link !== undefined) {
      image.style.opacity = 0;
      setTimeout(() => {
        preloadImageBluesky(link).then((a) => {
          image.src = link;
          image.style.opacity = 1;
                // let bsk_link = "https://bsky.app/profile/ollielynas.com/post/"+out["feed"][i]["post"]["uri"].split(".post/")[1];

          image.parentElement.href = "";
        });
      }, 500);
    }
}


const preloadImageBluesky = src => 
  new Promise((resolve, reject) => {
    const image = new Image()
    image.onload = resolve
    image.onerror = reject
    image.src = src
  })

