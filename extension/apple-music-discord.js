const SERVER = "http://localhost:4000/";

console.log("Apple Music Discord is working on this Page. Please Start the Server if you haven't.")

function getSongDetails() {
    let root = document.getElementsByClassName("chrome-player")[0];
    let song_root = root.children[0].firstChild.firstChild.shadowRoot;
    let info = song_root.children[1].firstChild.firstChild;
    let extra_info = info.children[1].firstChild.firstChild.firstChild.firstChild.firstChild.firstChild.firstChild;
    
    let title = info.firstChild.innerText;
    let album = extra_info.children[2].innerText; 
    let artist = extra_info.firstChild.innerText;
    let image = song_root.firstChild.firstChild.firstChild.src;
    let playing = root.shadowRoot.firstChild.firstChild.firstChild.shadowRoot.firstChild.children[1].children[1].innerText == "PAUSE\nPAUSE\nPAUSE";

    pingServer(title, album, artist, image, playing)
}

function pingServer(title, album, artist, image, playing) {
    let json = JSON.stringify({
        title: title,
        artist: artist,
        album: album,
        image: image,
        playing: playing,
    });

    console.log("Sending to " + SERVER + ": " + json);

    fetch(SERVER, {
        method: "POST",
        body: json,
        headers: {
            "Content-type": "application/json; charset=UTF-8"
        }
    });
}

setInterval(() => {
    getSongDetails()
}, 1000 * 5);
