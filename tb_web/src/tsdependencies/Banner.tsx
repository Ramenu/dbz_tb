import Unit from "./Unit";
import { getUnit } from "./Database";

enum BannerType 
{
    Null = 0x0,
    RareSummon = 0x1,
    DokkanFest = 0x2,
    FriendSummon = 0x4,
    DiscountSummon = 0x8
}

export class Banner
{
    featuredUnits: Unit[] = [];
    type: BannerType;
    image: string;
    constructor(bannerType : BannerType, featuredUnits : string[], bannerImage : string)
    {
        this.type = bannerType;
        featuredUnits.forEach((unitURL) => {
            this.featuredUnits.push(getUnit(unitURL));
        });
        this.image = bannerImage;
    }
}


export const ALL_BANNERS : Banner[] = [
    new Banner(
        BannerType.RareSummon,
        [
            "https://dbz-dokkanbattle.fandom.com/wiki/The_Saiyan_Among_Us_Goku",
            "https://dbz-dokkanbattle.fandom.com/wiki/Genius_of_War_Vegeta",
            "https://dbz-dokkanbattle.fandom.com/wiki/Stern_Teacher_Piccolo"
        ],
        "https://static.wikia.nocookie.net/dbz-dokkanbattle/images/c/c6/Banner_dokkan_1.jpg"
    )
];