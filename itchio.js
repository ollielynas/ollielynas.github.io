{
  const links = [
    {
      link: "https://ollie-lynas.itch.io/unammed-pixel-game",
      img: "https://img.itch.zone/aW1nLzE1ODAxNTg3LnBuZw==/original/%2BUtEMb.png",
    },
    {
      link: "https://ollielynas.com/reanimator",
      img: "https://ollielynas.com/reanimator/res/Screenshot%202024-08-24%20000556.png",
    },
    {
      link: "https://ollie-lynas.itch.io/quantum-chess",
      img: "https://img.itch.zone/aW1hZ2UvMTk4ODYwMy8xMjIwNjU4Ny5wbmc=/original/skQf1C.png",
    },
    { link: "https://activity.ollielynas.com", img: "./activity_book2.png" },
    { link: "https://ollie-lynas.itch.io/casio-basic-visual", img: "https://img.itch.zone/aW1hZ2UvMjA3NjIzNS8xMjIxMjcyMS5wbmc=/794x1000/qdV9yq.png" },
  ];
  
  setInterval(() => {
    update_image_itch();
  }, 7000);

  function update_image_itch() {
    let images = [
      document.getElementById("project_img1"),
      document.getElementById("project_img2"),
      document.getElementById("project_img3"),
      document.getElementById("project_img4"),
    ];

    let picked = [];

    let updated_images = false;

    for (let image of images) {
      if (image.src !="") {
        continue;
      }
      updated_images = true;
      
      let img_link = "";
      let click_link = "";
      while (true) {
        let i = Math.min(
          Math.floor(Math.random() * links.length),
          links.length - 1
        );
        img_link = links[i]["img"];
        click_link = links[i]["link"];
        let valid = true;
        for (let pick of picked) {
          if (pick == img_link) {
            valid = false;
          }
        }
        if (valid) {
          break;
        }
      }
      if (img_link !== undefined) {
          picked.push(img_link);
        preloadImageItchIo(img_link).then((a) => {
          image.src = img_link;
          image.style.opacity = 1;
          image.parentElement.href = click_link; 
        });
      }
    }

    if (updated_images) {return}

    let image = images[Math.floor(Math.min(Math.random() * 4, 3))];
    let image_link = "";
      let i = Math.min(
        Math.floor(Math.random() * links.length),
        links.length - 1
      );
      image_link = links[i]["img"];
      let valid = true;
      for (let image2 of images) {
        
        if (image2.src == image_link) {
          valid = false;
          return;
        }
      }


    if (image_link !== undefined) {
      image.style.opacity = 0;
      setTimeout(() => {
        preloadImageItchIo(image_link).then((a) => {
          image.src = image_link;
          image.style.opacity = 1;
        });
      }, 500);
    }
  }
  const preloadImageItchIo = (src) =>
    new Promise((resolve, reject) => {
      const image = new Image();
      image.onload = resolve;
      image.onerror = reject;
      image.src = src;
    });
}

update_image_itch();
window.update_image_itch