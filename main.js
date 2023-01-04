submitUrl = document.getElementById('submitUrl')

submitUrl.addEventListener("click", (e) => {
    url = document.getElementById('urlInput')
    console.log(url)
    fetch('https://api.codetabs.com/v1/proxy/'+url.value).then((response) => response.text()).then((text) => console.log(text));

});