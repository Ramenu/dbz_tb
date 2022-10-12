
import { Banner, BANNER_TYPE } from "./Banner";


export const bannerSummon = (banner) =>
{
    switch (banner.type)
    {
        default: return performRareSummon();
        case BANNER_TYPE.FriendSummon: return performFriendSummon();
        case BANNER_TYPE.Dokkanfest: return performDokkanFestSummon();
    }
}

const getFeaturedSSRUnits = (featuredUnits) =>
{
    
}

const performRareSummon = (banner) =>
{

}

const performFriendSummon = (banner) =>
{
    
}

const performDokkanFestSummon = (banner) =>
{

}

