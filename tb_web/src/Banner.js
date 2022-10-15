
export const BANNER_TYPE = {
    Null: 0x0, // implied as a rare summon by default
    Dokkanfest: 0x1,
    FriendSummon: 0x2
};

export class Banner
{
    constructor(bannerType, featuredUnits, bannerImage)
    {
        this.type = bannerType;
        this.featuredUnits = featuredUnits;
        this.image = bannerImage;
    }
}


export const ALL_BANNERS = [
    new Banner(
        BANNER_TYPE.Null,
        [
            "https://dbz-dokkanbattle.fandom.com/wiki/The_Saiyan_Among_Us_Goku",
            "https://dbz-dokkanbattle.fandom.com/wiki/Genius_of_War_Vegeta",
            "https://dbz-dokkanbattle.fandom.com/wiki/Stern_Teacher_Piccolo"
        ],
        "https://static.wikia.nocookie.net/dbz-dokkanbattle/images/c/c6/Banner_dokkan_1.jpg/revision/latest?cb=20180312125355"
    )
];