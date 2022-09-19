

const template = (async () => {
    const url = "https://dbz-dokkanbattle.fandom.com/wiki/All_Cards:_(1)001_to_(1)100";
    const response = await fetch(url);

    if (response.status >= 200 && response.status < 300)
    {
        const template = await response.text();

        const matches = template.match(/<td>1000020<\/td>/);
        
        console.log(matches);
        return template;
    }
    return -1;
})();

