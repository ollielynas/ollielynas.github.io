{
const links = [
    {link: "https://ollie-lynas.itch.io/unammed-pixel-game", 
        img: "https://img.itch.zone/aW1nLzE1ODAxNTg3LnBuZw==/original/%2BUtEMb.png"
    },
    {link: "https://ollielynas.com/reanimator", 
        img: "https://ollielynas.com/reanimator/res/Screenshot%202024-08-24%20000556.png"
    },
    {link: "https://ollie-lynas.itch.io/quantum-chess", 
        img: "https://img.itch.zone/aW1hZ2UvMTk4ODYwMy8xMjIwNjU4Ny5wbmc=/original/skQf1C.png"
    },
    {link: "https://activity.ollielynas.com", 
        img: "./activity_book2.png"
    }
]

    setInterval(() => {
        document.getElementById("project_img").style.opacity = 0;
        setTimeout(() => {
            update_image();
            
        }, 500)
    }, 5000)



function update_image() {

    let i = Math.min(Math.floor(Math.random() * (links.length)), links.length - 1);
    console.log(i, links)

    let link = links[i]["img"];


   if (link !== undefined) {
    preloadImageItchIo(link).then(
        a => {
            document.getElementById("project_img").src = link;
            document.getElementById("project_img").style.opacity = 1;
        }
    )
   }
}
const preloadImageItchIo = src => 
  new Promise((resolve, reject) => {
    const image = new Image()
    image.onload = resolve
    image.onerror = reject
    image.src = src
  })
}



update_image();

