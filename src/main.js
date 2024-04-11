
window.location.replace("https://ollielynas.github.io/md-website/#md_files/home.md");


let links = document.getElementById('links');
let thing = document.querySelector(".thing");

let things = ["optimization", "teamwork", "improving", "clarity"];


let thing_phase = 3;

function update_thing() {
    if (thing_phase == 0) {
        if (thing.innerText == "") {
            thing_phase += 1;
        }else {
            things[things.length -1] += [thing.innerText[thing.innerText.length -1]]
            thing.innerText = thing.innerText.slice(0, thing.innerText.length -2)
        }
    }else if (thing_phase == 1) {
        if (things[0] == "") {
            thing_phase += 1;
        }else {
            thing.innerText += [things[0].slice(0,1)];
            things[0] = " ";
        }
    }else if (thing_phase == 20) {
        thing_phase = 0;
        things += [""];
    }else {
        thing_phase += 1;
    }
    console.log(thing_phase, things)
    
}

setInterval(() => {update_thing()}, 100);

let left = 0;

window.left = 0;

