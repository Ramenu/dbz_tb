
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

    get type() {
        return this.type;
    }

    get featuredUnits() {
        return this.featuredUnits;
    }

    get image() {
        return this.image;
    }
}